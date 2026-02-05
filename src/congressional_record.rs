use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

/// Represents a daily congressional record
#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct DailyCongressionalRecord {
    #[pyo3(get)]
    #[serde(rename = "issueNumber")]
    pub issue_number: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "volumeNumber")]
    pub volume_number: Option<i32>,
    
    #[pyo3(get)]
    #[serde(rename = "issueDate")]
    pub issue_date: Option<String>,
    
    #[pyo3(get)]
    pub congress: Option<i32>,
    
    #[pyo3(get)]
    pub session: Option<i32>,
    
    #[pyo3(get)]
    #[serde(rename = "updateDate")]
    pub update_date: Option<String>,
    
    #[pyo3(get)]
    pub url: Option<String>,
}

#[pymethods]
impl DailyCongressionalRecord {
    fn __repr__(&self) -> String {
        format!(
            "DailyCongressionalRecord(volume={:?}, issue={:?}, date={:?})",
            self.volume_number, self.issue_number, self.issue_date
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyCongressionalRecordsResponse {
    #[serde(rename = "dailyCongressionalRecord")]
    pub daily_congressional_record: Vec<DailyCongressionalRecord>,
}
