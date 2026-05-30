"""Integration tests for newly added Congress.gov endpoint coverage."""

from __future__ import annotations

from urllib.parse import urlparse, urlsplit, urlunsplit

import pytest

from cdg_python_client import CDGPythonClient


def _path_segments(url: str) -> list[str]:
    return [segment for segment in urlparse(url).path.split("/") if segment]


def _append_path(url: str, suffix: str) -> str:
    parsed = urlsplit(url)
    path = f"{parsed.path.rstrip('/')}/{suffix.strip('/')}"
    return urlunsplit((parsed.scheme, parsed.netloc, path, parsed.query, parsed.fragment))


class TestAdditionalAmendmentEndpoints:
    def test_amendment_detail_and_subresources(self, client: CDGPythonClient):
        amendments = client.list_amendments_by_type(118, "hamdt", limit=1, format="json")
        assert len(amendments) > 0

        amendment = amendments[0]
        assert amendment.congress == 118
        assert amendment.number is not None
        assert amendment.amendment_type is not None

        detail = client.get_amendment(118, amendment.amendment_type, amendment.number, format="json")
        assert detail.congress == 118
        assert detail.number == amendment.number

        actions = client.get_amendment_actions(118, amendment.amendment_type, amendment.number, limit=3, format="json")
        nested = client.get_amendment_amendments(118, amendment.amendment_type, amendment.number, limit=3, format="json")
        cosponsors = client.get_amendment_cosponsors(118, amendment.amendment_type, amendment.number, limit=3, format="json")
        text_versions = client.get_amendment_text(118, amendment.amendment_type, amendment.number, limit=3, format="json")

        assert isinstance(actions, list)
        assert isinstance(nested, list)
        assert isinstance(cosponsors, list)
        assert isinstance(text_versions, list)


class TestAdditionalCommitteeEndpoints:
    def test_committee_additional_paths(self, client: CDGPythonClient):
        committee = client.get_committee_by_congress(118, "house", "hsif00", format="json")
        assert committee.system_code == "hsif00"

        house_communications = client.get_committee_house_communications("house", "hsif00", limit=2, format="json")
        reports = client.get_committee_reports("house", "hsif00", limit=2, format="json")
        nominations = client.get_committee_nominations("senate", "sscm00", limit=2, format="json")
        senate_communications = client.get_committee_senate_communications("senate", "ssfr00", limit=2, format="json")

        assert isinstance(house_communications, list)
        assert isinstance(reports, list)
        assert isinstance(nominations, list)
        assert isinstance(senate_communications, list)

        if committee.reports and committee.reports.url:
            linked_reports = client.get_committee_reports_by_link(committee.reports, limit=2)
            assert isinstance(linked_reports, list)

        if committee.nominations and committee.nominations.url:
            linked_nominations = client.get_committee_nominations_by_link(
                committee.nominations,
                limit=2,
            )
            assert isinstance(linked_nominations, list)


class TestCommitteeMeetingEndpoints:
    def test_committee_meetings(self, client: CDGPythonClient):
        meetings = client.list_committee_meetings(limit=1, format="json")
        assert len(meetings) > 0

        meeting = meetings[0]
        assert meeting.congress is not None
        assert meeting.chamber is not None
        assert meeting.event_id is not None

        by_congress = client.list_committee_meetings_by_congress(meeting.congress, limit=1, format="json")
        by_chamber = client.list_committee_meetings_by_chamber(
            meeting.congress,
            meeting.chamber,
            limit=1,
            format="json",
        )
        detail = client.get_committee_meeting(meeting.congress, meeting.chamber, meeting.event_id, format="json")

        assert isinstance(by_congress, list)
        assert isinstance(by_chamber, list)
        assert detail.event_id == meeting.event_id


class TestCongressionalRecordEndpoints:
    def test_bound_daily_and_legacy_record_paths(self, client: CDGPythonClient):
        bound = client.list_bound_congressional_records(limit=1, format="json")
        assert len(bound) > 0
        record = bound[0]
        assert record.date is not None

        year, month, day = [int(part) for part in record.date.split("-")]

        assert isinstance(client.list_bound_congressional_records_by_year(year, limit=1, format="json"), list)
        assert isinstance(client.list_bound_congressional_records_by_month(year, month, limit=1, format="json"), list)
        assert isinstance(client.get_bound_congressional_record(year, month, day, limit=1, format="json"), list)

        daily = client.list_congressional_records(limit=1, format="json")
        assert len(daily) > 0
        issue = daily[0]
        assert issue.volume_number is not None
        assert issue.issue_number is not None

        by_volume = client.list_daily_congressional_records_by_volume(issue.volume_number, limit=1, format="json")
        issue_detail = client.get_daily_congressional_record_issue(issue.volume_number, issue.issue_number, format="json")
        articles = client.get_daily_congressional_record_articles(issue.volume_number, issue.issue_number, limit=2, format="json")
        legacy = client.list_congressional_record(limit=1, format="json")

        assert isinstance(by_volume, list)
        assert issue_detail.volume_number == issue.volume_number
        assert isinstance(articles, list)
        assert legacy.total_count is not None

        issue_articles = (
            None
            if issue_detail.full_issue is None
            else issue_detail.full_issue.articles
        )
        if issue_articles and issue_articles.url:
            linked_articles = client.get_daily_congressional_record_articles_by_link(
                issue_articles,
                limit=2,
            )
            assert isinstance(linked_articles, list)


