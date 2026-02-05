"""Integration tests for amendment-related API endpoints."""

import pytest


class TestAmendmentsList:
    """Test amendment listing endpoints."""
    
    def test_list_amendments(self, client):
        """Test listing amendments returns valid data."""
        amendments = client.list_amendments(limit=5)
        
        assert isinstance(amendments, list)
        assert len(amendments) > 0
        assert len(amendments) <= 5
        
        # Validate first amendment structure
        amendment = amendments[0]
        assert hasattr(amendment, "congress")
        assert hasattr(amendment, "number")
        assert hasattr(amendment, "amendment_type")
        assert hasattr(amendment, "url")
        assert hasattr(amendment, "latest_action")
        
        # Validate data types
        if amendment.congress is not None:
            assert isinstance(amendment.congress, int)
        if amendment.number is not None:
            assert isinstance(amendment.number, str)
        if amendment.amendment_type is not None:
            assert isinstance(amendment.amendment_type, str)
    
    def test_list_amendments_by_congress(self, client):
        """Test listing amendments by congress number."""
        amendments = client.list_amendments_by_congress(congress=118, limit=3)
        
        assert isinstance(amendments, list)
        assert len(amendments) > 0
        
        # Verify all amendments are from congress 118
        for amendment in amendments:
            if amendment.congress is not None:
                assert amendment.congress == 118
    
    def test_amendment_has_latest_action(self, client):
        """Test that amendments have latest action information."""
        amendments = client.list_amendments(limit=1)
        
        assert len(amendments) > 0
        amendment = amendments[0]
        
        if amendment.latest_action is not None:
            assert hasattr(amendment.latest_action, "action_date")
            assert hasattr(amendment.latest_action, "text")
