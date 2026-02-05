use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

use crate::members::Sponsor;

// Response structures
#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct LatestAction {
    #[pyo3(get)]
    #[serde(rename = "actionDate")]
    pub action_date: Option<String>,
    
    #[pyo3(get)]
    pub text: Option<String>,
}

#[pymethods]
impl LatestAction {
    fn __repr__(&self) -> String {
        format!(
            "LatestAction(action_date={:?}, text={:?})",
            self.action_date, self.text
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct Bill {
    #[pyo3(get)]
    pub congress: Option<i32>,
    
    #[pyo3(get)]
    #[serde(rename = "latestAction")]
    pub latest_action: Option<LatestAction>,
    
    #[pyo3(get)]
    pub number: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "originChamber")]
    pub origin_chamber: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "originChamberCode")]
    pub origin_chamber_code: Option<String>,
    
    #[pyo3(get)]
    pub title: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "type")]
    pub bill_type: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "updateDate")]
    pub update_date: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "updateDateIncludingText")]
    pub update_date_including_text: Option<String>,
    
    #[pyo3(get)]
    pub url: Option<String>,
}

#[pymethods]
impl Bill {
    fn __repr__(&self) -> String {
        format!(
            "Bill(congress={:?}, number={:?}, title={:?}, type={:?})",
            self.congress, self.number, self.title, self.bill_type
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillsResponse {
    pub bills: Vec<Bill>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct Law {
    #[pyo3(get)]
    pub number: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "type")]
    pub law_type: Option<String>,
}

#[pymethods]
impl Law {
    fn __repr__(&self) -> String {
        format!("Law(number={:?}, type={:?})", self.number, self.law_type)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct PolicyArea {
    #[pyo3(get)]
    pub name: Option<String>,
}

#[pymethods]
impl PolicyArea {
    fn __repr__(&self) -> String {
        format!("PolicyArea(name={:?})", self.name)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct RelatedCount {
    pub count: Option<i32>,
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct BillDetail {
    #[pyo3(get)]
    pub congress: Option<i32>,
    
    #[pyo3(get)]
    #[serde(rename = "latestAction")]
    pub latest_action: Option<LatestAction>,
    
    #[pyo3(get)]
    pub number: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "originChamber")]
    pub origin_chamber: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "originChamberCode")]
    pub origin_chamber_code: Option<String>,
    
    #[pyo3(get)]
    pub title: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "type")]
    pub bill_type: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "updateDate")]
    pub update_date: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "updateDateIncludingText")]
    pub update_date_including_text: Option<String>,
    
    #[pyo3(get)]
    pub url: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "introducedDate")]
    pub introduced_date: Option<String>,
    
    #[pyo3(get)]
    pub sponsors: Option<Vec<Sponsor>>,
    
    #[pyo3(get)]
    #[serde(rename = "policyArea")]
    pub policy_area: Option<PolicyArea>,
    
    #[pyo3(get)]
    pub laws: Option<Vec<Law>>,
}

#[pymethods]
impl BillDetail {
    fn __repr__(&self) -> String {
        format!(
            "BillDetail(congress={:?}, number={:?}, title={:?}, type={:?})",
            self.congress, self.number, self.title, self.bill_type
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillDetailResponse {
    pub bill: BillDetail,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct Action {
    #[pyo3(get)]
    #[serde(rename = "actionCode")]
    pub action_code: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "actionDate")]
    pub action_date: Option<String>,
    
    #[pyo3(get)]
    pub text: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "type")]
    pub action_type: Option<String>,
}

#[pymethods]
impl Action {
    fn __repr__(&self) -> String {
        format!(
            "Action(date={:?}, type={:?}, text={:?})",
            self.action_date, self.action_type, self.text
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionsResponse {
    pub actions: Vec<Action>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct Amendment {
    #[pyo3(get)]
    pub congress: Option<i32>,
    
    #[pyo3(get)]
    #[serde(rename = "latestAction")]
    pub latest_action: Option<LatestAction>,
    
    #[pyo3(get)]
    pub number: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "type")]
    pub amendment_type: Option<String>,
    
    #[pyo3(get)]
    pub url: Option<String>,
}

#[pymethods]
impl Amendment {
    fn __repr__(&self) -> String {
        format!(
            "Amendment(congress={:?}, number={:?}, type={:?})",
            self.congress, self.number, self.amendment_type
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmendmentsResponse {
    pub amendments: Vec<Amendment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct Committee {
    #[pyo3(get)]
    pub name: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "systemCode")]
    pub system_code: Option<String>,
    
    #[pyo3(get)]
    pub url: Option<String>,
}

#[pymethods]
impl Committee {
    fn __repr__(&self) -> String {
        format!("Committee(name={:?}, system_code={:?})", self.name, self.system_code)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitteesResponse {
    pub committees: Vec<Committee>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct Cosponsor {
    #[pyo3(get)]
    #[serde(rename = "bioguideId")]
    pub bioguide_id: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "firstName")]
    pub first_name: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "lastName")]
    pub last_name: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "fullName")]
    pub full_name: Option<String>,
    
    #[pyo3(get)]
    pub state: Option<String>,
    
    #[pyo3(get)]
    pub party: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "sponsorshipDate")]
    pub sponsorship_date: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "isOriginalCosponsor")]
    pub is_original_cosponsor: Option<bool>,
}

