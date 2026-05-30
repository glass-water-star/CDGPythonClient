"""
Congress.gov API Python Client

A Python client for the Congress.gov API, implemented in Rust using PyO3 for high performance.
"""

from typing import (
    Any,
    AsyncIterator,
    Callable,
    Iterator,
    List,
    Mapping,
    Optional,
    Protocol,
    Sequence,
    Union,
)

BillNumberLike = Union[str, int]
JsonScalar = Union[str, int, float, bool, None]
JsonValue = Union[JsonScalar, Mapping[str, "JsonValue"], List["JsonValue"]]
PageItem = Mapping[str, JsonValue]
PageItems = List[PageItem]
RawApiResponse = Mapping[str, JsonValue]
LogLevel = Union[int, str]

class LoggerLike(Protocol):
    def log(self, level: int, msg: str, *args: object, **kwargs: object) -> object: ...

class UrlLink(Protocol):
    url: Optional[str]

class UrlListLink(Protocol):
    urls: Optional[Sequence[str]]

LinkLike = Union[str, UrlLink, UrlListLink]
LogTarget = Union[Callable[[Mapping[str, object]], object], LoggerLike]

class CDGClientError(Exception): ...
class CDGConfigurationError(CDGClientError): ...
class CDGInvalidUrlError(CDGClientError): ...
class CDGRequestError(CDGClientError): ...
class CDGHttpError(CDGClientError): ...
class CDGAuthError(CDGHttpError): ...
class CDGNotFoundError(CDGHttpError): ...
class CDGRateLimitError(CDGHttpError): ...
class CDGServerError(CDGHttpError): ...
class CDGDeserializationError(CDGClientError): ...

class LatestAction:
    """Represents the latest action taken on a bill."""
    action_date: Optional[str]
    text: Optional[str]
    
    def __repr__(self) -> str: ...

class Law:
    """Represents a law number and type."""
    number: Optional[str]
    law_type: Optional[str]
    
    def __repr__(self) -> str: ...

class Sponsor:
    """Represents a bill sponsor."""
    bioguide_id: Optional[str]
    first_name: Optional[str]
    last_name: Optional[str]
    full_name: Optional[str]
    state: Optional[str]
    party: Optional[str]
    url: Optional[str]
    
    def __repr__(self) -> str: ...

class PolicyArea:
    """Represents a policy area."""
    name: Optional[str]
    
    def __repr__(self) -> str: ...

class Bill:
    """Represents a bill in Congress."""
    congress: Optional[int]
    latest_action: Optional[LatestAction]
    number: Optional[str]
    origin_chamber: Optional[str]
    origin_chamber_code: Optional[str]
    title: Optional[str]
    bill_type: Optional[str]
    update_date: Optional[str]
    update_date_including_text: Optional[str]
    url: Optional[str]
    
    def __repr__(self) -> str: ...

class BillDetail:
    """Represents detailed information about a bill."""
    congress: Optional[int]
    latest_action: Optional[LatestAction]
    number: Optional[str]
    origin_chamber: Optional[str]
    origin_chamber_code: Optional[str]
    title: Optional[str]
    bill_type: Optional[str]
    update_date: Optional[str]
    update_date_including_text: Optional[str]
    url: Optional[str]
    introduced_date: Optional[str]
    sponsors: Optional[List[Sponsor]]
    policy_area: Optional[PolicyArea]
    laws: Optional[List[Law]]
    
    def __repr__(self) -> str: ...

class LawItem:
    """Represents a bill that became a law."""
    congress: Optional[int]
    latest_action: Optional[LatestAction]
    laws: Optional[List[Law]]
    number: Optional[str]
    origin_chamber: Optional[str]
    origin_chamber_code: Optional[str]
    title: Optional[str]
    law_type: Optional[str]
    update_date: Optional[str]
    update_date_including_text: Optional[str]
    url: Optional[str]
    
    def __repr__(self) -> str: ...

class LawDetail:
    """Represents detailed information about a law."""
    congress: Optional[int]
    latest_action: Optional[LatestAction]
    laws: Optional[List[Law]]
    number: Optional[str]
    origin_chamber: Optional[str]
    origin_chamber_code: Optional[str]
    title: Optional[str]
    law_type: Optional[str]
    update_date: Optional[str]
    update_date_including_text: Optional[str]
    url: Optional[str]
    
    def __repr__(self) -> str: ...

class Action:
    """Represents an action taken on a bill."""
    action_code: Optional[str]
    action_date: Optional[str]
    text: Optional[str]
    action_type: Optional[str]
    
    def __repr__(self) -> str: ...

class Amendment:
    """Represents an amendment to a bill."""
    congress: Optional[int]
    latest_action: Optional[LatestAction]
    number: Optional[str]
    amendment_type: Optional[str]
    url: Optional[str]
    
    def __repr__(self) -> str: ...

class Committee:
    """Represents a congressional committee."""
    name: Optional[str]
    system_code: Optional[str]
    url: Optional[str]
    
    def __repr__(self) -> str: ...

class CommitteeBill:
    """Represents a bill relationship returned from committee endpoints."""
    action_date: Optional[str]
    congress: Optional[int]
    number: Optional[str]
    relationship_type: Optional[str]
    bill_type: Optional[str]
    update_date: Optional[str]
    url: Optional[str]

    def __repr__(self) -> str: ...

class CrsReport:
    """Represents a CRS report in list responses."""
    content_type: Optional[str]
    id: Optional[str]
    publish_date: Optional[str]
    status: Optional[str]
    title: Optional[str]
    update_date: Optional[str]
    url: Optional[str]
    version: Optional[int]
    
    def __repr__(self) -> str: ...

class CrsReportAuthor:
    """Represents an author of a CRS report."""
    author: Optional[str]
    
    def __repr__(self) -> str: ...

class CrsReportFormat:
    """Represents a format in which a CRS report is available."""
    format: Optional[str]
    url: Optional[str]
    
    def __repr__(self) -> str: ...

class CrsReportTopic:
    """Represents a topic of a CRS report."""
    topic: Optional[str]
    
    def __repr__(self) -> str: ...

class CrsReportRelatedMaterial:
    """Represents related material for a CRS report."""
    url: Optional[str]
    congress: Optional[int]
    number: Optional[str]
    title: Optional[str]
    material_type: Optional[str]
    
    def __repr__(self) -> str: ...

class CrsReportDetail:
    """Represents detailed information about a CRS report."""
    authors: Optional[List[CrsReportAuthor]]
    content_type: Optional[str]
    formats: Optional[List[CrsReportFormat]]
    id: Optional[str]
    publish_date: Optional[str]
    related_materials: Optional[List[CrsReportRelatedMaterial]]
    status: Optional[str]
    summary: Optional[str]
    title: Optional[str]
    topics: Optional[List[CrsReportTopic]]
    update_date: Optional[str]
    url: Optional[str]
    version: Optional[int]
    
    def __repr__(self) -> str: ...

class HearingDate:
    """Represents a hearing date."""
    date: Optional[str]
    
    def __repr__(self) -> str: ...

class AssociatedMeeting:
    """Represents an associated meeting for a hearing."""
    event_id: Optional[str]
    url: Optional[str]
    
    def __repr__(self) -> str: ...

class HearingFormat:
    """Represents a format option for a hearing."""
    format_type: Optional[str]
    url: Optional[str]
    
    def __repr__(self) -> str: ...

class HearingCommittee:
    """Represents a committee/subcommittee in a hearing."""
    name: Optional[str]
    system_code: Optional[str]
    url: Optional[str]
    
    def __repr__(self) -> str: ...

class Hearing:
    """Represents a congressional hearing."""
    chamber: Optional[str]
    congress: Optional[int]
    jacket_number: Optional[int]
    number: Optional[int]
    part: Optional[int]
    title: Optional[str]
    update_date: Optional[str]
    url: Optional[str]
    associated_meeting: Optional[AssociatedMeeting]
    citation: Optional[str]
    committees: Optional[List[HearingCommittee]]
    dates: Optional[List[HearingDate]]
    formats: Optional[List[HearingFormat]]
    library_of_congress_identifier: Optional[str]
    
    def __repr__(self) -> str: ...

