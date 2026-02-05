use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

/// Represents a subcommittee
#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct Subcommittee {
    #[pyo3(get)]
    pub name: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "systemCode")]
    pub system_code: Option<String>,
    
    #[pyo3(get)]
    pub url: Option<String>,
}

#[pymethods]
impl Subcommittee {
    fn __repr__(&self) -> String {
        format!("Subcommittee(name={:?}, code={:?})", self.name, self.system_code)
    }
}

/// Represents a parent committee
#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct ParentCommittee {
    #[pyo3(get)]
    pub name: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "systemCode")]
    pub system_code: Option<String>,
    
    #[pyo3(get)]
    pub url: Option<String>,
}

#[pymethods]
impl ParentCommittee {
    fn __repr__(&self) -> String {
        format!("ParentCommittee(name={:?}, code={:?})", self.name, self.system_code)
    }
}

/// Represents a committee in list responses
#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct CommitteeItem {
    #[pyo3(get)]
    pub chamber: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "committeeTypeCode")]
    pub committee_type_code: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "updateDate")]
    pub update_date: Option<String>,
    
    #[pyo3(get)]
    pub name: Option<String>,
    
    #[pyo3(get)]
    pub parent: Option<ParentCommittee>,
    
    #[pyo3(get)]
    pub subcommittees: Option<Vec<Subcommittee>>,
    
    #[pyo3(get)]
    #[serde(rename = "systemCode")]
    pub system_code: Option<String>,
    
    #[pyo3(get)]
    pub url: Option<String>,
}

#[pymethods]
impl CommitteeItem {
    fn __repr__(&self) -> String {
        format!(
            "CommitteeItem(chamber={:?}, name={:?}, type={:?})",
            self.chamber, self.name, self.committee_type_code
        )
    }
}

/// Represents committee history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct CommitteeHistory {
    #[pyo3(get)]
    #[serde(rename = "libraryOfCongressName")]
    pub library_of_congress_name: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "officialName")]
    pub official_name: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "startDate")]
    pub start_date: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "endDate")]
    pub end_date: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "updateDate")]
    pub update_date: Option<String>,
}

#[pymethods]
impl CommitteeHistory {
    fn __repr__(&self) -> String {
        format!(
            "CommitteeHistory(name={:?}, start={:?})",
            self.official_name, self.start_date
        )
    }
}

/// Represents a resource count with URL
#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct ResourceCount {
    #[pyo3(get)]
    pub count: Option<i32>,
    
    #[pyo3(get)]
    pub url: Option<String>,
}

#[pymethods]
impl ResourceCount {
    fn __repr__(&self) -> String {
        format!("ResourceCount(count={:?})", self.count)
    }
}

/// Represents detailed committee information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct CommitteeDetailInfo {
    #[pyo3(get)]
    pub bills: Option<ResourceCount>,
    
    #[pyo3(get)]
    pub communications: Option<ResourceCount>,
    
    #[pyo3(get)]
    pub history: Option<Vec<CommitteeHistory>>,
    
    #[pyo3(get)]
    #[serde(rename = "isCurrent")]
    pub is_current: Option<bool>,
    
    #[pyo3(get)]
    pub reports: Option<ResourceCount>,
    
    #[pyo3(get)]
    pub subcommittees: Option<Vec<Subcommittee>>,
    
    #[pyo3(get)]
    #[serde(rename = "systemCode")]
    pub system_code: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "type")]
    pub committee_type: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "updateDate")]
    pub update_date: Option<String>,
}

#[pymethods]
impl CommitteeDetailInfo {
    fn __repr__(&self) -> String {
        format!(
            "CommitteeDetailInfo(code={:?}, type={:?}, current={:?})",
            self.system_code, self.committee_type, self.is_current
        )
    }
}

/// Represents a committee bill relationship
#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct CommitteeBill {
    #[pyo3(get)]
    #[serde(rename = "actionDate")]
    pub action_date: Option<String>,
    
    #[pyo3(get)]
    pub congress: Option<i32>,
    
    #[pyo3(get)]
    pub number: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "relationshipType")]
    pub relationship_type: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "type")]
    pub bill_type: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "updateDate")]
    pub update_date: Option<String>,
    
    #[pyo3(get)]
    pub url: Option<String>,
}

#[pymethods]
impl CommitteeBill {
    fn __repr__(&self) -> String {
        format!(
            "CommitteeBill(congress={:?}, type={:?}, number={:?})",
            self.congress, self.bill_type, self.number
        )
    }
}