#[pymethods]
impl Cosponsor {
    fn __repr__(&self) -> String {
        format!(
            "Cosponsor(name={:?}, party={:?}, state={:?})",
            self.full_name, self.party, self.state
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CosponsorsResponse {
    pub cosponsors: Vec<Cosponsor>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct RelatedBill {
    #[pyo3(get)]
    pub congress: Option<i32>,
    
    #[pyo3(get)]
    pub number: Option<i32>,
    
    #[pyo3(get)]
    #[serde(rename = "type")]
    pub bill_type: Option<String>,
    
    #[pyo3(get)]
    pub title: Option<String>,
    
    #[pyo3(get)]
    pub url: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "latestAction")]
    pub latest_action: Option<LatestAction>,
    
    #[pyo3(get)]
    #[serde(rename = "relationshipDetails")]
    pub relationship_details: Option<Vec<RelationshipDetail>>,
}

#[pymethods]
impl RelatedBill {
    fn __repr__(&self) -> String {
        format!(
            "RelatedBill(congress={:?}, number={:?}, type={:?})",
            self.congress, self.number, self.bill_type
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct RelationshipDetail {
    #[pyo3(get)]
    #[serde(rename = "identifiedBy")]
    pub identified_by: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "type")]
    pub relationship_type: Option<String>,
}

#[pymethods]
impl RelationshipDetail {
    fn __repr__(&self) -> String {
        format!(
            "RelationshipDetail(identified_by={:?}, type={:?})",
            self.identified_by, self.relationship_type
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelatedBillsResponse {
    #[serde(rename = "relatedBills")]
    pub related_bills: Option<Vec<RelatedBill>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct Subject {
    #[pyo3(get)]
    pub name: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "updateDate")]
    pub update_date: Option<String>,
}

#[pymethods]
impl Subject {
    fn __repr__(&self) -> String {
        format!("Subject(name={:?})", self.name)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubjectsResponse {
    #[serde(rename = "legislativeSubjects")]
    pub legislative_subjects: Option<Vec<Subject>>,
    #[serde(rename = "policyArea")]
    pub policy_area: Option<PolicyArea>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct Summary {
    #[pyo3(get)]
    #[serde(rename = "actionDate")]
    pub action_date: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "actionDesc")]
    pub action_desc: Option<String>,
    
    #[pyo3(get)]
    pub text: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "updateDate")]
    pub update_date: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "versionCode")]
    pub version_code: Option<String>,
}

#[pymethods]
impl Summary {
    fn __repr__(&self) -> String {
        format!(
            "Summary(action_date={:?}, action_desc={:?})",
            self.action_date, self.action_desc
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummariesResponse {
    pub summaries: Vec<Summary>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct TextVersion {
    #[pyo3(get)]
    pub date: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "type")]
    pub text_type: Option<String>,
    
    #[pyo3(get)]
    pub formats: Option<Vec<TextFormat>>,
}

#[pymethods]
impl TextVersion {
    fn __repr__(&self) -> String {
        format!(
            "TextVersion(date={:?}, type={:?})",
            self.date, self.text_type
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct TextFormat {
    #[pyo3(get)]
    #[serde(rename = "type")]
    pub format_type: Option<String>,
    
    #[pyo3(get)]
    pub url: Option<String>,
}

#[pymethods]
impl TextFormat {
    fn __repr__(&self) -> String {
        format!(
            "TextFormat(type={:?}, url={:?})",
            self.format_type, self.url
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextVersionsResponse {
    #[serde(rename = "textVersions")]
    pub text_versions: Vec<TextVersion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct Title {
    #[pyo3(get)]
    pub title: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "titleType")]
    pub title_type: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "titleTypeCode")]
    pub title_type_code: Option<i32>,
}

#[pymethods]
impl Title {
    fn __repr__(&self) -> String {
        format!(
            "Title(title={:?}, type={:?})",
            self.title, self.title_type
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitlesResponse {
    pub titles: Vec<Title>,
}
