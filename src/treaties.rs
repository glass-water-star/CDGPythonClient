use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

use crate::bills::CountLink;
use crate::nominations::NominationCommittee;

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
pub struct TreatyParts {
    #[pyo3(get)]
    pub count: Option<i32>,

    #[pyo3(get)]
    pub urls: Option<Vec<String>>,
}

#[pymethods]
impl TreatyParts {
    fn __repr__(&self) -> String {
        format!("TreatyParts(count={:?})", self.count)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct TreatyCountryParty {
    #[pyo3(get)]
    pub name: Option<String>,
}

#[pymethods]
impl TreatyCountryParty {
    fn __repr__(&self) -> String {
        format!("TreatyCountryParty(name={:?})", self.name)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct TreatyIndexTerm {
    #[pyo3(get)]
    pub name: Option<String>,
}

#[pymethods]
impl TreatyIndexTerm {
    fn __repr__(&self) -> String {
        format!("TreatyIndexTerm(name={:?})", self.name)
    }
}

/// Represents a treaty
#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct Treaty {
    #[pyo3(get)]
    #[serde(rename = "congressReceived")]
    pub congress: Option<i32>,

    #[pyo3(get)]
    #[serde(rename = "congressConsidered")]
    pub congress_considered: Option<i32>,

    #[pyo3(get)]
    #[serde(deserialize_with = "string_or_int")]
    pub number: Option<String>,

    #[pyo3(get)]
    #[serde(rename = "partNumber")]
    pub part_number: Option<String>,

    #[pyo3(get)]
    #[serde(rename = "suffix")]
    pub treaty_suffix: Option<String>,

    #[pyo3(get)]
    pub parts: Option<TreatyParts>,

    #[pyo3(get)]
    pub topic: Option<String>,

    #[pyo3(get)]
    #[serde(rename = "inForceDate")]
    pub in_force_date: Option<String>,

    #[pyo3(get)]
    #[serde(rename = "transmittedDate")]
    pub transmitted_date: Option<String>,

    #[pyo3(get)]
    #[serde(rename = "updateDate")]
    pub update_date: Option<String>,

    #[pyo3(get)]
    pub url: Option<String>,

    #[pyo3(get)]
    pub actions: Option<CountLink>,

    #[pyo3(get)]
    #[serde(rename = "countriesParties")]
    pub countries_parties: Option<Vec<TreatyCountryParty>>,

    #[pyo3(get)]
    #[serde(rename = "indexTerms")]
    pub index_terms: Option<Vec<TreatyIndexTerm>>,

    #[pyo3(get)]
    #[serde(rename = "oldNumber")]
    pub old_number: Option<String>,

    #[pyo3(get)]
    #[serde(rename = "oldNumberDisplayName")]
    pub old_number_display_name: Option<String>,

    #[pyo3(get)]
    #[serde(rename = "resolutionText")]
    pub resolution_text: Option<String>,
}

#[pymethods]
impl Treaty {
    fn __repr__(&self) -> String {
        format!(
            "Treaty(congress={:?}, number={:?}, topic={:?})",
            self.congress, self.number, self.topic
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreatiesResponse {
    pub treaties: Vec<Treaty>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreatyDetailResponse {
    pub treaty: Treaty,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreatyPartDetailResponse {
    pub treaty: Vec<Treaty>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreatyCommitteesResponse {
    #[serde(rename = "treatyCommittees")]
    pub treaty_committees: Vec<NominationCommittee>,
}