class Cosponsor:
    """Represents a bill cosponsor."""
    bioguide_id: Optional[str]
    first_name: Optional[str]
    last_name: Optional[str]
    full_name: Optional[str]
    state: Optional[str]
    party: Optional[str]
    sponsorship_date: Optional[str]
    is_original_cosponsor: Optional[bool]
    
    def __repr__(self) -> str: ...

class RelationshipDetail:
    """Represents details about bill relationships."""
    identified_by: Optional[str]
    relationship_type: Optional[str]
    
    def __repr__(self) -> str: ...

class RelatedBill:
    """Represents a related bill."""
    congress: Optional[int]
    number: Optional[str]
    bill_type: Optional[str]
    title: Optional[str]
    url: Optional[str]
    relationship_details: Optional[List[RelationshipDetail]]
    
    def __repr__(self) -> str: ...

class Subject:
    """Represents a legislative subject."""
    name: Optional[str]
    update_date: Optional[str]
    
    def __repr__(self) -> str: ...

class Summary:
    """Represents a bill summary."""
    action_date: Optional[str]
    action_desc: Optional[str]
    text: Optional[str]
    update_date: Optional[str]
    version_code: Optional[str]
    
    def __repr__(self) -> str: ...

class TextFormat:
    """Represents a text format for bill text."""
    format_type: Optional[str]
    url: Optional[str]
    
    def __repr__(self) -> str: ...

class TextVersion:
    """Represents a text version of a bill."""
    date: Optional[str]
    text_type: Optional[str]
    formats: Optional[List[TextFormat]]
    
    def __repr__(self) -> str: ...

class Title:
    """Represents a bill title."""
    title: Optional[str]
    title_type: Optional[str]
    title_type_code: Optional[int]
    
    def __repr__(self) -> str: ...

class ApiPage:
    """Represents a raw Congress.gov response page with pagination metadata."""
    items: PageItems
    raw_response: RawApiResponse
    item_key: Optional[str]
    count: Optional[int]
    next_url: Optional[str]
    previous_url: Optional[str]
    offset: Optional[int]
    limit: Optional[int]

    def has_next(self) -> bool: ...
    def __repr__(self) -> str: ...

class CDGPythonClient:
    """
    Client for interacting with the Congress.gov API.
    
    Example:
        >>> client = CDGPythonClient(api_key="your_api_key")
        >>> bills = client.list_bills(limit=10)
        >>> bill = client.get_bill(congress=118, bill_type="hr", bill_number=1)
        >>> members = client.list_members(limit=10, current_member=True)
    """
    def __init__(
        self,
        api_key: str,
        timeout_seconds: Optional[float] = None,
        user_agent: Optional[str] = None,
    ) -> None:
        """
        Initialize the Congress.gov API client.
        
        Args:
            api_key: Your Congress.gov API key
            timeout_seconds: Optional per-request timeout in seconds
            user_agent: Optional custom HTTP User-Agent header
        """
        ...

    def configure_timeout(self, timeout_seconds: Optional[float] = None) -> None:
        """Set or clear the per-request timeout in seconds for this client."""
        ...

    def get_timeout(self) -> Optional[float]:
        """Return the configured per-request timeout in seconds, if any."""
        ...

    def configure_user_agent(self, user_agent: Optional[str] = None) -> None:
        """Set or clear the custom HTTP User-Agent header for this client."""
        ...

    def get_user_agent(self) -> Optional[str]:
        """Return the configured custom HTTP User-Agent header, if any."""
        ...

    def configure_logging(
        self,
        target: Optional[LogTarget] = None,
        *,
        level: LogLevel = "INFO",
    ) -> None:
        """
        Enable optional per-client request logging using a logger-like object or callback.

        Callback/log payloads include structured fields such as `event`, `method`, redacted `url`,
        `path`, `attempt`, and, when available, `status_code`, `elapsed_ms`, `error`, `offset`,
        and `limit`. The API key is never included in logged URLs.
        """
        ...

    def disable_logging(self) -> None:
        """Disable optional request logging for this client."""
        ...

    def is_logging_enabled(self) -> bool:
        """Return whether optional request logging is currently enabled."""
        ...

    def fetch_page(
        self,
        path_or_url: str,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
    ) -> ApiPage:
        """Fetch a raw Congress.gov response page from a relative API path or absolute API URL."""
        ...

    def fetch_pages(
        self,
        path_or_url: str,
        *,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
        max_pages: Optional[int] = None,
    ) -> Iterator[ApiPage]:
        """Yield successive pages for a relative API path or absolute API URL."""
        ...

    def iter_items(
        self,
        path_or_url: str,
        *,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
        max_pages: Optional[int] = None,
        max_items: Optional[int] = None,
    ) -> Iterator[PageItem]:
        """Yield raw JSON-like items across successive pages for a relative API path or absolute API URL."""
        ...

    def follow_link(
        self,
        link_or_url: LinkLike,
        *,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
        url_index: int = 0,
    ) -> ApiPage:
        """Follow a raw URL string or an object exposing `url` or `urls`."""
        ...

    def get_amendment_actions_by_link(
        self,
        link_or_url: LinkLike,
        *,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
        url_index: int = 0,
    ) -> List[Action]: ...

    def get_amendment_text_by_link(
        self,
        link_or_url: LinkLike,
        *,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
        url_index: int = 0,
    ) -> List[TextVersion]: ...

    def get_committee_bills_by_link(
        self,
        link_or_url: LinkLike,
        *,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
        url_index: int = 0,
    ) -> List[CommitteeBill]: ...

    def get_committee_reports_by_link(
        self,
        link_or_url: LinkLike,
        *,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
        url_index: int = 0,
    ) -> List[CommitteeReportItem]: ...

    def get_committee_nominations_by_link(
        self,
        link_or_url: LinkLike,
        *,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
        url_index: int = 0,
    ) -> List[Nomination]: ...

    def get_committee_report_text_by_link(
        self,
        link_or_url: LinkLike,
        *,
        url_index: int = 0,
    ) -> List[CommitteeReportText]: ...

    def get_committee_print_text_by_link(
        self,
        link_or_url: LinkLike,
        *,
        url_index: int = 0,
    ) -> List[CommitteePrintText]: ...

    def get_daily_congressional_record_articles_by_link(
        self,
        link_or_url: LinkLike,
        *,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
        url_index: int = 0,
    ) -> List[DailyCongressionalRecordArticle]: ...

    def get_treaty_actions_by_link(
        self,
        link_or_url: LinkLike,
        *,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
        url_index: int = 0,
    ) -> List[Action]: ...

    def get_treaty_committees_by_link(
        self,
        link_or_url: LinkLike,
        *,
        url_index: int = 0,
    ) -> List[NominationCommittee]: ...

    def get_treaty_part_by_link(
        self,
        link_or_url: LinkLike,
        *,
        url_index: int = 0,
    ) -> List[Treaty]: ...

    def get_treaty_part_actions_by_link(
        self,
        link_or_url: LinkLike,
        *,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
        url_index: int = 0,
    ) -> List[Action]: ...

