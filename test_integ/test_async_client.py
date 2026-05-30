"""Integration tests for the async client surface."""

import pytest

from cdg_python_client import ApiPage


@pytest.mark.asyncio
async def test_async_list_bills_and_retry_config(async_client):
    """Async list operations should work with configured retry settings."""
    async_client.configure_retries(4, 0)

    assert async_client.get_retry_config() == (4, 0)

    bills = await async_client.list_bills(limit=2)

    assert isinstance(bills, list)
    assert 0 < len(bills) <= 2
    assert hasattr(bills[0], "bill_type")
    assert hasattr(bills[0], "number")


@pytest.mark.asyncio
async def test_async_fetch_page_and_iter_items(async_client):
    """Async pagination helpers should fetch pages and flatten items."""
    page = await async_client.fetch_page("/bill", limit=2)

    assert isinstance(page, ApiPage)
    assert isinstance(page.items, list)
    assert len(page.items) > 0

    items = []
    async for bill in async_client.iter_items("/bill", limit=2, max_items=3):
        items.append(bill)

    assert 0 < len(items) <= 3
    assert all(isinstance(item, dict) for item in items)
    assert all("url" in item for item in items)


@pytest.mark.asyncio
async def test_async_follow_link_uses_detail_links(async_client):
    """Async follow_link should fetch a page from a raw next-page URL."""
    page = await async_client.fetch_page("/bill", limit=1)
    assert page.next_url is not None

    actions_page = await async_client.follow_link(page.next_url, limit=1)

    assert isinstance(actions_page, ApiPage)
    assert isinstance(actions_page.items, list)
    assert len(actions_page.items) > 0


@pytest.mark.asyncio
async def test_async_bill_detail_accepts_integer_bill_number(async_client):
    """Async bill detail helpers should normalize integer bill numbers."""
    bill = await async_client.get_bill(118, "hr", 1)

    assert bill is not None
    assert bill.bill_type is not None
    assert bill.bill_type.lower() == "hr"
    assert bill.number == "1"


@pytest.mark.asyncio
async def test_async_members_and_crs_reports(async_client):
    """Async client should cover multiple endpoint families beyond bills."""
    members = await async_client.list_members(limit=2, current_member=True)
    reports = await async_client.list_crs_reports(limit=2)

    assert isinstance(members, list)
    assert 0 < len(members) <= 2
    assert hasattr(members[0], "bioguide_id")

    assert isinstance(reports, list)
    assert 0 < len(reports) <= 2
    assert hasattr(reports[0], "id")


@pytest.mark.asyncio
async def test_async_committee_and_summary_families(async_client):
    """Async client should cover committee and summary endpoints."""
    reports = await async_client.list_committee_reports(limit=1)
    committees = await async_client.list_committees(limit=1)
    meetings = await async_client.list_committee_meetings(limit=1)
    summaries = await async_client.list_summaries(limit=1)

    assert isinstance(reports, list)
    assert 0 < len(reports) <= 1
    assert hasattr(reports[0], "citation")

    assert isinstance(committees, list)
    assert 0 < len(committees) <= 1
    assert hasattr(committees[0], "system_code")

    assert isinstance(meetings, list)
    assert 0 < len(meetings) <= 1
    assert hasattr(meetings[0], "event_id")

    assert isinstance(summaries, list)
    assert len(summaries) <= 1
    if summaries:
        assert hasattr(summaries[0], "action_date")


@pytest.mark.asyncio
async def test_async_nomination_treaty_and_record_families(async_client):
    """Async client should cover nomination, treaty, and record endpoints."""
    nominations = await async_client.list_nominations(limit=1)
    treaties = await async_client.list_treaties(limit=1)
    bound_records = await async_client.list_bound_congressional_records(limit=1)
    daily_records = await async_client.list_congressional_records(limit=1)

    assert isinstance(nominations, list)
    assert 0 < len(nominations) <= 1
    assert hasattr(nominations[0], "number")

    assert isinstance(treaties, list)
    assert 0 < len(treaties) <= 1
    assert hasattr(treaties[0], "number")

    assert isinstance(bound_records, list)
    assert 0 < len(bound_records) <= 1
    assert hasattr(bound_records[0], "date")

    assert isinstance(daily_records, list)
    assert 0 < len(daily_records) <= 1
    assert hasattr(daily_records[0], "issue_number")


@pytest.mark.asyncio
async def test_async_communication_and_requirement_families(async_client):
    """Async client should cover communications, requirements, and congress endpoints."""
    house_items = await async_client.list_house_communications(limit=1)
    senate_items = await async_client.list_senate_communications(limit=1)
    requirements = await async_client.list_house_requirements(limit=1)
    congress = await async_client.get_current_congress()

    assert isinstance(house_items, list)
    assert 0 < len(house_items) <= 1
    assert hasattr(house_items[0], "communication_type")

    assert isinstance(senate_items, list)
    assert 0 < len(senate_items) <= 1
    assert hasattr(senate_items[0], "communication_type")

    assert isinstance(requirements, list)
    assert 0 < len(requirements) <= 1
    assert hasattr(requirements[0], "number")

    assert congress is not None
    assert hasattr(congress, "name")


@pytest.mark.asyncio
async def test_async_typed_link_helpers(async_client):
    """Async typed link helpers should reuse typed endpoint models."""
    amendment = await async_client.get_amendment(118, "hamdt", "1")

    if amendment.actions and amendment.actions.url:
        actions = await async_client.get_amendment_actions_by_link(amendment.actions, limit=2)
        assert isinstance(actions, list)

    if amendment.text_versions and amendment.text_versions.url:
        text_versions = await async_client.get_amendment_text_by_link(
            amendment.text_versions,
            limit=2,
        )
        assert isinstance(text_versions, list)
