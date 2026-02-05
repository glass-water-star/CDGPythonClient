"""Integration tests for committee-related API endpoints."""

import pytest


class TestCommitteesList:
    """Test committee listing endpoints."""
    
    def test_list_committees(self, client):
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
    
    def test_committee_has_required_fields(self, client):
        """Test that committees have required fields populated."""
        committees = client.list_committees(limit=5)
        
        assert len(committees) > 0
        
        for committee in committees:
            # At least name should be present for all committees
            assert committee.name is not None
            assert len(committee.name) > 0
