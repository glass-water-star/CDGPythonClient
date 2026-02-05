use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

/// Represents a hearing date
#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct HearingDate {
    #[pyo3(get)]
    pub date: Option<String>,
}

#[pymethods]
impl HearingDate {
    fn __repr__(&self) -> String {
        format!("HearingDate(date={:?})", self.date)
    }
}

/// Represents an associated meeting
#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct AssociatedMeeting {
    #[pyo3(get)]
    #[serde(rename = "eventId")]
    pub event_id: Option<String>,
    
    #[pyo3(get)]
    pub url: Option<String>,
}

#[pymethods]
impl AssociatedMeeting {
    fn __repr__(&self) -> String {
        format!("AssociatedMeeting(event_id={:?})", self.event_id)
    }
}

/// Represents a format option for a hearing
#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct HearingFormat {
    #[pyo3(get)]
    #[serde(rename = "type")]
    pub format_type: Option<String>,
    
    #[pyo3(get)]
    pub url: Option<String>,
}

#[pymethods]
impl HearingFormat {
    fn __repr__(&self) -> String {
        format!("HearingFormat(type={:?})", self.format_type)
    }
}

/// Represents a committee/subcommittee in a hearing
#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct HearingCommittee {
    #[pyo3(get)]
    pub name: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "systemCode")]
    pub system_code: Option<String>,
    
    #[pyo3(get)]
    pub url: Option<String>,
}

#[pymethods]
impl HearingCommittee {
    fn __repr__(&self) -> String {
        format!("HearingCommittee(name={:?}, code={:?})", self.name, self.system_code)
    }
}

/// Represents a hearing in list responses
#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct Hearing {
    #[pyo3(get)]
    pub chamber: Option<String>,
    
    #[pyo3(get)]
    pub congress: Option<i32>,
    
    #[pyo3(get)]
    #[serde(rename = "jacketNumber")]
    pub jacket_number: Option<i32>,
    
    #[pyo3(get)]
    pub number: Option<i32>,
    
    #[pyo3(get)]
    pub part: Option<i32>,
    
    #[pyo3(get)]
    pub title: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "updateDate")]
    pub update_date: Option<String>,
    
    #[pyo3(get)]
    pub url: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "associatedMeeting")]
    pub associated_meeting: Option<AssociatedMeeting>,
    
    #[pyo3(get)]
    pub citation: Option<String>,
    
    #[pyo3(get)]
    pub committees: Option<Vec<HearingCommittee>>,
    
    #[pyo3(get)]
    pub dates: Option<Vec<HearingDate>>,
    
    #[pyo3(get)]
    pub formats: Option<Vec<HearingFormat>>,
    
    #[pyo3(get)]
    #[serde(rename = "libraryOfCongressIdentifier")]
    pub library_of_congress_identifier: Option<String>,
}

#[pymethods]
impl Hearing {
    fn __repr__(&self) -> String {
        format!(
            "Hearing(congress={:?}, chamber={:?}, jacket_number={:?}, title={:?})",
            self.congress, self.chamber, self.jacket_number, self.title
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HearingsResponse {
    pub hearings: Vec<Hearing>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HearingDetailResponse {
    pub hearing: Hearing,
}