/// Represents a committee report
#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct CommitteeReportItem {
    #[pyo3(get)]
    pub citation: Option<String>,
    
    #[pyo3(get)]
    pub congress: Option<i32>,
    
    #[pyo3(get)]
    pub number: Option<String>,
    
    #[pyo3(get)]
    pub part: Option<i32>,
    
    #[pyo3(get)]
    #[serde(rename = "type")]
    pub report_type: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "updateDate")]
    pub update_date: Option<String>,
    
    #[pyo3(get)]
    pub url: Option<String>,
}

#[pymethods]
impl CommitteeReportItem {
    fn __repr__(&self) -> String {
        format!(
            "CommitteeReportItem(citation={:?}, type={:?})",
            self.citation, self.report_type
        )
    }
}

/// Represents detailed committee report information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct CommitteeReportDetail {
    #[pyo3(get)]
    pub citation: Option<String>,
    
    #[pyo3(get)]
    pub congress: Option<i32>,
    
    #[pyo3(get)]
    #[serde(rename = "isConferenceReport")]
    pub is_conference_report: Option<bool>,
    
    #[pyo3(get)]
    pub number: Option<String>,
    
    #[pyo3(get)]
    pub part: Option<i32>,
    
    #[pyo3(get)]
    pub text: Option<ResourceCount>,
    
    #[pyo3(get)]
    pub title: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "type")]
    pub report_type: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "updateDate")]
    pub update_date: Option<String>,
}

#[pymethods]
impl CommitteeReportDetail {
    fn __repr__(&self) -> String {
        format!(
            "CommitteeReportDetail(citation={:?}, title={:?})",
            self.citation, self.title
        )
    }
}

/// Represents committee report text format
#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct CommitteeReportText {
    #[pyo3(get)]
    #[serde(rename = "type")]
    pub text_type: Option<String>,
    
    #[pyo3(get)]
    pub url: Option<String>,
}

#[pymethods]
impl CommitteeReportText {
    fn __repr__(&self) -> String {
        format!("CommitteeReportText(type={:?})", self.text_type)
    }
}

/// Represents a committee print
#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct CommitteePrintItem {
    #[pyo3(get)]
    pub chamber: Option<String>,
    
    #[pyo3(get)]
    pub citation: Option<String>,
    
    #[pyo3(get)]
    pub congress: Option<i32>,
    
    #[pyo3(get)]
    #[serde(rename = "jacketNumber")]
    pub jacket_number: Option<i32>,
    
    #[pyo3(get)]
    pub number: Option<String>,
    
    #[pyo3(get)]
    pub title: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "updateDate")]
    pub update_date: Option<String>,
}

#[pymethods]
impl CommitteePrintItem {
    fn __repr__(&self) -> String {
        format!(
            "CommitteePrintItem(citation={:?}, chamber={:?})",
            self.citation, self.chamber
        )
    }
}

/// Represents detailed committee print information
#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct CommitteePrintDetail {
    #[pyo3(get)]
    pub chamber: Option<String>,
    
    #[pyo3(get)]
    pub citation: Option<String>,
    
    #[pyo3(get)]
    pub congress: Option<i32>,
    
    #[pyo3(get)]
    #[serde(rename = "jacketNumber")]
    pub jacket_number: Option<i32>,
    
    #[pyo3(get)]
    pub number: Option<String>,
    
    #[pyo3(get)]
    pub text: Option<ResourceCount>,
    
    #[pyo3(get)]
    pub title: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "updateDate")]
    pub update_date: Option<String>,
}

#[pymethods]
impl CommitteePrintDetail {
    fn __repr__(&self) -> String {
        format!(
            "CommitteePrintDetail(citation={:?}, title={:?})",
            self.citation, self.title
        )
    }
}

/// Represents committee print text format
#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct CommitteePrintText {
    #[pyo3(get)]
    #[serde(rename = "type")]
    pub text_type: Option<String>,
    
    #[pyo3(get)]
    pub url: Option<String>,
}

#[pymethods]
impl CommitteePrintText {
    fn __repr__(&self) -> String {
        format!("CommitteePrintText(type={:?})", self.text_type)
    }
}

// Response structures (not exposed to Python)

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitteesResponse {
    pub committees: Vec<CommitteeItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitteeDetailResponse {
    pub committee: CommitteeDetailInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitteeBillsResponse {
    pub bills: Vec<CommitteeBill>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitteeReportsResponse {
    pub reports: Vec<CommitteeReportItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitteeReportDetailResponse {
    pub report: CommitteeReportDetail,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitteeReportTextResponse {
    pub text: Vec<CommitteeReportText>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitteePrintsResponse {
    #[serde(rename = "committeePrints")]
    pub committee_prints: Vec<CommitteePrintItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitteePrintDetailResponse {
    #[serde(rename = "committeePrint")]
    pub committee_print: CommitteePrintDetail,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitteePrintTextResponse {
    pub text: Vec<CommitteePrintText>,
}
