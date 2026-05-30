"""Integration tests for law operations."""

import pytest
from cdg_python_client import CDGNotFoundError, CDGPythonClient

class TestLawsList:
    """Tests for listing laws."""
    
    def test_list_laws_by_congress_basic(self, client: CDGPythonClient):
        """Test getting laws for Congress 118."""
        laws = client.list_laws_by_congress(118, limit=5)
        
        assert isinstance(laws, list)
        assert len(laws) > 0
        assert len(laws) <= 5
        
        # Verify structure of first law
        law = laws[0]
        assert hasattr(law, 'congress')
        assert hasattr(law, 'number')
        assert hasattr(law, 'origin_chamber')
        assert hasattr(law, 'origin_chamber_code')
        assert hasattr(law, 'law_type')
        assert hasattr(law, 'update_date')
        assert hasattr(law, 'url')
    
    def test_list_laws_with_pagination(self, client: CDGPythonClient):
        """Test listing laws with pagination."""
        # Get first page
        first_page = client.list_laws_by_congress(118, offset=0, limit=3)
        assert len(first_page) > 0
        
        # Get second page
        second_page = client.list_laws_by_congress(118, offset=3, limit=3)
        assert len(second_page) > 0
        
        # Verify pages are different
        if len(first_page) > 0 and len(second_page) > 0:
            first_urls = {law.url for law in first_page if law.url}
            second_urls = {law.url for law in second_page if law.url}
            assert not first_urls.intersection(second_urls), "Pages should not overlap"


class TestLawsByCongress:
    """Tests for listing laws by congress."""
    
    def test_list_laws_by_congress(self, client: CDGPythonClient    ):
        """Test getting laws for a specific congress."""
        # Use Congress 118 which should have laws
        laws = client.list_laws_by_congress(118, limit=5)
        
        assert isinstance(laws, list)
        assert len(laws) > 0
        
        # Verify all laws are from the requested congress
        for law in laws:
            assert law.congress == 118
    
    def test_list_laws_by_recent_congress(self, client: CDGPythonClient):
        """Test getting laws for Congress 117."""
        laws = client.list_laws_by_congress(117, limit=3)
        
        assert isinstance(laws, list)
        assert len(laws) > 0
        
        for law in laws:
            assert law.congress == 117
            # Check the nested laws array contains law information
            if law.laws:
                assert len(law.laws) > 0


class TestLawsByType:
    """Tests for listing laws by congress and type."""
    
    def test_list_public_laws(self, client: CDGPythonClient):
        """Test getting public laws for a specific congress."""
        # Note: 'pub' filters bills that became public laws
        laws = client.list_laws_by_type(118, "pub", limit=5)
        
        assert isinstance(laws, list)
        assert len(laws) > 0
        
        # Verify all are from the requested congress
        for law in laws:
            assert law.congress == 118
            # These are bills that became public laws
            if law.laws:
                assert len(law.laws) > 0
    
    def test_list_private_laws(self, client: CDGPythonClient):
        """Test getting private laws for a specific congress."""
        # Use an older congress that likely has private laws
        laws = client.list_laws_by_type(117, "priv", limit=5)
        
        # Private laws are less common, so we may not get results
        assert isinstance(laws, list)
        
        # If we got results, verify they're all from the requested congress
        for law in laws:
            assert law.congress == 117
    
    def test_list_laws_different_types(self, client: CDGPythonClient    ):
        """Test that we can filter by law type."""
        public_laws = client.list_laws_by_type(118, "pub", limit=3)
        
        assert len(public_laws) > 0
        
        # Verify we got results from congress 118
        for law in public_laws:
            assert law.congress == 118


