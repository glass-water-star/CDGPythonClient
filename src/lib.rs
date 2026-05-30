use pyo3::create_exception;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

mod bills;
mod client;
mod committee_meetings;
mod committees;
mod communications;
mod congressional_record;
mod crsreport;
mod hearings;
mod house_votes;
mod laws;
mod members;
mod nominations;
mod requirements;
mod sessions;
mod summaries;
mod treaties;

use client::{
    configure_client_retries, get_client_retry_config, ApiPage, AsyncClientCore, CDGPythonClient,
};

use bills::{
    Action, ActionCommittee, Amendment, AmendmentDetail, Bill, BillDetail, BillTitle, Committee,
    Cosponsor, CountLink, LatestAction, Law, PolicyArea, RelatedBill, RelationshipDetail,
    SourceSystem, Subject, Summary, TextFormat, TextVersion,
};

use members::Sponsor;

use sessions::{Congress, Session};

use house_votes::{HouseVote, HouseVoteDetail, HouseVoteMembers, MemberVote, Party, VoteParty};

use committees::{
    CommitteeBill, CommitteeDetailInfo, CommitteeHistory, CommitteeItem, CommitteePrintDetail,
    CommitteePrintItem, CommitteePrintText, CommitteeReportDetail, CommitteeReportItem,
    CommitteeReportText, ParentCommittee, ResourceCount, Subcommittee,
};

use committee_meetings::{CommitteeMeeting, CommitteeMeetingLocation, CommitteeMeetingVideo};
use communications::{
    CommunicationType, HouseCommunication, MatchingRequirement, SenateCommunication,
};
use congressional_record::{
    BoundCongressionalRecord, CongressionalRecord, CongressionalRecordIssue,
    CongressionalRecordLink, CongressionalRecordLinks, CongressionalRecordPdfItem,
    DailyCongressionalRecord, DailyCongressionalRecordArticle,
    DailyCongressionalRecordArticleGroup, DailyCongressionalRecordFullIssue,
    DailyCongressionalRecordIssue, RecordResource, RecordSection, RecordTextLink,
};
use crsreport::{
    CrsReport, CrsReportAuthor, CrsReportDetail, CrsReportFormat, CrsReportRelatedMaterial,
    CrsReportTopic,
};
use hearings::{AssociatedMeeting, Hearing, HearingCommittee, HearingDate, HearingFormat};
use laws::{LawDetail, LawItem};
use nominations::{
    Nomination, NominationCommittee, NominationCommitteeActivity, NominationHearing,
    NominationType, Nominee,
};
use requirements::HouseRequirement;
use summaries::SummaryItem;
use treaties::{Treaty, TreatyCountryParty, TreatyIndexTerm, TreatyParts};

create_exception!(cdg_python_client, CDGClientError, PyException);
create_exception!(cdg_python_client, CDGConfigurationError, CDGClientError);
create_exception!(cdg_python_client, CDGInvalidUrlError, CDGClientError);
create_exception!(cdg_python_client, CDGRequestError, CDGClientError);
create_exception!(cdg_python_client, CDGHttpError, CDGClientError);
create_exception!(cdg_python_client, CDGAuthError, CDGHttpError);
create_exception!(cdg_python_client, CDGNotFoundError, CDGHttpError);
create_exception!(cdg_python_client, CDGRateLimitError, CDGHttpError);
create_exception!(cdg_python_client, CDGServerError, CDGHttpError);
create_exception!(cdg_python_client, CDGDeserializationError, CDGClientError);

