"""Pytest configuration for integration tests."""

import asyncio
import inspect
from pathlib import Path
import os
import time

import pytest


_REQUEST_METHOD_PREFIXES = ("list_", "get_", "fetch_page", "follow_link")


def _load_dotenv() -> None:
    """Load key=value pairs from the repository .env file if it exists."""
    env_path = Path(__file__).resolve().parent.parent / ".env"
    if not env_path.exists():
        return

    for raw_line in env_path.read_text(encoding="utf-8").splitlines():
        line = raw_line.strip()
        if not line or line.startswith("#") or "=" not in line:
            continue

        key, value = line.split("=", 1)
        key = key.strip()
        value = value.strip().strip("\"'")
        os.environ.setdefault(key, value)


_load_dotenv()


def _is_request_method(name: str) -> bool:
    return name.startswith(_REQUEST_METHOD_PREFIXES)


def _freeze(value):
    if isinstance(value, (str, int, float, bool, type(None))):
        return value
    if isinstance(value, dict):
        return tuple(sorted((key, _freeze(item)) for key, item in value.items()))
    if isinstance(value, (list, tuple)):
        return tuple(_freeze(item) for item in value)
    if isinstance(value, set):
        return tuple(sorted(_freeze(item) for item in value))

    url = getattr(value, "url", None)
    if url is not None:
        return ("url", url)

    urls = getattr(value, "urls", None)
    if urls is not None:
        return ("urls", tuple(urls))

    return repr(value)


def _request_key(name: str, args: tuple, kwargs: dict) -> tuple:
    return (name, _freeze(args), _freeze(kwargs))


class _LiveRequestPacing:
    """Session-wide pacing and caching for live API requests."""

    def __init__(self, min_delay_seconds: float) -> None:
        self.min_delay_seconds = min_delay_seconds
        self.last_request_at: float | None = None
        self.sync_cache: dict[tuple, object] = {}
        self.async_cache: dict[tuple, object] = {}

    def wait_for_sync_slot(self) -> None:
        if self.min_delay_seconds <= 0:
            self.last_request_at = time.monotonic()
            return

        now = time.monotonic()
        if self.last_request_at is not None:
            elapsed = now - self.last_request_at
            if elapsed < self.min_delay_seconds:
                time.sleep(self.min_delay_seconds - elapsed)
        self.last_request_at = time.monotonic()

    async def wait_for_async_slot(self) -> None:
        if self.min_delay_seconds <= 0:
            self.last_request_at = time.monotonic()
            return

        now = time.monotonic()
        if self.last_request_at is not None:
            elapsed = now - self.last_request_at
            if elapsed < self.min_delay_seconds:
                await asyncio.sleep(self.min_delay_seconds - elapsed)
        self.last_request_at = time.monotonic()


class CachedIntegrationClient:
    """Wrap the sync client to cache identical live requests for the test session."""

    def __init__(self, client, pacing: _LiveRequestPacing):
        self._client = client
        self._pacing = pacing

    def __getattr__(self, name):
        attr = getattr(self._client, name)
        if not callable(attr) or not _is_request_method(name):
            return attr

        def call(*args, **kwargs):
            key = _request_key(name, args, kwargs)
            if key not in self._pacing.sync_cache:
                self._pacing.wait_for_sync_slot()
                self._pacing.sync_cache[key] = attr(*args, **kwargs)
            return self._pacing.sync_cache[key]

        return call


class CachedAsyncIntegrationClient:
    """Wrap the async client to cache identical live requests for the test session."""

    def __init__(self, client, pacing: _LiveRequestPacing):
        self._client = client
        self._pacing = pacing

    def __getattr__(self, name):
        attr = getattr(self._client, name)
        if not callable(attr) or not _is_request_method(name):
            return attr

        if inspect.iscoroutinefunction(attr):
            async def call(*args, **kwargs):
                key = _request_key(name, args, kwargs)
                if key not in self._pacing.async_cache:
                    await self._pacing.wait_for_async_slot()
                    self._pacing.async_cache[key] = await attr(*args, **kwargs)
                return self._pacing.async_cache[key]

            return call

        return attr


@pytest.fixture(scope="session")
def api_key():
    """
    Get API key from environment variable or repository .env file.
    
    Supported variables: CONGRESS_API_KEY or API_KEY
    
    To run integration tests:
        export CONGRESS_API_KEY=your_api_key
        pytest test_integ/
    """
    key = os.environ.get("CONGRESS_API_KEY") or os.environ.get("API_KEY")
    if not key or key == "your_api_key_here":
        pytest.skip(
            "No Congress.gov API key configured. Set CONGRESS_API_KEY or API_KEY in the environment "
            "or in the repository .env file."
        )
    return key


@pytest.fixture(scope="session")
def integration_pacing():
    """Provide session-wide pacing and caching for live API requests."""
    min_delay_ms = int(os.environ.get("CDG_INTEG_MIN_DELAY_MS", "100"))
    return _LiveRequestPacing(max(min_delay_ms, 0) / 1000)


@pytest.fixture(scope="session")
def client(api_key, integration_pacing):
    """Provide a cached, paced CDGPythonClient for integration testing."""
    from cdg_python_client import CDGPythonClient

    return CachedIntegrationClient(CDGPythonClient(api_key=api_key), integration_pacing)


@pytest.fixture(scope="session")
def async_client(api_key, integration_pacing):
    """Provide a cached, paced AsyncCDGPythonClient for integration testing."""
    from cdg_python_client import AsyncCDGPythonClient

    return CachedAsyncIntegrationClient(
        AsyncCDGPythonClient(api_key=api_key),
        integration_pacing,
    )
