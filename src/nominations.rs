use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

use crate::bills::LatestAction;

fn string_or_int<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error;
    use serde_json::Value;

    match Value::deserialize(deserializer)? {
        Value::Null => Ok(None),
        Value::String(value) => Ok(Some(value)),
        Value::Number(value) => Ok(Some(value.to_string())),
        other => Err(Error::custom(format!(
            "expected string or number, got {:?}",
            other
        ))),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct NominationType {
    #[pyo3(get)]
    #[serde(rename = "isCivilian")]
    pub is_civilian: Option<bool>,
}

#[pymethods]
impl NominationType {
    fn __repr__(&self) -> String {
        format!("NominationType(is_civilian={:?})", self.is_civilian)
    }
}

/// Represents a nomination
#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct Nomination {
    #[pyo3(get)]
    pub congress: Option<i32>,

    #[pyo3(get)]
    #[serde(deserialize_with = "string_or_int")]
    pub number: Option<String>,

    #[pyo3(get)]
    #[serde(rename = "partNumber")]
    pub part_number: Option<String>,

    #[pyo3(get)]
    pub citation: Option<String>,

    #[pyo3(get)]
    pub description: Option<String>,

    #[pyo3(get)]
    #[serde(rename = "latestAction")]
    pub latest_action: Option<LatestAction>,

    #[pyo3(get)]
    #[serde(rename = "nominationType")]
    pub nomination_type: Option<NominationType>,

    #[pyo3(get)]
    pub organization: Option<String>,

    #[pyo3(get)]
    #[serde(rename = "receivedDate")]
    pub received_date: Option<String>,

    #[pyo3(get)]
    #[serde(rename = "updateDate")]
    pub update_date: Option<String>,

    #[pyo3(get)]
    pub url: Option<String>,
}

#[pymethods]
impl Nomination {
    fn __repr__(&self) -> String {
        format!(
            "Nomination(congress={:?}, number={:?}, citation={:?})",
            self.congress, self.number, self.citation
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct NominationCommitteeActivity {
    #[pyo3(get)]
    pub date: Option<String>,

    #[pyo3(get)]
    pub name: Option<String>,
}

#[pymethods]
impl NominationCommitteeActivity {
    fn __repr__(&self) -> String {
        format!(
            "NominationCommitteeActivity(date={:?}, name={:?})",
            self.date, self.name
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct NominationCommittee {
    #[pyo3(get)]
    pub activities: Option<Vec<NominationCommitteeActivity>>,

    #[pyo3(get)]
    pub chamber: Option<String>,

    #[pyo3(get)]
    pub name: Option<String>,

    #[pyo3(get)]
    #[serde(rename = "systemCode")]
    pub system_code: Option<String>,

    #[pyo3(get)]
    #[serde(rename = "type")]
    pub committee_type: Option<String>,

    #[pyo3(get)]
    pub url: Option<String>,
}

#[pymethods]
impl NominationCommittee {
    fn __repr__(&self) -> String {
        format!(
            "NominationCommittee(name={:?}, chamber={:?})",
            self.name, self.chamber
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct NominationHearing {
    #[pyo3(get)]
    pub chamber: Option<String>,

    #[pyo3(get)]
    pub citation: Option<String>,

    #[pyo3(get)]
    pub date: Option<String>,

    #[pyo3(get)]
    #[serde(rename = "jacketNumber")]
    pub jacket_number: Option<i32>,

    #[pyo3(get)]
    pub number: Option<i32>,
}

#[pymethods]
impl NominationHearing {
    fn __repr__(&self) -> String {
        format!(
            "NominationHearing(citation={:?}, jacket_number={:?})",
            self.citation, self.jacket_number
        )
    }
}

/// Represents a nominee
#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct Nominee {
    #[pyo3(get)]
    #[serde(rename = "firstName")]
    pub first_name: Option<String>,

    #[pyo3(get)]
    #[serde(rename = "lastName")]
    pub last_name: Option<String>,

    #[pyo3(get)]
    pub name: Option<String>,

    #[pyo3(get)]
    pub position: Option<String>,

    #[pyo3(get)]
    pub state: Option<String>,
}

#[pymethods]
impl Nominee {
    fn __repr__(&self) -> String {
        format!(
            "Nominee(name={:?}, position={:?})",
            self.name, self.position
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NominationsResponse {
    pub nominations: Vec<Nomination>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NominationDetailResponse {
    pub nomination: Nomination,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NomineesResponse {
    pub nominees: Vec<Nominee>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NominationCommitteesResponse {
    pub committees: Vec<NominationCommittee>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NominationHearingsResponse {
    pub hearings: Vec<NominationHearing>,
}
