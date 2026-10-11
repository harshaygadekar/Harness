//! OpenAI-compatible chat completions (non-stream first).
//! Env: HARNESS_API_KEY | OPENAI_API_KEY, HARNESS_BASE_URL, HARNESS_MODEL.
//! Streaming: TODO — wire SSE when non-stream path is solid. (Non-stream only,
//! so token usage is read from each response body's `usage`.)

use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

use harness_core::{Message, ModelClient, ModelResponse, Role, ToolCall, ToolSpec};
use serde::{Deserialize, Serialize};
use thiserror::Error;

const DEFAULT_BASE: &str = "https://api.openai.com/v1";
/// Fixed UA: some provider edges (Cloudflare) reject unknown/missing agents.
pub const USER_AGENT: &str = concat!("harness/", env!("CARGO_PKG_VERSION"));
const DEFAULT_TIMEOUT_SECS: u64 = 60;
/// Bounded provider retry (outside core): max retries per request.
pub const MAX_RETRIES: u32 = 3;
const DEFAULT_BACKOFF_BASE: Duration = Duration::from_secs(2);
/// Cap on total added wait across retries of a single request.
const MAX_TOTAL_RETRY_WAIT: Duration = Duration::from_secs(30);

/// Retry policy knobs (tests shrink the backoff base).
#[derive(Clone, Debug)]
pub struct RetryPolicy {
    pub max_retries: u32,
    pub backoff_base: Duration,
    pub max_total_wait: Duration,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_retries: MAX_RETRIES,
            backoff_base: DEFAULT_BACKOFF_BASE,
            max_total_wait: MAX_TOTAL_RETRY_WAIT,
        }
    }
}

fn is_retryable_status(code: u16) -> bool {
    matches!(code, 429 | 500 | 502 | 503 | 504)
}

/// Small jitter (0..=25% of d) without pulling a rand dependency.
fn jitter(d: Duration) -> Duration {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|t| t.subsec_nanos())
        .unwrap_or(0) as u64;
    let max = d.as_millis() as u64 / 4;
    if max == 0 {
        return d;
    }
    d + Duration::from_millis(nanos % (max + 1))
}

fn parse_retry_after(v: Option<&str>) -> Option<Duration> {
    v.and_then(|s| s.trim().parse::<u64>().ok()).map(Duration::from_secs)
}

enum Attempt {
    Done(Result<ModelResponse, OpenAiError>),
    Retry(OpenAiError, Option<Duration>),
}

#[derive(Debug, Error)]
pub enum OpenAiError {
    #[error("missing API key (set HARNESS_API_KEY or OPENAI_API_KEY)")]
    MissingKey,
    #[error("missing model (set HARNESS_MODEL)")]
    MissingModel,
    #[error("http: {0}")]
    Http(String),
    #[error("parse: {0}")]
    Parse(String),
}

#[derive(Clone)]
pub struct OpenAiCompat {
    api_key: String,
    base_url: String,
    model: String,
    timeout: Duration,
    retry: RetryPolicy,
    retries: Arc<AtomicU32>,
    retry_wait_ms: Arc<AtomicU64>,
    usage: Arc<std::sync::Mutex<UsageTotals>>,
}

/// Summed provider token usage over a run (measurement only). Each field is
/// None until at least one response reported it; never defaults to 0.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct UsageTotals {
    pub prompt: Option<u64>,
    pub completion: Option<u64>,
    pub total: Option<u64>,
    pub reasoning: Option<u64>,
}

fn add_opt(a: &mut Option<u64>, b: Option<u64>) {
    if let Some(b) = b {
        *a = Some(a.unwrap_or(0) + b);
    }
}

impl UsageTotals {
    fn add(&mut self, u: &OaiUsage) {
        add_opt(&mut self.prompt, u.prompt_tokens);
        add_opt(&mut self.completion, u.completion_tokens);
        add_opt(&mut self.total, u.total_tokens);
        let r = u
            .completion_tokens_details
            .as_ref()
            .and_then(|d| d.reasoning_tokens)
            .or(u.reasoning_tokens);
        add_opt(&mut self.reasoning, r);
    }
}

