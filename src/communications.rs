use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

use crate::bills::Committee;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct CommunicationType {
    #[pyo3(get)]
    pub code: Option<String>,

    #[pyo3(get)]
    pub name: Option<String>,
}

#[pymethods]
impl CommunicationType {
    fn __repr__(&self) -> String {
        format!(
            "CommunicationType(code={:?}, name={:?})",
            self.code, self.name
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct MatchingRequirement {
    #[pyo3(get)]
    pub number: Option<i32>,

    #[pyo3(get)]
    pub url: Option<String>,
}

#[pymethods]
impl MatchingRequirement {
    fn __repr__(&self) -> String {
        format!(
            "MatchingRequirement(number={:?}, url={:?})",
            self.number, self.url
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct HouseCommunication {
    #[pyo3(get)]
    #[serde(rename = "abstract")]
    pub abstract_text: Option<String>,

    #[pyo3(get)]
    pub chamber: Option<String>,

    #[pyo3(get)]
    pub committees: Option<Vec<Committee>>,

    #[pyo3(get)]
    #[serde(rename = "communicationType")]
    pub communication_type: Option<CommunicationType>,

    #[pyo3(get)]
    pub congress: Option<i32>,

    #[pyo3(get)]
    #[serde(rename = "congressionalRecordDate")]
    pub congressional_record_date: Option<String>,

    #[pyo3(get)]
    #[serde(rename = "isRulemaking")]
    pub is_rulemaking: Option<String>,

    #[pyo3(get)]
    #[serde(rename = "legalAuthority")]
    pub legal_authority: Option<String>,

    #[pyo3(get)]
    #[serde(rename = "matchingRequirements")]
    pub matching_requirements: Option<Vec<MatchingRequirement>>,

    #[pyo3(get)]
    pub number: Option<i32>,

    #[pyo3(get)]
    #[serde(rename = "reportNature")]
    pub report_nature: Option<String>,

    #[pyo3(get)]
    #[serde(rename = "sessionNumber")]
    pub session_number: Option<i32>,

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

    #[pyo3(get)]
    #[serde(rename = "referralDate")]
    pub referral_date: Option<String>,
}

#[pymethods]
impl HouseCommunication {
    fn __repr__(&self) -> String {
        format!(
            "HouseCommunication(congress={:?}, number={:?}, type={:?})",
            self.congress,
            self.number,
            self.communication_type
                .as_ref()
                .and_then(|item| item.code.clone())
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct SenateCommunication {
    #[pyo3(get)]
    #[serde(rename = "abstract")]
    pub abstract_text: Option<String>,

    #[pyo3(get)]
    pub chamber: Option<String>,

    #[pyo3(get)]
    pub committees: Option<Vec<Committee>>,

    #[pyo3(get)]
    #[serde(rename = "communicationType")]
    pub communication_type: Option<CommunicationType>,

    #[pyo3(get)]
    pub congress: Option<i32>,

    #[pyo3(get)]
    #[serde(rename = "congressionalRecordDate")]
    pub congressional_record_date: Option<String>,

    #[pyo3(get)]
    pub number: Option<i32>,

    #[pyo3(get)]
    #[serde(rename = "sessionNumber")]
    pub session_number: Option<i32>,

    #[pyo3(get)]
    #[serde(rename = "updateDate")]
    pub update_date: Option<String>,

    #[pyo3(get)]
    pub url: Option<String>,

    #[pyo3(get)]
    #[serde(rename = "referralDate")]
    pub referral_date: Option<String>,
}

#[pymethods]
impl SenateCommunication {
    fn __repr__(&self) -> String {
        format!(
            "SenateCommunication(congress={:?}, number={:?}, type={:?})",
            self.congress,
            self.number,
            self.communication_type
                .as_ref()
                .and_then(|item| item.code.clone())
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HouseCommunicationsResponse {
    #[serde(rename = "houseCommunications")]
    pub house_communications: Vec<HouseCommunication>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HouseCommunicationDetailResponse {
    #[serde(rename = "houseCommunication")]
    pub house_communication: HouseCommunication,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SenateCommunicationsResponse {
    #[serde(rename = "senateCommunications")]
    pub senate_communications: Vec<SenateCommunication>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SenateCommunicationDetailResponse {
    #[serde(rename = "senateCommunication")]
    pub senate_communication: SenateCommunication,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchingCommunicationsResponse {
    #[serde(rename = "matchingCommunications")]
    pub matching_communications: Vec<HouseCommunication>,
}
