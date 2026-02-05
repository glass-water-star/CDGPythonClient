use pyo3::prelude::*;
use reqwest::blocking::Client;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use thiserror::Error;

use crate::bills::{
    Action, ActionsResponse, Amendment, AmendmentsResponse, Bill, BillDetail, 
    BillDetailResponse, BillsResponse, Committee, CommitteesResponse, Cosponsor, 
    CosponsorsResponse, RelatedBill, RelatedBillsResponse, Subject, 
    SubjectsResponse, SummariesResponse, Summary, TextVersion, TextVersionsResponse, 
    Title, TitlesResponse,
};
use crate::members::{
    CosponsoredLegislationResponse, MemberResponse, MembersResponse, Sponsor, 
    SponsoredLegislationResponse,
};
use crate::sessions::{Congress, CongressesResponse, CongressResponse};
use crate::house_votes::{
    HouseVote, HouseVoteDetail, HouseVoteDetailResponse, HouseVoteMembers,
    HouseVoteMembersResponse, HouseVotesResponse,
};
use crate::committees::{
    CommitteeBill, CommitteeBillsResponse, CommitteeDetailInfo, CommitteeDetailResponse,
    CommitteeItem, CommitteePrintDetail, CommitteePrintDetailResponse, CommitteePrintItem,
    CommitteePrintsResponse, CommitteePrintText, CommitteePrintTextResponse,
    CommitteeReportDetail, CommitteeReportDetailResponse, CommitteeReportItem,
    CommitteeReportsResponse, CommitteeReportText, CommitteeReportTextResponse,
    CommitteesResponse as CommitteesListResponse,
};
use crate::nominations::{Nomination, NominationDetailResponse, NominationsResponse, Nominee, NomineesResponse};
use crate::treaties::{Treaty, TreatiesResponse, TreatyDetailResponse};
use crate::hearings::{Hearing, HearingsResponse, HearingDetailResponse};
use crate::congressional_record::{DailyCongressionalRecord, DailyCongressionalRecordsResponse};
use crate::laws::{LawDetail, LawDetailResponse, LawItem, LawsResponse};
use crate::summaries::{SummaryItem, SummariesListResponse};
use crate::crsreport::{CrsReport, CrsReportDetail, CrsReportDetailResponse, CrsReportsResponse};

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("HTTP request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),
    
    #[error("API error: {0}")]
    ApiError(String),
    
    #[allow(dead_code)]
    #[error("Missing API key")]
    MissingApiKey,
}

pub type ApiResult<T> = Result<T, ApiError>;

pub struct CongressApiClient {
    client: Client,
    api_key: String,
    base_url: String,
}

impl CongressApiClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
            base_url: "https://api.congress.gov/v3".to_string(),
        }
    }

    pub fn get<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        params: Option<HashMap<String, String>>,
    ) -> ApiResult<T> {
        let url = format!("{}{}", self.base_url, endpoint);
        
        let mut request = self.client.get(&url).query(&[("api_key", &self.api_key)]);
        
        if let Some(params) = params {
            for (key, value) in params {
                request = request.query(&[(key.as_str(), value.as_str())]);
            }
        }
        
        let response = request.send()?;
        
        if !response.status().is_success() {
            return Err(ApiError::ApiError(format!(
                "API returned status: {}",
                response.status()
            )));
        }
        
        Ok(response.json()?)
    }
}

// PyO3 wrapper class for Congress.gov API
#[pyclass]
pub struct CDGPythonClient {
    client: CongressApiClient,
}

#[pymethods]
impl CDGPythonClient {
    #[new]
    pub fn new(api_key: String) -> Self {
        Self {
            client: CongressApiClient::new(api_key),
        }
    }

    // ========== Bill Endpoints ==========

    /// Get a list of bills sorted by date of latest action
    #[pyo3(signature = (format=None, offset=None, limit=None, from_date_time=None, to_date_time=None))]
    pub fn list_bills(
        &self,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
        from_date_time: Option<String>,
        to_date_time: Option<String>,
    ) -> PyResult<Vec<Bill>> {
        let mut params = HashMap::new();
        
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        if let Some(from) = from_date_time {
            params.insert("fromDateTime".to_string(), from);
        }
        if let Some(to) = to_date_time {
            params.insert("toDateTime".to_string(), to);
        }
        
        let response: BillsResponse = self
            .client
            .get("/bill", Some(params))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        
        Ok(response.bills)
    }

    /// Get bills filtered by congress
    #[pyo3(signature = (congress, format=None, offset=None, limit=None, from_date_time=None, to_date_time=None))]
    pub fn list_bills_by_congress(
        &self,
        congress: i32,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
        from_date_time: Option<String>,
        to_date_time: Option<String>,
    ) -> PyResult<Vec<Bill>> {
        let mut params = HashMap::new();
        
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        if let Some(from) = from_date_time {
            params.insert("fromDateTime".to_string(), from);
        }
        if let Some(to) = to_date_time {
            params.insert("toDateTime".to_string(), to);
        }
        
        let endpoint = format!("/bill/{}", congress);
        let response: BillsResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        
        Ok(response.bills)
    }

    /// Get bills filtered by congress and bill type
    #[pyo3(signature = (congress, bill_type, format=None, offset=None, limit=None, from_date_time=None, to_date_time=None))]
    pub fn list_bills_by_type(
        &self,
        congress: i32,
        bill_type: String,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
        from_date_time: Option<String>,
        to_date_time: Option<String>,
    ) -> PyResult<Vec<Bill>> {
        let mut params = HashMap::new();
        
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        if let Some(from) = from_date_time {
            params.insert("fromDateTime".to_string(), from);
        }
        if let Some(to) = to_date_time {
            params.insert("toDateTime".to_string(), to);
        }
        
        let endpoint = format!("/bill/{}/{}", congress, bill_type);
        let response: BillsResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        
        Ok(response.bills)
    }

