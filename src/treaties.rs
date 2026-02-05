use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

/// Represents a treaty
#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct Treaty {
    #[pyo3(get)]
    pub congress: Option<i32>,
    
    #[pyo3(get)]
    pub number: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "partNumber")]
    pub part_number: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "suffix")]
    pub treaty_suffix: Option<String>,
    
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
