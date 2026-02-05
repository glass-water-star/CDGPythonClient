use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

/// Represents a House of Representatives roll call vote
#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct HouseVote {
    #[pyo3(get)]
    pub congress: Option<i32>,
    
    #[pyo3(get)]
    pub identifier: Option<i64>,
    
    #[pyo3(get)]
    #[serde(rename = "legislationNumber")]
    pub legislation_number: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "legislationType")]
    pub legislation_type: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "legislationUrl")]
    pub legislation_url: Option<String>,
    
    #[pyo3(get)]
    pub result: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "rollCallNumber")]
    pub roll_call_number: Option<i32>,
    
    #[pyo3(get)]
    #[serde(rename = "sessionNumber")]
    pub session_number: Option<i32>,
    
    #[pyo3(get)]
    #[serde(rename = "sourceDataURL")]
    pub source_data_url: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "startDate")]
    pub start_date: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "updateDate")]
    pub update_date: Option<String>,
    
    #[pyo3(get)]
    pub url: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "voteType")]
    pub vote_type: Option<String>,
}

#[pymethods]
impl HouseVote {
    fn __repr__(&self) -> String {
        format!(
            "HouseVote(congress={:?}, session={:?}, roll_call={:?}, result={:?})",
            self.congress, self.session_number, self.roll_call_number, self.result
        )
    }
}

/// Represents party information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct Party {
    #[pyo3(get)]
    pub name: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "type")]
    pub party_type: Option<String>,
}

#[pymethods]
impl Party {
    fn __repr__(&self) -> String {
        format!("Party(name={:?}, type={:?})", self.name, self.party_type)
    }
}

/// Represents vote totals by party
#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct VoteParty {
    #[pyo3(get)]
    #[serde(rename = "nayTotal")]
    pub nay_total: Option<i32>,
    
    #[pyo3(get)]
    #[serde(rename = "notVotingTotal")]
    pub not_voting_total: Option<i32>,
    
    #[pyo3(get)]
    #[serde(rename = "presentTotal")]
    pub present_total: Option<i32>,
    
    #[pyo3(get)]
    #[serde(rename = "voteParty")]
    pub vote_party: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "yeaTotal")]
    pub yea_total: Option<i32>,
    
    #[pyo3(get)]
    pub party: Option<Party>,
}

#[pymethods]
impl VoteParty {
    fn __repr__(&self) -> String {
        format!(
            "VoteParty(party={:?}, yea={:?}, nay={:?})",
            self.vote_party, self.yea_total, self.nay_total
        )
    }
}

/// Represents detailed house vote information with party totals
#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct HouseVoteDetail {
    #[pyo3(get)]
    pub congress: Option<i32>,
    
    #[pyo3(get)]
    pub identifier: Option<i64>,
    
    #[pyo3(get)]
    #[serde(rename = "legislationNumber")]
    pub legislation_number: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "legislationType")]
    pub legislation_type: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "legislationUrl")]
    pub legislation_url: Option<String>,
    
    #[pyo3(get)]
    pub result: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "rollCallNumber")]
    pub roll_call_number: Option<i32>,
    
    #[pyo3(get)]
    #[serde(rename = "sessionNumber")]
    pub session_number: Option<i32>,
    
    #[pyo3(get)]
    #[serde(rename = "sourceDataURL")]
    pub source_data_url: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "startDate")]
    pub start_date: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "updateDate")]
    pub update_date: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "voteType")]
    pub vote_type: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "votePartyTotal")]
    pub vote_party_total: Option<Vec<VoteParty>>,
    
    #[pyo3(get)]
    #[serde(rename = "voteQuestion")]
    pub vote_question: Option<String>,
}

#[pymethods]
impl HouseVoteDetail {
    fn __repr__(&self) -> String {
        format!(
            "HouseVoteDetail(congress={:?}, session={:?}, roll_call={:?}, question={:?})",
            self.congress, self.session_number, self.roll_call_number, self.vote_question
        )
    }
}

/// Represents how a member voted
#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct MemberVote {
    #[pyo3(get)]
    #[serde(rename = "bioguideID")]
    pub bioguide_id: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "firstName")]
    pub first_name: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "lastName")]
    pub last_name: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "voteCast")]
    pub vote_cast: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "voteParty")]
    pub vote_party: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "voteState")]
    pub vote_state: Option<String>,
}

#[pymethods]
impl MemberVote {
    fn __repr__(&self) -> String {
        format!(
            "MemberVote(name={:?} {:?}, vote={:?})",
            self.first_name, self.last_name, self.vote_cast
        )
    }
}

/// Represents house vote with member voting details
#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct HouseVoteMembers {
    #[pyo3(get)]
    pub congress: Option<i32>,
    
    #[pyo3(get)]
    pub identifier: Option<i64>,
    
    #[pyo3(get)]
    #[serde(rename = "legislationNumber")]
    pub legislation_number: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "legislationType")]
    pub legislation_type: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "legislationUrl")]
    pub legislation_url: Option<String>,
    
    #[pyo3(get)]
    pub result: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "rollCallNumber")]
    pub roll_call_number: Option<i32>,
    
    #[pyo3(get)]
    #[serde(rename = "sessionNumber")]
    pub session_number: Option<i32>,
    
    #[pyo3(get)]
    #[serde(rename = "sourceDataURL")]
    pub source_data_url: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "startDate")]
    pub start_date: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "updateDate")]
    pub update_date: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "voteType")]
    pub vote_type: Option<String>,
    
    #[pyo3(get)]
    pub results: Option<Vec<MemberVote>>,
    
    #[pyo3(get)]
    #[serde(rename = "voteQuestion")]
    pub vote_question: Option<String>,
}

#[pymethods]
impl HouseVoteMembers {
    fn __repr__(&self) -> String {
        format!(
            "HouseVoteMembers(congress={:?}, session={:?}, roll_call={:?}, members={:?})",
            self.congress, self.session_number, self.roll_call_number, 
            self.results.as_ref().map(|r| r.len())
        )
    }
}

/// Response structure for list of house votes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HouseVotesResponse {
    #[serde(rename = "houseRollCallVotes")]
    pub votes: Vec<HouseVote>,
}

/// Response structure for a single house vote detail
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HouseVoteDetailResponse {
    #[serde(rename = "houseRollCallVote")]
    pub vote: HouseVoteDetail,
}

/// Response structure for house vote with member votes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HouseVoteMembersResponse {
    #[serde(rename = "houseRollCallVoteMemberVotes")]
    pub vote: HouseVoteMembers,
}
