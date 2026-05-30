"""
Integration tests for House vote operations.

Note: House vote API endpoints are in BETA status.
"""

import pytest
import os
from cdg_python_client import CDGPythonClient


class TestHouseVotesList:
    """Tests for list_house_votes endpoint."""
    
    def test_list_house_votes_basic(self, client: CDGPythonClient):
        """Test getting a basic list of house votes."""

        votes = client.list_house_votes(limit=5)
        
        assert votes is not None
        assert isinstance(votes, list)
        assert len(votes) <= 5
        
        if len(votes) > 0:
            vote = votes[0]
            assert hasattr(vote, 'congress')
            assert hasattr(vote, 'roll_call_number')
            assert hasattr(vote, 'session_number')
            print(f"Sample vote: {vote}")
    
    def test_list_house_votes_with_pagination(self, client: CDGPythonClient):
        """Test list with pagination parameters."""
        votes = client.list_house_votes(offset=10, limit=5)
        
        assert votes is not None
        assert isinstance(votes, list)
    
    def test_list_house_votes_with_date_filter(self, client: CDGPythonClient):
        """Test list with date filtering."""
        votes = client.list_house_votes(
            from_date="2024-01-01T00:00:00Z",
            limit=5
        )
        
        assert votes is not None
        assert isinstance(votes, list)


class TestHouseVotesByCongress:
    """Tests for list_house_votes_by_congress endpoint."""
    
    def test_list_votes_by_congress(self, client: CDGPythonClient):
        """Test getting votes for a specific congress."""
        votes = client.list_house_votes_by_congress(118, limit=5)
        
        assert votes is not None
        assert isinstance(votes, list)
        
        if len(votes) > 0:
            vote = votes[0]
            assert vote.congress == 118
            print(f"Congress 118 vote: {vote}")


class TestHouseVotesBySession:
    """Tests for list_house_votes_by_session endpoint."""
    
    def test_list_votes_by_session(self, client: CDGPythonClient):
        """Test getting votes for a specific congress and session."""
        votes = client.list_house_votes_by_session(118, 2, limit=5)
        
        assert votes is not None
        assert isinstance(votes, list)
        
        if len(votes) > 0:
            vote = votes[0]
            assert vote.congress == 118
            assert vote.session_number == 2
            print(f"Congress 118, Session 2 vote: {vote}")


class TestHouseVoteDetail:
    """Tests for get_house_vote endpoint."""
    
    def test_get_house_vote(self, client: CDGPythonClient):
        """Test getting detailed information about a specific vote."""
        
        # Get a vote from the list first
        votes = client.list_house_votes_by_session(118, 2, limit=1)
        
        if len(votes) == 0:
            pytest.skip("No votes available for testing")
        
        vote_basic = votes[0]
        
        assert vote_basic.congress is not None
        assert vote_basic.session_number is not None
        assert vote_basic.roll_call_number is not None
        # Get detailed vote information
        vote_detail = client.get_house_vote(vote_basic.congress, vote_basic.session_number, vote_basic.roll_call_number)
        
        assert vote_detail is not None
        assert vote_detail.congress == vote_basic.congress
        assert vote_detail.session_number == vote_basic.session_number
        assert vote_detail.roll_call_number == vote_basic.roll_call_number
        assert hasattr(vote_detail, 'vote_party_total')
        assert hasattr(vote_detail, 'vote_question')
        
        print(f"Vote detail: {vote_detail}")
        
        if vote_detail.vote_party_total:
            print(f"Party totals: {vote_detail.vote_party_total}")


class TestHouseVoteMembers:
    """Tests for get_house_vote_members endpoint."""
    
    def test_get_vote_members(self, client: CDGPythonClient):
        """Test getting member voting information for a specific vote."""
        # Get a vote from the list first
        votes = client.list_house_votes_by_session(118, 2, limit=1)
        
        if len(votes) == 0:
            pytest.skip("No votes available for testing")
        
        vote_basic = votes[0]
        assert vote_basic.congress is not None
        assert vote_basic.session_number is not None
        assert vote_basic.roll_call_number is not None
        
        # Get member voting information
        vote_members = client.get_house_vote_members(vote_basic.congress, vote_basic.session_number, vote_basic.roll_call_number, limit=10)
        
        assert vote_members is not None
        assert vote_members.congress == vote_basic.congress
        assert vote_members.session_number == vote_basic.session_number
        assert vote_members.roll_call_number == vote_basic.roll_call_number
        assert hasattr(vote_members, 'results')
        
        print(f"Vote members info: {vote_members}")
        
        if vote_members.results and len(vote_members.results) > 0:
            member_vote = vote_members.results[0]
            assert hasattr(member_vote, 'bioguide_id')
            assert hasattr(member_vote, 'first_name')
            assert hasattr(member_vote, 'last_name')
            assert hasattr(member_vote, 'vote_cast')
            print(f"Sample member vote: {member_vote}")


class TestHouseVoteWorkflow:
    """Integration test showing a complete house vote workflow."""
    
    def test_complete_workflow(self, client: CDGPythonClient):
        """Test a complete workflow: list -> detail -> members."""

        # Step 1: Get recent votes for Congress 118, Session 2
        print("\n1. Getting recent votes...")
        votes = client.list_house_votes_by_session(118, 2, limit=3)
        assert len(votes) > 0
        print(f"   Found {len(votes)} votes")
        
        # Step 2: Get detailed info for the first vote
        vote = votes[0]
        assert vote.congress is not None
        assert vote.session_number is not None
        assert vote.roll_call_number is not None

        vote_detail = client.get_house_vote(
            vote.congress, 
            vote.session_number, 
            vote.roll_call_number
        )
        
        print(f"   Vote question: {vote_detail.vote_question}")
        print(f"   Result: {vote_detail.result}")
        
        if vote_detail.vote_party_total:
            for party_vote in vote_detail.vote_party_total:
                print(f"   {party_vote.vote_party}: Yea={party_vote.yea_total}, Nay={party_vote.nay_total}")
        
        # Step 3: Get member votes
        print(f"\n3. Getting member votes...")
        vote_members = client.get_house_vote_members(
            vote.congress,
            vote.session_number,
            vote.roll_call_number,
            limit=5
        )
        
        if vote_members.results:
            print(f"   Sample of {len(vote_members.results)} member votes:")
            for member in vote_members.results[:3]:
                print(f"   - {member.first_name} {member.last_name} ({member.vote_party}): {member.vote_cast}")
        
        print("\n✅ Workflow completed successfully!")
