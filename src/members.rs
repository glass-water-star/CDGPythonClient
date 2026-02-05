use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

use crate::bills::Bill;

/// Represents a congressional member (used as Sponsor in bills)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct Sponsor {
    #[pyo3(get)]
    #[serde(rename = "bioguideId")]
    pub bioguide_id: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "firstName")]
    pub first_name: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "lastName")]
    pub last_name: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "fullName")]
    pub full_name: Option<String>,
    
    #[pyo3(get)]
    pub state: Option<String>,
    
    #[pyo3(get)]
    pub party: Option<String>,
    
    #[pyo3(get)]
    pub url: Option<String>,
}

#[pymethods]
impl Sponsor {
    fn __repr__(&self) -> String {
        format!(
            "Sponsor(name={:?}, party={:?}, state={:?})",
            self.full_name, self.party, self.state
        )
    }
}

/// Response structure for list of members
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MembersResponse {
    pub members: Vec<Sponsor>,
}

/// Response structure for a single member
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberResponse {
    pub member: Sponsor,
}

/// Response structure for sponsored legislation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SponsoredLegislationResponse {
    #[serde(rename = "sponsoredLegislation")]
    pub sponsored_legislation: Vec<Bill>,
}

/// Response structure for cosponsored legislation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CosponsoredLegislationResponse {
    #[serde(rename = "cosponsoredLegislation")]
    pub cosponsored_legislation: Vec<Bill>,
}