class TestCommunicationAndRequirementEndpoints:
    def test_house_and_senate_communications(self, client: CDGPythonClient):
        house_items = client.list_house_communications(limit=1, format="json")
        senate_items = client.list_senate_communications(limit=1, format="json")

        assert len(house_items) > 0
        assert len(senate_items) > 0

        house_item = house_items[0]
        senate_item = senate_items[0]

        assert house_item.communication_type is not None
        assert senate_item.communication_type is not None

        assert isinstance(client.list_house_communications_by_congress(house_item.congress, limit=1, format="json"), list)
        assert isinstance(
            client.list_house_communications_by_type(
                house_item.congress,
                house_item.communication_type.code,
                limit=1,
                format="json",
            ),
            list,
        )
        assert isinstance(client.list_senate_communications_by_congress(senate_item.congress, limit=1, format="json"), list)
        assert isinstance(
            client.list_senate_communications_by_type(
                senate_item.congress,
                senate_item.communication_type.code,
                limit=1,
                format="json",
            ),
            list,
        )

        house_detail = client.get_house_communication(
            house_item.congress,
            house_item.communication_type.code,
            house_item.number,
            format="json",
        )
        senate_detail = client.get_senate_communication(
            senate_item.congress,
            senate_item.communication_type.code,
            senate_item.number,
            format="json",
        )

        assert house_detail.number == house_item.number
        assert senate_detail.number == senate_item.number

    def test_house_requirements(self, client: CDGPythonClient):
        requirements = client.list_house_requirements(limit=1, format="json")
        assert len(requirements) > 0

        requirement = requirements[0]
        detail = client.get_house_requirement(requirement.number, format="json")
        matching = client.get_house_requirement_matching_communications(requirement.number, limit=2, format="json")

        assert detail.number == requirement.number
        assert isinstance(matching, list)


class TestAdditionalMemberSummaryNominationTreatyEndpoints:
    def test_member_and_summary_filters(self, client: CDGPythonClient):
        members = client.list_members_by_congress_state_district(118, "ca", 12, limit=1, format="json")
        summaries = client.list_summaries_by_bill_type(118, "hr", limit=2, format="json")

        assert isinstance(members, list)
        assert len(members) > 0
        assert isinstance(summaries, list)

    def test_nomination_subresources(self, client: CDGPythonClient):
        nominations = client.list_nominations_by_congress(118, limit=3, format="json")
        assert len(nominations) > 0

        nomination = nominations[0]
        assert nomination.number is not None

        actions = client.get_nomination_actions(118, nomination.number, limit=2, format="json")
        committees = client.get_nomination_committees(118, nomination.number, format="json")
        hearings = client.get_nomination_hearings(118, nomination.number, limit=2, format="json")

        assert isinstance(actions, list)
        assert isinstance(committees, list)
        assert isinstance(hearings, list)

        ordinal_candidate = None
        ordinal_nominations = client.list_nominations_by_congress(119, limit=10, format="json")
        for item in ordinal_nominations:
            if item.part_number and item.part_number != "00":
                ordinal_candidate = item
                break

        if ordinal_candidate is None:
            pytest.skip("No nomination with a non-default ordinal was available.")

        nominees = client.get_nomination_ordinal(
            ordinal_candidate.congress,
            ordinal_candidate.number,
            ordinal_candidate.part_number,
            limit=2,
            format="json",
        )
        assert isinstance(nominees, list)

    def test_treaty_subresources(self, client: CDGPythonClient):
        treaties = client.list_treaties_by_congress(118, limit=2, format="json")
        assert len(treaties) > 0

        treaty = treaties[0]
        assert treaty.number is not None

        actions = client.get_treaty_actions(118, treaty.number, limit=2, format="json")
        committees = client.get_treaty_committees(118, treaty.number, format="json")

        assert isinstance(actions, list)
        assert isinstance(committees, list)

        if treaty.actions and treaty.actions.url:
            linked_actions = client.get_treaty_actions_by_link(treaty.actions, limit=2)
            assert isinstance(linked_actions, list)

        part_candidate = None
        for congress in [114, 113, 112]:
            items = client.list_treaties_by_congress(congress, limit=10, format="json")
            for item in items:
                if item.parts and item.parts.urls:
                    part_candidate = (congress, item.parts.urls[0])
                    break
            if part_candidate:
                break

        if part_candidate is None:
            pytest.skip("No treaty with part URLs was available.")

        congress, url = part_candidate
        segments = _path_segments(url)
        treaty_number = segments[-2]
        treaty_suffix = segments[-1]

        part = client.get_treaty_part(congress, treaty_number, treaty_suffix, format="json")
        part_actions = client.get_treaty_part_actions(congress, treaty_number, treaty_suffix, limit=2, format="json")
        linked_part = client.get_treaty_part_by_link(url)
        linked_part_actions = client.get_treaty_part_actions_by_link(
            f"https://api.congress.gov/v3/treaty/{congress}/{treaty_number}/{treaty_suffix}/actions?format=json",
            limit=2,
        )

        assert isinstance(part, list)
        assert isinstance(part_actions, list)
        assert isinstance(linked_part, list)
        assert isinstance(linked_part_actions, list)