class AsyncCDGPythonClient:
    """
    Async facade for interacting with the Congress.gov API.

    This client preserves the synchronous API shape while exposing awaitable methods.
    """

    sync_client: CDGPythonClient

    def __init__(
        self,
        api_key: str,
        timeout_seconds: Optional[float] = None,
        user_agent: Optional[str] = None,
    ) -> None: ...

    def configure_retries(self, retry_attempts: int, retry_base_delay_ms: int) -> None: ...

    def get_retry_config(self) -> tuple[int, int]: ...

    def configure_timeout(self, timeout_seconds: Optional[float] = None) -> None: ...

    def get_timeout(self) -> Optional[float]: ...

    def configure_user_agent(self, user_agent: Optional[str] = None) -> None: ...

    def get_user_agent(self) -> Optional[str]: ...

    def configure_logging(
        self,
        target: Optional[LogTarget] = None,
        *,
        level: LogLevel = "INFO",
    ) -> None:
        """
        Enable optional per-client request logging using a logger-like object or callback.

        Callback/log payloads include structured fields such as `event`, `method`, redacted `url`,
        `path`, `attempt`, and, when available, `status_code`, `elapsed_ms`, `error`, `offset`,
        and `limit`. The API key is never included in logged URLs.
        """
        ...

    def disable_logging(self) -> None: ...

    def is_logging_enabled(self) -> bool: ...

    async def fetch_page(
        self,
        path_or_url: str,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
    ) -> ApiPage: ...

    async def fetch_pages(
        self,
        path_or_url: str,
        *,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
        max_pages: Optional[int] = None,
    ) -> AsyncIterator[ApiPage]: ...

    async def iter_items(
        self,
        path_or_url: str,
        *,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
        max_pages: Optional[int] = None,
        max_items: Optional[int] = None,
    ) -> AsyncIterator[PageItem]: ...

    async def follow_link(
        self,
        link_or_url: LinkLike,
        *,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
        url_index: int = 0,
    ) -> ApiPage: ...

    async def get_amendment_actions_by_link(
        self,
        link_or_url: LinkLike,
        *,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
        url_index: int = 0,
    ) -> List[Action]: ...

    async def get_amendment_text_by_link(
        self,
        link_or_url: LinkLike,
        *,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
        url_index: int = 0,
    ) -> List[TextVersion]: ...

    async def get_committee_bills_by_link(
        self,
        link_or_url: LinkLike,
        *,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
        url_index: int = 0,
    ) -> List[CommitteeBill]: ...

    async def get_committee_reports_by_link(
        self,
        link_or_url: LinkLike,
        *,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
        url_index: int = 0,
    ) -> List[CommitteeReportItem]: ...

    async def get_committee_nominations_by_link(
        self,
        link_or_url: LinkLike,
        *,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
        url_index: int = 0,
    ) -> List[Nomination]: ...

    async def get_committee_report_text_by_link(
        self,
        link_or_url: LinkLike,
        *,
        url_index: int = 0,
    ) -> List[CommitteeReportText]: ...

    async def get_committee_print_text_by_link(
        self,
        link_or_url: LinkLike,
        *,
        url_index: int = 0,
    ) -> List[CommitteePrintText]: ...

    async def get_daily_congressional_record_articles_by_link(
        self,
        link_or_url: LinkLike,
        *,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
        url_index: int = 0,
    ) -> List[DailyCongressionalRecordArticle]: ...

    async def get_treaty_actions_by_link(
        self,
        link_or_url: LinkLike,
        *,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
        url_index: int = 0,
    ) -> List[Action]: ...

    async def get_treaty_committees_by_link(
        self,
        link_or_url: LinkLike,
        *,
        url_index: int = 0,
    ) -> List[NominationCommittee]: ...

    async def get_treaty_part_by_link(
        self,
        link_or_url: LinkLike,
        *,
        url_index: int = 0,
    ) -> List[Treaty]: ...

    async def get_treaty_part_actions_by_link(
        self,
        link_or_url: LinkLike,
        *,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
        url_index: int = 0,
    ) -> List[Action]: ...

    def __dir__(self) -> List[str]: ...

    def __getattr__(self, name: str) -> object: ...
    
    async def list_bills(
        self,
        format: Optional[str] = None,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
        from_date_time: Optional[str] = None,
        to_date_time: Optional[str] = None,
    ) -> List[Bill]:
        """
        Get a list of bills sorted by date of latest action.
        
        Args:
            format: Response format (json or xml)
            offset: Offset for pagination
            limit: Number of results to return (max 250)
            from_date_time: Start date-time filter (ISO 8601)
            to_date_time: End date-time filter (ISO 8601)
            
        Returns:
            List of Bill objects
        """
        ...
    
    async def list_bills_by_congress(
        self,
        congress: int,
        format: Optional[str] = None,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
        from_date_time: Optional[str] = None,
        to_date_time: Optional[str] = None,
    ) -> List[Bill]:
        """
        Get bills filtered by congress number.
        
        Args:
            congress: Congress number (e.g., 118)
            format: Response format (json or xml)
            offset: Offset for pagination
            limit: Number of results to return (max 250)
            from_date_time: Start date-time filter (ISO 8601)
            to_date_time: End date-time filter (ISO 8601)
            
        Returns:
            List of Bill objects
        """
        ...
    
    async def list_bills_by_type(
        self,
        congress: int,
        bill_type: str,
        format: Optional[str] = None,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
        from_date_time: Optional[str] = None,
        to_date_time: Optional[str] = None,
    ) -> List[Bill]:
        """
        Get bills filtered by congress and bill type.
        
        Args:
            congress: Congress number (e.g., 118)
            bill_type: Bill type (hr, s, hjres, sjres, hconres, sconres, hres, sres)
            format: Response format (json or xml)
            offset: Offset for pagination
            limit: Number of results to return (max 250)
            from_date_time: Start date-time filter (ISO 8601)
            to_date_time: End date-time filter (ISO 8601)
            
        Returns:
            List of Bill objects
        """
        ...
    
    async def get_bills(
        self,
        congress: Optional[int] = None,
        bill_type: Optional[str] = None,
        format: Optional[str] = None,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
        from_date_time: Optional[str] = None,
        to_date_time: Optional[str] = None,
    ) -> List[Bill]:
        """
        Get a list of bills with optional filtering by congress and/or bill type.
        
        Args:
            congress: Congress number (e.g., 118) (optional)
            bill_type: Bill type (hr, s, hjres, sjres, hconres, sconres, hres, sres) (optional)
            format: Response format (json or xml)
            offset: Offset for pagination
            limit: Number of results to return (max 250)
            from_date_time: Start date-time filter (ISO 8601)
            to_date_time: End date-time filter (ISO 8601)
            
        Returns:
            List of Bill objects
        """
        ...
    
    async def get_bill(
        self,
        congress: int,
        bill_type: str,
        bill_number: BillNumberLike,
    ) -> BillDetail:
        """
        Get detailed information for a specific bill.
        
        Args:
            congress: Congress number (e.g., 118)
            bill_type: Bill type (hr, s, hjres, sjres, hconres, sconres, hres, sres)
            bill_number: Bill number
            format: Response format (json or xml)
        Returns:
            BillDetail object with comprehensive bill information
        """
        ...
    
    async def get_bill_actions(
        self,
        congress: int,
        bill_type: str,
        bill_number: BillNumberLike,
        format: Optional[str] = None,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
    ) -> List[Action]:
        """
        Get the list of actions on a specified bill.
        
        Args:
            congress: Congress number (e.g., 118)
            bill_type: Bill type (hr, s, hjres, sjres, hconres, sconres, hres, sres)
            bill_number: Bill number
            format: Response format (json or xml)
            offset: Offset for pagination
            limit: Number of results to return (max 250)
            
        Returns:
            List of Action objects
        """
        ...
    
    async def get_bill_amendments(
        self,
        congress: int,
        bill_type: str,
        bill_number: BillNumberLike,
        format: Optional[str] = None,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
    ) -> List[Amendment]:
        """
        Get the list of amendments to a specified bill.
        
        Args:
            congress: Congress number (e.g., 118)
            bill_type: Bill type (hr, s, hjres, sjres, hconres, sconres, hres, sres)
            bill_number: Bill number
            format: Response format (json or xml)
            offset: Offset for pagination
            limit: Number of results to return (max 250)
            
        Returns:
            List of Amendment objects
        """
        ...
    
    async def get_bill_committees(
        self,
        congress: int,
        bill_type: str,
        bill_number: BillNumberLike,
        format: Optional[str] = None,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
    ) -> List[Committee]:
        """
        Get the list of committees associated with a specified bill.
        
        Args:
            congress: Congress number (e.g., 118)
            bill_type: Bill type (hr, s, hjres, sjres, hconres, sconres, hres, sres)
            bill_number: Bill number
            format: Response format (json or xml)
            offset: Offset for pagination
            limit: Number of results to return (max 250)
            
        Returns:
            List of Committee objects
        """
        ...
    
    async def get_bill_cosponsors(
        self,
        congress: int,
        bill_type: str,
        bill_number: BillNumberLike,
        format: Optional[str] = None,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
    ) -> List[Cosponsor]:
        """
        Get the list of cosponsors on a specified bill.
        
        Args:
            congress: Congress number (e.g., 118)
            bill_type: Bill type (hr, s, hjres, sjres, hconres, sconres, hres, sres)
            bill_number: Bill number
            format: Response format (json or xml)
            offset: Offset for pagination
            limit: Number of results to return (max 250)
            
        Returns:
            List of Cosponsor objects
        """
        ...
    
    async def get_related_bills(
        self,
        congress: int,
        bill_type: str,
        bill_number: BillNumberLike,
        format: Optional[str] = None,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
    ) -> List[RelatedBill]:
        """
        Get the list of related bills to a specified bill.
        
        Args:
            congress: Congress number (e.g., 118)
            bill_type: Bill type (hr, s, hjres, sjres, hconres, sconres, hres, sres)
            bill_number: Bill number
            format: Response format (json or xml)
            offset: Offset for pagination
            limit: Number of results to return (max 250)
            
        Returns:
            List of RelatedBill objects
        """
        ...
    
    async def get_bill_subjects(
        self,
        congress: int,
        bill_type: str,
        bill_number: BillNumberLike,
        format: Optional[str] = None,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
    ) -> List[Subject]:
        """
        Get the list of legislative subjects on a specified bill.
        
        Args:
            congress: Congress number (e.g., 118)
            bill_type: Bill type (hr, s, hjres, sjres, hconres, sconres, hres, sres)
            bill_number: Bill number
            format: Response format (json or xml)
            offset: Offset for pagination
            limit: Number of results to return (max 250)
            
        Returns:
            List of Subject objects
        """
        ...
    
    async def get_bill_summaries(
        self,
        congress: int,
        bill_type: str,
        bill_number: BillNumberLike,
        format: Optional[str] = None,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
    ) -> List[Summary]:
        """
        Get the list of summaries for a specified bill.
        
        Args:
            congress: Congress number (e.g., 118)
            bill_type: Bill type (hr, s, hjres, sjres, hconres, sconres, hres, sres)
            bill_number: Bill number
            format: Response format (json or xml)
            offset: Offset for pagination
            limit: Number of results to return (max 250)
            
        Returns:
            List of Summary objects
        """
        ...
    
    async def get_bill_detail(
        self,
        congress: int,
        bill_type: str,
        bill_number: BillNumberLike,
        format: Optional[str] = None,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
    ) -> BillDetail:
        """
        Get the list of detailed information for a specified bill.
        
        Args:
            congress: Congress number (e.g., 118)
            bill_type: Bill type (hr, s, hjres, sjres, hconres, sconres, hres, sres)
            bill_number: Bill number
            format: Response format (json or xml)
            offset: Offset for pagination
            limit: Number of results to return (max 250)
            
        Returns:
            List of BillDetail objects
        """
        ...
    
    async def get_bill_titles(
        self,
        congress: int,
        bill_type: str,
        bill_number: BillNumberLike,
        format: Optional[str] = None,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
    ) -> List[Title]:
        """
        Get the list of titles for a specified bill.
        
        Args:
            congress: Congress number (e.g., 118)
            bill_type: Bill type (hr, s, hjres, sjres, hconres, sconres, hres, sres)
            bill_number: Bill number
            format: Response format (json or xml)
            offset: Offset for pagination
            limit: Number of results to return (max 250)
            
        Returns:
            List of Title objects
        """
        ...

    async def get_bill_text(
        self,
        congress: int,
        bill_type: str,
        bill_number: BillNumberLike,
        format: Optional[str] = None,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
    ) -> List[TextVersion]:
        """
        Get the list of text versions for a specified bill.

        Args:
            congress: Congress number (e.g., 118)
            bill_type: Bill type (hr, s, hjres, sjres, hconres, sconres, hres, sres)
            bill_number: Bill number
            format: Response format (json or xml)
            offset: Offset for pagination
            limit: Number of results to return (max 250)

        Returns:
            List of TextVersion objects
        """
        ...
    
    # Amendment endpoints
    
    async def list_amendments(
        self,
        format: Optional[str] = None,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
        from_date_time: Optional[str] = None,
        to_date_time: Optional[str] = None,
    ) -> List[Amendment]:
        """
        Get a list of amendments sorted by date of latest action.
        
        Args:
            format: Response format (json or xml)
            offset: Offset for pagination
            limit: Number of results to return (max 250)
            from_date_time: Start date-time filter (ISO 8601)
            to_date_time: End date-time filter (ISO 8601)
            
        Returns:
            List of Amendment objects
        """
        ...
    
    async def list_amendments_by_congress(
        self,
        congress: int,
        format: Optional[str] = None,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
        from_date_time: Optional[str] = None,
        to_date_time: Optional[str] = None,
    ) -> List[Amendment]:
        """
        Get amendments filtered by congress number.
        
        Args:
            congress: Congress number (e.g., 118)
            format: Response format (json or xml)
            offset: Offset for pagination
            limit: Number of results to return (max 250)
            from_date_time: Start date-time filter (ISO 8601)
            to_date_time: End date-time filter (ISO 8601)
            
        Returns:
            List of Amendment objects
        """
        ...
    
    # Member endpoints
    
    async def list_members(
        self,
        format: Optional[str] = None,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
        from_date_time: Optional[str] = None,
        to_date_time: Optional[str] = None,
        current_member: Optional[bool] = None,
    ) -> List[Sponsor]:
        """
        Get a list of congressional members.
        
        Args:
            format: Response format (json or xml)
            offset: Offset for pagination
            limit: Number of results to return (max 250)
            from_date_time: Start date-time filter (ISO 8601)
            to_date_time: End date-time filter (ISO 8601)
            current_member: Filter for current members only
            
        Returns:
            List of Sponsor objects (representing members)
        """
        ...
    
    async def get_member(self, bioguide_id: str) -> Sponsor:
        """
        Get detailed information for a specified congressional member.
        
        Args:
            bioguide_id: The Bioguide ID of the member
            
        Returns:
            Sponsor object with member information
        """
        ...
    
    async def list_members_by_congress(
        self,
        congress: int,
        format: Optional[str] = None,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
        current_member: Optional[bool] = None,
    ) -> List[Sponsor]:
        """
        Get the list of members by congress.
        
        Args:
            congress: Congress number (e.g., 118)
            format: Response format (json or xml)
            offset: Offset for pagination
            limit: Number of results to return (max 250)
            current_member: Filter for current members only
            
        Returns:
            List of Sponsor objects (representing members)
        """
        ...
    
    async def get_member_sponsored_legislation(
        self,
        bioguide_id: str,
        format: Optional[str] = None,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
    ) -> List[Bill]:
        """
        Get legislation sponsored by a specified member.
        
        Args:
            bioguide_id: The Bioguide ID of the member
            format: Response format (json or xml)
            offset: Offset for pagination
            limit: Number of results to return (max 250)
            
        Returns:
            List of Bill objects
        """
        ...
    
    async def get_member_cosponsored_legislation(
        self,
        bioguide_id: str,
        format: Optional[str] = None,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
    ) -> List[Bill]:
        """
        Get legislation cosponsored by a specified member.
        
        Args:
            bioguide_id: The Bioguide ID of the member
            format: Response format (json or xml)
            offset: Offset for pagination
            limit: Number of results to return (max 250)
            
        Returns:
            List of Bill objects
        """
        ...
    
    async def list_members_by_state(
        self,
        state_code: str,
        format: Optional[str] = None,
        limit: Optional[int] = None,
        current_member: Optional[bool] = None,
    ) -> List[Sponsor]:
        """
        Get the list of members by state.
        
        Args:
            state_code: Two-letter state code (e.g., 'CA', 'NY')
            format: Response format (json or xml)
            limit: Number of results to return (max 250)
            current_member: Filter for current members only
            
        Returns:
            List of Sponsor objects (representing members)
        """
        ...
    
    async def list_members_by_state_district(
        self,
        state_code: str,
        district: int,
        format: Optional[str] = None,
        current_member: Optional[bool] = None,
    ) -> List[Sponsor]:
        """
        Get the list of members by state and district.
        
        Args:
            state_code: Two-letter state code (e.g., 'CA', 'NY')
            district: Congressional district number
            format: Response format (json or xml)
            current_member: Filter for current members only
            
        Returns:
            List of Sponsor objects (representing members)
        """
        ...
    
    # Committee endpoints
    
    async def list_committees(
        self,
        format: Optional[str] = None,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
    ) -> List[Committee]:
        """
        Get a list of committees.
        
        Args:
            format: Response format (json or xml)
            offset: Offset for pagination
            limit: Number of results to return (max 250)
            
        Returns:
            List of Committee objects
        """
        ...

    async def list_committees_by_chamber(
        self,
        chamber: str,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
        format: Optional[str] = None,
    ) -> List[Committee]:
        ...

    async def list_committees_by_congress(
        self,
        congress: int,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
        format: Optional[str] = None,
    ) -> List[Committee]:
        ...

    async def list_committees_by_congress_and_chamber(
        self,
        congress: int,
        chamber: str,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
        format: Optional[str] = None,
    ) -> List[Committee]:
        ...

    async def get_committee(
        self,
        chamber: str,
        committee_code: str,
        format: Optional[str] = None,
    ) -> Any:
        ...

    async def get_committee_bills(
        self,
        chamber: str,
        committee_code: str,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
        format: Optional[str] = None,
    ) -> List[CommitteeBill]:
        ...
    
    # Congress/Session endpoints
    
    async def list_congresses(
        self,
        format: Optional[str] = None,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
    ) -> List[Congress]:
        """
        Get a list of congresses and congressional sessions.
        
        Args:
            format: Response format (json or xml)
            offset: Offset for pagination
            limit: Number of results to return (max 250)
            
        Returns:
            List of Congress objects
        """
        ...
    
    async def get_congress(
        self,
        congress: int,
        format: Optional[str] = None,
    ) -> Congress:
        """
        Get information about a specific congress.
        
        Args:
            congress: The congress number (e.g., 117)
            format: Response format (json or xml)
            
        Returns:
            Congress object
        """
        ...
    
    async def get_current_congress(
        self,
        format: Optional[str] = None,
    ) -> Congress:
        """
        Get information about the current congress.
        
        Args:
            format: Response format (json or xml)
            
        Returns:
            Congress object
        """
        ...
    
    # House Vote Operations (BETA)
    
    async def list_house_votes(
        self,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
        from_date: Optional[str] = None,
        to_date: Optional[str] = None,
        sort: Optional[str] = None,
        format: Optional[str] = None,
    ) -> List[HouseVote]:
        """
        Get a list of house votes (BETA).
        
        Args:
            offset: Offset for pagination
            limit: Maximum number of results
            from_date: Filter votes from this date (ISO format)
            to_date: Filter votes to this date (ISO format)
            sort: Sort order
            format: Response format (json or xml)
            
        Returns:
            List of HouseVote objects
        """
        ...
    
    async def list_house_votes_by_congress(
        self,
        congress: int,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
        from_date: Optional[str] = None,
        to_date: Optional[str] = None,
        sort: Optional[str] = None,
        format: Optional[str] = None,
    ) -> List[HouseVote]:
        """
        Get house votes for a specific congress (BETA).
        
        Args:
            congress: Congress number (e.g., 118)
            offset: Offset for pagination
            limit: Maximum number of results
            from_date: Filter votes from this date (ISO format)
            to_date: Filter votes to this date (ISO format)
            sort: Sort order
            format: Response format (json or xml)
            
        Returns:
            List of HouseVote objects
        """
        ...
    
    async def list_house_votes_by_session(
        self,
        congress: int,
        session: int,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
        from_date: Optional[str] = None,
        to_date: Optional[str] = None,
        sort: Optional[str] = None,
        format: Optional[str] = None,
    ) -> List[HouseVote]:
        """
        Get house votes for a specific congress and session (BETA).
        
        Args:
            congress: Congress number (e.g., 118)
            session: Session number (1 or 2)
            offset: Offset for pagination
            limit: Maximum number of results
            from_date: Filter votes from this date (ISO format)
            to_date: Filter votes to this date (ISO format)
            sort: Sort order
            format: Response format (json or xml)
            
        Returns:
            List of HouseVote objects
        """
        ...
    
    async def get_house_vote(
        self,
        congress: int,
        session: int,
        vote_number: int,
        format: Optional[str] = None,
    ) -> HouseVoteDetail:
        """
        Get detailed information about a specific house vote (BETA).
        
        Args:
            congress: Congress number (e.g., 118)
            session: Session number (1 or 2)
            vote_number: Roll call vote number
            format: Response format (json or xml)
            
        Returns:
            HouseVoteDetail object with party totals
        """
        ...
    
    async def get_house_vote_members(
        self,
        congress: int,
        session: int,
        vote_number: int,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
        format: Optional[str] = None,
    ) -> HouseVoteMembers:
        """
        Get how members voted on a specific house vote (BETA).
        
        Args:
            congress: Congress number (e.g., 118)
            session: Session number (1 or 2)
            vote_number: Roll call vote number
            offset: Offset for pagination
            limit: Maximum number of results
            format: Response format (json or xml)
            
        Returns:
            HouseVoteMembers object with individual member votes
        """
        ...

    async def list_crs_reports(
        self,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
        from_date: Optional[str] = None,
        to_date: Optional[str] = None,
        format: Optional[str] = None,
    ) -> List[CrsReport]:
        """
        Get a list of CRS reports.
        
        Args:
            offset: Offset for pagination
            limit: Maximum number of results
            from_date: Filter reports from this date (ISO format)
            to_date: Filter reports to this date (ISO format)
            format: Response format (json or xml)
            
        Returns:
            List of CrsReport objects representing CRS reports
        """
        ...
    
    async def get_crs_report(
        self,
        report_id: str,
        format: Optional[str] = None,
    ) -> CrsReportDetail:
        """
        Get detailed information about a specific CRS report.
        
        Args:
            report_id: The ID of the CRS report
            format: Response format (json or xml)
            
        Returns:
            CrsReportDetail object with detailed information about the CRS report
        """
        ...
    
    async def list_hearings(
        self,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
        sort: Optional[str] = None,
        format: Optional[str] = None,
    ) -> List[Hearing]:
        """
        Get a list of all hearings.
        
        Args:
            offset: Pagination offset
            limit: Number of results to return
            sort: Sort order
            format: Response format (json or xml)
            
        Returns:
            List of Hearing objects
        """
        ...
    
    async def list_hearings_by_congress(
        self,
        congress: int,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
        sort: Optional[str] = None,
        format: Optional[str] = None,
    ) -> List[Hearing]:
        """
        Get hearings by congress.
        
        Args:
            congress: The congress number
            offset: Pagination offset
            limit: Number of results to return
            sort: Sort order
            format: Response format (json or xml)
            
        Returns:
            List of Hearing objects for the specified congress
        """
        ...
    
    async def list_hearings_by_chamber(
        self,
        congress: int,
        chamber: str,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
        format: Optional[str] = None,
    ) -> List[Hearing]:
        """
        Get hearings by congress and chamber.
        
        Args:
            congress: The congress number
            chamber: Chamber (e.g., 'house', 'senate')
            offset: Pagination offset
            limit: Number of results to return
            format: Response format (json or xml)
            
        Returns:
            List of Hearing objects for the specified congress and chamber
        """
        ...
    
    async def get_hearing(
        self,
        congress: int,
        chamber: str,
        jacket_number: int,
        format: Optional[str] = None,
    ) -> Hearing:
        """
        Get a specific hearing.
        
        Args:
            congress: The congress number
            chamber: Chamber (e.g., 'house', 'senate')
            jacket_number: The hearing jacket number
            format: Response format (json or xml)
            
        Returns:
            Hearing object with detailed information
        """
        ...
    
    # Law endpoints
    
    async def list_laws(
        self,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
        format: Optional[str] = None,
    ) -> List[LawItem]:
        """
        Get a list of all laws.
        
        Args:
            offset: Offset for pagination
            limit: Number of results to return
            format: Response format (json or xml)
            
        Returns:
            List of LawItem objects
        """
        ...
    
    async def list_laws_by_congress(
        self,
        congress: int,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
        format: Optional[str] = None,
    ) -> List[LawItem]:
        """
        Get laws by congress.
        
        Args:
            congress: Congress number (e.g., 118)
            offset: Offset for pagination
            limit: Number of results to return
            format: Response format (json or xml)
            
        Returns:
            List of LawItem objects for the specified congress
        """
        ...
    
    async def list_laws_by_type(
        self,
        congress: int,
        law_type: str,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
        format: Optional[str] = None,
    ) -> List[LawItem]:
        """
        Get laws by congress and type.
        
        Args:
            congress: Congress number (e.g., 118)
            law_type: The law type. Values are either "pub" (public laws) or "priv" (private laws)
            offset: Offset for pagination
            limit: Number of results to return
            format: Response format (json or xml)
            
        Returns:
            List of LawItem objects for the specified congress and type
        """
        ...
    
    async def get_law(
        self,
        congress: int,
        law_type: str,
        law_number: str,
        format: Optional[str] = None,
    ) -> LawDetail:
        """
        Get a specific law by bill type and bill number.
        
        Note: Despite the swagger documentation referring to "lawType" and "lawNumber",
        the actual API endpoint uses the BILL's type and number, not the resulting law's type/number.
        For example, to get the law that HR 4984 became, use law_type="hr" and law_number="4984"
        
        Args:
            congress: Congress number (e.g., 118)
            law_type: Bill type like "hr", "s", "hjres", "sjres" (case-insensitive)
                     This is NOT "pub"/"priv" - those are for list_laws_by_type()
            law_number: The bill number as string (e.g., "346" or "4984")
            format: Response format (json or xml)
            
        Returns:
            LawDetail object with detailed information about the law
        """
        ...

    async def list_summaries(
        self,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
        format: Optional[str] = None,
    ) -> List[SummaryItem]:
        """
        Get a list of summaries.

        Args:
            offset: Offset for pagination
            limit: Number of results to return
            format: Response format (json or xml)

        Returns:
            List of SummaryItem objects
        """
        ...

    async def list_summaries_by_congress(
        self,
        congress: int,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
        format: Optional[str] = None,
    ) -> List[SummaryItem]:
        """
        Get summaries by congress.

        Args:
            congress: Congress number (e.g., 118)
            offset: Offset for pagination
            limit: Number of results to return
            format: Response format (json or xml)

        Returns:
            List of SummaryItem objects for the specified congress
        """
        ...

    async def get_bill(
        self,
        congress: int,
        bill_type: str,
        bill_number: BillNumberLike,
        format: Optional[str] = None,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
        from_date_time: Optional[str] = None,
        to_date_time: Optional[str] = None,
    ) -> BillDetail: ...

    async def list_amendments_by_type(
        self,
        congress: int,
        amendment_type: str,
        format: Optional[str] = None,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
        from_date_time: Optional[str] = None,
        to_date_time: Optional[str] = None,
    ) -> List[Amendment]: ...

    async def get_amendment(
        self,
        congress: int,
        amendment_type: str,
        amendment_number: str,
        format: Optional[str] = None,
    ) -> AmendmentDetail: ...

    async def get_amendment_actions(
        self,
        congress: int,
        amendment_type: str,
        amendment_number: str,
        format: Optional[str] = None,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
    ) -> List[Action]: ...

    async def get_amendment_amendments(
        self,
        congress: int,
        amendment_type: str,
        amendment_number: str,
        format: Optional[str] = None,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
    ) -> List[Amendment]: ...

    async def get_amendment_cosponsors(
        self,
        congress: int,
        amendment_type: str,
        amendment_number: str,
        format: Optional[str] = None,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
    ) -> List[Cosponsor]: ...

    async def get_amendment_text(
        self,
        congress: int,
        amendment_type: str,
        amendment_number: str,
        format: Optional[str] = None,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
    ) -> List[TextVersion]: ...

    async def list_members_by_congress_state_district(
        self,
        congress: int,
        state_code: str,
        district: int,
        format: Optional[str] = None,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
        current_member: Optional[bool] = None,
    ) -> List[Sponsor]: ...

    async def get_committee_by_congress(
        self,
        congress: int,
        chamber: str,
        committee_code: str,
        format: Optional[str] = None,
    ) -> CommitteeDetailInfo: ...

    async def get_committee_house_communications(
        self,
        chamber: str,
        committee_code: str,
        format: Optional[str] = None,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
    ) -> List[HouseCommunication]: ...

    async def get_committee_senate_communications(
        self,
        chamber: str,
        committee_code: str,
        format: Optional[str] = None,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
    ) -> List[SenateCommunication]: ...

    async def get_committee_nominations(
        self,
        chamber: str,
        committee_code: str,
        format: Optional[str] = None,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
    ) -> List[Nomination]: ...

    async def get_committee_reports(
        self,
        chamber: str,
        committee_code: str,
        format: Optional[str] = None,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
    ) -> List[CommitteeReportItem]: ...

    async def list_committee_meetings(
        self,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
        from_date_time: Optional[str] = None,
        to_date_time: Optional[str] = None,
        format: Optional[str] = None,
    ) -> List[CommitteeMeeting]: ...

    async def list_committee_meetings_by_congress(
        self,
        congress: int,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
        from_date_time: Optional[str] = None,
        to_date_time: Optional[str] = None,
        format: Optional[str] = None,
    ) -> List[CommitteeMeeting]: ...

    async def list_committee_meetings_by_chamber(
        self,
        congress: int,
        chamber: str,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
        from_date_time: Optional[str] = None,
        to_date_time: Optional[str] = None,
        format: Optional[str] = None,
    ) -> List[CommitteeMeeting]: ...

    async def get_committee_meeting(
        self,
        congress: int,
        chamber: str,
        event_id: str,
        format: Optional[str] = None,
    ) -> CommitteeMeeting: ...

    async def list_bound_congressional_records(
        self,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
        format: Optional[str] = None,
    ) -> List[BoundCongressionalRecord]: ...

    async def list_bound_congressional_records_by_year(
        self,
        year: int,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
        format: Optional[str] = None,
    ) -> List[BoundCongressionalRecord]: ...

    async def list_bound_congressional_records_by_month(
        self,
        year: int,
        month: int,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
        format: Optional[str] = None,
    ) -> List[BoundCongressionalRecord]: ...

    async def get_bound_congressional_record(
        self,
        year: int,
        month: int,
        day: int,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
        format: Optional[str] = None,
    ) -> List[BoundCongressionalRecord]: ...

    async def list_congressional_record(
        self,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
        format: Optional[str] = None,
    ) -> CongressionalRecord: ...

    async def list_daily_congressional_records_by_volume(
        self,
        volume_number: int,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
        format: Optional[str] = None,
    ) -> List[DailyCongressionalRecord]: ...

    async def get_daily_congressional_record_issue(
        self,
        volume_number: int,
        issue_number: str,
        format: Optional[str] = None,
    ) -> DailyCongressionalRecordIssue: ...

    async def get_daily_congressional_record_articles(
        self,
        volume_number: int,
        issue_number: str,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
        format: Optional[str] = None,
    ) -> List[DailyCongressionalRecordArticleGroup]: ...

    async def list_house_communications(
        self,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
        format: Optional[str] = None,
    ) -> List[HouseCommunication]: ...

    async def list_house_communications_by_congress(
        self,
        congress: int,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
        format: Optional[str] = None,
    ) -> List[HouseCommunication]: ...

    async def list_house_communications_by_type(
        self,
        congress: int,
        communication_type: str,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
        format: Optional[str] = None,
    ) -> List[HouseCommunication]: ...

    async def get_house_communication(
        self,
        congress: int,
        communication_type: str,
        communication_number: int,
        format: Optional[str] = None,
    ) -> HouseCommunication: ...

    async def list_senate_communications(
        self,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
        format: Optional[str] = None,
    ) -> List[SenateCommunication]: ...

    async def list_senate_communications_by_congress(
        self,
        congress: int,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
        format: Optional[str] = None,
    ) -> List[SenateCommunication]: ...

    async def list_senate_communications_by_type(
        self,
        congress: int,
        communication_type: str,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
        format: Optional[str] = None,
    ) -> List[SenateCommunication]: ...

    async def get_senate_communication(
        self,
        congress: int,
        communication_type: str,
        communication_number: int,
        format: Optional[str] = None,
    ) -> SenateCommunication: ...

    async def list_house_requirements(
        self,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
        format: Optional[str] = None,
    ) -> List[HouseRequirement]: ...

    async def get_house_requirement(
        self,
        requirement_number: int,
        format: Optional[str] = None,
    ) -> HouseRequirement: ...

    async def get_house_requirement_matching_communications(
        self,
        requirement_number: int,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
        format: Optional[str] = None,
    ) -> List[HouseCommunication]: ...

    async def get_nomination_actions(
        self,
        congress: int,
        nomination_number: str,
        format: Optional[str] = None,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
    ) -> List[Action]: ...

    async def get_nomination_committees(
        self,
        congress: int,
        nomination_number: str,
        format: Optional[str] = None,
    ) -> List[NominationCommittee]: ...

    async def get_nomination_hearings(
        self,
        congress: int,
        nomination_number: str,
        format: Optional[str] = None,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
    ) -> List[NominationHearing]: ...

    async def get_nomination_ordinal(
        self,
        congress: int,
        nomination_number: str,
        ordinal: str,
        format: Optional[str] = None,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
    ) -> List[Nominee]: ...

    async def get_treaty_actions(
        self,
        congress: int,
        treaty_number: str,
        format: Optional[str] = None,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
    ) -> List[Action]: ...

    async def get_treaty_committees(
        self,
        congress: int,
        treaty_number: str,
        format: Optional[str] = None,
    ) -> List[NominationCommittee]: ...

    async def get_treaty_part(
        self,
        congress: int,
        treaty_number: str,
        treaty_suffix: str,
        format: Optional[str] = None,
    ) -> List[Treaty]: ...

    async def get_treaty_part_actions(
        self,
        congress: int,
        treaty_number: str,
        treaty_suffix: str,
        format: Optional[str] = None,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
    ) -> List[Action]: ...

    async def list_summaries_by_bill_type(
        self,
        congress: int,
        bill_type: str,
        offset: Optional[int] = None,
        limit: Optional[int] = None,
        format: Optional[str] = None,
    ) -> List[SummaryItem]: ...

