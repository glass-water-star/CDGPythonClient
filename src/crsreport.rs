use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

/// Represents a CRS report format
#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct CrsReportFormat {
    #[pyo3(get)]
    pub format: Option<String>,
    
    #[pyo3(get)]
    pub url: Option<String>,
}

#[pymethods]
impl CrsReportFormat {
    fn __repr__(&self) -> String {
        format!("CrsReportFormat(format={:?})", self.format)
    }
}

/// Represents an author of a CRS report
#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct CrsReportAuthor {
    #[pyo3(get)]
    pub author: Option<String>,
}

#[pymethods]
impl CrsReportAuthor {
    fn __repr__(&self) -> String {
        format!("CrsReportAuthor(author={:?})", self.author)
    }
}

/// Represents a topic of a CRS report
#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct CrsReportTopic {
    #[pyo3(get)]
    pub topic: Option<String>,
}

#[pymethods]
impl CrsReportTopic {
    fn __repr__(&self) -> String {
        format!("CrsReportTopic(topic={:?})", self.topic)
    }
}

/// Represents related material for a CRS report
#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct CrsReportRelatedMaterial {
    #[pyo3(get)]
    #[serde(rename = "URL")]
    pub url: Option<String>,
    
    #[pyo3(get)]
    pub congress: Option<i32>,
    
    #[pyo3(get)]
    pub number: Option<String>,
    
    #[pyo3(get)]
    pub title: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "type")]
    pub material_type: Option<String>,
}

#[pymethods]
impl CrsReportRelatedMaterial {
    fn __repr__(&self) -> String {
        format!(
            "CrsReportRelatedMaterial(number={:?}, type={:?})",
            self.number, self.material_type
        )
    }
}

/// Represents a CRS report in list responses
#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct CrsReport {
    #[pyo3(get)]
    #[serde(rename = "contentType")]
    pub content_type: Option<String>,
    
    #[pyo3(get)]
    pub id: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "publishDate")]
    pub publish_date: Option<String>,
    
    #[pyo3(get)]
    pub status: Option<String>,
    
    #[pyo3(get)]
    pub title: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "updateDate")]
    pub update_date: Option<String>,
    
    #[pyo3(get)]
    pub url: Option<String>,
    
    #[pyo3(get)]
    pub version: Option<i32>,
}

#[pymethods]
impl CrsReport {
    fn __repr__(&self) -> String {
        format!(
            "CrsReport(id={:?}, title={:?}, version={:?})",
            self.id, self.title, self.version
        )
    }
}

/// Represents detailed information about a CRS report
#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct CrsReportDetail {
    #[pyo3(get)]
    pub authors: Option<Vec<CrsReportAuthor>>,
    
    #[pyo3(get)]
    #[serde(rename = "contentType")]
    pub content_type: Option<String>,
    
    #[pyo3(get)]
    pub formats: Option<Vec<CrsReportFormat>>,
    
    #[pyo3(get)]
    pub id: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "publishDate")]
    pub publish_date: Option<String>,
    
    #[pyo3(get)]
    #[serde(rename = "relatedMaterials")]
    pub related_materials: Option<Vec<CrsReportRelatedMaterial>>,
    
    #[pyo3(get)]
    pub status: Option<String>,
    
    #[pyo3(get)]
    pub summary: Option<String>,
    
    #[pyo3(get)]
    pub title: Option<String>,
    
    #[pyo3(get)]
    pub topics: Option<Vec<CrsReportTopic>>,
    
    #[pyo3(get)]
    #[serde(rename = "updateDate")]
    pub update_date: Option<String>,
    
    #[pyo3(get)]
    pub url: Option<String>,
    
    #[pyo3(get)]
    pub version: Option<i32>,
}

#[pymethods]
impl CrsReportDetail {
    fn __repr__(&self) -> String {
        format!(
            "CrsReportDetail(id={:?}, title={:?}, version={:?})",
            self.id, self.title, self.version
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrsReportsResponse {
    #[serde(rename = "CRSReports")]
    pub crs_reports: Vec<CrsReport>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrsReportDetailResponse {
    #[serde(rename = "CRSReport")]
    pub report: CrsReportDetail,
}
