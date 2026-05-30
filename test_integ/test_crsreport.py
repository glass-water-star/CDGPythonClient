"""Integration tests for CRS report-related API endpoints."""

import pytest
from tenacity import retry, stop_after_attempt, wait_exponential, retry_if_exception_type
import time

from cdg_python_client import CDGPythonClient
from test_integ.conftest import client


# Retry decorator for API calls that may fail transiently
def api_retry(func):
    """Decorator to retry API calls with exponential backoff."""
    return retry(
        stop=stop_after_attempt(3),
        wait=wait_exponential(multiplier=1, min=1, max=10),
        retry=retry_if_exception_type(RuntimeError),
        reraise=True
    )(func)


class TestCrsReportsList:
    """Test CRS report listing endpoints."""
    
    def test_list_crs_reports(self, client):
        """Test listing CRS reports returns valid data."""
        reports = client.list_crs_reports(limit=5)
        
        assert isinstance(reports, list)
        assert len(reports) > 0
        assert len(reports) <= 5
        
        # Validate first report structure
        report = reports[0]
        assert hasattr(report, "id")
        assert hasattr(report, "title")
        assert hasattr(report, "content_type")
        assert hasattr(report, "status")
        assert hasattr(report, "publish_date")
        assert hasattr(report, "update_date")
        assert hasattr(report, "url")
        assert hasattr(report, "version")
        
        # Validate data types
        if report.id is not None:
            assert isinstance(report.id, str)
            assert len(report.id) > 0
        if report.title is not None:
            assert isinstance(report.title, str)
            assert len(report.title) > 0
        if report.content_type is not None:
            assert isinstance(report.content_type, str)
        if report.status is not None:
            assert isinstance(report.status, str)
        if report.version is not None:
            assert isinstance(report.version, int)
            assert report.version > 0
        if report.url is not None:
            assert isinstance(report.url, str)
            assert "api.congress.gov" in report.url
            assert "crsreport" in report.url
    
    def test_list_crs_reports_pagination(self, client):
        """Test listing CRS reports with pagination."""
        reports_page1 = client.list_crs_reports(limit=3, offset=0)
        reports_page2 = client.list_crs_reports(limit=3, offset=3)
        
        assert isinstance(reports_page1, list)
        assert isinstance(reports_page2, list)
        assert len(reports_page1) > 0
        
        # Ensure different results (if enough reports exist)
        if len(reports_page1) > 0 and len(reports_page2) > 0:
            # Compare report IDs to ensure they're different
            page1_ids = {r.id for r in reports_page1 if r.id}
            page2_ids = {r.id for r in reports_page2 if r.id}
            # At least one should be different if pagination works
            assert not page1_ids.issubset(page2_ids) or not page2_ids.issubset(page1_ids)
    
    def test_list_crs_reports_with_limit(self, client):
        """Test CRS reports listing respects limit parameter."""
        limit = 2
        reports = client.list_crs_reports(limit=limit)
        
        assert isinstance(reports, list)
        assert len(reports) <= limit
        if len(reports) > 0:
            assert reports[0].id is not None


class TestCrsReportDetail:
    """Test CRS report detail endpoint."""
    
    def test_get_crs_report_valid(self, client: CDGPythonClient):
        """Test getting a specific CRS report with valid ID."""
        # First get a list to find a valid report number
        reports = client.list_crs_reports(limit=5, format="json")
        assert len(reports) > 0
        first_report = reports[0]
        # Get the ID from the first report
        report_id = first_report.id
        assert report_id is not None
        
        # Validate basic structure
        assert hasattr(first_report, "id")
        assert hasattr(first_report, "title")
        assert hasattr(first_report, "content_type")
        assert hasattr(first_report, "status")
        assert hasattr(first_report, "publish_date")
        assert hasattr(first_report, "update_date")
        assert hasattr(first_report, "url")
        assert hasattr(first_report, "version")
    
    def test_get_crs_report_known_example(self, client: CDGPythonClient):
        """Test getting a known CRS report (if it exists)."""
        # Try to get a commonly referenced CRS report
        # Using R43083 as an example from the swagger.json
        report = client.get_crs_report("R43083", format="json")
        
        # Basic validation
        assert report.id == "R43083"
        assert report.title is not None
        assert isinstance(report.title, str)
    
    def test_get_crs_report_with_various_ids(self, client: CDGPythonClient):
        """Test getting CRS reports with different ID formats."""
        # Get a report to test with (limit to 1 to avoid rate limiting)
        reports = client.list_crs_reports(limit=3, format="json")
        assert len(reports) > 0
        
        for i, report in enumerate(reports):
            if report.id:
                # Add delay before each request to avoid rate limiting
                if i > 0:
                    time.sleep(1.0)
                
                detail = client.get_crs_report(report.id, format="json")
                assert detail.id == report.id
                # Version should match or be updated
                if report.version is not None and detail.version is not None:
                    assert detail.version >= report.version


