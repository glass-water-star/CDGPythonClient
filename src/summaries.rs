use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

/// Represents a summary item
#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct SummaryItem {
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
impl SummaryItem {
    fn __repr__(&self) -> String {
        format!(
            "SummaryItem(action={:?}, date={:?})",
            self.action_desc, self.action_date
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummariesListResponse {
    pub summaries: Vec<SummaryItem>,
}
