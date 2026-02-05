use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

/// Represents a nomination
#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct Nomination {
    #[pyo3(get)]
    pub congress: Option<i32>,
    
    #[pyo3(get)]
    pub number: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "partNumber")]
    pub part_number: Option<String>,
    
    #[pyo3(get)]
    pub citation: Option<String>,
    
    #[pyo3(get)]
    pub description: Option<String>,
    
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
        format!("Nominee(name={:?}, position={:?})", self.name, self.position)
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