/// A Python module implemented in Rust for interacting with the Congress.gov API
#[pymodule]
fn cdg_python_client(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Add the main client
    let py = m.py();
    m.add("CDGClientError", py.get_type::<CDGClientError>())?;
    m.add(
        "CDGConfigurationError",
        py.get_type::<CDGConfigurationError>(),
    )?;
    m.add("CDGInvalidUrlError", py.get_type::<CDGInvalidUrlError>())?;
    m.add("CDGRequestError", py.get_type::<CDGRequestError>())?;
    m.add("CDGHttpError", py.get_type::<CDGHttpError>())?;
    m.add("CDGAuthError", py.get_type::<CDGAuthError>())?;
    m.add("CDGNotFoundError", py.get_type::<CDGNotFoundError>())?;
    m.add("CDGRateLimitError", py.get_type::<CDGRateLimitError>())?;
    m.add("CDGServerError", py.get_type::<CDGServerError>())?;
    m.add(
        "CDGDeserializationError",
        py.get_type::<CDGDeserializationError>(),
    )?;
    m.add_class::<CDGPythonClient>()?;
    m.add_class::<AsyncClientCore>()?;
    m.add_class::<ApiPage>()?;
    m.add_function(wrap_pyfunction!(configure_client_retries, m)?)?;
    m.add_function(wrap_pyfunction!(get_client_retry_config, m)?)?;

    // Add data structures
    m.add_class::<Bill>()?;
    m.add_class::<BillDetail>()?;
    m.add_class::<LatestAction>()?;
    m.add_class::<Law>()?;
    m.add_class::<Sponsor>()?;
    m.add_class::<PolicyArea>()?;
    m.add_class::<Action>()?;
    m.add_class::<ActionCommittee>()?;
    m.add_class::<Amendment>()?;
    m.add_class::<AmendmentDetail>()?;
    m.add_class::<Committee>()?;
    m.add_class::<CountLink>()?;
    m.add_class::<Cosponsor>()?;
    m.add_class::<RelatedBill>()?;
    m.add_class::<RelationshipDetail>()?;
    m.add_class::<SourceSystem>()?;
    m.add_class::<Subject>()?;
    m.add_class::<Summary>()?;
    m.add_class::<TextVersion>()?;
    m.add_class::<TextFormat>()?;
    m.add_class::<BillTitle>()?;

    // Add session-related structures
    m.add_class::<Congress>()?;
    m.add_class::<Session>()?;

    // Add house vote-related structures
    m.add_class::<HouseVote>()?;
    m.add_class::<HouseVoteDetail>()?;
    m.add_class::<HouseVoteMembers>()?;
    m.add_class::<MemberVote>()?;
    m.add_class::<Party>()?;
    m.add_class::<VoteParty>()?;

    // Add committee-related structures
    m.add_class::<CommitteeItem>()?;
    m.add_class::<CommitteeDetailInfo>()?;
    m.add_class::<CommitteeHistory>()?;
    m.add_class::<Subcommittee>()?;
    m.add_class::<ParentCommittee>()?;
    m.add_class::<ResourceCount>()?;
    m.add_class::<CommitteeBill>()?;
    m.add_class::<CommitteeReportItem>()?;
    m.add_class::<CommitteeReportDetail>()?;
    m.add_class::<CommitteeReportText>()?;
    m.add_class::<CommitteePrintItem>()?;
    m.add_class::<CommitteePrintDetail>()?;
    m.add_class::<CommitteePrintText>()?;

    // Add nomination-related structures
    m.add_class::<Nomination>()?;
    m.add_class::<NominationCommittee>()?;
    m.add_class::<NominationCommitteeActivity>()?;
    m.add_class::<NominationHearing>()?;
    m.add_class::<NominationType>()?;
    m.add_class::<Nominee>()?;

    // Add treaty-related structures
    m.add_class::<Treaty>()?;
    m.add_class::<TreatyCountryParty>()?;
    m.add_class::<TreatyIndexTerm>()?;
    m.add_class::<TreatyParts>()?;

    // Add hearing-related structures
    m.add_class::<Hearing>()?;
    m.add_class::<AssociatedMeeting>()?;
    m.add_class::<HearingFormat>()?;
    m.add_class::<HearingCommittee>()?;
    m.add_class::<HearingDate>()?;

    // Add congressional record structures
    m.add_class::<BoundCongressionalRecord>()?;
    m.add_class::<CongressionalRecord>()?;
    m.add_class::<CongressionalRecordIssue>()?;
    m.add_class::<CongressionalRecordLink>()?;
    m.add_class::<CongressionalRecordLinks>()?;
    m.add_class::<CongressionalRecordPdfItem>()?;
    m.add_class::<DailyCongressionalRecord>()?;
    m.add_class::<DailyCongressionalRecordArticle>()?;
    m.add_class::<DailyCongressionalRecordArticleGroup>()?;
    m.add_class::<DailyCongressionalRecordFullIssue>()?;
    m.add_class::<DailyCongressionalRecordIssue>()?;
    m.add_class::<RecordResource>()?;
    m.add_class::<RecordSection>()?;
    m.add_class::<RecordTextLink>()?;

    // Add communication and requirement structures
    m.add_class::<CommunicationType>()?;
    m.add_class::<HouseCommunication>()?;
    m.add_class::<MatchingRequirement>()?;
    m.add_class::<SenateCommunication>()?;
    m.add_class::<HouseRequirement>()?;

    // Add committee meeting structures
    m.add_class::<CommitteeMeeting>()?;
    m.add_class::<CommitteeMeetingLocation>()?;
    m.add_class::<CommitteeMeetingVideo>()?;

    // Add law-related structures
    m.add_class::<LawItem>()?;
    m.add_class::<LawDetail>()?;

    // Add summary structures
    m.add_class::<SummaryItem>()?;

    // Add CRS report structures
    m.add_class::<CrsReport>()?;
    m.add_class::<CrsReportDetail>()?;
    m.add_class::<CrsReportAuthor>()?;
    m.add_class::<CrsReportFormat>()?;
    m.add_class::<CrsReportTopic>()?;
    m.add_class::<CrsReportRelatedMaterial>()?;

    Ok(())
}
