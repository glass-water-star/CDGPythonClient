use pyo3::prelude::*;

mod client;
mod bills;
mod members;
mod sessions;
mod house_votes;
mod committees;
mod nominations;
mod treaties;
mod hearings;
mod congressional_record;
mod laws;
mod summaries;
mod crsreport;

use client::CDGPythonClient;

use bills::{
    Action, Amendment, Bill, BillDetail, Committee, Cosponsor,
    LatestAction, Law, PolicyArea, RelatedBill, RelationshipDetail,
    Subject, Summary, TextFormat, TextVersion, Title,
};

use members::Sponsor;

use sessions::{
    Congress, Session,
};

use house_votes::{
    HouseVote, HouseVoteDetail, HouseVoteMembers, MemberVote, Party, VoteParty,
};

use committees::{
    CommitteeBill, CommitteeDetailInfo, CommitteeHistory, CommitteeItem,
    CommitteePrintDetail, CommitteePrintItem, CommitteePrintText,
    CommitteeReportDetail, CommitteeReportItem, CommitteeReportText,
    ParentCommittee, ResourceCount, Subcommittee,
};

use nominations::{Nomination, Nominee};
use treaties::Treaty;
use hearings::{AssociatedMeeting, Hearing, HearingCommittee, HearingDate, HearingFormat};
use congressional_record::DailyCongressionalRecord;
use laws::{LawDetail, LawItem};
use summaries::SummaryItem;
use crsreport::{CrsReport, CrsReportAuthor, CrsReportDetail, CrsReportFormat, CrsReportRelatedMaterial, CrsReportTopic};

/// A Python module implemented in Rust for interacting with the Congress.gov API
#[pymodule]
fn cdg_python_client(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Add the main client
    m.add_class::<CDGPythonClient>()?;
    
    // Add data structures
    m.add_class::<Bill>()?;
    m.add_class::<BillDetail>()?;
    m.add_class::<LatestAction>()?;
    m.add_class::<Law>()?;
    m.add_class::<Sponsor>()?;
    m.add_class::<PolicyArea>()?;
    m.add_class::<Action>()?;
    m.add_class::<Amendment>()?;
    m.add_class::<Committee>()?;
    m.add_class::<Cosponsor>()?;
    m.add_class::<RelatedBill>()?;
    m.add_class::<RelationshipDetail>()?;
    m.add_class::<Subject>()?;
    m.add_class::<Summary>()?;
    m.add_class::<TextVersion>()?;
    m.add_class::<TextFormat>()?;
    m.add_class::<Title>()?;
    
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
    m.add_class::<Nominee>()?;
    
    // Add treaty-related structures
    m.add_class::<Treaty>()?;
    
    // Add hearing-related structures
    m.add_class::<Hearing>()?;
    m.add_class::<AssociatedMeeting>()?;
    m.add_class::<HearingFormat>()?;
    m.add_class::<HearingCommittee>()?;
    m.add_class::<HearingDate>()?;
    
    // Add congressional record structures
    m.add_class::<DailyCongressionalRecord>()?;
    
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