class Session:
    """Represents a Congressional session."""
    chamber: Optional[str]
    number: Optional[int]
    start_date: Optional[str]
    end_date: Optional[str]
    
    def __repr__(self) -> str: ...

class Congress:
    """Represents a Congress with its sessions."""
    end_year: Optional[str]
    name: Optional[str]
    sessions: Optional[List[Session]]
    start_year: Optional[str]
    url: Optional[str]
    
    def __repr__(self) -> str: ...

class Party:
    """Represents a political party."""
    name: Optional[str]
    party_type: Optional[str]
    
    def __repr__(self) -> str: ...

class VoteParty:
    """Represents vote totals by party."""
    nay_total: Optional[int]
    not_voting_total: Optional[int]
    present_total: Optional[int]
    vote_party: Optional[str]
    yea_total: Optional[int]
    party: Optional[Party]
    
    def __repr__(self) -> str: ...

class HouseVote:
    """Represents a House of Representatives roll call vote."""
    congress: Optional[int]
    identifier: Optional[int]
    legislation_number: Optional[str]
    legislation_type: Optional[str]
    legislation_url: Optional[str]
    result: Optional[str]
    roll_call_number: Optional[int]
    session_number: Optional[int]
    source_data_url: Optional[str]
    start_date: Optional[str]
    update_date: Optional[str]
    url: Optional[str]
    vote_type: Optional[str]
    
    def __repr__(self) -> str: ...

