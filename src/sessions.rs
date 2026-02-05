use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

/// Represents a Congressional session
#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct Session {
    #[pyo3(get)]
    pub chamber: Option<String>,
    
    #[pyo3(get)]
    pub number: Option<i32>,
    
    #[pyo3(get)]
    #[serde(rename = "startDate")]
    pub start_date: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "endDate")]
    pub end_date: Option<String>,
}

#[pymethods]
impl Session {
    fn __repr__(&self) -> String {
        format!(
            "Session(chamber={:?}, number={:?}, start_date={:?}, end_date={:?})",
            self.chamber, self.number, self.start_date, self.end_date
        )
    }
}

/// Represents a Congress with its sessions
#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct Congress {
    #[pyo3(get)]
    #[serde(rename = "endYear")]
    pub end_year: Option<String>,
    
    #[pyo3(get)]
    pub name: Option<String>,
    
    #[pyo3(get)]
    pub sessions: Option<Vec<Session>>,
    
    #[pyo3(get)]
    #[serde(rename = "startYear")]
    pub start_year: Option<String>,
    
    #[pyo3(get)]
    pub url: Option<String>,
}

#[pymethods]
impl Congress {
    fn __repr__(&self) -> String {
        format!(
            "Congress(name={:?}, start_year={:?}, end_year={:?})",
            self.name, self.start_year, self.end_year
        )
    }
}

/// Response structure for congress list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CongressesResponse {
    pub congresses: Vec<Congress>,
}

/// Response structure for a single congress
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CongressResponse {
    pub congress: Congress,
}
