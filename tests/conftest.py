"""Pytest configuration and fixtures for cdg_python_client tests."""

import pytest


@pytest.fixture
def api_key():
    """Provide a test API key."""
    return "test_api_key_12345"


@pytest.fixture
def client(api_key):
    """Provide a CDGPythonClient instance for testing."""
    from cdg_python_client import CDGPythonClient
    return CDGPythonClient(api_key=api_key)


@pytest.fixture
def async_client(api_key):
    """Provide an AsyncCDGPythonClient instance for testing."""
    from cdg_python_client import AsyncCDGPythonClient
    return AsyncCDGPythonClient(api_key=api_key)