class TestLawDetail:
    """Tests for getting detailed law information."""
    
    def test_get_law_detail(self, client: CDGPythonClient):
        """Test getting detailed information for a specific law."""
        # Get a list of laws first
        laws = client.list_laws_by_type(118, "pub", limit=3)
        assert len(laws) > 0
        print(f"Got laws: {laws}")
        law_item = laws[0]
        
        assert law_item.congress is not None
        assert law_item.law_type is not None
        assert law_item.number is not None

        # Use law type and number directly
        law_detail = client.get_bill_detail(
            law_item.congress,
            law_item.law_type,  # e.g., "pub"
            law_item.number      # e.g., "346"
        )
        
        # Verify detail has information
        assert law_detail.congress == law_item.congress
        assert hasattr(law_detail, 'title')
        assert hasattr(law_detail, 'number')
    
    def test_get_known_law(self, client: CDGPythonClient):
        """Test getting a well-known law."""
        # HR 346 became Public Law 118-4
        law = client.get_bill_detail(110, "hconres", "10", format="json")
        
        assert law.congress == 110
        assert law.bill_type == "HCONRES"
        assert law.title is not None
        assert isinstance(law.title, str)
        assert len(law.title) > 0


class TestLawEdgeCases:
    """Tests for law edge cases and error handling."""
    
    def test_get_nonexistent_law(self, client: CDGPythonClient):
        """Test getting a law that doesn't exist."""
        with pytest.raises(CDGNotFoundError) as exc_info:
            # Bill number 99999 should not exist
            client.get_bill_detail(118, "hr", "99999", format="json")
        
        assert "404" in str(exc_info.value)
    
    def test_invalid_law_type(self, client: CDGPythonClient ):
        """Test getting laws with invalid type."""
        # Invalid law types return 404 error
        with pytest.raises(CDGNotFoundError) as exc_info:
            client.list_laws_by_type(118, "invalid", limit=1)
        
        assert "404" in str(exc_info.value)


class TestLawDataValidation:
    """Tests for validating law data fields."""
    
    def test_law_has_required_fields(self, client: CDGPythonClient):
        """Test that laws have all required fields."""
        laws = client.list_laws_by_congress(118, limit=1)
        
        if len(laws) == 0:
            pytest.skip("No laws available")
        
        law = laws[0]
        
        # Check required fields
        assert law.congress is not None
        assert isinstance(law.congress, int)
        assert law.congress > 0
        
        # Bill number is a string
        assert law.number is not None
        assert isinstance(law.number, str)
        
        # Bill type should exist
        assert law.law_type is not None
        assert isinstance(law.law_type, str)
    
    def test_law_detail_has_title(self, client: CDGPythonClient):
        """Test that law details include title information."""
        # Get a public law that should have a title
        laws = client.list_laws_by_type(118, "pub", limit=1)
        
        if len(laws) == 0:
            pytest.skip("No public laws available")
        
        law_item = laws[0]
        
        # Use law type and number
        assert law_item.congress is not None
        assert law_item.law_type is not None
        assert law_item.number is not None
        law_detail = client.get_bill_detail(law_item.congress, law_item.law_type, law_item.number, format='json')
        
        # Detail should have title
        assert hasattr(law_detail, 'title')


class TestLawWorkflow:
    """Tests for complete law workflow."""
    
    def test_complete_workflow(self, client: CDGPythonClient):
        """Test a complete workflow: list -> filter by type -> get detail."""
        print("\n1. Getting laws for Congress 118...")
        all_laws = client.list_laws_by_congress(118, limit=5, format="json")
        assert len(all_laws) > 0
        print(f"   Found {len(all_laws)} laws")
        
        print("\n2. Filtering public laws...")
        public_laws = client.list_laws_by_type(118, "pub", limit=3, format="json")
        assert len(public_laws) > 0
        print(f"   Found {len(public_laws)} bills that became public laws")
        
        print("\n3. Getting details for first public law...")
        first_law = public_laws[0]
        
        # Use law type and number directly
        assert first_law.congress is not None
        assert first_law.law_type is not None
        assert first_law.number is not None
        law_detail = client.get_bill_detail(first_law.congress, first_law.law_type, first_law.number, format='json')
        
        assert law_detail.congress == first_law.congress
        
        print("\n   Workflow completed successfully!")