class HouseVoteDetail:
    """Represents detailed house vote information with party totals."""
    congress: Optional[int]
    identifier: Optional[int]
    legislation_number: Optional[str]
    legislation_type: Optional[str]
    legislation_url: Optional[str]
    result: Optional[str]
    roll_call_number: Optional[int]
    session_number: Optional[int]
    source_data_url: Optional[str]
    start_date: Optional[str]
    update_date: Optional[str]
    vote_type: Optional[str]
    vote_party_total: Optional[List[VoteParty]]
    vote_question: Optional[str]
    
    def __repr__(self) -> str: ...

class MemberVote:
    """Represents how a member voted."""
    bioguide_id: Optional[str]
    first_name: Optional[str]
    last_name: Optional[str]
    vote_cast: Optional[str]
    vote_party: Optional[str]
    vote_state: Optional[str]
    
    def __repr__(self) -> str: ...

class HouseVoteMembers:
    """Represents house vote with member voting details."""
    congress: Optional[int]
    identifier: Optional[int]
    legislation_number: Optional[str]
    legislation_type: Optional[str]
    legislation_url: Optional[str]
    result: Optional[str]
    roll_call_number: Optional[int]
    session_number: Optional[int]
    source_data_url: Optional[str]
    start_date: Optional[str]
    update_date: Optional[str]
    vote_type: Optional[str]
    results: Optional[List[MemberVote]]
    vote_question: Optional[str]
    
    def __repr__(self) -> str: ...

