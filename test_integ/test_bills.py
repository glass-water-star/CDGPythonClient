"""Integration tests for bill-related API endpoints."""

import pytest


class TestBillsList:
    """Test bill listing endpoints."""
    
    def test_list_bills(self, client):
        """Test listing bills returns valid data."""
        bills = client.list_bills(limit=5)
        
        assert isinstance(bills, list)
        assert len(bills) > 0
        assert len(bills) <= 5
        
        # Validate first bill structure
        bill = bills[0]
        assert hasattr(bill, "congress")
        assert hasattr(bill, "number")
        assert hasattr(bill, "title")
        assert hasattr(bill, "bill_type")
        assert hasattr(bill, "url")
        
        # Validate data types
        if bill.congress is not None:
            assert isinstance(bill.congress, int)
        if bill.number is not None:
            assert isinstance(bill.number, str)
        if bill.title is not None:
            assert isinstance(bill.title, str)
    
    def test_list_bills_by_congress(self, client):
        """Test listing bills by congress number."""
        bills = client.list_bills_by_congress(congress=118, limit=3)
        
        assert isinstance(bills, list)
        assert len(bills) > 0
        
        # Verify all bills are from congress 118
        for bill in bills:
            if bill.congress is not None:
                assert bill.congress == 118
    
    def test_list_bills_by_type(self, client):
        """Test listing bills by congress and type."""
        bills = client.list_bills_by_type(congress=118, bill_type="hr", limit=3)
        
        assert isinstance(bills, list)
        assert len(bills) > 0
        
        # Verify all bills are from congress 118 and type "hr"
        for bill in bills:
            if bill.congress is not None:
                assert bill.congress == 118
            if bill.bill_type is not None:
                assert bill.bill_type.lower() == "hr"


class TestBillDetail:
    """Test bill detail endpoint."""
    
    def test_get_bill(self, client):
        """Test getting detailed bill information."""
        bill = client.get_bill(congress=118, bill_type="hr", bill_number=1)
        
        assert bill is not None
        assert bill.congress == 118
        assert bill.bill_type is not None
        assert bill.number is not None
        assert bill.title is not None
        
        # Check for detail-specific fields
        assert hasattr(bill, "introduced_date")
        assert hasattr(bill, "sponsors")
        assert hasattr(bill, "policy_area")
        
        # Validate sponsors if present
        if bill.sponsors is not None:
            assert isinstance(bill.sponsors, list)
            if len(bill.sponsors) > 0:
                sponsor = bill.sponsors[0]
                assert hasattr(sponsor, "full_name")
                assert hasattr(sponsor, "party")
                assert hasattr(sponsor, "state")


class TestBillSubEndpoints:
    """Test bill sub-endpoints (actions, amendments, etc.)."""
    
    def test_get_bill_actions(self, client):
        """Test getting bill actions."""
        actions = client.get_bill_actions(
            congress=118,
            bill_type="hr",
            bill_number=1,
            limit=5
        )
        
        assert isinstance(actions, list)
        if len(actions) > 0:
            action = actions[0]
            assert hasattr(action, "action_date")
            assert hasattr(action, "text")
            assert hasattr(action, "action_type")
    
    def test_get_bill_amendments(self, client):
        """Test getting bill amendments."""
        amendments = client.get_bill_amendments(
            congress=118,
            bill_type="hr",
            bill_number=1,
            limit=5
        )
        
        assert isinstance(amendments, list)
        # Amendments may or may not exist for a bill
        if len(amendments) > 0:
            amendment = amendments[0]
            assert hasattr(amendment, "congress")
            assert hasattr(amendment, "number")
            assert hasattr(amendment, "amendment_type")
    
    def test_get_bill_committees(self, client):
        """Test getting bill committees."""
        committees = client.get_bill_committees(
            congress=118,
            bill_type="hr",
            bill_number=1,
            limit=5
        )
        
        assert isinstance(committees, list)
        if len(committees) > 0:
            committee = committees[0]
            assert hasattr(committee, "name")
            assert hasattr(committee, "system_code")
    
    def test_get_bill_cosponsors(self, client):
        """Test getting bill cosponsors."""
        cosponsors = client.get_bill_cosponsors(
            congress=118,
            bill_type="hr",
            bill_number=1,
            limit=5
        )
        
        assert isinstance(cosponsors, list)
        if len(cosponsors) > 0:
            cosponsor = cosponsors[0]
            assert hasattr(cosponsor, "full_name")
            assert hasattr(cosponsor, "party")
            assert hasattr(cosponsor, "state")
            assert hasattr(cosponsor, "sponsorship_date")
    
    def test_get_related_bills(self, client):
        """Test getting related bills."""
        # Try a different bill that's more likely to have related bills
        # Using S.1 (a major piece of legislation) instead of HR.1
        related = client.get_related_bills(
            congress=117,
            bill_type="s",
            bill_number=1,
            limit=5
        )
        
        assert isinstance(related, list)
        # Not all bills have related bills
        if len(related) > 0:
            related_bill = related[0]
            assert hasattr(related_bill, "congress")
            assert hasattr(related_bill, "number")
            assert hasattr(related_bill, "bill_type")
            assert hasattr(related_bill, "title")
    
    def test_get_bill_subjects(self, client):
        """Test getting bill subjects."""
        subjects = client.get_bill_subjects(
            congress=118,
            bill_type="hr",
            bill_number=1,
            limit=5
        )
        
        assert isinstance(subjects, list)
        # Not all bills have subjects assigned yet
        if len(subjects) > 0:
            subject = subjects[0]
            assert hasattr(subject, "name")
            assert subject.name is not None
    
    def test_get_bill_summaries(self, client):
        """Test getting bill summaries."""
        summaries = client.get_bill_summaries(
            congress=118,
            bill_type="hr",
            bill_number=1,
            limit=5
        )
        
        assert isinstance(summaries, list)
        if len(summaries) > 0:
            summary = summaries[0]
            assert hasattr(summary, "action_date")
            assert hasattr(summary, "text")
    
    def test_get_bill_text(self, client):
        """Test getting bill text versions."""
        text_versions = client.get_bill_text(
            congress=118,
            bill_type="hr",
            bill_number=1,
            limit=5
        )
        
        assert isinstance(text_versions, list)
        if len(text_versions) > 0:
            text_version = text_versions[0]
            assert hasattr(text_version, "date")
            assert hasattr(text_version, "text_type")
            assert hasattr(text_version, "formats")
            
            if text_version.formats is not None:
                assert isinstance(text_version.formats, list)
    
    def test_get_bill_titles(self, client):
        """Test getting bill titles."""
        titles = client.get_bill_titles(
            congress=118,
            bill_type="hr",
            bill_number=1,
            limit=5
        )
        
        assert isinstance(titles, list)
        assert len(titles) > 0  # Every bill should have at least one title
        
        title = titles[0]
        assert hasattr(title, "title")
        assert hasattr(title, "title_type")
        assert title.title is not None