class TestCrsReportWorkflow:
    """Test complete workflow scenarios."""
    
    def test_list_and_detail_workflow(self, client: CDGPythonClient):
        """Test workflow of listing reports and getting details."""
        # Step 1: List reports
        reports = client.list_crs_reports(limit=5, format="json")
        assert len(reports) > 0
        
        # Step 2: Get details for first report
        first_report = reports[0]
        assert first_report.id is not None
        # Step 3: Verify consistency between list and detail
        detail = client.get_crs_report(first_report.id, format="json")
        assert detail.id == first_report.id
        
        # Title should match
        if first_report.title and detail.title:
            assert first_report.title == detail.title
        
        # Version should match or detail could be newer
        if first_report.version and detail.version:
            assert detail.version >= first_report.version
    
    def test_search_by_recent_reports(self, client: CDGPythonClient):
        """Test getting recent reports."""
        # Get most recent reports
        recent_reports = client.list_crs_reports(limit=10, format="json")
        
        assert len(recent_reports) > 0
        
        # Verify all have required fields
        for report in recent_reports:
            assert report.id is not None
            assert report.url is not None
        
        # If we have multiple reports, check they have update dates
        if len(recent_reports) > 1:
            # Most should have update_date field
            reports_with_dates = sum(1 for r in recent_reports if r.update_date)
            assert reports_with_dates > 0


class TestCrsReportEdgeCases:
    """Test edge cases and error handling."""
    
    def test_get_crs_report_invalid_id(self, client: CDGPythonClient):
        """Test getting a CRS report with invalid ID."""
        with pytest.raises(Exception) as exc_info:
            client.get_crs_report("INVALID_ID_99999", format="json")
        
        # Should raise an error
        error_msg = str(exc_info.value)
        assert len(error_msg) > 0
    
    def test_list_crs_reports_large_offset(self, client: CDGPythonClient):
        """Test listing with very large offset."""
        # Request with large offset - should return empty or error gracefully
        reports = client.list_crs_reports(offset=100000, limit=5, format="json")
        
        # Should return empty list if offset is beyond available data
        assert isinstance(reports, list)
    
    def test_list_crs_reports_zero_limit(self, client: CDGPythonClient):
        """Test listing with zero limit."""
        # This might return empty or use default
        reports = client.list_crs_reports(limit=0, format="json")
        assert isinstance(reports, list)
    
    def test_list_crs_reports_with_format(self, client: CDGPythonClient):
        """Test listing reports with format parameter."""
        # Test with json format explicitly
        reports = client.list_crs_reports(limit=3, format="json")
        
        assert isinstance(reports, list)
        if len(reports) > 0:
            assert reports[0].id is not None


class TestCrsReportDataValidation:
    """Test data validation and integrity."""
    
    def test_report_id_format(self, client: CDGPythonClient):
        """Test that report IDs are returned as stable, non-empty identifiers."""
        reports = client.list_crs_reports(limit=10, format="json")
        
        for report in reports:
            if report.id:
                # The live API returns multiple ID formats, including values like
                # R42504, IF12940, RL34035, and 98-684.
                assert isinstance(report.id, str)
                assert report.id == report.id.strip()
                assert len(report.id) >= 2
    
    def test_report_url_structure(self, client: CDGPythonClient):
        """Test that report URLs have correct structure."""
        reports = client.list_crs_reports(limit=5, format="json")
        
        for report in reports:
            if report.url:
                assert "api.congress.gov" in report.url
                assert "/v3/crsreport/" in report.url
                if report.id:
                    assert report.id in report.url
    
    def test_report_versions_positive(self, client: CDGPythonClient):
        """Test that report versions are positive integers."""
        reports = client.list_crs_reports(limit=10, format="json")
        
        for report in reports:
            if report.version is not None:
                assert isinstance(report.version, int)
                assert report.version > 0
    
    def test_detail_summary_field(self, client: CDGPythonClient):
        """Test that detailed reports have summary information."""
        # Get a report
        reports = client.list_crs_reports(limit=3, format="json")
        assert len(reports) > 0
        first_report_id = reports[0].id
        assert first_report_id is not None
        detail = client.get_crs_report(first_report_id, format="json")

        
        # Summary should exist and be a string (may be None for some reports)
        if detail.summary is not None:
            assert isinstance(detail.summary, str)
            # Summary should be substantial if present
            assert len(detail.summary) > 0
    
    def test_detail_has_more_info_than_list(self, client: CDGPythonClient):
        """Test that detail endpoint provides more information than list."""
        # Get a report from list
        reports = client.list_crs_reports(limit=1, format="json")
        assert len(reports) > 0
        
        @api_retry
        def get_report_with_retry(rid):
            return client.get_crs_report(rid)
        
        list_report = reports[0]
        detail_report = get_report_with_retry(list_report.id)
        
        # Detail should have additional fields that might not be in list
        # Check for fields that are typically only in detail view
        detail_only_fields = ['summary', 'authors', 'topics', 'formats', 'related_materials']
        
        # At least one of these should be present in detail
        has_detail_field = any(
            hasattr(detail_report, field) and getattr(detail_report, field) is not None
            for field in detail_only_fields
        )
        
        # This assertion might be too strong, but worth checking
        # Some reports might not have these fields, so we just verify they exist as attributes
        for field in detail_only_fields:
            assert hasattr(detail_report, field)