class SourceSystem:
    code: Optional[int]
    name: Optional[str]
    def __repr__(self) -> str: ...

class ActionCommittee:
    name: Optional[str]
    system_code: Optional[str]
    url: Optional[str]
    def __repr__(self) -> str: ...

class CountLink:
    count: Optional[int]
    url: Optional[str]
    def __repr__(self) -> str: ...

class AmendmentDetail:
    actions: Optional[CountLink]
    amended_bill: Optional[BillDetail]
    chamber: Optional[str]
    congress: Optional[int]
    description: Optional[str]
    latest_action: Optional[LatestAction]
    number: Optional[str]
    sponsors: Optional[List[Sponsor]]
    submitted_date: Optional[str]
    text_versions: Optional[CountLink]
    amendment_type: Optional[str]
    update_date: Optional[str]
    def __repr__(self) -> str: ...

class CommunicationType:
    code: Optional[str]
    name: Optional[str]
    def __repr__(self) -> str: ...

class MatchingRequirement:
    number: Optional[int]
    url: Optional[str]
    def __repr__(self) -> str: ...

class HouseCommunication:
    abstract_text: Optional[str]
    chamber: Optional[str]
    committees: Optional[List[Committee]]
    communication_type: Optional[CommunicationType]
    congress: Optional[int]
    congressional_record_date: Optional[str]
    is_rulemaking: Optional[str]
    legal_authority: Optional[str]
    matching_requirements: Optional[List[MatchingRequirement]]
    number: Optional[int]
    report_nature: Optional[str]
    session_number: Optional[int]
    submitting_agency: Optional[str]
    submitting_official: Optional[str]
    update_date: Optional[str]
    url: Optional[str]
    referral_date: Optional[str]
    def __repr__(self) -> str: ...

