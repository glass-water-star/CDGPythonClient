"""Integration tests for committee-related API endpoints."""

import pytest

from cdg_python_client import CDGPythonClient


class TestCommitteesList:
    """Test committee listing endpoints."""
    
    def test_list_committees(self, client: CDGPythonClient):
        """Test listing committees returns valid data."""
        committees = client.list_committees(limit=10)
        
        assert isinstance(committees, list)
        assert len(committees) > 0
        assert len(committees) <= 10
        
        # Validate first committee structure
        committee = committees[0]
        assert hasattr(committee, "name")
        assert hasattr(committee, "system_code")
        assert hasattr(committee, "url")
        
        # Validate data types
        if committee.name is not None:
            assert isinstance(committee.name, str)
            assert len(committee.name) > 0
        if committee.system_code is not None:
            assert isinstance(committee.system_code, str)
        if committee.url is not None:
            assert isinstance(committee.url, str)
    
    def test_committee_has_required_fields(self, client: CDGPythonClient):
        """Test that committees have required fields populated."""
        committees = client.list_committees(limit=5, format="json")
        
        assert len(committees) > 0
        
        for committee in committees:
            # At least name should be present for all committees
            assert committee.name is not None
            assert len(committee.name) > 0

    def test_list_committee_prints(self, client: CDGPythonClient):
        """Test committee prints tolerate live payloads with missing number fields."""
        committee_prints = client.list_committee_prints(limit=5, format="json")

        assert isinstance(committee_prints, list)
        assert len(committee_prints) > 0
        assert len(committee_prints) <= 5

        committee_print = committee_prints[0]
        assert hasattr(committee_print, "jacket_number")
        assert hasattr(committee_print, "number")