impl std::fmt::Debug for OpenAiCompat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Jessie: never print the API key.
        f.debug_struct("OpenAiCompat")
            .field("api_key", &"[redacted]")
            .field("base_url", &self.base_url)
            .field("model", &self.model)
            .field("timeout", &self.timeout)
            .field("retries", &self.retries.load(Ordering::Relaxed))
            .field("retry_wait_ms", &self.retry_wait_ms.load(Ordering::Relaxed))
            .finish()
    }
}


fn scrub_http_body(s: &str) -> String {
    // Jessie: truncate + redact common key prefixes from provider error bodies.
    // Redacts the WHOLE token (all key-ish chars) for every occurrence.
    let out: String = s.chars().take(400).collect();
    redact_keys(&out)
}

fn redact_keys(s: &str) -> String {
    const PATS: [&str; 4] = ["csk-", "AIza", "sk-", "Bearer "];
    let is_key = |c: char| c.is_ascii_alphanumeric() || c == '-' || c == '_' || c == '.';
    let mut out = String::with_capacity(s.len());
    let mut i = 0;
    let mut prev_is_key = false;
    'outer: while i < s.len() {
        // Only match a prefix at a token boundary: start of string, or the
        // previous char is not a key character (ASCII alnum / - _ .).
        if !prev_is_key {
            for pat in PATS {
                if s[i..].starts_with(pat) {
                    let mut j = i + pat.len();
                    while let Some(c) = s[j..].chars().next() {
                        if !is_key(c) { break; }
                        j += c.len_utf8();
                    }
                    out.push_str(pat);
                    out.push_str("[redacted]");
                    // Last consumed input char determines the next boundary.
                    prev_is_key = s[..j].chars().last().map(is_key).unwrap_or(false);
                    i = j;
                    continue 'outer;
                }
            }
        }
        let c = s[i..].chars().next().unwrap();
        out.push(c);
        prev_is_key = is_key(c);
        i += c.len_utf8();
    }
    out
}

impl OpenAiCompat {
    pub fn from_env() -> Result<Self, OpenAiError> {
        let api_key = std::env::var("HARNESS_API_KEY")
            .or_else(|_| std::env::var("OPENAI_API_KEY"))
            .map_err(|_| OpenAiError::MissingKey)?;
        let base_url = std::env::var("HARNESS_BASE_URL").unwrap_or_else(|_| DEFAULT_BASE.into());
        let model = std::env::var("HARNESS_MODEL").map_err(|_| OpenAiError::MissingModel)?;
        Ok(Self {
            api_key,
            base_url: base_url.trim_end_matches('/').to_string(),
            model,
            timeout: Duration::from_secs(DEFAULT_TIMEOUT_SECS),
            retry: RetryPolicy::default(),
            retries: Arc::new(AtomicU32::new(0)),
            retry_wait_ms: Arc::new(AtomicU64::new(0)),
            usage: Arc::default(),
        })
    }

