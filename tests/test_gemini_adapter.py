from __future__ import annotations

import os

import pytest

from agentic_harness.plugins.gemini import GeminiModel, raise_scrubbed
from agentic_harness.sanitize import safe_model_error


def test_raise_scrubbed_redacts_google_keys():
    aiza = "AIzaSyDummyKeyValue123456789012"
    aq = "AQ." + "y" * 24
    with pytest.raises(RuntimeError) as ei:
        raise_scrubbed(RuntimeError(f"upstream leak {aiza} and {aq}"))
    msg = str(ei.value)
    assert aiza not in msg
    assert aq not in msg
    assert "MODEL_FAILED" in msg
    assert "[REDACTED]" in msg


def test_safe_model_error_helper():
    err = safe_model_error(ValueError("token AIzaSyDummyKeyValue123456789012"))
    assert "AIzaSyDummyKeyValue123456789012" not in str(err)


def test_gemini_model_repr_hides_key(monkeypatch):
    monkeypatch.setenv("GEMINI_API_KEY", "AQ." + "z" * 24)
    # Clear GOOGLE if set
    monkeypatch.delenv("GOOGLE_API_KEY", raising=False)
    m = GeminiModel(model="gemini-flash-latest")
    dumped = repr(m)
    assert "AQ." not in dumped
    assert "api_key" not in dumped.lower() or "_api_key" not in dumped