class SenateCommunication:
    abstract_text: Optional[str]
    chamber: Optional[str]
    committees: Optional[List[Committee]]
    communication_type: Optional[CommunicationType]
    congress: Optional[int]
    congressional_record_date: Optional[str]
    number: Optional[int]
    session_number: Optional[int]
    update_date: Optional[str]
    url: Optional[str]
    referral_date: Optional[str]
    def __repr__(self) -> str: ...

class HouseRequirement:
    active_record: Optional[bool]
    frequency: Optional[str]
    legal_authority: Optional[str]
    nature: Optional[str]
    number: Optional[int]
    parent_agency: Optional[str]
    submitting_agency: Optional[str]
    submitting_official: Optional[str]
    update_date: Optional[str]
    url: Optional[str]
    def __repr__(self) -> str: ...

class CommitteeMeetingLocation:
    building: Optional[str]
    room: Optional[str]
    def __repr__(self) -> str: ...

class CommitteeMeetingVideo:
    name: Optional[str]
    url: Optional[str]
    def __repr__(self) -> str: ...

class CommitteeMeeting:
    chamber: Optional[str]
    committees: Optional[List[Committee]]
    congress: Optional[int]
    date: Optional[str]
    event_id: Optional[str]
    location: Optional[CommitteeMeetingLocation]
    meeting_status: Optional[str]
    title: Optional[str]
    meeting_type: Optional[str]
    update_date: Optional[str]
    url: Optional[str]
    videos: Optional[List[CommitteeMeetingVideo]]
    def __repr__(self) -> str: ...