    pub fn new(api_key: impl Into<String>, base_url: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            base_url: base_url.into().trim_end_matches('/').to_string(),
            model: model.into(),
            timeout: Duration::from_secs(DEFAULT_TIMEOUT_SECS),
            retry: RetryPolicy::default(),
            retries: Arc::new(AtomicU32::new(0)),
            retry_wait_ms: Arc::new(AtomicU64::new(0)),
            usage: Arc::default(),
        }
    }

    pub fn with_retry_policy(mut self, retry: RetryPolicy) -> Self {
        self.retry = retry;
        self
    }

    /// Total provider retries performed by this client (all requests).
    pub fn retry_count(&self) -> u32 {
        self.retries.load(Ordering::Relaxed)
    }

    /// Shared counter handle (survives moves of the client).
    pub fn retry_counter(&self) -> Arc<AtomicU32> {
        self.retries.clone()
    }

    /// Total measured backoff sleep (ms) across all retries of this client.
    pub fn retry_wait_ms(&self) -> u64 {
        self.retry_wait_ms.load(Ordering::Relaxed)
    }

    /// Summed token usage reported by the provider across all responses.
    pub fn usage_totals(&self) -> UsageTotals {
        self.usage.lock().map(|u| u.clone()).unwrap_or_default()
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    fn complete_inner(
        &self,
        messages: &[Message],
        tools: &[ToolSpec],
    ) -> Result<ModelResponse, OpenAiError> {
        let body = ChatRequest {
            model: self.model.clone(),
            messages: messages.iter().map(to_openai_message).collect(),
            tools: if tools.is_empty() {
                None
            } else {
                Some(tools.iter().map(to_openai_tool).collect())
            },
            stream: Some(false),
        };

        let url = format!("{}/chat/completions", self.base_url);
        let agent = ureq::AgentBuilder::new()
            .timeout_connect(Duration::from_secs(10))
            .timeout_read(self.timeout)
            .build();

        let mut attempt: u32 = 0;
        let mut waited = Duration::ZERO;
        loop {
            match self.attempt_once(&agent, &url, &body) {
                Attempt::Done(r) => return r,
                Attempt::Retry(err, retry_after) => {
                    if attempt >= self.retry.max_retries {
                        return Err(err);
                    }
                    let backoff = jitter(self.retry.backoff_base * 2u32.pow(attempt));
                    let mut wait = retry_after.unwrap_or(backoff);
                    let remaining = self.retry.max_total_wait.saturating_sub(waited);
                    if remaining.is_zero() {
                        return Err(err);
                    }
                    if wait > remaining {
                        wait = remaining;
                    }
                    // Log only the scrubbed error class; never the key.
                    eprintln!(
                        "model retry {}/{} after {} ms: {}",
                        attempt + 1,
                        self.retry.max_retries,
                        wait.as_millis(),
                        err.to_string().lines().next().unwrap_or("")
                    );
                    let t0 = std::time::Instant::now();
                    std::thread::sleep(wait);
                    self.retry_wait_ms
                        .fetch_add(t0.elapsed().as_millis() as u64, Ordering::Relaxed);
                    waited += wait;
                    attempt += 1;
                    self.retries.fetch_add(1, Ordering::Relaxed);
                }
            }
        }
    }

    fn attempt_once(&self, agent: &ureq::Agent, url: &str, body: &ChatRequest) -> Attempt {
        let resp = agent
            .post(url)
            .set("Authorization", &format!("Bearer {}", self.api_key))
            .set("Content-Type", "application/json")
            .set("User-Agent", USER_AGENT)
            .send_json(body);

        let resp = match resp {
            Ok(r) => r,
            Err(ureq::Error::Status(code, r)) => {
                let retry_after = parse_retry_after(r.header("retry-after"));
                let body = r.into_string().unwrap_or_default();
                let body = scrub_http_body(&body);
                let err = OpenAiError::Http(format!("status {code}: {body}"));
                return if is_retryable_status(code) {
                    Attempt::Retry(err, retry_after)
                } else {
                    Attempt::Done(Err(err))
                };
            }
            // Transport: connect error, DNS, timeout, reset — all retryable.
            Err(e) => return Attempt::Retry(OpenAiError::Http(e.to_string()), None),
        };

        // Read timeouts can also surface while reading the body.
        let text = match resp.into_string() {
            Ok(t) => t,
            Err(e) => return Attempt::Retry(OpenAiError::Http(format!("read body: {e}")), None),
        };
        if let Some(u) = parse_usage(&text) {
            if let Ok(mut t) = self.usage.lock() {
                t.add(&u);
            }
        }
        Attempt::Done(parse_chat_response(&text))
    }
}

fn parse_usage(text: &str) -> Option<OaiUsage> {
    #[derive(Deserialize)]
    struct U {
        usage: Option<OaiUsage>,
    }
    serde_json::from_str::<U>(text).ok().and_then(|u| u.usage)
}

#[derive(Deserialize, Default)]
struct OaiUsage {
    prompt_tokens: Option<u64>,
    completion_tokens: Option<u64>,
    total_tokens: Option<u64>,
    reasoning_tokens: Option<u64>,
    completion_tokens_details: Option<OaiCompletionDetails>,
}

