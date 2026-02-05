use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

use crate::bills::{LatestAction, Law};

/// Represents a bill that became a law
#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct LawItem {
    #[pyo3(get)]
    pub congress: Option<i32>,
    
    #[pyo3(get)]
    #[serde(rename = "latestAction")]
    pub latest_action: Option<LatestAction>,
    
    #[pyo3(get)]
    pub laws: Option<Vec<Law>>,
    
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
    pub law_type: Option<String>,
    
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
impl LawItem {
    fn __repr__(&self) -> String {
        format!(
            "LawItem(congress={:?}, type={:?}, number={:?}, title={:?})",
            self.congress, self.law_type, self.number, self.title
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct LawDetail {
    #[pyo3(get)]
    pub congress: Option<i32>,
    
    #[pyo3(get)]
    #[serde(rename = "latestAction")]
    pub latest_action: Option<LatestAction>,
    
    #[pyo3(get)]
    pub laws: Option<Vec<Law>>,
    
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
    pub law_type: Option<String>,
    
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
impl LawDetail {
    fn __repr__(&self) -> String {
        format!(
            "LawDetail(congress={:?}, type={:?}, number={:?}, title={:?})",
            self.congress, self.law_type, self.number, self.title
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LawsResponse {
    pub bills: Vec<LawItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LawDetailResponse {
    pub bill: LawDetail,
}
