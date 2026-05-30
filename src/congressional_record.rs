use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

fn string_or_int<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::Error;
    use serde_json::Value;

    match Value::deserialize(deserializer)? {
        Value::Null => Ok(None),
        Value::String(value) => Ok(Some(value)),
        Value::Number(value) => Ok(Some(value.to_string())),
        other => Err(Error::custom(format!(
            "expected string or number, got {:?}",
            other
        ))),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct RecordTextLink {
    #[pyo3(get)]
    pub part: Option<String>,

    #[pyo3(get)]
    #[serde(rename = "type")]
    pub text_type: Option<String>,

    #[pyo3(get)]
    pub url: Option<String>,
}

#[pymethods]
impl RecordTextLink {
    fn __repr__(&self) -> String {
        format!(
            "RecordTextLink(type={:?}, part={:?})",
            self.text_type, self.part
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct RecordSection {
    #[pyo3(get)]
    #[serde(rename = "endPage")]
    #[serde(deserialize_with = "string_or_int")]
    pub end_page: Option<String>,

    #[pyo3(get)]
    pub name: Option<String>,

    #[pyo3(get)]
    #[serde(rename = "startPage")]
    #[serde(deserialize_with = "string_or_int")]
    pub start_page: Option<String>,

    #[pyo3(get)]
    pub text: Option<Vec<RecordTextLink>>,
}

#[pymethods]
impl RecordSection {
    fn __repr__(&self) -> String {
        format!(
            "RecordSection(name={:?}, start_page={:?}, end_page={:?})",
            self.name, self.start_page, self.end_page
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct RecordResource {
    #[pyo3(get)]
    pub count: Option<i32>,

    #[pyo3(get)]
    pub url: Option<String>,
}

#[pymethods]
impl RecordResource {
    fn __repr__(&self) -> String {
        format!("RecordResource(count={:?})", self.count)
    }
}

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
    #[serde(rename = "sessionNumber")]
    pub session_number: Option<i32>,

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
#[pyclass]
pub struct DailyCongressionalRecordArticle {
    #[pyo3(get)]
    #[serde(rename = "endPage")]
    #[serde(deserialize_with = "string_or_int")]
    pub end_page: Option<String>,

    #[pyo3(get)]
    #[serde(rename = "startPage")]
    #[serde(deserialize_with = "string_or_int")]
    pub start_page: Option<String>,

    #[pyo3(get)]
    pub text: Option<Vec<RecordTextLink>>,

    #[pyo3(get)]
    pub title: Option<String>,
}

#[pymethods]
impl DailyCongressionalRecordArticle {
    fn __repr__(&self) -> String {
        format!(
            "DailyCongressionalRecordArticle(title={:?}, start_page={:?})",
            self.title, self.start_page
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct DailyCongressionalRecordArticleGroup {
    #[pyo3(get)]
    pub name: Option<String>,

    #[pyo3(get)]
    #[serde(rename = "sectionArticles")]
    pub section_articles: Option<Vec<DailyCongressionalRecordArticle>>,
}

#[pymethods]
impl DailyCongressionalRecordArticleGroup {
    fn __repr__(&self) -> String {
        format!("DailyCongressionalRecordArticleGroup(name={:?})", self.name)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct DailyCongressionalRecordFullIssue {
    #[pyo3(get)]
    pub articles: Option<RecordResource>,

    #[pyo3(get)]
    #[serde(rename = "entireIssue")]
    pub entire_issue: Option<Vec<RecordTextLink>>,

    #[pyo3(get)]
    pub sections: Option<Vec<RecordSection>>,
}

#[pymethods]
impl DailyCongressionalRecordFullIssue {
    fn __repr__(&self) -> String {
        format!(
            "DailyCongressionalRecordFullIssue(articles={:?})",
            self.articles
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct DailyCongressionalRecordIssue {
    #[pyo3(get)]
    pub congress: Option<i32>,

    #[pyo3(get)]
    #[serde(rename = "fullIssue")]
    pub full_issue: Option<DailyCongressionalRecordFullIssue>,

    #[pyo3(get)]
    #[serde(rename = "issueDate")]
    pub issue_date: Option<String>,

    #[pyo3(get)]
    #[serde(rename = "issueNumber")]
    pub issue_number: Option<String>,

    #[pyo3(get)]
    #[serde(rename = "sessionNumber")]
    pub session_number: Option<i32>,

    #[pyo3(get)]
    #[serde(rename = "updateDate")]
    pub update_date: Option<String>,

    #[pyo3(get)]
    pub url: Option<String>,

    #[pyo3(get)]
    #[serde(rename = "volumeNumber")]
    pub volume_number: Option<i32>,
}

#[pymethods]
impl DailyCongressionalRecordIssue {
    fn __repr__(&self) -> String {
        format!(
            "DailyCongressionalRecordIssue(volume={:?}, issue={:?})",
            self.volume_number, self.issue_number
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct BoundCongressionalRecord {
    #[pyo3(get)]
    pub congress: Option<i32>,

    #[pyo3(get)]
    pub date: Option<String>,

    #[pyo3(get)]
    #[serde(rename = "dailyDigest")]
    pub daily_digest: Option<RecordSection>,

    #[pyo3(get)]
    pub sections: Option<Vec<RecordSection>>,

    #[pyo3(get)]
    #[serde(rename = "sessionNumber")]
    pub session_number: Option<i32>,

    #[pyo3(get)]
    #[serde(rename = "updateDate")]
    pub update_date: Option<String>,

    #[pyo3(get)]
    pub url: Option<String>,

    #[pyo3(get)]
    #[serde(rename = "volumeNumber")]
    pub volume_number: Option<i32>,
}

#[pymethods]
impl BoundCongressionalRecord {
    fn __repr__(&self) -> String {
        format!(
            "BoundCongressionalRecord(volume={:?}, date={:?})",
            self.volume_number, self.date
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct CongressionalRecordPdfItem {
    #[pyo3(get)]
    #[serde(rename = "Part")]
    pub part: Option<String>,

    #[pyo3(get)]
    #[serde(rename = "Url")]
    pub url: Option<String>,
}

#[pymethods]
impl CongressionalRecordPdfItem {
    fn __repr__(&self) -> String {
        format!("CongressionalRecordPdfItem(part={:?})", self.part)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct CongressionalRecordLink {
    #[pyo3(get)]
    #[serde(rename = "Label")]
    pub label: Option<String>,

    #[pyo3(get)]
    #[serde(rename = "Ordinal")]
    pub ordinal: Option<i32>,

    #[pyo3(get)]
    #[serde(rename = "PDF")]
    pub pdf: Option<Vec<CongressionalRecordPdfItem>>,
}

#[pymethods]
impl CongressionalRecordLink {
    fn __repr__(&self) -> String {
        format!(
            "CongressionalRecordLink(label={:?}, ordinal={:?})",
            self.label, self.ordinal
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct CongressionalRecordLinks {
    #[pyo3(get)]
    #[serde(rename = "Digest")]
    pub digest: Option<CongressionalRecordLink>,

    #[pyo3(get)]
    #[serde(rename = "FullRecord")]
    pub full_record: Option<CongressionalRecordLink>,

    #[pyo3(get)]
    #[serde(rename = "House")]
    pub house: Option<CongressionalRecordLink>,

    #[pyo3(get)]
    #[serde(rename = "Remarks")]
    pub remarks: Option<CongressionalRecordLink>,

    #[pyo3(get)]
    #[serde(rename = "Senate")]
    pub senate: Option<CongressionalRecordLink>,
}

#[pymethods]
impl CongressionalRecordLinks {
    fn __repr__(&self) -> String {
        format!("CongressionalRecordLinks(digest={:?})", self.digest)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct CongressionalRecordIssue {
    #[pyo3(get)]
    #[serde(rename = "Congress")]
    pub congress: Option<String>,

    #[pyo3(get)]
    #[serde(rename = "Id")]
    pub id: Option<i32>,

    #[pyo3(get)]
    #[serde(rename = "Issue")]
    pub issue: Option<String>,

    #[pyo3(get)]
    #[serde(rename = "Links")]
    pub links: Option<CongressionalRecordLinks>,

    #[pyo3(get)]
    #[serde(rename = "PublishDate")]
    pub publish_date: Option<String>,

    #[pyo3(get)]
    #[serde(rename = "Session")]
    pub session: Option<String>,

    #[pyo3(get)]
    #[serde(rename = "Volume")]
    pub volume: Option<String>,
}

#[pymethods]
impl CongressionalRecordIssue {
    fn __repr__(&self) -> String {
        format!(
            "CongressionalRecordIssue(congress={:?}, volume={:?}, issue={:?})",
            self.congress, self.volume, self.issue
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[pyclass]
pub struct CongressionalRecord {
    #[pyo3(get)]
    #[serde(rename = "IndexStart")]
    pub index_start: Option<i32>,

    #[pyo3(get)]
    #[serde(rename = "Issues")]
    pub issues: Option<Vec<CongressionalRecordIssue>>,

    #[pyo3(get)]
    #[serde(rename = "SetSize")]
    pub set_size: Option<i32>,

    #[pyo3(get)]
    #[serde(rename = "TotalCount")]
    pub total_count: Option<i32>,
}

#[pymethods]
impl CongressionalRecord {
    fn __repr__(&self) -> String {
        format!(
            "CongressionalRecord(index_start={:?}, total_count={:?})",
            self.index_start, self.total_count
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyCongressionalRecordsResponse {
    #[serde(rename = "dailyCongressionalRecord")]
    pub daily_congressional_record: Vec<DailyCongressionalRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyCongressionalRecordIssueResponse {
    pub issue: DailyCongressionalRecordIssue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyCongressionalRecordArticlesResponse {
    pub articles: Vec<DailyCongressionalRecordArticleGroup>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoundCongressionalRecordsResponse {
    #[serde(rename = "boundCongressionalRecord")]
    pub bound_congressional_record: Vec<BoundCongressionalRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CongressionalRecordResponse {
    #[serde(rename = "Results")]
    pub results: CongressionalRecord,
}
