"""
Integration tests for member-related endpoints.
"""
import pytest
from cdg_python_client import CDGPythonClient


class TestMembersByState:
    """Test member listing by state."""
    
    def test_list_members_by_state(self, client, api_key):
        """Test getting members by state code."""
        if not api_key:
            pytest.skip("API key not provided")
        
        # Test with California
        members = client.list_members_by_state("CA", limit=5)
        
        assert isinstance(members, list)
        assert len(members) > 0
        assert len(members) <= 5
        
        # Check that members have required fields
        for member in members:
            assert hasattr(member, 'bioguide_id')
            assert hasattr(member, 'state')
            # State should be California for all results
            # Note: Some members may have None state, so we check if state exists
            if member.state:
                assert 'California' in member.state or 'CA' in str(member.state)
    
    def test_list_members_by_state_current(self, client, api_key):
        """Test getting current members by state."""
        if not api_key:
            pytest.skip("API key not provided")
        
        members = client.list_members_by_state("NY", current_member=True, limit=10)
        
        assert isinstance(members, list)
        assert len(members) > 0


class TestMembersByStateDistrict:
    """Test member listing by state and district."""
    
    def test_list_members_by_state_district(self, client, api_key):
        """Test getting members by state and district."""
        if not api_key:
            pytest.skip("API key not provided")
        
        # Test with California, District 1
        members = client.list_members_by_state_district("CA", 1)
        
        assert isinstance(members, list)
        # A specific district should have at most a few representatives
        for member in members:
            assert hasattr(member, 'bioguide_id')
    
    def test_list_members_by_state_district_current(self, client, api_key):
        """Test getting current members by state and district."""
        if not api_key:
            pytest.skip("API key not provided")
        
        members = client.list_members_by_state_district("TX", 10, current_member=True)
        
        assert isinstance(members, list)


class TestAllMemberEndpoints:
    """Test all member endpoints together to ensure they work."""
    
    def test_member_workflow(self, client, api_key):
        """Test a complete member workflow."""
        if not api_key:
            pytest.skip("API key not provided")
        
        # 1. Get members from a state
        members = client.list_members_by_state("VT", limit=5)
        assert len(members) > 0
        
        # 2. Get a specific member's details
        if members and members[0].bioguide_id:
            bioguide_id = members[0].bioguide_id
            member = client.get_member(bioguide_id)
            assert member.bioguide_id == bioguide_id
            
            # 3. Get their sponsored legislation
            sponsored = client.get_member_sponsored_legislation(bioguide_id, limit=5)
            assert isinstance(sponsored, list)
            
            # 4. Get their cosponsored legislation
            cosponsored = client.get_member_cosponsored_legislation(bioguide_id, limit=5)
            assert isinstance(cosponsored, list)