    /// Get detailed information for a specified bill
    pub fn get_bill(
        &self,
        congress: i32,
        bill_type: String,
        bill_number: i32,
    ) -> PyResult<BillDetail> {
        let endpoint = format!("/bill/{}/{}/{}", congress, bill_type, bill_number);
        let response: BillDetailResponse = self
            .client
            .get(&endpoint, None)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        
        Ok(response.bill)
    }

    /// Get the list of actions on a specified bill
    #[pyo3(signature = (congress, bill_type, bill_number, format=None, offset=None, limit=None))]
    pub fn get_bill_actions(
        &self,
        congress: i32,
        bill_type: String,
        bill_number: i32,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> PyResult<Vec<Action>> {
        let mut params = HashMap::new();
        
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        
        let endpoint = format!("/bill/{}/{}/{}/actions", congress, bill_type, bill_number);
        let response: ActionsResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        
        Ok(response.actions)
    }

    /// Get the list of amendments to a specified bill
    #[pyo3(signature = (congress, bill_type, bill_number, format=None, offset=None, limit=None))]
    pub fn get_bill_amendments(
        &self,
        congress: i32,
        bill_type: String,
        bill_number: i32,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> PyResult<Vec<Amendment>> {
        let mut params = HashMap::new();
        
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        
        let endpoint = format!("/bill/{}/{}/{}/amendments", congress, bill_type, bill_number);
        let response: AmendmentsResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        
        Ok(response.amendments)
    }

    /// Get the list of committees associated with a specified bill
    #[pyo3(signature = (congress, bill_type, bill_number, format=None, offset=None, limit=None))]
    pub fn get_bill_committees(
        &self,
        congress: i32,
        bill_type: String,
        bill_number: i32,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> PyResult<Vec<Committee>> {
        let mut params = HashMap::new();
        
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        
        let endpoint = format!("/bill/{}/{}/{}/committees", congress, bill_type, bill_number);
        let response: CommitteesResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        
        Ok(response.committees)
    }

    /// Get the list of cosponsors on a specified bill
    #[pyo3(signature = (congress, bill_type, bill_number, format=None, offset=None, limit=None))]
    pub fn get_bill_cosponsors(
        &self,
        congress: i32,
        bill_type: String,
        bill_number: i32,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> PyResult<Vec<Cosponsor>> {
        let mut params = HashMap::new();
        
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        
        let endpoint = format!("/bill/{}/{}/{}/cosponsors", congress, bill_type, bill_number);
        let response: CosponsorsResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        
        Ok(response.cosponsors)
    }

    /// Get the list of related bills to a specified bill
    #[pyo3(signature = (congress, bill_type, bill_number, format=None, offset=None, limit=None))]
    pub fn get_related_bills(
        &self,
        congress: i32,
        bill_type: String,
        bill_number: i32,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> PyResult<Vec<RelatedBill>> {
        let mut params = HashMap::new();
        
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        
        let endpoint = format!("/bill/{}/{}/{}/relatedbills", congress, bill_type, bill_number);
        let response: RelatedBillsResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        
        Ok(response.related_bills.unwrap_or_default())
    }

    /// Get the list of legislative subjects on a specified bill
    #[pyo3(signature = (congress, bill_type, bill_number, format=None, offset=None, limit=None))]
    pub fn get_bill_subjects(
        &self,
        congress: i32,
        bill_type: String,
        bill_number: i32,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> PyResult<Vec<Subject>> {
        let mut params = HashMap::new();
        
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        
        let endpoint = format!("/bill/{}/{}/{}/subjects", congress, bill_type, bill_number);
        let response: SubjectsResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        
        Ok(response.legislative_subjects.unwrap_or_default())
    }

    /// Get the list of summaries for a specified bill
    #[pyo3(signature = (congress, bill_type, bill_number, format=None, offset=None, limit=None))]
    pub fn get_bill_summaries(
        &self,
        congress: i32,
        bill_type: String,
        bill_number: i32,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> PyResult<Vec<Summary>> {
        let mut params = HashMap::new();
        
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        
        let endpoint = format!("/bill/{}/{}/{}/summaries", congress, bill_type, bill_number);
        let response: SummariesResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        
        Ok(response.summaries)
    }

    /// Get the list of text versions for a specified bill
    #[pyo3(signature = (congress, bill_type, bill_number, format=None, offset=None, limit=None))]
    pub fn get_bill_text(
        &self,
        congress: i32,
        bill_type: String,
        bill_number: i32,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> PyResult<Vec<TextVersion>> {
        let mut params = HashMap::new();
        
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        
        let endpoint = format!("/bill/{}/{}/{}/text", congress, bill_type, bill_number);
        let response: TextVersionsResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        
        Ok(response.text_versions)
    }

    /// Get the list of titles for a specified bill
    #[pyo3(signature = (congress, bill_type, bill_number, format=None, offset=None, limit=None))]
    pub fn get_bill_titles(
        &self,
        congress: i32,
        bill_type: String,
        bill_number: i32,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> PyResult<Vec<Title>> {
        let mut params = HashMap::new();
        
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        
        let endpoint = format!("/bill/{}/{}/{}/titles", congress, bill_type, bill_number);
        let response: TitlesResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        
        Ok(response.titles)
    }

    // ========== Amendment Endpoints ==========

    /// Get a list of amendments sorted by date of latest action
    #[pyo3(signature = (format=None, offset=None, limit=None, from_date_time=None, to_date_time=None))]
    pub fn list_amendments(
        &self,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
        from_date_time: Option<String>,
        to_date_time: Option<String>,
    ) -> PyResult<Vec<Amendment>> {
        let mut params = HashMap::new();
        
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        if let Some(from) = from_date_time {
            params.insert("fromDateTime".to_string(), from);
        }
        if let Some(to) = to_date_time {
            params.insert("toDateTime".to_string(), to);
        }
        
        let response: AmendmentsResponse = self
            .client
            .get("/amendment", Some(params))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        
        Ok(response.amendments)
    }

    /// Get amendments filtered by congress
    #[pyo3(signature = (congress, format=None, offset=None, limit=None, from_date_time=None, to_date_time=None))]
    pub fn list_amendments_by_congress(
        &self,
        congress: i32,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
        from_date_time: Option<String>,
        to_date_time: Option<String>,
    ) -> PyResult<Vec<Amendment>> {
        let mut params = HashMap::new();
        
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        if let Some(from) = from_date_time {
            params.insert("fromDateTime".to_string(), from);
        }
        if let Some(to) = to_date_time {
            params.insert("toDateTime".to_string(), to);
        }
        
        let endpoint = format!("/amendment/{}", congress);
        let response: AmendmentsResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        
        Ok(response.amendments)
    }

    // ========== Member Endpoints ==========

    /// Get a list of congressional members
    #[pyo3(signature = (format=None, offset=None, limit=None, from_date_time=None, to_date_time=None, current_member=None))]
    pub fn list_members(
        &self,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
        from_date_time: Option<String>,
        to_date_time: Option<String>,
        current_member: Option<bool>,
    ) -> PyResult<Vec<Sponsor>> {
        let mut params = HashMap::new();
        
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        if let Some(from) = from_date_time {
            params.insert("fromDateTime".to_string(), from);
        }
        if let Some(to) = to_date_time {
            params.insert("toDateTime".to_string(), to);
        }
        if let Some(cm) = current_member {
            params.insert("currentMember".to_string(), cm.to_string());
        }
        
        let response: MembersResponse = self
            .client
            .get("/member", Some(params))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        
        Ok(response.members)
    }

    /// Get detailed information for a specified congressional member
    pub fn get_member(&self, bioguide_id: String) -> PyResult<Sponsor> {
        let endpoint = format!("/member/{}", bioguide_id);
        let response: MemberResponse = self
            .client
            .get(&endpoint, None)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        
        Ok(response.member)
    }

    /// Get the list of members by congress
    #[pyo3(signature = (congress, format=None, offset=None, limit=None, current_member=None))]
    pub fn list_members_by_congress(
        &self,
        congress: i32,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
        current_member: Option<bool>,
    ) -> PyResult<Vec<Sponsor>> {
        let mut params = HashMap::new();
        
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        if let Some(cm) = current_member {
            params.insert("currentMember".to_string(), cm.to_string());
        }
        
        let endpoint = format!("/member/congress/{}", congress);
        let response: MembersResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        
        Ok(response.members)
    }

    /// Get legislation sponsored by a specified member
    #[pyo3(signature = (bioguide_id, format=None, offset=None, limit=None))]
    pub fn get_member_sponsored_legislation(
        &self,
        bioguide_id: String,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> PyResult<Vec<Bill>> {
        let mut params = HashMap::new();
        
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        
        let endpoint = format!("/member/{}/sponsored-legislation", bioguide_id);
        let response: SponsoredLegislationResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        
        Ok(response.sponsored_legislation)
    }

    /// Get legislation cosponsored by a specified member
    #[pyo3(signature = (bioguide_id, format=None, offset=None, limit=None))]
    pub fn get_member_cosponsored_legislation(
        &self,
        bioguide_id: String,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> PyResult<Vec<Bill>> {
        let mut params = HashMap::new();
        
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        
        let endpoint = format!("/member/{}/cosponsored-legislation", bioguide_id);
        let response: CosponsoredLegislationResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        
        Ok(response.cosponsored_legislation)
    }

    /// Get the list of members by state
    #[pyo3(signature = (state_code, format=None, limit=None, current_member=None))]
    pub fn list_members_by_state(
        &self,
        state_code: String,
        format: Option<String>,
        limit: Option<i32>,
        current_member: Option<bool>,
    ) -> PyResult<Vec<Sponsor>> {
        let mut params = HashMap::new();
        
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        if let Some(cm) = current_member {
            params.insert("currentMember".to_string(), cm.to_string());
        }
        
        let endpoint = format!("/member/{}", state_code);
        let response: MembersResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        
        Ok(response.members)
    }

    /// Get the list of members by state and district
    #[pyo3(signature = (state_code, district, format=None, current_member=None))]
    pub fn list_members_by_state_district(
        &self,
        state_code: String,
        district: i32,
        format: Option<String>,
        current_member: Option<bool>,
    ) -> PyResult<Vec<Sponsor>> {
        let mut params = HashMap::new();
        
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(cm) = current_member {
            params.insert("currentMember".to_string(), cm.to_string());
        }
        
        let endpoint = format!("/member/{}/{}", state_code, district);
        let response: MembersResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        
        Ok(response.members)
    }

    // ========== Committee Endpoints ==========

    /// Get a list of committees
    #[pyo3(signature = (format=None, offset=None, limit=None))]
    pub fn list_committees(
        &self,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> PyResult<Vec<CommitteeItem>> {
        let mut params = HashMap::new();
        
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        
        let response: CommitteesListResponse = self
            .client
            .get("/committee", Some(params))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        
        Ok(response.committees)
    }

    // ========== Congress/Session Endpoints ==========

    /// Get a list of congresses and congressional sessions
    #[pyo3(signature = (format=None, offset=None, limit=None))]
    pub fn list_congresses(
        &self,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> PyResult<Vec<Congress>> {
        let mut params = HashMap::new();
        
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        
        let response: CongressesResponse = self
            .client
            .get("/congress", Some(params))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        
        Ok(response.congresses)
    }

    /// Get information about a specific congress
    #[pyo3(signature = (congress, format=None))]
    pub fn get_congress(
        &self,
        congress: i32,
        format: Option<String>,
    ) -> PyResult<Congress> {
        let mut params = HashMap::new();
        
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        
        let endpoint = format!("/congress/{}", congress);
        let response: CongressResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        
        Ok(response.congress)
    }

    /// Get information about the current congress
    #[pyo3(signature = (format=None))]
    pub fn get_current_congress(
        &self,
        format: Option<String>,
    ) -> PyResult<Congress> {
        let mut params = HashMap::new();
        
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        
        let response: CongressResponse = self
            .client
            .get("/congress/current", Some(params))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        
        Ok(response.congress)
    }

    // ========================================
    // House Vote Operations
    // ========================================

    /// Get a list of house votes (BETA)
    #[pyo3(signature = (offset=None, limit=None, from_date=None, to_date=None, sort=None, format=None))]
    pub fn list_house_votes(
        &self,
        offset: Option<i32>,
        limit: Option<i32>,
        from_date: Option<String>,
        to_date: Option<String>,
        sort: Option<String>,
        format: Option<String>,
    ) -> PyResult<Vec<HouseVote>> {
        let mut params = HashMap::new();
        
        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(from) = from_date {
            params.insert("fromDateTime".to_string(), from);
        }
        if let Some(to) = to_date {
            params.insert("toDateTime".to_string(), to);
        }
        if let Some(s) = sort {
            params.insert("sort".to_string(), s);
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        
        let response: HouseVotesResponse = self
            .client
            .get("/house-vote", Some(params))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        
        Ok(response.votes)
    }

    /// Get house votes for a specific congress (BETA)
    #[pyo3(signature = (congress, offset=None, limit=None, from_date=None, to_date=None, sort=None, format=None))]
    pub fn list_house_votes_by_congress(
        &self,
        congress: i32,
        offset: Option<i32>,
        limit: Option<i32>,
        from_date: Option<String>,
        to_date: Option<String>,
        sort: Option<String>,
        format: Option<String>,
    ) -> PyResult<Vec<HouseVote>> {
        let mut params = HashMap::new();
        
        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(from) = from_date {
            params.insert("fromDateTime".to_string(), from);
        }
        if let Some(to) = to_date {
            params.insert("toDateTime".to_string(), to);
        }
        if let Some(s) = sort {
            params.insert("sort".to_string(), s);
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        
        let endpoint = format!("/house-vote/{}", congress);
        let response: HouseVotesResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        
        Ok(response.votes)
    }

    /// Get house votes for a specific congress and session (BETA)
    #[pyo3(signature = (congress, session, offset=None, limit=None, from_date=None, to_date=None, sort=None, format=None))]
    pub fn list_house_votes_by_session(
        &self,
        congress: i32,
        session: i32,
        offset: Option<i32>,
        limit: Option<i32>,
        from_date: Option<String>,
        to_date: Option<String>,
        sort: Option<String>,
        format: Option<String>,
    ) -> PyResult<Vec<HouseVote>> {
        let mut params = HashMap::new();
        
        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(from) = from_date {
            params.insert("fromDateTime".to_string(), from);
        }
        if let Some(to) = to_date {
            params.insert("toDateTime".to_string(), to);
        }
        if let Some(s) = sort {
            params.insert("sort".to_string(), s);
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        
        let endpoint = format!("/house-vote/{}/{}", congress, session);
        let response: HouseVotesResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        
        Ok(response.votes)
    }

    /// Get detailed information about a specific house vote (BETA)
    #[pyo3(signature = (congress, session, vote_number, format=None))]
    pub fn get_house_vote(
        &self,
        congress: i32,
        session: i32,
        vote_number: i32,
        format: Option<String>,
    ) -> PyResult<HouseVoteDetail> {
        let mut params = HashMap::new();
        
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        
        let endpoint = format!("/house-vote/{}/{}/{}", congress, session, vote_number);
        let response: HouseVoteDetailResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        
        Ok(response.vote)
    }

    /// Get how members voted on a specific house vote (BETA)
    #[pyo3(signature = (congress, session, vote_number, offset=None, limit=None, format=None))]
    pub fn get_house_vote_members(
        &self,
        congress: i32,
        session: i32,
        vote_number: i32,
        offset: Option<i32>,
        limit: Option<i32>,
        format: Option<String>,
    ) -> PyResult<HouseVoteMembers> {
        let mut params = HashMap::new();
        
        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        
        let endpoint = format!("/house-vote/{}/{}/{}/members", congress, session, vote_number);
        let response: HouseVoteMembersResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        
        Ok(response.vote)
    }

    // ========================================
    // Committee Operations
    // ========================================

    /// Get committees filtered by chamber
    #[pyo3(signature = (chamber, offset=None, limit=None, format=None))]
    pub fn list_committees_by_chamber(
        &self,
        chamber: String,
        offset: Option<i32>,
        limit: Option<i32>,
        format: Option<String>,
    ) -> PyResult<Vec<CommitteeItem>> {
        let mut params = HashMap::new();
        
        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        
        let endpoint = format!("/committee/{}", chamber);
        let response: CommitteesListResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        
        Ok(response.committees)
    }

    /// Get committees filtered by congress
    #[pyo3(signature = (congress, offset=None, limit=None, format=None))]
    pub fn list_committees_by_congress(
        &self,
        congress: i32,
        offset: Option<i32>,
        limit: Option<i32>,
        format: Option<String>,
    ) -> PyResult<Vec<CommitteeItem>> {
        let mut params = HashMap::new();
        
        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        
        let endpoint = format!("/committee/{}", congress);
        let response: CommitteesListResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        
        Ok(response.committees)
    }

    /// Get committees filtered by congress and chamber
    #[pyo3(signature = (congress, chamber, offset=None, limit=None, format=None))]
    pub fn list_committees_by_congress_and_chamber(
        &self,
        congress: i32,
        chamber: String,
        offset: Option<i32>,
        limit: Option<i32>,
        format: Option<String>,
    ) -> PyResult<Vec<CommitteeItem>> {
        let mut params = HashMap::new();
        
        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        
        let endpoint = format!("/committee/{}/{}", congress, chamber);
        let response: CommitteesListResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        
        Ok(response.committees)
    }

    /// Get detailed information about a specific committee
    #[pyo3(signature = (chamber, committee_code, format=None))]
    pub fn get_committee(
        &self,
        chamber: String,
        committee_code: String,
        format: Option<String>,
    ) -> PyResult<CommitteeDetailInfo> {
        let mut params = HashMap::new();
        
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        
        let endpoint = format!("/committee/{}/{}", chamber, committee_code);
        let response: CommitteeDetailResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        
        Ok(response.committee)
    }

    /// Get bills associated with a committee
    #[pyo3(signature = (chamber, committee_code, offset=None, limit=None, format=None))]
    pub fn get_committee_bills(
        &self,
        chamber: String,
        committee_code: String,
        offset: Option<i32>,
        limit: Option<i32>,
        format: Option<String>,
    ) -> PyResult<Vec<CommitteeBill>> {
        let mut params = HashMap::new();
        
        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        
        let endpoint = format!("/committee/{}/{}/bills", chamber, committee_code);
        let response: CommitteeBillsResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        
        Ok(response.bills)
    }

    // ========================================
    // Committee Report Operations
    // ========================================

    /// Get a list of all committee reports
    #[pyo3(signature = (offset=None, limit=None, from_date=None, to_date=None, sort=None, format=None))]
    pub fn list_committee_reports(
        &self,
        offset: Option<i32>,
        limit: Option<i32>,
        from_date: Option<String>,
        to_date: Option<String>,
        sort: Option<String>,
        format: Option<String>,
    ) -> PyResult<Vec<CommitteeReportItem>> {
        let mut params = HashMap::new();
        
        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(from) = from_date {
            params.insert("fromDateTime".to_string(), from);
        }
        if let Some(to) = to_date {
            params.insert("toDateTime".to_string(), to);
        }
        if let Some(s) = sort {
            params.insert("sort".to_string(), s);
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        
        let response: CommitteeReportsResponse = self
            .client
            .get("/committee-report", Some(params))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        
        Ok(response.reports)
    }

    /// Get committee reports filtered by congress
    #[pyo3(signature = (congress, offset=None, limit=None, from_date=None, to_date=None, sort=None, format=None))]
    pub fn list_committee_reports_by_congress(
        &self,
        congress: i32,
        offset: Option<i32>,
        limit: Option<i32>,
        from_date: Option<String>,
        to_date: Option<String>,
        sort: Option<String>,
        format: Option<String>,
    ) -> PyResult<Vec<CommitteeReportItem>> {
        let mut params = HashMap::new();
        
        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(from) = from_date {
            params.insert("fromDateTime".to_string(), from);
        }
        if let Some(to) = to_date {
            params.insert("toDateTime".to_string(), to);
        }
        if let Some(s) = sort {
            params.insert("sort".to_string(), s);
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        
        let endpoint = format!("/committee-report/{}", congress);
        let response: CommitteeReportsResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        
        Ok(response.reports)
    }

    /// Get committee reports filtered by congress and report type
    #[pyo3(signature = (congress, report_type, offset=None, limit=None, from_date=None, to_date=None, sort=None, format=None))]
    pub fn list_committee_reports_by_type(
        &self,
        congress: i32,
        report_type: String,
        offset: Option<i32>,
        limit: Option<i32>,
        from_date: Option<String>,
        to_date: Option<String>,
        sort: Option<String>,
        format: Option<String>,
    ) -> PyResult<Vec<CommitteeReportItem>> {
        let mut params = HashMap::new();
        
        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(from) = from_date {
            params.insert("fromDateTime".to_string(), from);
        }
        if let Some(to) = to_date {
            params.insert("toDateTime".to_string(), to);
        }
        if let Some(s) = sort {
            params.insert("sort".to_string(), s);
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        
        let endpoint = format!("/committee-report/{}/{}", congress, report_type);
        let response: CommitteeReportsResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        
        Ok(response.reports)
    }

    /// Get detailed information about a specific committee report
    #[pyo3(signature = (congress, report_type, report_number, format=None))]
    pub fn get_committee_report(
        &self,
        congress: i32,
        report_type: String,
        report_number: i32,
        format: Option<String>,
    ) -> PyResult<CommitteeReportDetail> {
        let mut params = HashMap::new();
        
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        
        let endpoint = format!("/committee-report/{}/{}/{}", congress, report_type, report_number);
        let response: CommitteeReportDetailResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        
        Ok(response.report)
    }

    /// Get text formats available for a committee report
    #[pyo3(signature = (congress, report_type, report_number, format=None))]
    pub fn get_committee_report_text(
        &self,
        congress: i32,
        report_type: String,
        report_number: i32,
        format: Option<String>,
    ) -> PyResult<Vec<CommitteeReportText>> {
        let mut params = HashMap::new();
        
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        
        let endpoint = format!("/committee-report/{}/{}/{}/text", congress, report_type, report_number);
        let response: CommitteeReportTextResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        
        Ok(response.text)
    }

    // ========================================
    // Committee Print Operations
    // ========================================

    /// Get a list of all committee prints
    #[pyo3(signature = (offset=None, limit=None, from_date=None, to_date=None, sort=None, format=None))]
    pub fn list_committee_prints(
        &self,
        offset: Option<i32>,
        limit: Option<i32>,
        from_date: Option<String>,
        to_date: Option<String>,
        sort: Option<String>,
        format: Option<String>,
    ) -> PyResult<Vec<CommitteePrintItem>> {
        let mut params = HashMap::new();
        
        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(from) = from_date {
            params.insert("fromDateTime".to_string(), from);
        }
        if let Some(to) = to_date {
            params.insert("toDateTime".to_string(), to);
        }
        if let Some(s) = sort {
            params.insert("sort".to_string(), s);
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        
        let response: CommitteePrintsResponse = self
            .client
            .get("/committee-print", Some(params))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        
        Ok(response.committee_prints)
    }

    /// Get committee prints filtered by congress
    #[pyo3(signature = (congress, offset=None, limit=None, from_date=None, to_date=None, sort=None, format=None))]
    pub fn list_committee_prints_by_congress(
        &self,
        congress: i32,
        offset: Option<i32>,
        limit: Option<i32>,
        from_date: Option<String>,
        to_date: Option<String>,
        sort: Option<String>,
        format: Option<String>,
    ) -> PyResult<Vec<CommitteePrintItem>> {
        let mut params = HashMap::new();
        
        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(from) = from_date {
            params.insert("fromDateTime".to_string(), from);
        }
        if let Some(to) = to_date {
            params.insert("toDateTime".to_string(), to);
        }
        if let Some(s) = sort {
            params.insert("sort".to_string(), s);
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        
        let endpoint = format!("/committee-print/{}", congress);
        let response: CommitteePrintsResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        
        Ok(response.committee_prints)
    }

    /// Get committee prints filtered by congress and chamber
    #[pyo3(signature = (congress, chamber, offset=None, limit=None, from_date=None, to_date=None, sort=None, format=None))]
    pub fn list_committee_prints_by_chamber(
        &self,
        congress: i32,
        chamber: String,
        offset: Option<i32>,
        limit: Option<i32>,
        from_date: Option<String>,
        to_date: Option<String>,
        sort: Option<String>,
        format: Option<String>,
    ) -> PyResult<Vec<CommitteePrintItem>> {
        let mut params = HashMap::new();
        
        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(from) = from_date {
            params.insert("fromDateTime".to_string(), from);
        }
        if let Some(to) = to_date {
            params.insert("toDateTime".to_string(), to);
        }
        if let Some(s) = sort {
            params.insert("sort".to_string(), s);
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        
        let endpoint = format!("/committee-print/{}/{}", congress, chamber);
        let response: CommitteePrintsResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        
        Ok(response.committee_prints)
    }

    /// Get detailed information about a specific committee print
    #[pyo3(signature = (congress, chamber, jacket_number, format=None))]
    pub fn get_committee_print(
        &self,
        congress: i32,
        chamber: String,
        jacket_number: i32,
        format: Option<String>,
    ) -> PyResult<CommitteePrintDetail> {
        let mut params = HashMap::new();
        
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        
        let endpoint = format!("/committee-print/{}/{}/{}", congress, chamber, jacket_number);
        let response: CommitteePrintDetailResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        
        Ok(response.committee_print)
    }

    /// Get text formats available for a committee print
    #[pyo3(signature = (congress, chamber, jacket_number, format=None))]
    pub fn get_committee_print_text(
        &self,
        congress: i32,
        chamber: String,
        jacket_number: i32,
        format: Option<String>,
    ) -> PyResult<Vec<CommitteePrintText>> {
        let mut params = HashMap::new();
        
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        
        let endpoint = format!("/committee-print/{}/{}/{}/text", congress, chamber, jacket_number);
        let response: CommitteePrintTextResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        
        Ok(response.text)
    }

    // ========================================
    // Nomination Operations
    // ========================================

    /// Get a list of all nominations
    #[pyo3(signature = (offset=None, limit=None, sort=None, format=None))]
    pub fn list_nominations(
        &self,
        offset: Option<i32>,
        limit: Option<i32>,
        sort: Option<String>,
        format: Option<String>,
    ) -> PyResult<Vec<Nomination>> {
        let mut params = HashMap::new();
        
        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(s) = sort {
            params.insert("sort".to_string(), s);
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        
        let response: NominationsResponse = self
            .client
            .get("/nomination", Some(params))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        
        Ok(response.nominations)
    }

    /// Get nominations by congress
    #[pyo3(signature = (congress, offset=None, limit=None, sort=None, format=None))]
    pub fn list_nominations_by_congress(
        &self,
        congress: i32,
        offset: Option<i32>,
        limit: Option<i32>,
        sort: Option<String>,
        format: Option<String>,
    ) -> PyResult<Vec<Nomination>> {
        let mut params = HashMap::new();
        
        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(s) = sort {
            params.insert("sort".to_string(), s);
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        
        let endpoint = format!("/nomination/{}", congress);
        let response: NominationsResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        
        Ok(response.nominations)
    }

    /// Get a specific nomination
    #[pyo3(signature = (congress, nomination_number, format=None))]
    pub fn get_nomination(
        &self,
        congress: i32,
        nomination_number: String,
        format: Option<String>,
    ) -> PyResult<Nomination> {
        let mut params = HashMap::new();
        
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        
        let endpoint = format!("/nomination/{}/{}", congress, nomination_number);
        let response: NominationDetailResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        
        Ok(response.nomination)
    }

    /// Get nominees for a nomination
    #[pyo3(signature = (congress, nomination_number, offset=None, limit=None, format=None))]
    pub fn get_nomination_nominees(
        &self,
        congress: i32,
        nomination_number: String,
        offset: Option<i32>,
        limit: Option<i32>,
        format: Option<String>,
    ) -> PyResult<Vec<Nominee>> {
        let mut params = HashMap::new();
        
        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        
        let endpoint = format!("/nomination/{}/{}/nominees", congress, nomination_number);
        let response: NomineesResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        
        Ok(response.nominees)
    }

    // ========================================
    // Treaty Operations
    // ========================================

    /// Get a list of all treaties
    #[pyo3(signature = (offset=None, limit=None, sort=None, format=None))]
    pub fn list_treaties(
        &self,
        offset: Option<i32>,
        limit: Option<i32>,
        sort: Option<String>,
        format: Option<String>,
    ) -> PyResult<Vec<Treaty>> {
        let mut params = HashMap::new();
        
        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(s) = sort {
            params.insert("sort".to_string(), s);
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        
        let response: TreatiesResponse = self
            .client
            .get("/treaty", Some(params))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        
        Ok(response.treaties)
    }

    /// Get treaties by congress
    #[pyo3(signature = (congress, offset=None, limit=None, sort=None, format=None))]
    pub fn list_treaties_by_congress(
        &self,
        congress: i32,
        offset: Option<i32>,
        limit: Option<i32>,
        sort: Option<String>,
        format: Option<String>,
    ) -> PyResult<Vec<Treaty>> {
        let mut params = HashMap::new();
        
        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(s) = sort {
            params.insert("sort".to_string(), s);
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        
        let endpoint = format!("/treaty/{}", congress);
        let response: TreatiesResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        
        Ok(response.treaties)
    }

    /// Get a specific treaty
    #[pyo3(signature = (congress, treaty_number, format=None))]
    pub fn get_treaty(
        &self,
        congress: i32,
        treaty_number: String,
        format: Option<String>,
    ) -> PyResult<Treaty> {
        let mut params = HashMap::new();
        
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        
        let endpoint = format!("/treaty/{}/{}", congress, treaty_number);
        let response: TreatyDetailResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        
        Ok(response.treaty)
    }

    // ========================================
    // Hearing Operations
    // ========================================

    /// Get a list of all hearings
    #[pyo3(signature = (offset=None, limit=None, sort=None, format=None))]
    pub fn list_hearings(
        &self,
        offset: Option<i32>,
        limit: Option<i32>,
        sort: Option<String>,
        format: Option<String>,
    ) -> PyResult<Vec<Hearing>> {
        let mut params = HashMap::new();
        
        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(s) = sort {
            params.insert("sort".to_string(), s);
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        
        let response: HearingsResponse = self
            .client
            .get("/hearing", Some(params))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        
        Ok(response.hearings)
    }

    /// Get hearings by congress
    #[pyo3(signature = (congress, offset=None, limit=None, sort=None, format=None))]
    pub fn list_hearings_by_congress(
        &self,
        congress: i32,
        offset: Option<i32>,
        limit: Option<i32>,
        sort: Option<String>,
        format: Option<String>,
    ) -> PyResult<Vec<Hearing>> {
        let mut params = HashMap::new();
        
        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(s) = sort {
            params.insert("sort".to_string(), s);
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        
        let endpoint = format!("/hearing/{}", congress);
        let response: HearingsResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        
        Ok(response.hearings)
    }

    /// Get hearings by congress and chamber
    #[pyo3(signature = (congress, chamber, offset=None, limit=None, sort=None, format=None))]
    pub fn list_hearings_by_chamber(
        &self,
        congress: i32,
        chamber: String,
        offset: Option<i32>,
        limit: Option<i32>,
        sort: Option<String>,
        format: Option<String>,
    ) -> PyResult<Vec<Hearing>> {
        let mut params = HashMap::new();
        
        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(s) = sort {
            params.insert("sort".to_string(), s);
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        
        let endpoint = format!("/hearing/{}/{}", congress, chamber.to_lowercase());
        let response: HearingsResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        
        Ok(response.hearings)
    }

    /// Get a specific hearing
    #[pyo3(signature = (congress, chamber, jacket_number, format=None))]
    pub fn get_hearing(
        &self,
        congress: i32,
        chamber: String,
        jacket_number: i32,
        format: Option<String>,
    ) -> PyResult<Hearing> {
        let mut params = HashMap::new();
        
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        
        let endpoint = format!("/hearing/{}/{}/{}", congress, chamber.to_lowercase(), jacket_number);
        let response: HearingDetailResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        
        Ok(response.hearing)
    }

    // ========================================
    // Congressional Record Operations
    // ========================================

    /// Get daily congressional records
    #[pyo3(signature = (offset=None, limit=None, format=None))]
    pub fn list_congressional_records(
        &self,
        offset: Option<i32>,
        limit: Option<i32>,
        format: Option<String>,
    ) -> PyResult<Vec<DailyCongressionalRecord>> {
        let mut params = HashMap::new();
        
        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        
        let response: DailyCongressionalRecordsResponse = self
            .client
            .get("/daily-congressional-record", Some(params))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        
        Ok(response.daily_congressional_record)
    }

    // ========================================
    // Law Operations
    // ========================================

    /// Get a list of all laws
    #[pyo3(signature = (offset=None, limit=None, format=None))]
    pub fn list_laws(
        &self,
        offset: Option<i32>,
        limit: Option<i32>,
        format: Option<String>,
    ) -> PyResult<Vec<LawItem>> {
        let mut params = HashMap::new();
        
        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        
        let response: LawsResponse = self
            .client
            .get("/law", Some(params))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        
        Ok(response.bills)
    }

    /// Get laws by congress
    #[pyo3(signature = (congress, offset=None, limit=None, format=None))]
    pub fn list_laws_by_congress(
        &self,
        congress: i32,
        offset: Option<i32>,
        limit: Option<i32>,
        format: Option<String>,
    ) -> PyResult<Vec<LawItem>> {
        let mut params = HashMap::new();
        
        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        
        let endpoint = format!("/law/{}", congress);
        let response: LawsResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        
        Ok(response.bills)
    }

    /// Get laws by congress and type
    /// Parameters:
    ///   - congress: The congress number (e.g., 118)
    ///   - law_type: The law type. Values are either "pub" (public laws) or "priv" (private laws)
    ///   - offset: Pagination offset (optional)
    ///   - limit: Number of results to return (optional)
    ///   - format: Response format (optional)
    #[pyo3(signature = (congress, law_type, offset=None, limit=None, format=None))]
    pub fn list_laws_by_type(
        &self,
        congress: i32,
        law_type: String,
        offset: Option<i32>,
        limit: Option<i32>,
        format: Option<String>,
    ) -> PyResult<Vec<LawItem>> {
        let mut params = HashMap::new();
        
        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        
        let endpoint = format!("/law/{}/{}", congress, law_type);
        let response: LawsResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        
        Ok(response.bills)
    }

    /// Get a specific law by bill type and bill number
    /// 
    /// Note: Despite the swagger documentation referring to "lawType" and "lawNumber",
    /// the actual API endpoint uses the BILL's type and number, not the resulting law's type/number.
    /// For example, to get the law that HR 4984 became, use law_type="hr" and law_number="4984"
    /// 
    /// Parameters:
    ///   - congress: The congress number (e.g., 118)
    ///   - law_type: Bill type like "hr", "s", "hjres", "sjres" (case-insensitive, will be lowercased)
    ///               This is NOT "pub"/"priv" - those are for list_laws_by_type()
    ///   - law_number: The bill number as string (e.g., "346" or "4984")
    #[pyo3(signature = (congress, law_type, law_number, format=None))]
    pub fn get_law(
        &self,
        congress: i32,
        law_type: String,
        law_number: String,
        format: Option<String>,
    ) -> PyResult<LawDetail> {
        let mut params = HashMap::new();
        
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        
        // API expects lowercase bill type
        let law_type_lower = law_type.to_lowercase();
        let endpoint = format!("/law/{}/{}/{}", congress, law_type_lower, law_number);
        let response: LawDetailResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        
        Ok(response.bill)
    }

    // ========================================
    // Summaries Operations
    // ========================================

    /// Get a list of summaries
    #[pyo3(signature = (offset=None, limit=None, format=None))]
    pub fn list_summaries(
        &self,
        offset: Option<i32>,
        limit: Option<i32>,
        format: Option<String>,
    ) -> PyResult<Vec<SummaryItem>> {
        let mut params = HashMap::new();
        
        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        
        let response: SummariesListResponse = self
            .client
            .get("/summaries", Some(params))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        
        Ok(response.summaries)
    }

    /// Get summaries by congress
    #[pyo3(signature = (congress, offset=None, limit=None, format=None))]
    pub fn list_summaries_by_congress(
        &self,
        congress: i32,
        offset: Option<i32>,
        limit: Option<i32>,
        format: Option<String>,
    ) -> PyResult<Vec<SummaryItem>> {
        let mut params = HashMap::new();
        
        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        
        let endpoint = format!("/summaries/{}", congress);
        let response: SummariesListResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        
        Ok(response.summaries)
    }

    // ========================================
    // CRS Report Operations
    // ========================================

    /// Get a list of CRS reports
    #[pyo3(signature = (offset=None, limit=None, from_date_time=None, to_date_time=None, format=None))]
    pub fn list_crs_reports(
        &self,
        offset: Option<i32>,
        limit: Option<i32>,
        from_date_time: Option<String>,
        to_date_time: Option<String>,
        format: Option<String>,
    ) -> PyResult<Vec<CrsReport>> {
        let mut params = HashMap::new();
        
        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(from) = from_date_time {
            params.insert("fromDateTime".to_string(), from);
        }
        if let Some(to) = to_date_time {
            params.insert("toDateTime".to_string(), to);
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        
        let response: CrsReportsResponse = self
            .client
            .get("/crsreport", Some(params))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        
        Ok(response.crs_reports)
    }

    /// Get detailed information for a specific CRS report
    #[pyo3(signature = (report_number, format=None))]
    pub fn get_crs_report(
        &self,
        report_number: String,
        format: Option<String>,
    ) -> PyResult<CrsReportDetail> {
        let mut params = HashMap::new();
        
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        
        let endpoint = format!("/crsreport/{}", report_number);
        let response: CrsReportDetailResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("{}", e)))?;
        
        Ok(response.report)
    }
}

