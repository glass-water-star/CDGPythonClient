"""Pytest configuration for integration tests."""

import os
import pytest


@pytest.fixture(scope="session")
def api_key():
    """
    Get API key from environment variable.
    
    Required environment variable: CONGRESS_API_KEY
    
    To run integration tests:
        export CONGRESS_API_KEY=your_api_key
        pytest tests/test_integ/
    """
    key = os.environ.get("CONGRESS_API_KEY")
    if not key:
        pytest.skip("CONGRESS_API_KEY environment variable not set. Skipping integration tests.")
    return key


@pytest.fixture(scope="session")
def client(api_key):
    """Provide a CDGPythonClient instance for integration testing."""
    from cdg_python_client import CDGPythonClient
    return CDGPythonClient(api_key=api_key)
