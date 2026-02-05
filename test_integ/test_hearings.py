"""Integration tests for hearing-related API endpoints."""

import pytest


class TestHearingsList:
    """Test hearing listing endpoints."""
    
    def test_list_hearings(self, client):
        """Test listing hearings returns valid data."""
        hearings = client.list_hearings(limit=5)
        
        assert isinstance(hearings, list)
        assert len(hearings) > 0
        assert len(hearings) <= 5
        
        # Validate first hearing structure
        hearing = hearings[0]
        assert hasattr(hearing, "congress")
        assert hasattr(hearing, "chamber")
        assert hasattr(hearing, "jacket_number")
        assert hasattr(hearing, "title")
        assert hasattr(hearing, "url")
        
        # Validate data types
        if hearing.congress is not None:
            assert isinstance(hearing.congress, int)
            assert hearing.congress > 0
        if hearing.chamber is not None:
            assert isinstance(hearing.chamber, str)
            assert hearing.chamber in ["House", "Senate", "house", "senate"]
        if hearing.jacket_number is not None:
            assert isinstance(hearing.jacket_number, int)
        if hearing.title is not None:
            assert isinstance(hearing.title, str)
            assert len(hearing.title) > 0
        if hearing.url is not None:
            assert isinstance(hearing.url, str)
            assert "api.congress.gov" in hearing.url
    
    def test_list_hearings_pagination(self, client):
        """Test listing hearings with pagination."""
        hearings_page1 = client.list_hearings(limit=3, offset=0)
        hearings_page2 = client.list_hearings(limit=3, offset=3)
        
        assert isinstance(hearings_page1, list)
        assert isinstance(hearings_page2, list)
        assert len(hearings_page1) > 0
        
        # Ensure different results (if enough hearings exist)
        if len(hearings_page2) > 0:
            # Compare first hearing from each page
            assert hearings_page1[0].jacket_number != hearings_page2[0].jacket_number or \
                   hearings_page1[0].congress != hearings_page2[0].congress


class TestHearingsByCongress:
    """Test hearing listing by congress."""
    
    def test_list_hearings_by_congress(self, client):
        """Test listing hearings for a specific congress."""
        congress = 118
        hearings = client.list_hearings_by_congress(congress=congress, limit=5)
        
        assert isinstance(hearings, list)
        assert len(hearings) > 0
        assert len(hearings) <= 5
        
        # Verify all hearings are from the specified congress
        for hearing in hearings:
            if hearing.congress is not None:
                assert hearing.congress == congress
    
    def test_list_hearings_by_congress_117(self, client):
        """Test listing hearings for congress 117."""
        hearings = client.list_hearings_by_congress(congress=117, limit=3)
        
        assert isinstance(hearings, list)
        assert len(hearings) > 0
        
        for hearing in hearings:
            if hearing.congress is not None:
                assert hearing.congress == 117


class TestHearingsByChamber:
    """Test hearing listing by congress and chamber."""
    
    def test_list_hearings_by_chamber_house(self, client):
        """Test listing House hearings for a specific congress."""
        hearings = client.list_hearings_by_chamber(
            congress=118,
            chamber="house",
            limit=5
        )
        
        assert isinstance(hearings, list)
        assert len(hearings) > 0
        assert len(hearings) <= 5
        
        # Verify all hearings are from House and congress 118
        for hearing in hearings:
            if hearing.congress is not None:
                assert hearing.congress == 118
            if hearing.chamber is not None:
                assert hearing.chamber.lower() == "house"
    
    def test_list_hearings_by_chamber_senate(self, client):
        """Test listing Senate hearings for a specific congress."""
        hearings = client.list_hearings_by_chamber(
            congress=118,
            chamber="senate",
            limit=5
        )
        
        assert isinstance(hearings, list)
        
        # Verify all hearings are from Senate and congress 118
        for hearing in hearings:
            if hearing.congress is not None:
                assert hearing.congress == 118
            if hearing.chamber is not None:
                assert hearing.chamber.lower() == "senate"


