"""
Congress.gov API Python Client

A Python client for the Congress.gov API, implemented in Rust using PyO3 for high performance.
"""

from typing import Optional, List

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

class CDGPythonClient:
    """
    Client for interacting with the Congress.gov API.
    
    Example:
        >>> client = CDGPythonClient(api_key="your_api_key")
        >>> bills = client.list_bills(limit=10)
        >>> bill = client.get_bill(congress=118, bill_type="hr", bill_number=1)
        >>> members = client.list_members(limit=10, current_member=True)
    """
    
    def __init__(self, api_key: str) -> None:
        """
        Initialize the Congress.gov API client.
        
        Args:
            api_key: Your Congress.gov API key
        """
        ...
    
    def list_bills(
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
    
    def list_bills_by_congress(
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
    
    def list_bills_by_type(
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
    
    def get_bill(
        self,
        congress: int,
        bill_type: str,
        bill_number: int,
    ) -> BillDetail:
        """
        Get detailed information for a specific bill.
        
        Args:
            congress: Congress number (e.g., 118)
            bill_type: Bill type (hr, s, hjres, sjres, hconres, sconres, hres, sres)
            bill_number: Bill number
            
        Returns:
            BillDetail object with comprehensive bill information
        """
        ...
    
    def get_bill_actions(
        self,
        congress: int,
        bill_type: str,
        bill_number: int,
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
    
    def get_bill_amendments(
        self,
        congress: int,
        bill_type: str,
        bill_number: int,
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
    
    def get_bill_committees(
        self,
        congress: int,
        bill_type: str,
        bill_number: int,
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
    
    def get_bill_cosponsors(
        self,
        congress: int,
        bill_type: str,
        bill_number: int,
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
    
    def get_related_bills(
        self,
        congress: int,
        bill_type: str,
        bill_number: int,
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
    
    def get_bill_subjects(
        self,
        congress: int,
        bill_type: str,
        bill_number: int,
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
    
    def get_bill_summaries(
        self,
        congress: int,
        bill_type: str,
        bill_number: int,
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
    
    def get_bill_text(
        self,
        congress: int,
        bill_type: str,
        bill_number: int,
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
    
    def get_bill_titles(
        self,
        congress: int,
        bill_type: str,
        bill_number: int,
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
    
    # Amendment endpoints
    
    def list_amendments(
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
    
    def list_amendments_by_congress(
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
    
    def list_members(
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
    
    def get_member(self, bioguide_id: str) -> Sponsor:
        """
        Get detailed information for a specified congressional member.
        
        Args:
            bioguide_id: The Bioguide ID of the member
            
        Returns:
            Sponsor object with member information
        """
        ...
    
    def list_members_by_congress(
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
    
    def get_member_sponsored_legislation(
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
    
    def get_member_cosponsored_legislation(
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
    
    def list_members_by_state(
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
    
    def list_members_by_state_district(
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
    
    def list_committees(
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
    
    # Congress/Session endpoints
    
    def list_congresses(
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
    
    def get_congress(
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
    
    def get_current_congress(
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
    
    def list_house_votes(
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
    
    def list_house_votes_by_congress(
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
    
    def list_house_votes_by_session(
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
    
    def get_house_vote(
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
    
    def get_house_vote_members(
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

__all__ = [
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
    "Congress",
    "Session",
    "HouseVote",
    "HouseVoteDetail",
    "HouseVoteMembers",
    "MemberVote",
    "Party",
    "VoteParty",
]
