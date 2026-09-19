# Contributing

Repo: https://github.com/harshaygadekar/Harness

## Setup

```bash
git clone https://github.com/harshaygadekar/Harness.git
cd Harness
python -m venv .venv
source .venv/bin/activate
pip install -e ".[dev]"
```

Optional Gemini adapter:

```bash
pip install -e ".[gemini]"
cp .env.example .env
# put GEMINI_API_KEY in .env (never commit .env)
```

## Branch workflow

1. Update `main`: `git checkout main && git pull`
2. Branch: `git checkout -b feat/<short-name>` (or `fix/`, `docs/`)
3. Commit small, clear changes
4. Push: `git push -u origin HEAD`
5. Open a pull request into `main`

Do not commit secrets, `.env`, or API keys.