class RecordTextLink:
    part: Optional[str]
    text_type: Optional[str]
    url: Optional[str]
    def __repr__(self) -> str: ...

class RecordSection:
    end_page: Optional[str]
    name: Optional[str]
    start_page: Optional[str]
    text: Optional[List[RecordTextLink]]
    def __repr__(self) -> str: ...

class RecordResource:
    count: Optional[int]
    url: Optional[str]
    def __repr__(self) -> str: ...

class DailyCongressionalRecordFullIssue:
    articles: Optional[RecordResource]
    entire_issue: Optional[List[RecordTextLink]]
    sections: Optional[List[RecordSection]]
    def __repr__(self) -> str: ...

class DailyCongressionalRecordIssue:
    congress: Optional[int]
    full_issue: Optional[DailyCongressionalRecordFullIssue]
    issue_date: Optional[str]
    issue_number: Optional[str]
    session_number: Optional[int]
    update_date: Optional[str]
    url: Optional[str]
    volume_number: Optional[int]
    def __repr__(self) -> str: ...

class DailyCongressionalRecordArticle:
    end_page: Optional[str]
    start_page: Optional[str]
    text: Optional[List[RecordTextLink]]
    title: Optional[str]
    def __repr__(self) -> str: ...

class DailyCongressionalRecordArticleGroup:
    name: Optional[str]
    section_articles: Optional[List[DailyCongressionalRecordArticle]]
    def __repr__(self) -> str: ...

class BoundCongressionalRecord:
    congress: Optional[int]
    date: Optional[str]
    daily_digest: Optional[RecordSection]
    sections: Optional[List[RecordSection]]
    session_number: Optional[int]
    update_date: Optional[str]
    url: Optional[str]
    volume_number: Optional[int]
    def __repr__(self) -> str: ...

class CongressionalRecordPdfItem:
    part: Optional[str]
    url: Optional[str]
    def __repr__(self) -> str: ...

class CongressionalRecordLink:
    label: Optional[str]
    ordinal: Optional[int]
    pdf: Optional[List[CongressionalRecordPdfItem]]
    def __repr__(self) -> str: ...

class CongressionalRecordLinks:
    digest: Optional[CongressionalRecordLink]
    full_record: Optional[CongressionalRecordLink]
    house: Optional[CongressionalRecordLink]
    remarks: Optional[CongressionalRecordLink]
    senate: Optional[CongressionalRecordLink]
    def __repr__(self) -> str: ...

class CongressionalRecordIssue:
    congress: Optional[str]
    id: Optional[int]
    issue: Optional[str]
    links: Optional[CongressionalRecordLinks]
    publish_date: Optional[str]
    session: Optional[str]
    volume: Optional[str]
    def __repr__(self) -> str: ...

class CongressionalRecord:
    index_start: Optional[int]
    issues: Optional[List[CongressionalRecordIssue]]
    set_size: Optional[int]
    total_count: Optional[int]
    def __repr__(self) -> str: ...

class NominationType:
    is_civilian: Optional[bool]
    def __repr__(self) -> str: ...

class NominationCommitteeActivity:
    date: Optional[str]
    name: Optional[str]
    def __repr__(self) -> str: ...

class NominationCommittee:
    activities: Optional[List[NominationCommitteeActivity]]
    chamber: Optional[str]
    name: Optional[str]
    system_code: Optional[str]
    committee_type: Optional[str]
    url: Optional[str]
    def __repr__(self) -> str: ...

class NominationHearing:
    chamber: Optional[str]
    citation: Optional[str]
    date: Optional[str]
    jacket_number: Optional[int]
    number: Optional[int]
    def __repr__(self) -> str: ...

class TreatyParts:
    count: Optional[int]
    urls: Optional[List[str]]
    def __repr__(self) -> str: ...

class TreatyCountryParty:
    name: Optional[str]
    def __repr__(self) -> str: ...

class TreatyIndexTerm:
    name: Optional[str]
    def __repr__(self) -> str: ...

def configure_client_retries(
    client: CDGPythonClient,
    retry_attempts: int,
    retry_base_delay_ms: int,
) -> None:
    """
    Update the retry strategy for HTTP 503 responses for a single client instance.

    Args:
        client: The client instance to update
        retry_attempts: Maximum number of total attempts, including the initial request
        retry_base_delay_ms: Base delay in milliseconds for the linear retry backoff;
            with the default settings the client waits 1000ms before retry 2 and
            2000ms before retry 3
    """
    ...

def get_client_retry_config(client: CDGPythonClient) -> tuple[int, int]:
    """
    Return the current `(retry_attempts, retry_base_delay_ms)` tuple for a client instance.
    """
    ...

__all__ = [
    "CDGPythonClient",
    "AsyncCDGPythonClient",
    "CDGClientError",
    "CDGConfigurationError",
    "CDGInvalidUrlError",
    "CDGRequestError",
    "CDGHttpError",
    "CDGAuthError",
    "CDGNotFoundError",
    "CDGRateLimitError",
    "CDGServerError",
    "CDGDeserializationError",
    "ApiPage",
    "configure_client_retries",
    "get_client_retry_config",
    "ActionCommittee",
    "Bill",
    "BillDetail",
    "LatestAction",
    "Law",
    "Sponsor",
    "PolicyArea",
    "Action",
    "Amendment",
    "AmendmentDetail",
    "BoundCongressionalRecord",
    "Committee",
    "CommitteeMeeting",
    "CommitteeMeetingLocation",
    "CommitteeMeetingVideo",
    "CommunicationType",
    "CongressionalRecord",
    "CongressionalRecordIssue",
    "CongressionalRecordLink",
    "CongressionalRecordLinks",
    "CongressionalRecordPdfItem",
    "Cosponsor",
    "CountLink",
    "RelatedBill",
    "RelationshipDetail",
    "HouseCommunication",
    "HouseRequirement",
    "Subject",
    "Summary",
    "TextVersion",
    "TextFormat",
    "Title",
    "Congress",
    "Session",
    "HouseVote",
    "HouseVoteDetail",
    "HouseVoteMembers",
    "MemberVote",
    "Party",
    "VoteParty",
    "Hearing",
    "HearingDate",
    "HearingFormat",
    "HearingCommittee",
    "AssociatedMeeting",
    "CrsReport",
    "CrsReportDetail",
    "CrsReportAuthor",
    "CrsReportFormat",
    "CrsReportTopic",
    "CrsReportRelatedMaterial",
    "LawItem",
    "LawDetail",
    "DailyCongressionalRecord",
    "DailyCongressionalRecordArticle",
    "DailyCongressionalRecordArticleGroup",
    "DailyCongressionalRecordFullIssue",
    "DailyCongressionalRecordIssue",
    "MatchingRequirement",
    "Nomination",
    "NominationCommittee",
    "NominationCommitteeActivity",
    "NominationHearing",
    "NominationType",
    "Nominee",
    "RecordResource",
    "RecordSection",
    "RecordTextLink",
    "SenateCommunication",
    "SourceSystem",
    "Treaty",
    "TreatyCountryParty",
    "TreatyIndexTerm",
    "TreatyParts",
]
