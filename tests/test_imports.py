"""Test basic import statements for cdg_python_client."""

import pytest


def test_import_main_client():
    """Test that CDGPythonClient can be imported."""
    from cdg_python_client import CDGPythonClient
    assert CDGPythonClient is not None


def test_import_all_models():
    """Test that all model classes can be imported."""
    from cdg_python_client import (
        Bill,
        BillDetail,
        LatestAction,
        Law,
        Sponsor,
        PolicyArea,
        Action,
        Amendment,
        Committee,
        Cosponsor,
        RelatedBill,
        RelationshipDetail,
        Subject,
        Summary,
        TextVersion,
        TextFormat,
        Title,
    )
    
    # Verify all classes are not None
    assert Bill is not None
    assert BillDetail is not None
    assert LatestAction is not None
    assert Law is not None
    assert Sponsor is not None
    assert PolicyArea is not None
    assert Action is not None
    assert Amendment is not None
    assert Committee is not None
    assert Cosponsor is not None
    assert RelatedBill is not None
    assert RelationshipDetail is not None
    assert Subject is not None
    assert Summary is not None
    assert TextVersion is not None
    assert TextFormat is not None
    assert Title is not None


def test_client_instantiation():
    """Test that CDGPythonClient can be instantiated."""
    from cdg_python_client import CDGPythonClient
    
    client = CDGPythonClient(api_key="test_key")
    assert client is not None


def test_client_has_bill_methods():
    """Test that CDGPythonClient has all bill-related methods."""
    from cdg_python_client import CDGPythonClient
    
    client = CDGPythonClient(api_key="test_key")
    
    # Check for bill methods
    assert hasattr(client, "list_bills")
    assert hasattr(client, "list_bills_by_congress")
    assert hasattr(client, "list_bills_by_type")
    assert hasattr(client, "get_bill")
    assert hasattr(client, "get_bill_actions")
    assert hasattr(client, "get_bill_amendments")
    assert hasattr(client, "get_bill_committees")
    assert hasattr(client, "get_bill_cosponsors")
    assert hasattr(client, "get_related_bills")
    assert hasattr(client, "get_bill_subjects")
    assert hasattr(client, "get_bill_summaries")
    assert hasattr(client, "get_bill_text")
    assert hasattr(client, "get_bill_titles")


def test_client_has_amendment_methods():
    """Test that CDGPythonClient has amendment-related methods."""
    from cdg_python_client import CDGPythonClient
    
    client = CDGPythonClient(api_key="test_key")
    
    # Check for amendment methods
    assert hasattr(client, "list_amendments")
    assert hasattr(client, "list_amendments_by_congress")


def test_client_has_member_methods():
    """Test that CDGPythonClient has member-related methods."""
    from cdg_python_client import CDGPythonClient
    
    client = CDGPythonClient(api_key="test_key")
    
    # Check for member methods
    assert hasattr(client, "list_members")
    assert hasattr(client, "get_member")
    assert hasattr(client, "list_members_by_congress")
    assert hasattr(client, "get_member_sponsored_legislation")
    assert hasattr(client, "get_member_cosponsored_legislation")


def test_client_has_committee_methods():
    """Test that CDGPythonClient has committee-related methods."""
    from cdg_python_client import CDGPythonClient
    
    client = CDGPythonClient(api_key="test_key")
    
    # Check for committee methods
    assert hasattr(client, "list_committees")


def test_all_exports():
    """Test that __all__ contains all expected exports."""
    import cdg_python_client
    
    expected_exports = [
        "CDGPythonClient",
        "Bill",
        "BillDetail",
        "LatestAction",
        "Law",
        "Sponsor",
        "PolicyArea",
        "Action",
        "Amendment",
        "Committee",
        "Cosponsor",
        "RelatedBill",
        "RelationshipDetail",
        "Subject",
        "Summary",
        "TextVersion",
        "TextFormat",
        "Title",
    ]
    
    for export in expected_exports:
        assert export in cdg_python_client.__all__
        assert hasattr(cdg_python_client, export)