#[derive(Deserialize)]
struct OaiCompletionDetails {
    reasoning_tokens: Option<u64>,
}

fn parse_chat_response(text: &str) -> Result<ModelResponse, OpenAiError> {
    {

        let parsed: ChatResponse =
            serde_json::from_str(text).map_err(|e| OpenAiError::Parse(e.to_string()))?;

        let choice = parsed
            .choices
            .into_iter()
            .next()
            .ok_or_else(|| OpenAiError::Parse("no choices".into()))?;

        Ok(ModelResponse {
            message: from_openai_message(choice.message),
        })
    }
}

impl ModelClient for OpenAiCompat {
    fn complete(&mut self, messages: &[Message], tools: &[ToolSpec]) -> ModelResponse {
        match self.complete_inner(messages, tools) {
            Ok(r) => r,
            Err(e) => ModelResponse {
                message: Message {
                    role: Role::Assistant,
                    content: Some(format!("MODEL_ERROR: {e}")),
                    tool_calls: vec![],
                    tool_call_id: None,
                    name: None,
                },
            },
        }
    }
}

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<OaiMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<Vec<OaiTool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<bool>,
}

#[derive(Serialize, Deserialize)]
struct OaiMessage {
    role: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    tool_calls: Option<Vec<OaiToolCall>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    tool_call_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct OaiToolCall {
    id: String,
    #[serde(rename = "type")]
    type_: String,
    function: OaiFunctionCall,
    /// Gemini (and some providers) require this on subsequent turns.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    extra_content: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize)]
struct OaiFunctionCall {
    name: String,
    arguments: String,
}

#[derive(Serialize)]
struct OaiTool {
    #[serde(rename = "type")]
    type_: String,
    function: OaiFunctionDef,
}

#[derive(Serialize)]
struct OaiFunctionDef {
    name: String,
    description: String,
    parameters: serde_json::Value,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: OaiMessage,
}

fn to_openai_tool(t: &ToolSpec) -> OaiTool {
    OaiTool {
        type_: "function".into(),
        function: OaiFunctionDef {
            name: t.name.clone(),
            description: t.description.clone(),
            parameters: t.parameters.clone(),
        },
    }
}

fn to_openai_message(m: &Message) -> OaiMessage {
    let role = match m.role {
        Role::System => "system",
        Role::User => "user",
        Role::Assistant => "assistant",
        Role::Tool => "tool",
    };
    let tool_calls = if m.tool_calls.is_empty() {
        None
    } else {
        Some(
            m.tool_calls
                .iter()
                .map(|c| OaiToolCall {
                    id: c.id.clone(),
                    type_: "function".into(),
                    function: OaiFunctionCall {
                        name: c.name.clone(),
                        arguments: c.arguments.to_string(),
                    },
                    extra_content: c.extra_content.clone(),
                })
                .collect(),
        )
    };
    OaiMessage {
        role: role.into(),
        content: m.content.clone(),
        tool_calls,
        tool_call_id: m.tool_call_id.clone(),
        name: m.name.clone(),
    }
}

fn from_openai_message(m: OaiMessage) -> Message {
    let role = match m.role.as_str() {
        "system" => Role::System,
        "user" => Role::User,
        "tool" => Role::Tool,
        _ => Role::Assistant,
    };
    let tool_calls = m
        .tool_calls
        .unwrap_or_default()
        .into_iter()
        .map(|c| {
            let arguments = serde_json::from_str(&c.function.arguments)
                .unwrap_or_else(|_| serde_json::json!({"_raw": c.function.arguments}));
            ToolCall {
                id: c.id,
                name: c.function.name,
                arguments,
                extra_content: c.extra_content,
            }
        })
        .collect();
    Message {
        role,
        content: m.content,
        tool_calls,
        tool_call_id: m.tool_call_id,
        name: m.name,
    }
}

#[cfg(test)]
mod retry_tests {
    use super::*;
    use std::io::{BufRead, BufReader, Read, Write};
    use std::net::TcpListener;
    use std::sync::Mutex;

    const OK_BODY: &str = r#"{"choices":[{"message":{"role":"assistant","content":"hi"}}]}"#;

    /// Serves canned (status, body) responses in order; returns base URL + hit counter.
    fn serve(responses: Vec<(u16, &'static str)>) -> (String, Arc<Mutex<u32>>) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        let hits = Arc::new(Mutex::new(0u32));
        let h = hits.clone();
        std::thread::spawn(move || {
            for (status, body) in responses {
                let Ok((mut s, _)) = listener.accept() else { return };
                let mut r = BufReader::new(s.try_clone().unwrap());
                let mut len = 0usize;
                loop {
                    let mut line = String::new();
                    r.read_line(&mut line).unwrap();
                    if line == "\r\n" || line.is_empty() { break; }
                    let l = line.to_ascii_lowercase();
                    if let Some(v) = l.strip_prefix("content-length:") {
                        len = v.trim().parse().unwrap();
                    }
                }
                let mut buf = vec![0u8; len];
                r.read_exact(&mut buf).unwrap();
                *h.lock().unwrap() += 1;
                let resp = format!(
                    "HTTP/1.1 {status} X\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                    body.len()
                );
                s.write_all(resp.as_bytes()).unwrap();
            }
        });
        (format!("http://{addr}"), hits)
    }

    fn client(base: &str) -> OpenAiCompat {
        OpenAiCompat::new("test-key", base, "m").with_retry_policy(RetryPolicy {
            max_retries: 3,
            backoff_base: Duration::from_millis(5),
            max_total_wait: Duration::from_secs(5),
        })
    }

    #[test]
    fn retries_on_503_then_succeeds() {
        let (base, hits) = serve(vec![(503, r#"{"error":"busy"}"#), (200, OK_BODY)]);
        let c = client(&base);
        let r = c.complete_inner(&[], &[]).expect("should succeed after retry");
        assert_eq!(r.message.content.as_deref(), Some("hi"));
        assert_eq!(*hits.lock().unwrap(), 2);
        assert_eq!(c.retry_count(), 1);
    }

    #[test]
    fn scrub_redacts_full_csk_key_everywhere() {
        let fake = format!("csk-{}", "x1y2z3".repeat(8));
        let body = format!("{{\"error\":\"invalid key {fake}\",\"echo\":\"Bearer {fake}\"}}");
        let out = scrub_http_body(&body);
        assert!(!out.contains("x1y2z3"), "leaked: {out}");
        assert!(out.contains("csk-[redacted]"));
        assert!(out.contains("Bearer [redacted]"));
    }

    #[test]
    fn scrub_skips_embedded_sk_in_ordinary_words() {
        for word in ["task-list", "disk-usage", "risk-level"] {
            let out = scrub_http_body(word);
            assert_eq!(out, word, "ordinary word was redacted: {out}");
        }
    }

    #[test]
    fn scrub_redacts_keys_at_token_boundaries() {
        let csk = format!("csk-{}", "fakeval".repeat(4));
        let sk = format!("sk-{}", "fakeval".repeat(4));
        let bearer = format!("tok-{}", "fakeval".repeat(4));

        let a = scrub_http_body(&format!("key={csk}"));
        assert!(!a.contains("fakeval"), "leaked: {a}");
        assert!(a.contains("csk-[redacted]"), "{a}");

        let b = scrub_http_body(&format!("({sk})"));
        assert!(!b.contains("fakeval"), "leaked: {b}");
        assert!(b.contains("sk-[redacted]"), "{b}");

        let c = scrub_http_body(&format!("Authorization: Bearer {bearer}"));
        assert!(!c.contains("fakeval"), "leaked: {c}");
        assert!(c.contains("Bearer [redacted]"), "{c}");
    }

    #[test]
    fn no_retry_on_400() {
        let (base, hits) = serve(vec![(400, r#"{"error":"bad sk-abcdefghijkl"}"#), (200, OK_BODY)]);
        let c = client(&base);
        let e = c.complete_inner(&[], &[]).unwrap_err().to_string();
        assert!(e.contains("status 400"));
        assert!(!e.contains("abcdefgh"), "scrub kept: {e}");
        assert_eq!(*hits.lock().unwrap(), 1);
        assert_eq!(c.retry_count(), 0);
    }

    #[test]
    fn gives_up_after_max_retries() {
        let (base, hits) = serve(vec![(503, "{}"), (502, "{}"), (429, "{}"), (500, "{}"), (200, OK_BODY)]);
        let c = client(&base);
        let e = c.complete_inner(&[], &[]).unwrap_err().to_string();
        assert!(e.contains("status 500"), "{e}");
        assert_eq!(*hits.lock().unwrap(), 4);
        assert_eq!(c.retry_count(), 3);
    }

    #[test]
    fn retries_on_connection_error() {
        // Bind then drop: port refuses connections.
        let addr = TcpListener::bind("127.0.0.1:0").unwrap().local_addr().unwrap();
        let c = client(&format!("http://{addr}"));
        assert!(c.complete_inner(&[], &[]).is_err());
        assert_eq!(c.retry_count(), 3);
    }

    /// Accepts each connection and reads the full request; for the first
    /// `stalls` connections it then sleeps past the client read timeout
    /// before closing without answering. Later connections get 200 OK.
    fn serve_stalling(stalls: u32, total: u32, stall: Duration) -> (String, Arc<Mutex<u32>>) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        let hits = Arc::new(Mutex::new(0u32));
        let h = hits.clone();
        std::thread::spawn(move || {
            for i in 0..total {
                let Ok((mut s, _)) = listener.accept() else { return };
                let mut r = BufReader::new(s.try_clone().unwrap());
                let mut len = 0usize;
                loop {
                    let mut line = String::new();
                    if r.read_line(&mut line).unwrap_or(0) == 0 || line == "\r\n" { break; }
                    if let Some(v) = line.to_ascii_lowercase().strip_prefix("content-length:") {
                        len = v.trim().parse().unwrap();
                    }
                }
                let mut buf = vec![0u8; len];
                let _ = r.read_exact(&mut buf);
                *h.lock().unwrap() += 1;
                if i < stalls {
                    // Hold the socket open past the client read timeout, then drop.
                    std::thread::spawn(move || {
                        std::thread::sleep(stall);
                        drop(s);
                    });
                    continue;
                }
                let resp = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{OK_BODY}",
                    OK_BODY.len()
                );
                let _ = s.write_all(resp.as_bytes());
            }
        });
        (format!("http://{addr}"), hits)
    }

    #[test]
    fn read_timeout_retries_then_succeeds() {
        let (base, hits) = serve_stalling(2, 3, Duration::from_secs(3));
        let c = client(&base).with_timeout(Duration::from_millis(300));
        let t = std::time::Instant::now();
        let r = c.complete_inner(&[], &[]).expect("should succeed after read timeouts");
        assert_eq!(r.message.content.as_deref(), Some("hi"));
        assert_eq!(*hits.lock().unwrap(), 3);
        assert_eq!(c.retry_count(), 2);
        assert!(t.elapsed() < Duration::from_secs(10));
    }

    #[test]
    fn read_timeout_gives_up_after_max() {
        let (base, hits) = serve_stalling(10, 10, Duration::from_secs(3));
        let c = client(&base).with_timeout(Duration::from_millis(300));
        let t = std::time::Instant::now();
        let e = c.complete_inner(&[], &[]).unwrap_err().to_string();
        assert!(e.contains("http"), "{e}");
        assert_eq!(*hits.lock().unwrap(), 4);
        assert_eq!(c.retry_count(), 3);
        assert!(t.elapsed() < Duration::from_secs(10));
    }

    #[test]
    fn retry_wait_is_recorded() {
        let (base, _hits) = serve(vec![(503, "{}"), (503, "{}"), (200, OK_BODY)]);
        let c = OpenAiCompat::new("test-key", &base, "m").with_retry_policy(RetryPolicy {
            max_retries: 3,
            backoff_base: Duration::from_millis(50),
            max_total_wait: Duration::from_secs(5),
        });
        assert_eq!(c.retry_wait_ms(), 0);
        c.complete_inner(&[], &[]).unwrap();
        assert_eq!(c.retry_count(), 2);
        // Backoff 50ms + 100ms (plus <=25% jitter each).
        let w = c.retry_wait_ms();
        assert!((150..1000).contains(&w), "retry_wait_ms={w}");
    }

    /// Like `serve` but captures request headers (lowercased) per hit.
    fn serve_capture(responses: Vec<&'static str>) -> (String, Arc<Mutex<Vec<String>>>) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        let heads = Arc::new(Mutex::new(Vec::new()));
        let h = heads.clone();
        std::thread::spawn(move || {
            for body in responses {
                let Ok((mut s, _)) = listener.accept() else { return };
                let mut r = BufReader::new(s.try_clone().unwrap());
                let mut len = 0usize;
                let mut head = String::new();
                loop {
                    let mut line = String::new();
                    r.read_line(&mut line).unwrap();
                    if line == "\r\n" || line.is_empty() { break; }
                    let l = line.to_ascii_lowercase();
                    if let Some(v) = l.strip_prefix("content-length:") {
                        len = v.trim().parse().unwrap();
                    }
                    head.push_str(&l);
                }
                let mut buf = vec![0u8; len];
                r.read_exact(&mut buf).unwrap();
                h.lock().unwrap().push(head);
                let resp = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                    body.len()
                );
                s.write_all(resp.as_bytes()).unwrap();
            }
        });
        (format!("http://{addr}"), heads)
    }

    #[test]
    fn usage_summed_across_responses() {
        let a = r#"{"choices":[{"message":{"role":"assistant","content":"a"}}],"usage":{"prompt_tokens":100,"completion_tokens":20,"total_tokens":120,"completion_tokens_details":{"reasoning_tokens":5}}}"#;
        let b = r#"{"choices":[{"message":{"role":"assistant","content":"b"}}],"usage":{"prompt_tokens":150,"completion_tokens":30,"total_tokens":180,"completion_tokens_details":{"reasoning_tokens":7}}}"#;
        let (base, _) = serve_capture(vec![a, b]);
        let c = client(&base);
        c.complete_inner(&[], &[]).unwrap();
        c.complete_inner(&[], &[]).unwrap();
        assert_eq!(
            c.usage_totals(),
            UsageTotals { prompt: Some(250), completion: Some(50), total: Some(300), reasoning: Some(12) }
        );
    }

    #[test]
    fn usage_missing_stays_none() {
        let (base, _) = serve_capture(vec![OK_BODY]);
        let c = client(&base);
        c.complete_inner(&[], &[]).unwrap();
        assert_eq!(c.usage_totals(), UsageTotals::default());
        assert!(c.usage_totals().prompt.is_none());
    }

    #[test]
    fn usage_without_reasoning_leaves_reasoning_none() {
        let a = r#"{"choices":[{"message":{"role":"assistant","content":"a"}}],"usage":{"prompt_tokens":10,"completion_tokens":2,"total_tokens":12}}"#;
        let (base, _) = serve_capture(vec![a]);
        let c = client(&base);
        c.complete_inner(&[], &[]).unwrap();
        let u = c.usage_totals();
        assert_eq!(u.prompt, Some(10));
        assert_eq!(u.reasoning, None);
    }

    #[test]
    fn sends_fixed_user_agent() {
        let (base, heads) = serve_capture(vec![OK_BODY]);
        client(&base).complete_inner(&[], &[]).unwrap();
        let h = heads.lock().unwrap()[0].clone();
        let want = format!("user-agent: harness/{}", env!("CARGO_PKG_VERSION"));
        assert!(h.contains(&want), "headers: {h}");
    }

    #[test]
    fn retry_after_header_parsed() {
        assert_eq!(parse_retry_after(Some("7")), Some(Duration::from_secs(7)));
        assert_eq!(parse_retry_after(Some("Wed, 21 Oct")), None);
        assert!(!is_retryable_status(404));
        assert!(is_retryable_status(504));
    }
}

// TODO(stream): add `complete_stream` with SSE reader when product needs TTFB display.
