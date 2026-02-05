"""Integration tests for member-related API endpoints."""

import pytest


class TestMembersList:
    """Test member listing endpoints."""
    
    def test_list_members(self, client):
        """Test listing members returns valid data."""
        members = client.list_members(limit=5, current_member=True)
        
        assert isinstance(members, list)
        assert len(members) > 0
        assert len(members) <= 5
        
        # Validate first member structure
        member = members[0]
        assert hasattr(member, "bioguide_id")
        assert hasattr(member, "full_name")
        assert hasattr(member, "party")
        assert hasattr(member, "state")
        
        # Validate data types and values
        if member.full_name is not None:
            assert isinstance(member.full_name, str)
            assert len(member.full_name) > 0
        if member.party is not None:
            assert isinstance(member.party, str)
        if member.state is not None:
            assert isinstance(member.state, str)
    
    def test_list_members_by_congress(self, client):
        """Test listing members by congress number."""
        members = client.list_members_by_congress(
            congress=118,
            limit=5,
            current_member=True
        )
        
        assert isinstance(members, list)
        assert len(members) > 0
    
    def test_get_member(self, client):
        """Test getting detailed member information."""
        # First, get a member to retrieve their bioguide_id
        members = client.list_members(limit=1, current_member=True)
        assert len(members) > 0
        
        bioguide_id = members[0].bioguide_id
        assert bioguide_id is not None
        
        # Now get that specific member
        member = client.get_member(bioguide_id=bioguide_id)
        
        assert member is not None
        assert member.bioguide_id == bioguide_id
        # Member detail might have different field structure
        assert hasattr(member, "full_name")


class TestMemberLegislation:
    """Test member legislation endpoints."""
    
    def test_get_member_sponsored_legislation(self, client):
        """Test getting legislation sponsored by a member."""
        # Get a member first
        members = client.list_members(limit=1, current_member=True)
        assert len(members) > 0
        
        bioguide_id = members[0].bioguide_id
        assert bioguide_id is not None
        
        # Get their sponsored legislation
        bills = client.get_member_sponsored_legislation(
            bioguide_id=bioguide_id,
            limit=5
        )
        
        assert isinstance(bills, list)
        # Member may have no sponsored legislation
        if len(bills) > 0:
            bill = bills[0]
            assert hasattr(bill, "congress")
            assert hasattr(bill, "number")
            assert hasattr(bill, "title")
    
    def test_get_member_cosponsored_legislation(self, client):
        """Test getting legislation cosponsored by a member."""
        # Get a member first
        members = client.list_members(limit=1, current_member=True)
        assert len(members) > 0
        
        bioguide_id = members[0].bioguide_id
        assert bioguide_id is not None
        
        # Get their cosponsored legislation
        bills = client.get_member_cosponsored_legislation(
            bioguide_id=bioguide_id,
            limit=5
        )
        
        assert isinstance(bills, list)
        # Member may have no cosponsored legislation
        if len(bills) > 0:
            bill = bills[0]
            assert hasattr(bill, "congress")
            assert hasattr(bill, "number")
            assert hasattr(bill, "title")