class TestHearingDetail:
    """Test hearing detail endpoint."""
    
    def test_get_hearing(self, client):
        """Test getting detailed hearing information."""
        # First, get a list of hearings to find a valid one
        hearings = client.list_hearings_by_chamber(
            congress=117,
            chamber="house",
            limit=1
        )
        
        if len(hearings) == 0:
            pytest.skip("No hearings available for testing")
        
        hearing_list_item = hearings[0]
        assert hearing_list_item.congress is not None
        assert hearing_list_item.chamber is not None
        assert hearing_list_item.jacket_number is not None
        
        # Get the detailed hearing
        hearing = client.get_hearing(
            congress=hearing_list_item.congress,
            chamber=hearing_list_item.chamber,
            jacket_number=hearing_list_item.jacket_number
        )
        
        assert hearing is not None
        assert hearing.congress == hearing_list_item.congress
        assert hearing.jacket_number == hearing_list_item.jacket_number
        assert hearing.title is not None
        
        # Check for detail-specific fields
        assert hasattr(hearing, "citation")
        assert hasattr(hearing, "committees")
        assert hasattr(hearing, "dates")
        assert hasattr(hearing, "formats")
        assert hasattr(hearing, "associated_meeting")
        assert hasattr(hearing, "library_of_congress_identifier")
    
    def test_hearing_detail_has_committees(self, client):
        """Test that hearing details include committee information."""
        # Get a hearing from a recent congress
        hearings = client.list_hearings_by_chamber(
            congress=118,
            chamber="house",
            limit=5
        )
        
        if len(hearings) == 0:
            pytest.skip("No hearings available for testing")
        
        # Find a hearing with valid data
        for hearing_list_item in hearings:
            if (hearing_list_item.congress is not None and 
                hearing_list_item.chamber is not None and 
                hearing_list_item.jacket_number is not None):
                
                hearing = client.get_hearing(
                    congress=hearing_list_item.congress,
                    chamber=hearing_list_item.chamber,
                    jacket_number=hearing_list_item.jacket_number
                )
                
                # Validate committees if present
                if hearing.committees is not None:
                    assert isinstance(hearing.committees, list)
                    if len(hearing.committees) > 0:
                        committee = hearing.committees[0]
                        assert hasattr(committee, "name")
                        assert hasattr(committee, "system_code")
                        assert hasattr(committee, "url")
                break
    
    def test_hearing_detail_has_formats(self, client):
        """Test that hearing details include format information."""
        hearings = client.list_hearings_by_chamber(
            congress=117,
            chamber="house",
            limit=5
        )
        
        if len(hearings) == 0:
            pytest.skip("No hearings available for testing")
        
        # Find a hearing with formats
        for hearing_list_item in hearings:
            if (hearing_list_item.congress is not None and 
                hearing_list_item.chamber is not None and 
                hearing_list_item.jacket_number is not None):
                
                hearing = client.get_hearing(
                    congress=hearing_list_item.congress,
                    chamber=hearing_list_item.chamber,
                    jacket_number=hearing_list_item.jacket_number
                )
                
                # Validate formats if present
                if hearing.formats is not None:
                    assert isinstance(hearing.formats, list)
                    if len(hearing.formats) > 0:
                        format_item = hearing.formats[0]
                        assert hasattr(format_item, "format_type")
                        assert hasattr(format_item, "url")
                        if format_item.url is not None:
                            assert isinstance(format_item.url, str)
                    break


class TestHearingWorkflow:
    """Test a complete workflow using hearing endpoints."""
    
    def test_complete_workflow(self, client):
        """Test a complete workflow: list -> filter by congress/chamber -> detail."""
        print("\n1. Getting recent hearings...")
        all_hearings = client.list_hearings(limit=10)
        assert len(all_hearings) > 0
        print(f"   Found {len(all_hearings)} hearings")
        
        # Get a recent congress number from the results
        recent_congress = None
        for hearing in all_hearings:
            if hearing.congress is not None:
                recent_congress = hearing.congress
                break
        
        if recent_congress is None:
            pytest.skip("No hearings with congress information available")
        
        print(f"\n2. Getting hearings for Congress {recent_congress}...")
        congress_hearings = client.list_hearings_by_congress(
            congress=recent_congress,
            limit=5
        )
        assert len(congress_hearings) > 0
        print(f"   Found {len(congress_hearings)} hearings for Congress {recent_congress}")
        
        # Find a hearing with chamber information
        test_hearing = None
        for hearing in congress_hearings:
            if (hearing.congress is not None and 
                hearing.chamber is not None and 
                hearing.jacket_number is not None):
                test_hearing = hearing
                break
        
        if test_hearing is None:
            pytest.skip("No suitable hearing found for detail test")
        
        print(f"\n3. Getting hearings for {test_hearing.chamber} chamber...")
        chamber_hearings = client.list_hearings_by_chamber(
            congress=test_hearing.congress,
            chamber=test_hearing.chamber,
            limit=3
        )
        assert len(chamber_hearings) > 0
        print(f"   Found {len(chamber_hearings)} {test_hearing.chamber} hearings")
        
        print(f"\n4. Getting detailed information for hearing {test_hearing.jacket_number}...")
        detailed_hearing = client.get_hearing(
            congress=test_hearing.congress,
            chamber=test_hearing.chamber,
            jacket_number=test_hearing.jacket_number
        )
        
        assert detailed_hearing is not None
        assert detailed_hearing.congress == test_hearing.congress
        assert detailed_hearing.jacket_number == test_hearing.jacket_number
        
        print(f"\n   Hearing: {detailed_hearing.title}")
        if detailed_hearing.citation:
            print(f"   Citation: {detailed_hearing.citation}")
        if detailed_hearing.dates:
            print(f"   Dates: {len(detailed_hearing.dates)} date(s)")
        if detailed_hearing.committees:
            print(f"   Committees: {len(detailed_hearing.committees)} committee(s)")
        if detailed_hearing.formats:
            print(f"   Formats: {len(detailed_hearing.formats)} format(s) available")
        
        print("\n✓ Complete workflow test passed!")


class TestHearingEdgeCases:
    """Test edge cases and error handling."""
    
    def test_empty_results_handling(self, client):
        """Test handling of queries that might return no results."""
        # Use a very old congress that likely has no data
        hearings = client.list_hearings_by_congress(congress=80, limit=5)
        
        # Should return empty list, not error
        assert isinstance(hearings, list)
    
    def test_limit_parameter(self, client):
        """Test that limit parameter is respected."""
        for limit in [1, 3, 5]:
            hearings = client.list_hearings(limit=limit)
            assert len(hearings) <= limit
