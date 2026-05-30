use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

use crate::bills::Committee;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct CommitteeMeetingLocation {
    #[pyo3(get)]
    pub building: Option<String>,

    #[pyo3(get)]
    pub room: Option<String>,
}

#[pymethods]
impl CommitteeMeetingLocation {
    fn __repr__(&self) -> String {
        format!(
            "CommitteeMeetingLocation(building={:?}, room={:?})",
            self.building, self.room
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct CommitteeMeetingVideo {
    #[pyo3(get)]
    pub name: Option<String>,

    #[pyo3(get)]
    pub url: Option<String>,
}

#[pymethods]
impl CommitteeMeetingVideo {
    fn __repr__(&self) -> String {
        format!("CommitteeMeetingVideo(name={:?})", self.name)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct CommitteeMeeting {
    #[pyo3(get)]
    pub chamber: Option<String>,

    #[pyo3(get)]
    pub committees: Option<Vec<Committee>>,

    #[pyo3(get)]
    pub congress: Option<i32>,

    #[pyo3(get)]
    pub date: Option<String>,

    #[pyo3(get)]
    #[serde(rename = "eventId")]
    pub event_id: Option<String>,

    #[pyo3(get)]
    pub location: Option<CommitteeMeetingLocation>,

    #[pyo3(get)]
    #[serde(rename = "meetingStatus")]
    pub meeting_status: Option<String>,

    #[pyo3(get)]
    pub title: Option<String>,

    #[pyo3(get)]
    #[serde(rename = "type")]
    pub meeting_type: Option<String>,

    #[pyo3(get)]
    #[serde(rename = "updateDate")]
    pub update_date: Option<String>,

    #[pyo3(get)]
    pub url: Option<String>,

    #[pyo3(get)]
    pub videos: Option<Vec<CommitteeMeetingVideo>>,
}

#[pymethods]
impl CommitteeMeeting {
    fn __repr__(&self) -> String {
        format!(
            "CommitteeMeeting(congress={:?}, event_id={:?}, title={:?})",
            self.congress, self.event_id, self.title
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitteeMeetingsResponse {
    #[serde(rename = "committeeMeetings")]
    pub committee_meetings: Vec<CommitteeMeeting>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitteeMeetingDetailResponse {
    #[serde(rename = "committeeMeeting")]
    pub committee_meeting: CommitteeMeeting,
}
