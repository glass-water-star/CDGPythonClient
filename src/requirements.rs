use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct HouseRequirement {
    #[pyo3(get)]
    #[serde(rename = "activeRecord")]
    pub active_record: Option<bool>,

    #[pyo3(get)]
    pub frequency: Option<String>,

    #[pyo3(get)]
    #[serde(rename = "legalAuthority")]
    pub legal_authority: Option<String>,

    #[pyo3(get)]
    pub nature: Option<String>,

    #[pyo3(get)]
    pub number: Option<i32>,

    #[pyo3(get)]
    #[serde(rename = "parentAgency")]
    pub parent_agency: Option<String>,

    #[pyo3(get)]
    #[serde(rename = "submittingAgency")]
    pub submitting_agency: Option<String>,

    #[pyo3(get)]
    #[serde(rename = "submittingOfficial")]
    pub submitting_official: Option<String>,

    #[pyo3(get)]
    #[serde(rename = "updateDate")]
    pub update_date: Option<String>,

    #[pyo3(get)]
    pub url: Option<String>,
}

#[pymethods]
impl HouseRequirement {
    fn __repr__(&self) -> String {
        format!(
            "HouseRequirement(number={:?}, active_record={:?})",
            self.number, self.active_record
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HouseRequirementsResponse {
    #[serde(rename = "houseRequirements")]
    pub house_requirements: Vec<HouseRequirement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HouseRequirementDetailResponse {
    #[serde(rename = "houseRequirement")]
    pub house_requirement: HouseRequirement,
}
