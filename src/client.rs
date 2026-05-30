use chrono::{DateTime, Utc};
use pyo3::prelude::*;
use pyo3::types::{PyBool, PyDict, PyList};
use pyo3::{PyRef, PyRefMut};
use pyo3_async_runtimes::tokio::future_into_py;
use reqwest::header::{HeaderMap, RETRY_AFTER};
use reqwest::{Client, StatusCode, Url};
use serde::de::DeserializeOwned;
use serde_json::{self, Value};
use std::collections::HashMap;
use std::future::Future;
use std::sync::OnceLock;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use thiserror::Error;

use crate::bills::{
    Action, ActionsResponse, Amendment, AmendmentDetail, AmendmentDetailResponse,
    AmendmentsResponse, Bill, BillDetail, BillDetailResponse, BillTitle, BillsResponse, Committee,
    CommitteesResponse, Cosponsor, CosponsorsResponse, RelatedBill, RelatedBillsResponse, Subject,
    SubjectsResponse, SummariesResponse, Summary, TextVersion, TextVersionsResponse,
    TitlesResponse,
};
use crate::committee_meetings::{
    CommitteeMeeting, CommitteeMeetingDetailResponse, CommitteeMeetingsResponse,
};
use crate::committees::{
    CommitteeBill, CommitteeBillsResponse, CommitteeDetailInfo, CommitteeDetailResponse,
    CommitteeItem, CommitteePrintDetail, CommitteePrintDetailResponse, CommitteePrintItem,
    CommitteePrintText, CommitteePrintTextResponse, CommitteePrintsResponse, CommitteeReportDetail,
    CommitteeReportDetailResponse, CommitteeReportItem, CommitteeReportText,
    CommitteeReportTextResponse, CommitteeReportsResponse,
    CommitteesResponse as CommitteesListResponse,
};
use crate::communications::{
    HouseCommunication, HouseCommunicationDetailResponse, HouseCommunicationsResponse,
    MatchingCommunicationsResponse, SenateCommunication, SenateCommunicationDetailResponse,
    SenateCommunicationsResponse,
};
use crate::congressional_record::{
    BoundCongressionalRecord, BoundCongressionalRecordsResponse, CongressionalRecord,
    CongressionalRecordResponse, DailyCongressionalRecord, DailyCongressionalRecordArticleGroup,
    DailyCongressionalRecordArticlesResponse, DailyCongressionalRecordIssue,
    DailyCongressionalRecordIssueResponse, DailyCongressionalRecordsResponse,
};
use crate::crsreport::{CrsReport, CrsReportDetail, CrsReportDetailResponse, CrsReportsResponse};
use crate::hearings::{Hearing, HearingDetailResponse, HearingsResponse};
use crate::house_votes::{
    HouseVote, HouseVoteDetail, HouseVoteDetailResponse, HouseVoteMembers,
    HouseVoteMembersResponse, HouseVotesResponse,
};
use crate::laws::{LawDetail, LawDetailResponse, LawItem, LawsResponse};
use crate::members::{
    CosponsoredLegislationResponse, MemberResponse, MembersResponse, Sponsor,
    SponsoredLegislationResponse,
};
use crate::nominations::{
    Nomination, NominationDetailResponse, NominationsResponse, Nominee, NomineesResponse,
};
use crate::nominations::{
    NominationCommittee, NominationCommitteesResponse, NominationHearing,
    NominationHearingsResponse,
};
use crate::requirements::{
    HouseRequirement, HouseRequirementDetailResponse, HouseRequirementsResponse,
};
use crate::sessions::{Congress, CongressResponse, CongressesResponse};
use crate::summaries::{SummariesListResponse, SummaryItem};
use crate::treaties::{
    TreatiesResponse, Treaty, TreatyCommitteesResponse, TreatyDetailResponse,
    TreatyPartDetailResponse,
};

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("HTTP request failed: {0}")]
    RequestFailed(#[from] reqwest::Error),

    #[error("HTTP {status_code}: {message}")]
    HttpError {
        status_code: u16,
        url: Option<String>,
        message: String,
    },

    #[error("Failed to deserialize response from {context}: {message}\nResponse preview: {response_preview}")]
    DeserializationError {
        context: String,
        message: String,
        response_preview: String,
    },

    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    #[error("Client error: {0}")]
    ApiError(String),

    #[allow(dead_code)]
    #[error("Missing API key")]
    MissingApiKey,
}

pub type ApiResult<T> = Result<T, ApiError>;

pub struct CongressApiClient {
    client: Client,
    api_key: String,
    base_url: String,
    retry_config: RetryConfig,
    timeout_seconds: Option<f64>,
    user_agent: Option<String>,
    log_handler: Option<Py<PyAny>>,
}

#[derive(Clone, Copy, Debug)]
struct RetryConfig {
    max_attempts: u32,
    base_delay_ms: u64,
}

const MAX_RETRY_DELAY_MS: u64 = 30_000;
static SHARED_RUNTIME: OnceLock<Result<tokio::runtime::Runtime, String>> = OnceLock::new();

fn insert_optional_param<T: ToString>(
    params: &mut HashMap<String, String>,
    key: &str,
    value: Option<T>,
) {
    if let Some(value) = value {
        params.insert(key.to_string(), value.to_string());
    }
}

fn build_format_params(format: Option<String>) -> HashMap<String, String> {
    let mut params = HashMap::new();
    insert_optional_param(&mut params, "format", format);
    params
}

fn build_offset_limit_format_params(
    format: Option<String>,
    offset: Option<i32>,
    limit: Option<i32>,
) -> HashMap<String, String> {
    let mut params = HashMap::new();
    insert_optional_param(&mut params, "format", format);
    insert_optional_param(&mut params, "offset", offset);
    insert_optional_param(&mut params, "limit", limit);
    params
}

fn build_date_range_params(
    format: Option<String>,
    offset: Option<i32>,
    limit: Option<i32>,
    from_date_time: Option<String>,
    to_date_time: Option<String>,
) -> HashMap<String, String> {
    let mut params = build_offset_limit_format_params(format, offset, limit);
    insert_optional_param(&mut params, "fromDateTime", from_date_time);
    insert_optional_param(&mut params, "toDateTime", to_date_time);
    params
}

fn build_sort_date_range_params(
    offset: Option<i32>,
    limit: Option<i32>,
    from_date_time: Option<String>,
    to_date_time: Option<String>,
    sort: Option<String>,
    format: Option<String>,
) -> HashMap<String, String> {
    let mut params = build_offset_limit_format_params(format, offset, limit);
    insert_optional_param(&mut params, "fromDateTime", from_date_time);
    insert_optional_param(&mut params, "toDateTime", to_date_time);
    insert_optional_param(&mut params, "sort", sort);
    params
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            base_delay_ms: 1000,
        }
    }
}

impl Clone for CongressApiClient {
    fn clone(&self) -> Self {
        let log_handler = Python::with_gil(|py| {
            self.log_handler
                .as_ref()
                .map(|handler| handler.clone_ref(py))
        });

        Self {
            client: self.client.clone(),
            api_key: self.api_key.clone(),
            base_url: self.base_url.clone(),
            retry_config: self.retry_config,
            timeout_seconds: self.timeout_seconds,
            user_agent: self.user_agent.clone(),
            log_handler,
        }
    }
}

impl CongressApiClient {
    fn new(api_key: String, retry_config: RetryConfig) -> ApiResult<Self> {
        Ok(Self {
            client: Self::build_http_client(None, None)?,
            api_key,
            base_url: "https://api.congress.gov/v3".to_string(),
            retry_config,
            timeout_seconds: None,
            user_agent: None,
            log_handler: None,
        })
    }

    fn build_http_client(
        timeout_seconds: Option<f64>,
        user_agent: Option<&str>,
    ) -> ApiResult<Client> {
        let mut builder = Client::builder();

        if let Some(timeout_seconds) = timeout_seconds {
            builder = builder.timeout(Duration::from_secs_f64(timeout_seconds));
        }

        if let Some(user_agent) = user_agent {
            builder = builder.user_agent(user_agent);
        }

        builder.build().map_err(|error| {
            ApiError::ConfigurationError(format!("Failed to build HTTP client: {}", error))
        })
    }

    fn should_retry_status(status: StatusCode) -> bool {
        status == StatusCode::TOO_MANY_REQUESTS
            || matches!(
                status,
                StatusCode::INTERNAL_SERVER_ERROR
                    | StatusCode::BAD_GATEWAY
                    | StatusCode::SERVICE_UNAVAILABLE
                    | StatusCode::GATEWAY_TIMEOUT
            )
    }

    fn retry_after_delay(headers: &HeaderMap) -> Option<Duration> {
        let value = headers.get(RETRY_AFTER)?.to_str().ok()?.trim();
        if let Ok(seconds) = value.parse::<u64>() {
            return Some(Duration::from_secs(seconds));
        }

        let retry_at = DateTime::parse_from_rfc2822(value)
            .ok()?
            .with_timezone(&Utc);
        let delay = retry_at.signed_duration_since(Utc::now());

        if delay <= chrono::Duration::zero() {
            return Some(Duration::from_secs(0));
        }

        delay.to_std().ok()
    }

    fn jitter_ms(max_jitter_ms: u64) -> u64 {
        if max_jitter_ms == 0 {
            return 0;
        }

        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|duration| duration.subsec_nanos() as u64)
            .unwrap_or(0);

        nanos % (max_jitter_ms + 1)
    }

    fn retry_delay(&self, attempt: u32, retry_after: Option<Duration>) -> Duration {
        if let Some(retry_after) = retry_after {
            return retry_after;
        }

        if self.retry_config.base_delay_ms == 0 {
            return Duration::from_millis(0);
        }

        let exponent = attempt.saturating_sub(1).min(10);
        let exponential_delay_ms = self
            .retry_config
            .base_delay_ms
            .saturating_mul(1_u64 << exponent)
            .min(MAX_RETRY_DELAY_MS);
        let jitter_ms = Self::jitter_ms(exponential_delay_ms / 4);

        Duration::from_millis(exponential_delay_ms.saturating_add(jitter_ms))
    }

    fn rebuild_http_client(&mut self) -> ApiResult<()> {
        self.client = Self::build_http_client(self.timeout_seconds, self.user_agent.as_deref())?;
        Ok(())
    }

    fn set_timeout_seconds(&mut self, timeout_seconds: Option<f64>) -> ApiResult<()> {
        if let Some(timeout_seconds) = timeout_seconds {
            if !timeout_seconds.is_finite() || timeout_seconds <= 0.0 {
                return Err(ApiError::ConfigurationError(
                    "timeout_seconds must be a positive finite number".to_string(),
                ));
            }
        }

        self.timeout_seconds = timeout_seconds;
        self.rebuild_http_client()
    }

    fn timeout_seconds(&self) -> Option<f64> {
        self.timeout_seconds
    }

    fn set_user_agent(&mut self, user_agent: Option<String>) -> ApiResult<()> {
        if let Some(user_agent) = user_agent {
            if user_agent.trim().is_empty() {
                return Err(ApiError::ConfigurationError(
                    "user_agent cannot be empty".to_string(),
                ));
            }
            self.user_agent = Some(user_agent);
        } else {
            self.user_agent = None;
        }

        self.rebuild_http_client()
    }

    fn user_agent(&self) -> Option<String> {
        self.user_agent.clone()
    }

    pub fn get<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        params: Option<HashMap<String, String>>,
    ) -> ApiResult<T> {
        self.block_on(self.get_async(endpoint, params))
    }

    async fn get_async<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        params: Option<HashMap<String, String>>,
    ) -> ApiResult<T> {
        let mut attempt = 0;
        let request_url = self.apply_query_params(self.build_url(endpoint)?, params.clone());
        let log_url = self.redacted_url_string(&request_url);

        loop {
            attempt += 1;
            self.emit_log_event(
                "request_start",
                &log_url,
                attempt,
                None,
                None,
                None,
                params.as_ref(),
            );
            let started_at = Instant::now();

            let response = match self.client.get(request_url.clone()).send().await {
                Ok(resp) => resp,
                Err(e) => {
                    let elapsed_ms = started_at.elapsed().as_millis();
                    self.emit_log_event(
                        "request_transport_error",
                        &log_url,
                        attempt,
                        None,
                        Some(elapsed_ms),
                        Some(&e.to_string()),
                        params.as_ref(),
                    );

                    if attempt < self.retry_config.max_attempts {
                        let delay = self.retry_delay(attempt, None);
                        self.emit_log_event(
                            "request_retry",
                            &log_url,
                            attempt,
                            None,
                            Some(elapsed_ms),
                            Some(&e.to_string()),
                            params.as_ref(),
                        );
                        tokio::time::sleep(delay).await;
                        continue;
                    }

                    return Err(ApiError::RequestFailed(e));
                }
            };

            let status = response.status();
            let elapsed_ms = started_at.elapsed().as_millis();

            if Self::should_retry_status(status) && attempt < self.retry_config.max_attempts {
                let retry_after = Self::retry_after_delay(response.headers());
                let delay = self.retry_delay(attempt, retry_after);
                self.emit_log_event(
                    "request_retry",
                    &log_url,
                    attempt,
                    Some(status.as_u16()),
                    Some(elapsed_ms),
                    None,
                    params.as_ref(),
                );
                tokio::time::sleep(delay).await;
                continue;
            }

            if !status.is_success() {
                self.emit_log_event(
                    "request_http_error",
                    &log_url,
                    attempt,
                    Some(status.as_u16()),
                    Some(elapsed_ms),
                    Some(&format!("API returned status: {}", status)),
                    params.as_ref(),
                );
                return Err(ApiError::HttpError {
                    status_code: status.as_u16(),
                    url: Some(log_url.clone()),
                    message: format!("API returned status: {}", status),
                });
            }

            // Get response as text first for better error messages
            let response_text = response.text().await?;

            // Try to deserialize
            return serde_json::from_str(&response_text)
                .map_err(|e| {
                    self.emit_log_event(
                        "request_decode_error",
                        &log_url,
                        attempt,
                        Some(status.as_u16()),
                        Some(elapsed_ms),
                        Some(&e.to_string()),
                        params.as_ref(),
                    );
                    // Truncate response preview to avoid overwhelming error messages
                    let preview = if response_text.len() > 500 {
                        format!("{}... (truncated)", &response_text[..500])
                    } else {
                        response_text.clone()
                    };

                    ApiError::DeserializationError {
                        context: format!("endpoint '{}'", endpoint),
                        message: e.to_string(),
                        response_preview: preview,
                    }
                })
                .map(|value| {
                    self.emit_log_event(
                        "request_success",
                        &log_url,
                        attempt,
                        Some(status.as_u16()),
                        Some(elapsed_ms),
                        None,
                        params.as_ref(),
                    );
                    value
                });
        }
    }

    fn get_value_from_absolute_or_relative_url(
        &self,
        path_or_url: &str,
        params: Option<HashMap<String, String>>,
    ) -> ApiResult<Value> {
        self.block_on(self.get_value_from_absolute_or_relative_url_async(path_or_url, params))
    }

    async fn get_value_from_absolute_or_relative_url_async(
        &self,
        path_or_url: &str,
        params: Option<HashMap<String, String>>,
    ) -> ApiResult<Value> {
        let url = if path_or_url.starts_with("http://") || path_or_url.starts_with("https://") {
            self.build_absolute_url(path_or_url)?
        } else {
            self.build_url(path_or_url)?
        };

        self.get_value_from_url_async(url, params).await
    }

    fn build_url(&self, endpoint: &str) -> ApiResult<Url> {
        let normalized_endpoint = if endpoint.starts_with('/') {
            endpoint.to_string()
        } else {
            format!("/{}", endpoint)
        };
        let url = format!("{}{}", self.base_url, normalized_endpoint);

        Url::parse(&url)
            .map_err(|e| ApiError::InvalidUrl(format!("API endpoint '{}': {}", endpoint, e)))
    }

    fn build_absolute_url(&self, absolute_url: &str) -> ApiResult<Url> {
        let url = Url::parse(absolute_url)
            .map_err(|e| ApiError::InvalidUrl(format!("API URL '{}': {}", absolute_url, e)))?;

        if url.scheme() != "https" || url.host_str() != Some("api.congress.gov") {
            return Err(ApiError::InvalidUrl(format!(
                "Refusing to follow non-Congress.gov API URL: {}",
                absolute_url
            )));
        }

        Ok(url)
    }

    async fn get_value_from_url_async(
        &self,
        url: Url,
        params: Option<HashMap<String, String>>,
    ) -> ApiResult<Value> {
        let mut attempt = 0;
        let request_url = self.apply_query_params(url, params);
        let log_url = self.redacted_url_string(&request_url);

        loop {
            attempt += 1;
            self.emit_log_event("request_start", &log_url, attempt, None, None, None, None);
            let started_at = Instant::now();

            let response = match self.client.get(request_url.clone()).send().await {
                Ok(response) => response,
                Err(error) => {
                    self.emit_log_event(
                        "request_transport_error",
                        &log_url,
                        attempt,
                        None,
                        Some(started_at.elapsed().as_millis()),
                        Some(&error.to_string()),
                        None,
                    );
                    return Err(ApiError::RequestFailed(error));
                }
            };
            let status = response.status();
            let elapsed_ms = started_at.elapsed().as_millis();

            if status == reqwest::StatusCode::SERVICE_UNAVAILABLE
                && attempt < self.retry_config.max_attempts
            {
                self.emit_log_event(
                    "request_retry",
                    &log_url,
                    attempt,
                    Some(status.as_u16()),
                    Some(elapsed_ms),
                    None,
                    None,
                );
                let delay = Duration::from_millis(
                    self.retry_config
                        .base_delay_ms
                        .saturating_mul(attempt as u64),
                );
                tokio::time::sleep(delay).await;
                continue;
            }

            if !status.is_success() {
                self.emit_log_event(
                    "request_http_error",
                    &log_url,
                    attempt,
                    Some(status.as_u16()),
                    Some(elapsed_ms),
                    Some(&format!("API returned status: {}", status)),
                    None,
                );
                return Err(ApiError::HttpError {
                    status_code: status.as_u16(),
                    url: Some(log_url.clone()),
                    message: format!("API returned status: {}", status),
                });
            }

            let response_text = response.text().await?;
            return serde_json::from_str(&response_text)
                .map_err(|e| {
                    self.emit_log_event(
                        "request_decode_error",
                        &log_url,
                        attempt,
                        Some(status.as_u16()),
                        Some(elapsed_ms),
                        Some(&e.to_string()),
                        None,
                    );
                    let preview = if response_text.len() > 500 {
                        format!("{}... (truncated)", &response_text[..500])
                    } else {
                        response_text.clone()
                    };

                    ApiError::DeserializationError {
                        context: format!("URL '{}'", request_url),
                        message: e.to_string(),
                        response_preview: preview,
                    }
                })
                .map(|value| {
                    self.emit_log_event(
                        "request_success",
                        &log_url,
                        attempt,
                        Some(status.as_u16()),
                        Some(elapsed_ms),
                        None,
                        None,
                    );
                    value
                });
        }
    }

    fn apply_query_params(&self, mut url: Url, params: Option<HashMap<String, String>>) -> Url {
        let mut merged_pairs: HashMap<String, String> = url.query_pairs().into_owned().collect();
        merged_pairs.insert("api_key".to_string(), self.api_key.clone());

        if let Some(params) = params {
            for (key, value) in params {
                merged_pairs.insert(key, value);
            }
        }

        {
            let mut query_pairs = url.query_pairs_mut();
            query_pairs.clear();
            for (key, value) in merged_pairs {
                query_pairs.append_pair(&key, &value);
            }
        }

        url
    }

    fn set_log_handler(&mut self, handler: Option<Py<PyAny>>) {
        self.log_handler = handler;
    }

    fn clear_log_handler(&mut self) {
        self.log_handler = None;
    }

    fn logging_enabled(&self) -> bool {
        self.log_handler.is_some()
    }

    fn redacted_url_string(&self, url: &Url) -> String {
        let mut redacted = url.clone();
        let pairs: Vec<(String, String)> = redacted
            .query_pairs()
            .into_owned()
            .filter(|(key, _)| key != "api_key")
            .collect();

        {
            let mut query_pairs = redacted.query_pairs_mut();
            query_pairs.clear();
            for (key, value) in pairs {
                query_pairs.append_pair(&key, &value);
            }
        }

        redacted.to_string()
    }

    fn emit_log_event(
        &self,
        event: &str,
        url: &str,
        attempt: u32,
        status_code: Option<u16>,
        elapsed_ms: Option<u128>,
        error: Option<&str>,
        params: Option<&HashMap<String, String>>,
    ) {
        let Some(handler) = &self.log_handler else {
            return;
        };

        Python::with_gil(|py| {
            let payload = PyDict::new(py);
            let _ = payload.set_item("event", event);
            let _ = payload.set_item("method", "GET");
            let _ = payload.set_item("url", url);
            let _ = payload.set_item("attempt", attempt);

            if let Ok(parsed) = Url::parse(url) {
                let _ = payload.set_item("path", parsed.path());
            }

            if let Some(status_code) = status_code {
                let _ = payload.set_item("status_code", status_code);
            }

            if let Some(elapsed_ms) = elapsed_ms {
                let _ = payload.set_item("elapsed_ms", elapsed_ms);
            }

            if let Some(error) = error {
                let _ = payload.set_item("error", error);
            }

            if let Some(params) = params {
                if let Some(offset) = params.get("offset") {
                    let _ = payload.set_item("offset", offset);
                }
                if let Some(limit) = params.get("limit") {
                    let _ = payload.set_item("limit", limit);
                }
            }

            if let Err(error) = handler.call1(py, (payload,)) {
                error.print(py);
            }
        });
    }

    fn shared_runtime() -> ApiResult<&'static tokio::runtime::Runtime> {
        match SHARED_RUNTIME.get_or_init(|| {
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .map_err(|e| format!("Failed to create async runtime: {}", e))
        }) {
            Ok(runtime) => Ok(runtime),
            Err(message) => Err(ApiError::ApiError(message.clone())),
        }
    }

    fn block_on<F, T>(&self, future: F) -> ApiResult<T>
    where
        F: Future<Output = ApiResult<T>>,
    {
        Self::shared_runtime()?.block_on(future)
    }
}

#[pyclass]
pub struct ApiPage {
    #[pyo3(get)]
    pub items: Py<PyAny>,

    #[pyo3(get)]
    pub raw_response: Py<PyAny>,

    #[pyo3(get)]
    pub item_key: Option<String>,

    #[pyo3(get)]
    pub count: Option<i32>,

    #[pyo3(get)]
    pub next_url: Option<String>,

    #[pyo3(get)]
    pub previous_url: Option<String>,

    #[pyo3(get)]
    pub offset: Option<i32>,

    #[pyo3(get)]
    pub limit: Option<i32>,
}

#[pymethods]
impl ApiPage {
    pub fn has_next(&self) -> bool {
        self.next_url.is_some()
    }

    fn __repr__(&self) -> String {
        format!(
            "ApiPage(item_key={:?}, count={:?}, next_url={:?})",
            self.item_key, self.count, self.next_url
        )
    }
}

// PyO3 wrapper class for Congress.gov API
#[pyclass]
pub struct CDGPythonClient {
    client: CongressApiClient,
}

#[pyclass]
pub struct AsyncClientCore {
    client: CongressApiClient,
}

#[pymethods]
impl CDGPythonClient {
    #[pyo3(signature = (api_key, timeout_seconds=None, user_agent=None))]
    #[new]
    pub fn new(
        api_key: String,
        timeout_seconds: Option<f64>,
        user_agent: Option<String>,
    ) -> PyResult<Self> {
        if api_key.trim().is_empty() {
            return Err(api_py_err(ApiError::MissingApiKey));
        }

        let mut client =
            CongressApiClient::new(api_key, RetryConfig::default()).map_err(api_py_err)?;
        client
            .set_timeout_seconds(timeout_seconds)
            .map_err(api_py_err)?;
        client.set_user_agent(user_agent).map_err(api_py_err)?;

        Ok(Self { client })
    }

    fn retry_attempts(&self) -> i32 {
        self.client.retry_config.max_attempts as i32
    }

    fn retry_base_delay_ms(&self) -> i32 {
        self.client.retry_config.base_delay_ms as i32
    }

    fn set_retry_config(&mut self, retry_attempts: i32, retry_base_delay_ms: i32) -> PyResult<()> {
        if retry_attempts < 1 {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "retry_attempts must be at least 1",
            ));
        }

        if retry_base_delay_ms < 0 {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "retry_base_delay_ms cannot be negative",
            ));
        }

        self.client.retry_config.max_attempts = retry_attempts as u32;
        self.client.retry_config.base_delay_ms = retry_base_delay_ms as u64;

        Ok(())
    }

    #[pyo3(signature = (timeout_seconds=None))]
    fn configure_timeout(&mut self, timeout_seconds: Option<f64>) -> PyResult<()> {
        self.client
            .set_timeout_seconds(timeout_seconds)
            .map_err(api_py_err)
    }

    fn get_timeout(&self) -> Option<f64> {
        self.client.timeout_seconds()
    }

    #[pyo3(signature = (user_agent=None))]
    fn configure_user_agent(&mut self, user_agent: Option<String>) -> PyResult<()> {
        self.client.set_user_agent(user_agent).map_err(api_py_err)
    }

    fn get_user_agent(&self) -> Option<String> {
        self.client.user_agent()
    }

    #[pyo3(signature = (handler=None))]
    fn _set_log_handler(&mut self, handler: Option<Py<PyAny>>) {
        self.client.set_log_handler(handler);
    }

    fn _clear_log_handler(&mut self) {
        self.client.clear_log_handler();
    }

    fn _logging_enabled(&self) -> bool {
        self.client.logging_enabled()
    }

    #[pyo3(signature = (path_or_url, offset=None, limit=None))]
    pub fn fetch_page(
        &self,
        py: Python<'_>,
        path_or_url: String,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> PyResult<ApiPage> {
        let params = request_params(offset, limit);
        let response = self
            .client
            .get_value_from_absolute_or_relative_url(&path_or_url, params)
            .map_err(api_py_err)?;

        build_api_page(py, response, offset, limit)
    }

    // ========== Bill Endpoints ==========

    /// Get a list of bills sorted by date of latest action
    #[pyo3(signature = (format=None, offset=None, limit=None, from_date_time=None, to_date_time=None))]
    pub fn list_bills(
        &self,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
        from_date_time: Option<String>,
        to_date_time: Option<String>,
    ) -> PyResult<Vec<Bill>> {
        let params = build_date_range_params(format, offset, limit, from_date_time, to_date_time);

        let response: BillsResponse = self.client.get("/bill", Some(params)).map_err(api_py_err)?;

        Ok(response.bills)
    }

    /// Get bills filtered by congress
    #[pyo3(signature = (congress, format=None, offset=None, limit=None, from_date_time=None, to_date_time=None))]
    pub fn list_bills_by_congress(
        &self,
        congress: i32,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
        from_date_time: Option<String>,
        to_date_time: Option<String>,
    ) -> PyResult<Vec<Bill>> {
        let params = build_date_range_params(format, offset, limit, from_date_time, to_date_time);

        let endpoint = format!("/bill/{}", congress);
        let response: BillsResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.bills)
    }

    /// Get bills filtered by congress and bill type
    #[pyo3(signature = (congress, bill_type, format=None, offset=None, limit=None, from_date_time=None, to_date_time=None))]
    pub fn list_bills_by_type(
        &self,
        congress: i32,
        bill_type: String,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
        from_date_time: Option<String>,
        to_date_time: Option<String>,
    ) -> PyResult<Vec<Bill>> {
        let params = build_date_range_params(format, offset, limit, from_date_time, to_date_time);

        let endpoint = format!("/bill/{}/{}", congress, bill_type);
        let response: BillsResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.bills)
    }

    #[pyo3(signature = (congress=None, bill_type=None, format=None, offset=None, limit=None, from_date_time=None, to_date_time=None))]
    pub fn get_bills(
        &self,
        congress: Option<i32>,
        bill_type: Option<String>,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
        from_date_time: Option<String>,
        to_date_time: Option<String>,
    ) -> PyResult<Vec<Bill>> {
        let params = build_date_range_params(format, offset, limit, from_date_time, to_date_time);

        let endpoint = match (congress, bill_type) {
            (Some(c), Some(bt)) => format!("/bill/{}/{}", c, bt),
            (Some(c), None) => format!("/bill/{}", c),
            (None, _) => "/bill".to_string(),
        };

        let response: BillsResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.bills)
    }

    #[pyo3(signature = (congress, bill_type, bill_number, format=None, offset=None, limit=None, from_date_time=None, to_date_time=None))]
    pub fn get_bill_detail(
        &self,
        congress: i32,
        bill_type: String,
        bill_number: String,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
        from_date_time: Option<String>,
        to_date_time: Option<String>,
    ) -> PyResult<BillDetail> {
        let params = build_date_range_params(format, offset, limit, from_date_time, to_date_time);

        let endpoint = format!("/bill/{}/{}/{}", congress, bill_type, bill_number);

        let response: BillDetailResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.bill)
    }

    /// Get the list of actions on a specified bill
    #[pyo3(signature = (congress, bill_type, bill_number, format=None, offset=None, limit=None))]
    pub fn get_bill_actions(
        &self,
        congress: i32,
        bill_type: String,
        bill_number: String,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> PyResult<Vec<Action>> {
        let params = build_offset_limit_format_params(format, offset, limit);

        let endpoint = format!("/bill/{}/{}/{}/actions", congress, bill_type, bill_number);
        let response: ActionsResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.actions)
    }

    /// Get the list of amendments to a specified bill
    #[pyo3(signature = (congress, bill_type, bill_number, format=None, offset=None, limit=None))]
    pub fn get_bill_amendments(
        &self,
        congress: i32,
        bill_type: String,
        bill_number: String,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> PyResult<Vec<Amendment>> {
        let params = build_offset_limit_format_params(format, offset, limit);

        let endpoint = format!(
            "/bill/{}/{}/{}/amendments",
            congress, bill_type, bill_number
        );
        let response: AmendmentsResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.amendments)
    }

    /// Get the list of committees associated with a specified bill
    #[pyo3(signature = (congress, bill_type, bill_number, format=None, offset=None, limit=None))]
    pub fn get_bill_committees(
        &self,
        congress: i32,
        bill_type: String,
        bill_number: String,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> PyResult<Vec<Committee>> {
        let params = build_offset_limit_format_params(format, offset, limit);

        let endpoint = format!(
            "/bill/{}/{}/{}/committees",
            congress, bill_type, bill_number
        );
        let response: CommitteesResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.committees)
    }

    /// Get the list of cosponsors on a specified bill
    #[pyo3(signature = (congress, bill_type, bill_number, format=None, offset=None, limit=None))]
    pub fn get_bill_cosponsors(
        &self,
        congress: i32,
        bill_type: String,
        bill_number: String,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> PyResult<Vec<Cosponsor>> {
        let params = build_offset_limit_format_params(format, offset, limit);

        let endpoint = format!(
            "/bill/{}/{}/{}/cosponsors",
            congress, bill_type, bill_number
        );
        let response: CosponsorsResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.cosponsors)
    }

    /// Get the list of related bills to a specified bill
    #[pyo3(signature = (congress, bill_type, bill_number, format=None, offset=None, limit=None))]
    pub fn get_related_bills(
        &self,
        congress: i32,
        bill_type: String,
        bill_number: String,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> PyResult<Vec<RelatedBill>> {
        let params = build_offset_limit_format_params(format, offset, limit);

        let endpoint = format!(
            "/bill/{}/{}/{}/relatedbills",
            congress, bill_type, bill_number
        );
        let response: RelatedBillsResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.related_bills.unwrap_or_default())
    }

    /// Get the list of legislative subjects on a specified bill
    #[pyo3(signature = (congress, bill_type, bill_number, format=None, offset=None, limit=None))]
    pub fn get_bill_subjects(
        &self,
        congress: i32,
        bill_type: String,
        bill_number: String,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> PyResult<Vec<Subject>> {
        let params = build_offset_limit_format_params(format, offset, limit);

        let endpoint = format!("/bill/{}/{}/{}/subjects", congress, bill_type, bill_number);
        let response: SubjectsResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.legislative_subjects.unwrap_or_default())
    }

    /// Get the list of summaries for a specified bill
    #[pyo3(signature = (congress, bill_type, bill_number, format=None, offset=None, limit=None))]
    pub fn get_bill_summaries(
        &self,
        congress: i32,
        bill_type: String,
        bill_number: String,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> PyResult<Vec<Summary>> {
        let params = build_offset_limit_format_params(format, offset, limit);

        let endpoint = format!("/bill/{}/{}/{}/summaries", congress, bill_type, bill_number);
        let response: SummariesResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.summaries)
    }

    /// Get the list of text versions for a specified bill
    #[pyo3(signature = (congress, bill_type, bill_number, format=None, offset=None, limit=None))]
    pub fn get_bill_text(
        &self,
        congress: i32,
        bill_type: String,
        bill_number: String,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> PyResult<Vec<TextVersion>> {
        let params = build_offset_limit_format_params(format, offset, limit);

        let endpoint = format!("/bill/{}/{}/{}/text", congress, bill_type, bill_number);
        let response: TextVersionsResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.text_versions)
    }

    /// Get the list of titles for a specified bill
    #[pyo3(signature = (congress, bill_type, bill_number, format=None, offset=None, limit=None))]
    pub fn get_bill_titles(
        &self,
        congress: i32,
        bill_type: String,
        bill_number: String,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> PyResult<Vec<BillTitle>> {
        let params = build_offset_limit_format_params(format, offset, limit);

        let endpoint = format!("/bill/{}/{}/{}/titles", congress, bill_type, bill_number);
        let response: TitlesResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.titles)
    }

    // ========== Amendment Endpoints ==========

    /// Get a list of amendments sorted by date of latest action
    #[pyo3(signature = (format=None, offset=None, limit=None, from_date_time=None, to_date_time=None))]
    pub fn list_amendments(
        &self,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
        from_date_time: Option<String>,
        to_date_time: Option<String>,
    ) -> PyResult<Vec<Amendment>> {
        let mut params = HashMap::new();

        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        if let Some(from) = from_date_time {
            params.insert("fromDateTime".to_string(), from);
        }
        if let Some(to) = to_date_time {
            params.insert("toDateTime".to_string(), to);
        }

        let response: AmendmentsResponse = self
            .client
            .get("/amendment", Some(params))
            .map_err(api_py_err)?;

        Ok(response.amendments)
    }

    /// Get amendments filtered by congress
    #[pyo3(signature = (congress, format=None, offset=None, limit=None, from_date_time=None, to_date_time=None))]
    pub fn list_amendments_by_congress(
        &self,
        congress: i32,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
        from_date_time: Option<String>,
        to_date_time: Option<String>,
    ) -> PyResult<Vec<Amendment>> {
        let mut params = HashMap::new();

        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        if let Some(from) = from_date_time {
            params.insert("fromDateTime".to_string(), from);
        }
        if let Some(to) = to_date_time {
            params.insert("toDateTime".to_string(), to);
        }

        let endpoint = format!("/amendment/{}", congress);
        let response: AmendmentsResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.amendments)
    }

    // ========== Member Endpoints ==========

    /// Get a list of congressional members
    #[pyo3(signature = (format=None, offset=None, limit=None, from_date_time=None, to_date_time=None, current_member=None))]
    pub fn list_members(
        &self,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
        from_date_time: Option<String>,
        to_date_time: Option<String>,
        current_member: Option<bool>,
    ) -> PyResult<Vec<Sponsor>> {
        let mut params = HashMap::new();

        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        if let Some(from) = from_date_time {
            params.insert("fromDateTime".to_string(), from);
        }
        if let Some(to) = to_date_time {
            params.insert("toDateTime".to_string(), to);
        }
        if let Some(cm) = current_member {
            params.insert("currentMember".to_string(), cm.to_string());
        }

        let response: MembersResponse = self
            .client
            .get("/member", Some(params))
            .map_err(api_py_err)?;

        Ok(response.members)
    }

    /// Get detailed information for a specified congressional member
    pub fn get_member(&self, bioguide_id: String) -> PyResult<Sponsor> {
        let endpoint = format!("/member/{}", bioguide_id);
        let response: MemberResponse = self.client.get(&endpoint, None).map_err(api_py_err)?;

        Ok(response.member)
    }

    /// Get the list of members by congress
    #[pyo3(signature = (congress, format=None, offset=None, limit=None, current_member=None))]
    pub fn list_members_by_congress(
        &self,
        congress: i32,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
        current_member: Option<bool>,
    ) -> PyResult<Vec<Sponsor>> {
        let mut params = HashMap::new();

        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        if let Some(cm) = current_member {
            params.insert("currentMember".to_string(), cm.to_string());
        }

        let endpoint = format!("/member/congress/{}", congress);
        let response: MembersResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.members)
    }

    /// Get legislation sponsored by a specified member
    #[pyo3(signature = (bioguide_id, format=None, offset=None, limit=None))]
    pub fn get_member_sponsored_legislation(
        &self,
        bioguide_id: String,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> PyResult<Vec<Bill>> {
        let mut params = HashMap::new();

        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }

        let endpoint = format!("/member/{}/sponsored-legislation", bioguide_id);
        let response: SponsoredLegislationResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.sponsored_legislation)
    }

    /// Get legislation cosponsored by a specified member
    #[pyo3(signature = (bioguide_id, format=None, offset=None, limit=None))]
    pub fn get_member_cosponsored_legislation(
        &self,
        bioguide_id: String,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> PyResult<Vec<Bill>> {
        let mut params = HashMap::new();

        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }

        let endpoint = format!("/member/{}/cosponsored-legislation", bioguide_id);
        let response: CosponsoredLegislationResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.cosponsored_legislation)
    }

    /// Get the list of members by state
    #[pyo3(signature = (state_code, format=None, limit=None, current_member=None))]
    pub fn list_members_by_state(
        &self,
        state_code: String,
        format: Option<String>,
        limit: Option<i32>,
        current_member: Option<bool>,
    ) -> PyResult<Vec<Sponsor>> {
        let mut params = HashMap::new();

        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        if let Some(cm) = current_member {
            params.insert("currentMember".to_string(), cm.to_string());
        }

        let endpoint = format!("/member/{}", state_code);
        let response: MembersResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.members)
    }

    /// Get the list of members by state and district
    #[pyo3(signature = (state_code, district, format=None, current_member=None))]
    pub fn list_members_by_state_district(
        &self,
        state_code: String,
        district: i32,
        format: Option<String>,
        current_member: Option<bool>,
    ) -> PyResult<Vec<Sponsor>> {
        let mut params = HashMap::new();

        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(cm) = current_member {
            params.insert("currentMember".to_string(), cm.to_string());
        }

        let endpoint = format!("/member/{}/{}", state_code, district);
        let response: MembersResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.members)
    }

    // ========== Committee Endpoints ==========

    /// Get a list of committees
    #[pyo3(signature = (format=None, offset=None, limit=None))]
    pub fn list_committees(
        &self,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> PyResult<Vec<CommitteeItem>> {
        let mut params = HashMap::new();

        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }

        let response: CommitteesListResponse = self
            .client
            .get("/committee", Some(params))
            .map_err(api_py_err)?;

        Ok(response.committees)
    }

    // ========== Congress/Session Endpoints ==========

    /// Get a list of congresses and congressional sessions
    #[pyo3(signature = (format=None, offset=None, limit=None))]
    pub fn list_congresses(
        &self,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> PyResult<Vec<Congress>> {
        let mut params = HashMap::new();

        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }

        let response: CongressesResponse = self
            .client
            .get("/congress", Some(params))
            .map_err(api_py_err)?;

        Ok(response.congresses)
    }

    /// Get information about a specific congress
    #[pyo3(signature = (congress, format=None))]
    pub fn get_congress(&self, congress: i32, format: Option<String>) -> PyResult<Congress> {
        let mut params = HashMap::new();

        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }

        let endpoint = format!("/congress/{}", congress);
        let response: CongressResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.congress)
    }

    /// Get information about the current congress
    #[pyo3(signature = (format=None))]
    pub fn get_current_congress(&self, format: Option<String>) -> PyResult<Congress> {
        let mut params = HashMap::new();

        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }

        let response: CongressResponse = self
            .client
            .get("/congress/current", Some(params))
            .map_err(api_py_err)?;

        Ok(response.congress)
    }

    // ========================================
    // House Vote Operations
    // ========================================

    /// Get a list of house votes (BETA)
    #[pyo3(signature = (offset=None, limit=None, from_date=None, to_date=None, sort=None, format=None))]
    pub fn list_house_votes(
        &self,
        offset: Option<i32>,
        limit: Option<i32>,
        from_date: Option<String>,
        to_date: Option<String>,
        sort: Option<String>,
        format: Option<String>,
    ) -> PyResult<Vec<HouseVote>> {
        let mut params = HashMap::new();

        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(from) = from_date {
            params.insert("fromDateTime".to_string(), from);
        }
        if let Some(to) = to_date {
            params.insert("toDateTime".to_string(), to);
        }
        if let Some(s) = sort {
            params.insert("sort".to_string(), s);
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }

        let response: HouseVotesResponse = self
            .client
            .get("/house-vote", Some(params))
            .map_err(api_py_err)?;

        Ok(response.votes)
    }

    /// Get house votes for a specific congress (BETA)
    #[pyo3(signature = (congress, offset=None, limit=None, from_date=None, to_date=None, sort=None, format=None))]
    pub fn list_house_votes_by_congress(
        &self,
        congress: i32,
        offset: Option<i32>,
        limit: Option<i32>,
        from_date: Option<String>,
        to_date: Option<String>,
        sort: Option<String>,
        format: Option<String>,
    ) -> PyResult<Vec<HouseVote>> {
        let mut params = HashMap::new();

        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(from) = from_date {
            params.insert("fromDateTime".to_string(), from);
        }
        if let Some(to) = to_date {
            params.insert("toDateTime".to_string(), to);
        }
        if let Some(s) = sort {
            params.insert("sort".to_string(), s);
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }

        let endpoint = format!("/house-vote/{}", congress);
        let response: HouseVotesResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.votes)
    }

    /// Get house votes for a specific congress and session (BETA)
    #[pyo3(signature = (congress, session, offset=None, limit=None, from_date=None, to_date=None, sort=None, format=None))]
    pub fn list_house_votes_by_session(
        &self,
        congress: i32,
        session: i32,
        offset: Option<i32>,
        limit: Option<i32>,
        from_date: Option<String>,
        to_date: Option<String>,
        sort: Option<String>,
        format: Option<String>,
    ) -> PyResult<Vec<HouseVote>> {
        let mut params = HashMap::new();

        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(from) = from_date {
            params.insert("fromDateTime".to_string(), from);
        }
        if let Some(to) = to_date {
            params.insert("toDateTime".to_string(), to);
        }
        if let Some(s) = sort {
            params.insert("sort".to_string(), s);
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }

        let endpoint = format!("/house-vote/{}/{}", congress, session);
        let response: HouseVotesResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.votes)
    }

    /// Get detailed information about a specific house vote (BETA)
    #[pyo3(signature = (congress, session, vote_number, format=None))]
    pub fn get_house_vote(
        &self,
        congress: i32,
        session: i32,
        vote_number: i32,
        format: Option<String>,
    ) -> PyResult<HouseVoteDetail> {
        let mut params = HashMap::new();

        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }

        let endpoint = format!("/house-vote/{}/{}/{}", congress, session, vote_number);
        let response: HouseVoteDetailResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.vote)
    }

    /// Get how members voted on a specific house vote (BETA)
    #[pyo3(signature = (congress, session, vote_number, offset=None, limit=None, format=None))]
    pub fn get_house_vote_members(
        &self,
        congress: i32,
        session: i32,
        vote_number: i32,
        offset: Option<i32>,
        limit: Option<i32>,
        format: Option<String>,
    ) -> PyResult<HouseVoteMembers> {
        let mut params = HashMap::new();

        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }

        let endpoint = format!(
            "/house-vote/{}/{}/{}/members",
            congress, session, vote_number
        );
        let response: HouseVoteMembersResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.vote)
    }

    // ========================================
    // Committee Operations
    // ========================================

    /// Get committees filtered by chamber
    #[pyo3(signature = (chamber, offset=None, limit=None, format=None))]
    pub fn list_committees_by_chamber(
        &self,
        chamber: String,
        offset: Option<i32>,
        limit: Option<i32>,
        format: Option<String>,
    ) -> PyResult<Vec<CommitteeItem>> {
        let mut params = HashMap::new();

        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }

        let endpoint = format!("/committee/{}", chamber);
        let response: CommitteesListResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.committees)
    }

    /// Get committees filtered by congress
    #[pyo3(signature = (congress, offset=None, limit=None, format=None))]
    pub fn list_committees_by_congress(
        &self,
        congress: i32,
        offset: Option<i32>,
        limit: Option<i32>,
        format: Option<String>,
    ) -> PyResult<Vec<CommitteeItem>> {
        let mut params = HashMap::new();

        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }

        let endpoint = format!("/committee/{}", congress);
        let response: CommitteesListResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.committees)
    }

    /// Get committees filtered by congress and chamber
    #[pyo3(signature = (congress, chamber, offset=None, limit=None, format=None))]
    pub fn list_committees_by_congress_and_chamber(
        &self,
        congress: i32,
        chamber: String,
        offset: Option<i32>,
        limit: Option<i32>,
        format: Option<String>,
    ) -> PyResult<Vec<CommitteeItem>> {
        let mut params = HashMap::new();

        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }

        let endpoint = format!("/committee/{}/{}", congress, chamber);
        let response: CommitteesListResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.committees)
    }

    /// Get detailed information about a specific committee
    #[pyo3(signature = (chamber, committee_code, format=None))]
    pub fn get_committee(
        &self,
        chamber: String,
        committee_code: String,
        format: Option<String>,
    ) -> PyResult<CommitteeDetailInfo> {
        let params = build_format_params(format);

        let endpoint = format!("/committee/{}/{}", chamber, committee_code);
        let response: CommitteeDetailResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.committee)
    }

    /// Get bills associated with a committee
    #[pyo3(signature = (chamber, committee_code, offset=None, limit=None, format=None))]
    pub fn get_committee_bills(
        &self,
        chamber: String,
        committee_code: String,
        offset: Option<i32>,
        limit: Option<i32>,
        format: Option<String>,
    ) -> PyResult<Vec<CommitteeBill>> {
        let params = build_offset_limit_format_params(format, offset, limit);

        let endpoint = format!("/committee/{}/{}/bills", chamber, committee_code);
        let response: CommitteeBillsResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.bills)
    }

    // ========================================
    // Committee Report Operations
    // ========================================

    /// Get a list of all committee reports
    #[pyo3(signature = (offset=None, limit=None, from_date=None, to_date=None, sort=None, format=None))]
    pub fn list_committee_reports(
        &self,
        offset: Option<i32>,
        limit: Option<i32>,
        from_date: Option<String>,
        to_date: Option<String>,
        sort: Option<String>,
        format: Option<String>,
    ) -> PyResult<Vec<CommitteeReportItem>> {
        let params = build_sort_date_range_params(offset, limit, from_date, to_date, sort, format);

        let response: CommitteeReportsResponse = self
            .client
            .get("/committee-report", Some(params))
            .map_err(api_py_err)?;

        Ok(response.reports)
    }

    /// Get committee reports filtered by congress
    #[pyo3(signature = (congress, offset=None, limit=None, from_date=None, to_date=None, sort=None, format=None))]
    pub fn list_committee_reports_by_congress(
        &self,
        congress: i32,
        offset: Option<i32>,
        limit: Option<i32>,
        from_date: Option<String>,
        to_date: Option<String>,
        sort: Option<String>,
        format: Option<String>,
    ) -> PyResult<Vec<CommitteeReportItem>> {
        let params = build_sort_date_range_params(offset, limit, from_date, to_date, sort, format);

        let endpoint = format!("/committee-report/{}", congress);
        let response: CommitteeReportsResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.reports)
    }

    /// Get committee reports filtered by congress and report type
    #[pyo3(signature = (congress, report_type, offset=None, limit=None, from_date=None, to_date=None, sort=None, format=None))]
    pub fn list_committee_reports_by_type(
        &self,
        congress: i32,
        report_type: String,
        offset: Option<i32>,
        limit: Option<i32>,
        from_date: Option<String>,
        to_date: Option<String>,
        sort: Option<String>,
        format: Option<String>,
    ) -> PyResult<Vec<CommitteeReportItem>> {
        let params = build_sort_date_range_params(offset, limit, from_date, to_date, sort, format);

        let endpoint = format!("/committee-report/{}/{}", congress, report_type);
        let response: CommitteeReportsResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.reports)
    }

    /// Get detailed information about a specific committee report
    #[pyo3(signature = (congress, report_type, report_number, format=None))]
    pub fn get_committee_report(
        &self,
        congress: i32,
        report_type: String,
        report_number: i32,
        format: Option<String>,
    ) -> PyResult<CommitteeReportDetail> {
        let params = build_format_params(format);

        let endpoint = format!(
            "/committee-report/{}/{}/{}",
            congress, report_type, report_number
        );
        let response: CommitteeReportDetailResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.report)
    }

    /// Get text formats available for a committee report
    #[pyo3(signature = (congress, report_type, report_number, format=None))]
    pub fn get_committee_report_text(
        &self,
        congress: i32,
        report_type: String,
        report_number: i32,
        format: Option<String>,
    ) -> PyResult<Vec<CommitteeReportText>> {
        let params = build_format_params(format);

        let endpoint = format!(
            "/committee-report/{}/{}/{}/text",
            congress, report_type, report_number
        );
        let response: CommitteeReportTextResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.text)
    }

    // ========================================
    // Committee Print Operations
    // ========================================

    /// Get a list of all committee prints
    #[pyo3(signature = (offset=None, limit=None, from_date=None, to_date=None, sort=None, format=None))]
    pub fn list_committee_prints(
        &self,
        offset: Option<i32>,
        limit: Option<i32>,
        from_date: Option<String>,
        to_date: Option<String>,
        sort: Option<String>,
        format: Option<String>,
    ) -> PyResult<Vec<CommitteePrintItem>> {
        let params = build_sort_date_range_params(offset, limit, from_date, to_date, sort, format);

        let response: CommitteePrintsResponse = self
            .client
            .get("/committee-print", Some(params))
            .map_err(api_py_err)?;

        Ok(response.committee_prints)
    }

    /// Get committee prints filtered by congress
    #[pyo3(signature = (congress, offset=None, limit=None, from_date=None, to_date=None, sort=None, format=None))]
    pub fn list_committee_prints_by_congress(
        &self,
        congress: i32,
        offset: Option<i32>,
        limit: Option<i32>,
        from_date: Option<String>,
        to_date: Option<String>,
        sort: Option<String>,
        format: Option<String>,
    ) -> PyResult<Vec<CommitteePrintItem>> {
        let params = build_sort_date_range_params(offset, limit, from_date, to_date, sort, format);

        let endpoint = format!("/committee-print/{}", congress);
        let response: CommitteePrintsResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.committee_prints)
    }

    /// Get committee prints filtered by congress and chamber
    #[pyo3(signature = (congress, chamber, offset=None, limit=None, from_date=None, to_date=None, sort=None, format=None))]
    pub fn list_committee_prints_by_chamber(
        &self,
        congress: i32,
        chamber: String,
        offset: Option<i32>,
        limit: Option<i32>,
        from_date: Option<String>,
        to_date: Option<String>,
        sort: Option<String>,
        format: Option<String>,
    ) -> PyResult<Vec<CommitteePrintItem>> {
        let params = build_sort_date_range_params(offset, limit, from_date, to_date, sort, format);

        let endpoint = format!("/committee-print/{}/{}", congress, chamber);
        let response: CommitteePrintsResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.committee_prints)
    }

    /// Get detailed information about a specific committee print
    #[pyo3(signature = (congress, chamber, jacket_number, format=None))]
    pub fn get_committee_print(
        &self,
        congress: i32,
        chamber: String,
        jacket_number: i32,
        format: Option<String>,
    ) -> PyResult<CommitteePrintDetail> {
        let params = build_format_params(format);

        let endpoint = format!(
            "/committee-print/{}/{}/{}",
            congress, chamber, jacket_number
        );
        let response: CommitteePrintDetailResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.committee_print)
    }

    /// Get text formats available for a committee print
    #[pyo3(signature = (congress, chamber, jacket_number, format=None))]
    pub fn get_committee_print_text(
        &self,
        congress: i32,
        chamber: String,
        jacket_number: i32,
        format: Option<String>,
    ) -> PyResult<Vec<CommitteePrintText>> {
        let params = build_format_params(format);

        let endpoint = format!(
            "/committee-print/{}/{}/{}/text",
            congress, chamber, jacket_number
        );
        let response: CommitteePrintTextResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.text)
    }

    // ========================================
    // Nomination Operations
    // ========================================

    /// Get a list of all nominations
    #[pyo3(signature = (offset=None, limit=None, sort=None, format=None))]
    pub fn list_nominations(
        &self,
        offset: Option<i32>,
        limit: Option<i32>,
        sort: Option<String>,
        format: Option<String>,
    ) -> PyResult<Vec<Nomination>> {
        let mut params = HashMap::new();

        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(s) = sort {
            params.insert("sort".to_string(), s);
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }

        let response: NominationsResponse = self
            .client
            .get("/nomination", Some(params))
            .map_err(api_py_err)?;

        Ok(response.nominations)
    }

    /// Get nominations by congress
    #[pyo3(signature = (congress, offset=None, limit=None, sort=None, format=None))]
    pub fn list_nominations_by_congress(
        &self,
        congress: i32,
        offset: Option<i32>,
        limit: Option<i32>,
        sort: Option<String>,
        format: Option<String>,
    ) -> PyResult<Vec<Nomination>> {
        let mut params = HashMap::new();

        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(s) = sort {
            params.insert("sort".to_string(), s);
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }

        let endpoint = format!("/nomination/{}", congress);
        let response: NominationsResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.nominations)
    }

    /// Get a specific nomination
    #[pyo3(signature = (congress, nomination_number, format=None))]
    pub fn get_nomination(
        &self,
        congress: i32,
        nomination_number: String,
        format: Option<String>,
    ) -> PyResult<Nomination> {
        let mut params = HashMap::new();

        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }

        let endpoint = format!("/nomination/{}/{}", congress, nomination_number);
        let response: NominationDetailResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.nomination)
    }

    /// Get nominees for a nomination
    #[pyo3(signature = (congress, nomination_number, offset=None, limit=None, format=None))]
    pub fn get_nomination_nominees(
        &self,
        congress: i32,
        nomination_number: String,
        offset: Option<i32>,
        limit: Option<i32>,
        format: Option<String>,
    ) -> PyResult<Vec<Nominee>> {
        let mut params = HashMap::new();

        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }

        let endpoint = format!("/nomination/{}/{}/nominees", congress, nomination_number);
        let response: NomineesResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.nominees)
    }

    // ========================================
    // Treaty Operations
    // ========================================

    /// Get a list of all treaties
    #[pyo3(signature = (offset=None, limit=None, sort=None, format=None))]
    pub fn list_treaties(
        &self,
        offset: Option<i32>,
        limit: Option<i32>,
        sort: Option<String>,
        format: Option<String>,
    ) -> PyResult<Vec<Treaty>> {
        let mut params = HashMap::new();

        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(s) = sort {
            params.insert("sort".to_string(), s);
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }

        let response: TreatiesResponse = self
            .client
            .get("/treaty", Some(params))
            .map_err(api_py_err)?;

        Ok(response.treaties)
    }

    /// Get treaties by congress
    #[pyo3(signature = (congress, offset=None, limit=None, sort=None, format=None))]
    pub fn list_treaties_by_congress(
        &self,
        congress: i32,
        offset: Option<i32>,
        limit: Option<i32>,
        sort: Option<String>,
        format: Option<String>,
    ) -> PyResult<Vec<Treaty>> {
        let mut params = HashMap::new();

        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(s) = sort {
            params.insert("sort".to_string(), s);
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }

        let endpoint = format!("/treaty/{}", congress);
        let response: TreatiesResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.treaties)
    }

    /// Get a specific treaty
    #[pyo3(signature = (congress, treaty_number, format=None))]
    pub fn get_treaty(
        &self,
        congress: i32,
        treaty_number: String,
        format: Option<String>,
    ) -> PyResult<Treaty> {
        let mut params = HashMap::new();

        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }

        let endpoint = format!("/treaty/{}/{}", congress, treaty_number);
        let response: TreatyDetailResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.treaty)
    }

    // ========================================
    // Hearing Operations
    // ========================================

    /// Get a list of all hearings
    #[pyo3(signature = (offset=None, limit=None, sort=None, format=None))]
    pub fn list_hearings(
        &self,
        offset: Option<i32>,
        limit: Option<i32>,
        sort: Option<String>,
        format: Option<String>,
    ) -> PyResult<Vec<Hearing>> {
        let mut params = HashMap::new();

        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(s) = sort {
            params.insert("sort".to_string(), s);
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }

        let response: HearingsResponse = self
            .client
            .get("/hearing", Some(params))
            .map_err(api_py_err)?;

        Ok(response.hearings)
    }

    /// Get hearings by congress
    #[pyo3(signature = (congress, offset=None, limit=None, sort=None, format=None))]
    pub fn list_hearings_by_congress(
        &self,
        congress: i32,
        offset: Option<i32>,
        limit: Option<i32>,
        sort: Option<String>,
        format: Option<String>,
    ) -> PyResult<Vec<Hearing>> {
        let mut params = HashMap::new();

        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(s) = sort {
            params.insert("sort".to_string(), s);
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }

        let endpoint = format!("/hearing/{}", congress);
        let response: HearingsResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.hearings)
    }

    /// Get hearings by congress and chamber
    #[pyo3(signature = (congress, chamber, offset=None, limit=None, format=None))]
    pub fn list_hearings_by_chamber(
        &self,
        congress: i32,
        chamber: String,
        offset: Option<i32>,
        limit: Option<i32>,
        format: Option<String>,
    ) -> PyResult<Vec<Hearing>> {
        let mut params = HashMap::new();

        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }

        let endpoint = format!("/hearing/{}/{}", congress, chamber.to_lowercase());
        let response: HearingsResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.hearings)
    }

    /// Get a specific hearing
    #[pyo3(signature = (congress, chamber, jacket_number, format=None))]
    pub fn get_hearing(
        &self,
        congress: i32,
        chamber: String,
        jacket_number: i32,
        format: Option<String>,
    ) -> PyResult<Hearing> {
        let mut params = HashMap::new();

        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }

        let endpoint = format!(
            "/hearing/{}/{}/{}",
            congress,
            chamber.to_lowercase(),
            jacket_number
        );
        let response: HearingDetailResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.hearing)
    }

    // ========================================
    // Congressional Record Operations
    // ========================================

    /// Get daily congressional records
    #[pyo3(signature = (offset=None, limit=None, format=None))]
    pub fn list_congressional_records(
        &self,
        offset: Option<i32>,
        limit: Option<i32>,
        format: Option<String>,
    ) -> PyResult<Vec<DailyCongressionalRecord>> {
        let mut params = HashMap::new();

        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }

        let response: DailyCongressionalRecordsResponse = self
            .client
            .get("/daily-congressional-record", Some(params))
            .map_err(api_py_err)?;

        Ok(response.daily_congressional_record)
    }

    // ========================================
    // Law Operations
    // ========================================

    /// Get a list of all laws
    #[pyo3(signature = (offset=None, limit=None, format=None))]
    pub fn list_laws(
        &self,
        offset: Option<i32>,
        limit: Option<i32>,
        format: Option<String>,
    ) -> PyResult<Vec<LawItem>> {
        let mut params = HashMap::new();

        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }

        let response: LawsResponse = self.client.get("/law", Some(params)).map_err(api_py_err)?;

        Ok(response.bills)
    }

    /// Get laws by congress
    #[pyo3(signature = (congress, offset=None, limit=None, format=None))]
    pub fn list_laws_by_congress(
        &self,
        congress: i32,
        offset: Option<i32>,
        limit: Option<i32>,
        format: Option<String>,
    ) -> PyResult<Vec<LawItem>> {
        let mut params = HashMap::new();

        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }

        let endpoint = format!("/law/{}", congress);
        let response: LawsResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.bills)
    }

    /// Get laws by congress and type
    /// Parameters:
    ///   - congress: The congress number (e.g., 118)
    ///   - law_type: The law type. Values are either "pub" (public laws) or "priv" (private laws)
    ///   - offset: Pagination offset (optional)
    ///   - limit: Number of results to return (optional)
    ///   - format: Response format (optional)
    #[pyo3(signature = (congress, law_type, offset=None, limit=None, format=None))]
    pub fn list_laws_by_type(
        &self,
        congress: i32,
        law_type: String,
        offset: Option<i32>,
        limit: Option<i32>,
        format: Option<String>,
    ) -> PyResult<Vec<LawItem>> {
        let mut params = HashMap::new();

        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }

        let endpoint = format!("/law/{}/{}", congress, law_type);
        let response: LawsResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.bills)
    }

    /// Get a specific law by bill type and bill number
    ///
    /// Note: Despite the swagger documentation referring to "lawType" and "lawNumber",
    /// the actual API endpoint uses the BILL's type and number, not the resulting law's type/number.
    /// For example, to get the law that HR 4984 became, use law_type="hr" and law_number="4984"
    ///
    /// Parameters:
    ///   - congress: The congress number (e.g., 118)
    ///   - law_type: Bill type like "hr", "s", "hjres", "sjres" (case-insensitive, will be lowercased)
    ///               This is NOT "pub"/"priv" - those are for list_laws_by_type()
    ///   - law_number: The bill number as string (e.g., "346" or "4984")
    #[pyo3(signature = (congress, law_type, law_number, format=None))]
    pub fn get_law(
        &self,
        congress: i32,
        law_type: String,
        law_number: String,
        format: Option<String>,
    ) -> PyResult<LawDetail> {
        let mut params = HashMap::new();

        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }

        // API expects lowercase bill type
        let law_type_lower = law_type.to_lowercase();
        let endpoint = format!("/law/{}/{}/{}", congress, law_type_lower, law_number);
        let response: LawDetailResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.bill)
    }

    // ========================================
    // Summaries Operations
    // ========================================

    /// Get a list of summaries
    #[pyo3(signature = (offset=None, limit=None, format=None))]
    pub fn list_summaries(
        &self,
        offset: Option<i32>,
        limit: Option<i32>,
        format: Option<String>,
    ) -> PyResult<Vec<SummaryItem>> {
        let mut params = HashMap::new();

        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }

        let response: SummariesListResponse = self
            .client
            .get("/summaries", Some(params))
            .map_err(api_py_err)?;

        Ok(response.summaries)
    }

    /// Get summaries by congress
    #[pyo3(signature = (congress, offset=None, limit=None, format=None))]
    pub fn list_summaries_by_congress(
        &self,
        congress: i32,
        offset: Option<i32>,
        limit: Option<i32>,
        format: Option<String>,
    ) -> PyResult<Vec<SummaryItem>> {
        let mut params = HashMap::new();

        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }

        let endpoint = format!("/summaries/{}", congress);
        let response: SummariesListResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.summaries)
    }

    // ========================================
    // CRS Report Operations
    // ========================================

    /// Get a list of CRS reports
    #[pyo3(signature = (offset=None, limit=None, from_date_time=None, to_date_time=None, format=None))]
    pub fn list_crs_reports(
        &self,
        offset: Option<i32>,
        limit: Option<i32>,
        from_date_time: Option<String>,
        to_date_time: Option<String>,
        format: Option<String>,
    ) -> PyResult<Vec<CrsReport>> {
        let mut params = HashMap::new();

        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(from) = from_date_time {
            params.insert("fromDateTime".to_string(), from);
        }
        if let Some(to) = to_date_time {
            params.insert("toDateTime".to_string(), to);
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }

        let response: CrsReportsResponse = self
            .client
            .get("/crsreport", Some(params))
            .map_err(api_py_err)?;

        Ok(response.crs_reports)
    }

    /// Get detailed information for a specific CRS report
    #[pyo3(signature = (report_number, format=None))]
    pub fn get_crs_report(
        &self,
        report_number: String,
        format: Option<String>,
    ) -> PyResult<CrsReportDetail> {
        let mut params = HashMap::new();

        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }

        let endpoint = format!("/crsreport/{}", report_number);
        let response: CrsReportDetailResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.report)
    }

    /// Compatibility alias for get_bill_detail.
    #[pyo3(signature = (congress, bill_type, bill_number, format=None, offset=None, limit=None, from_date_time=None, to_date_time=None))]
    pub fn get_bill(
        &self,
        congress: i32,
        bill_type: String,
        bill_number: String,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
        from_date_time: Option<String>,
        to_date_time: Option<String>,
    ) -> PyResult<BillDetail> {
        self.get_bill_detail(
            congress,
            bill_type,
            bill_number,
            format,
            offset,
            limit,
            from_date_time,
            to_date_time,
        )
    }

    /// Get amendments filtered by congress and amendment type.
    #[pyo3(signature = (congress, amendment_type, format=None, offset=None, limit=None, from_date_time=None, to_date_time=None))]
    pub fn list_amendments_by_type(
        &self,
        congress: i32,
        amendment_type: String,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
        from_date_time: Option<String>,
        to_date_time: Option<String>,
    ) -> PyResult<Vec<Amendment>> {
        let mut params = HashMap::new();

        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        if let Some(from) = from_date_time {
            params.insert("fromDateTime".to_string(), from);
        }
        if let Some(to) = to_date_time {
            params.insert("toDateTime".to_string(), to);
        }

        let endpoint = format!("/amendment/{}/{}", congress, amendment_type.to_lowercase());
        let response: AmendmentsResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.amendments)
    }

    /// Get detail for a specific amendment.
    #[pyo3(signature = (congress, amendment_type, amendment_number, format=None))]
    pub fn get_amendment(
        &self,
        congress: i32,
        amendment_type: String,
        amendment_number: String,
        format: Option<String>,
    ) -> PyResult<AmendmentDetail> {
        let mut params = HashMap::new();

        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }

        let endpoint = format!(
            "/amendment/{}/{}/{}",
            congress,
            amendment_type.to_lowercase(),
            amendment_number
        );
        let response: AmendmentDetailResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.amendment)
    }

    #[pyo3(signature = (congress, amendment_type, amendment_number, format=None, offset=None, limit=None))]
    pub fn get_amendment_actions(
        &self,
        congress: i32,
        amendment_type: String,
        amendment_number: String,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> PyResult<Vec<Action>> {
        let mut params = HashMap::new();

        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }

        let endpoint = format!(
            "/amendment/{}/{}/{}/actions",
            congress,
            amendment_type.to_lowercase(),
            amendment_number
        );
        let response: ActionsResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.actions)
    }

    #[pyo3(signature = (congress, amendment_type, amendment_number, format=None, offset=None, limit=None))]
    pub fn get_amendment_amendments(
        &self,
        congress: i32,
        amendment_type: String,
        amendment_number: String,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> PyResult<Vec<Amendment>> {
        let mut params = HashMap::new();

        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }

        let endpoint = format!(
            "/amendment/{}/{}/{}/amendments",
            congress,
            amendment_type.to_lowercase(),
            amendment_number
        );
        let response: AmendmentsResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.amendments)
    }

    #[pyo3(signature = (congress, amendment_type, amendment_number, format=None, offset=None, limit=None))]
    pub fn get_amendment_cosponsors(
        &self,
        congress: i32,
        amendment_type: String,
        amendment_number: String,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> PyResult<Vec<Cosponsor>> {
        let mut params = HashMap::new();

        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }

        let endpoint = format!(
            "/amendment/{}/{}/{}/cosponsors",
            congress,
            amendment_type.to_lowercase(),
            amendment_number
        );
        let response: CosponsorsResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.cosponsors)
    }

    #[pyo3(signature = (congress, amendment_type, amendment_number, format=None, offset=None, limit=None))]
    pub fn get_amendment_text(
        &self,
        congress: i32,
        amendment_type: String,
        amendment_number: String,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> PyResult<Vec<TextVersion>> {
        let mut params = HashMap::new();

        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }

        let endpoint = format!(
            "/amendment/{}/{}/{}/text",
            congress,
            amendment_type.to_lowercase(),
            amendment_number
        );
        let response: TextVersionsResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.text_versions)
    }

    #[pyo3(signature = (congress, state_code, district, format=None, offset=None, limit=None, current_member=None))]
    pub fn list_members_by_congress_state_district(
        &self,
        congress: i32,
        state_code: String,
        district: i32,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
        current_member: Option<bool>,
    ) -> PyResult<Vec<Sponsor>> {
        let mut params = HashMap::new();

        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        if let Some(cm) = current_member {
            params.insert("currentMember".to_string(), cm.to_string());
        }

        let endpoint = format!("/member/congress/{}/{}/{}", congress, state_code, district);
        let response: MembersResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.members)
    }

    #[pyo3(signature = (congress, chamber, committee_code, format=None))]
    pub fn get_committee_by_congress(
        &self,
        congress: i32,
        chamber: String,
        committee_code: String,
        format: Option<String>,
    ) -> PyResult<CommitteeDetailInfo> {
        let params = build_format_params(format);

        let endpoint = format!(
            "/committee/{}/{}/{}",
            congress,
            chamber.to_lowercase(),
            committee_code
        );
        let response: CommitteeDetailResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.committee)
    }

    #[pyo3(signature = (chamber, committee_code, format=None, offset=None, limit=None))]
    pub fn get_committee_house_communications(
        &self,
        chamber: String,
        committee_code: String,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> PyResult<Vec<HouseCommunication>> {
        let params = build_offset_limit_format_params(format, offset, limit);

        let endpoint = format!(
            "/committee/{}/{}/house-communication",
            chamber.to_lowercase(),
            committee_code
        );
        let response: HouseCommunicationsResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.house_communications)
    }

    #[pyo3(signature = (chamber, committee_code, format=None, offset=None, limit=None))]
    pub fn get_committee_senate_communications(
        &self,
        chamber: String,
        committee_code: String,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> PyResult<Vec<SenateCommunication>> {
        let params = build_offset_limit_format_params(format, offset, limit);

        let endpoint = format!(
            "/committee/{}/{}/senate-communication",
            chamber.to_lowercase(),
            committee_code
        );
        let response: SenateCommunicationsResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.senate_communications)
    }

    #[pyo3(signature = (chamber, committee_code, format=None, offset=None, limit=None))]
    pub fn get_committee_nominations(
        &self,
        chamber: String,
        committee_code: String,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> PyResult<Vec<Nomination>> {
        let params = build_offset_limit_format_params(format, offset, limit);

        let endpoint = format!(
            "/committee/{}/{}/nominations",
            chamber.to_lowercase(),
            committee_code
        );
        let response: NominationsResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.nominations)
    }

    #[pyo3(signature = (chamber, committee_code, format=None, offset=None, limit=None))]
    pub fn get_committee_reports(
        &self,
        chamber: String,
        committee_code: String,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> PyResult<Vec<CommitteeReportItem>> {
        let params = build_offset_limit_format_params(format, offset, limit);

        let endpoint = format!(
            "/committee/{}/{}/reports",
            chamber.to_lowercase(),
            committee_code
        );
        let response: CommitteeReportsResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.reports)
    }

    #[pyo3(signature = (offset=None, limit=None, from_date_time=None, to_date_time=None, format=None))]
    pub fn list_committee_meetings(
        &self,
        offset: Option<i32>,
        limit: Option<i32>,
        from_date_time: Option<String>,
        to_date_time: Option<String>,
        format: Option<String>,
    ) -> PyResult<Vec<CommitteeMeeting>> {
        let mut params = HashMap::new();
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        if let Some(from) = from_date_time {
            params.insert("fromDateTime".to_string(), from);
        }
        if let Some(to) = to_date_time {
            params.insert("toDateTime".to_string(), to);
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }

        let response: CommitteeMeetingsResponse = self
            .client
            .get("/committee-meeting", Some(params))
            .map_err(api_py_err)?;

        Ok(response.committee_meetings)
    }

    #[pyo3(signature = (congress, offset=None, limit=None, from_date_time=None, to_date_time=None, format=None))]
    pub fn list_committee_meetings_by_congress(
        &self,
        congress: i32,
        offset: Option<i32>,
        limit: Option<i32>,
        from_date_time: Option<String>,
        to_date_time: Option<String>,
        format: Option<String>,
    ) -> PyResult<Vec<CommitteeMeeting>> {
        let mut params = HashMap::new();
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        if let Some(from) = from_date_time {
            params.insert("fromDateTime".to_string(), from);
        }
        if let Some(to) = to_date_time {
            params.insert("toDateTime".to_string(), to);
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }

        let endpoint = format!("/committee-meeting/{}", congress);
        let response: CommitteeMeetingsResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.committee_meetings)
    }

    #[pyo3(signature = (congress, chamber, offset=None, limit=None, from_date_time=None, to_date_time=None, format=None))]
    pub fn list_committee_meetings_by_chamber(
        &self,
        congress: i32,
        chamber: String,
        offset: Option<i32>,
        limit: Option<i32>,
        from_date_time: Option<String>,
        to_date_time: Option<String>,
        format: Option<String>,
    ) -> PyResult<Vec<CommitteeMeeting>> {
        let mut params = HashMap::new();
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        if let Some(from) = from_date_time {
            params.insert("fromDateTime".to_string(), from);
        }
        if let Some(to) = to_date_time {
            params.insert("toDateTime".to_string(), to);
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }

        let endpoint = format!("/committee-meeting/{}/{}", congress, chamber.to_lowercase());
        let response: CommitteeMeetingsResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.committee_meetings)
    }

    #[pyo3(signature = (congress, chamber, event_id, format=None))]
    pub fn get_committee_meeting(
        &self,
        congress: i32,
        chamber: String,
        event_id: String,
        format: Option<String>,
    ) -> PyResult<CommitteeMeeting> {
        let mut params = HashMap::new();
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }

        let endpoint = format!(
            "/committee-meeting/{}/{}/{}",
            congress,
            chamber.to_lowercase(),
            event_id
        );
        let response: CommitteeMeetingDetailResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.committee_meeting)
    }

    #[pyo3(signature = (offset=None, limit=None, format=None))]
    pub fn list_bound_congressional_records(
        &self,
        offset: Option<i32>,
        limit: Option<i32>,
        format: Option<String>,
    ) -> PyResult<Vec<BoundCongressionalRecord>> {
        let mut params = HashMap::new();
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }

        let response: BoundCongressionalRecordsResponse = self
            .client
            .get("/bound-congressional-record", Some(params))
            .map_err(api_py_err)?;

        Ok(response.bound_congressional_record)
    }
    #[pyo3(signature = (year, offset=None, limit=None, format=None))]
    pub fn list_bound_congressional_records_by_year(
        &self,
        year: i32,
        offset: Option<i32>,
        limit: Option<i32>,
        format: Option<String>,
    ) -> PyResult<Vec<BoundCongressionalRecord>> {
        let mut params = HashMap::new();
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }

        let endpoint = format!("/bound-congressional-record/{}", year);
        let response: BoundCongressionalRecordsResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.bound_congressional_record)
    }

    #[pyo3(signature = (year, month, offset=None, limit=None, format=None))]
    pub fn list_bound_congressional_records_by_month(
        &self,
        year: i32,
        month: i32,
        offset: Option<i32>,
        limit: Option<i32>,
        format: Option<String>,
    ) -> PyResult<Vec<BoundCongressionalRecord>> {
        let mut params = HashMap::new();
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }

        let endpoint = format!("/bound-congressional-record/{}/{}", year, month);
        let response: BoundCongressionalRecordsResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.bound_congressional_record)
    }

    #[pyo3(signature = (year, month, day, offset=None, limit=None, format=None))]
    pub fn get_bound_congressional_record(
        &self,
        year: i32,
        month: i32,
        day: i32,
        offset: Option<i32>,
        limit: Option<i32>,
        format: Option<String>,
    ) -> PyResult<Vec<BoundCongressionalRecord>> {
        let mut params = HashMap::new();
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }

        let endpoint = format!("/bound-congressional-record/{}/{}/{}", year, month, day);
        let response: BoundCongressionalRecordsResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.bound_congressional_record)
    }

    #[pyo3(signature = (offset=None, limit=None, format=None))]
    pub fn list_congressional_record(
        &self,
        offset: Option<i32>,
        limit: Option<i32>,
        format: Option<String>,
    ) -> PyResult<CongressionalRecord> {
        let mut params = HashMap::new();
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }

        let response: CongressionalRecordResponse = self
            .client
            .get("/congressional-record", Some(params))
            .map_err(api_py_err)?;

        Ok(response.results)
    }

    #[pyo3(signature = (volume_number, offset=None, limit=None, format=None))]
    pub fn list_daily_congressional_records_by_volume(
        &self,
        volume_number: i32,
        offset: Option<i32>,
        limit: Option<i32>,
        format: Option<String>,
    ) -> PyResult<Vec<DailyCongressionalRecord>> {
        let mut params = HashMap::new();
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }

        let endpoint = format!("/daily-congressional-record/{}", volume_number);
        let response: DailyCongressionalRecordsResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.daily_congressional_record)
    }

    #[pyo3(signature = (volume_number, issue_number, format=None))]
    pub fn get_daily_congressional_record_issue(
        &self,
        volume_number: i32,
        issue_number: String,
        format: Option<String>,
    ) -> PyResult<DailyCongressionalRecordIssue> {
        let mut params = HashMap::new();
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }

        let endpoint = format!(
            "/daily-congressional-record/{}/{}",
            volume_number, issue_number
        );
        let response: DailyCongressionalRecordIssueResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.issue)
    }

    #[pyo3(signature = (volume_number, issue_number, offset=None, limit=None, format=None))]
    pub fn get_daily_congressional_record_articles(
        &self,
        volume_number: i32,
        issue_number: String,
        offset: Option<i32>,
        limit: Option<i32>,
        format: Option<String>,
    ) -> PyResult<Vec<DailyCongressionalRecordArticleGroup>> {
        let mut params = HashMap::new();
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }

        let endpoint = format!(
            "/daily-congressional-record/{}/{}/articles",
            volume_number, issue_number
        );
        let response: DailyCongressionalRecordArticlesResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.articles)
    }

    #[pyo3(signature = (offset=None, limit=None, format=None))]
    pub fn list_house_communications(
        &self,
        offset: Option<i32>,
        limit: Option<i32>,
        format: Option<String>,
    ) -> PyResult<Vec<HouseCommunication>> {
        let mut params = HashMap::new();
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }

        let response: HouseCommunicationsResponse = self
            .client
            .get("/house-communication", Some(params))
            .map_err(api_py_err)?;

        Ok(response.house_communications)
    }

    #[pyo3(signature = (congress, offset=None, limit=None, format=None))]
    pub fn list_house_communications_by_congress(
        &self,
        congress: i32,
        offset: Option<i32>,
        limit: Option<i32>,
        format: Option<String>,
    ) -> PyResult<Vec<HouseCommunication>> {
        let mut params = HashMap::new();
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }

        let endpoint = format!("/house-communication/{}", congress);
        let response: HouseCommunicationsResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.house_communications)
    }

    #[pyo3(signature = (congress, communication_type, offset=None, limit=None, format=None))]
    pub fn list_house_communications_by_type(
        &self,
        congress: i32,
        communication_type: String,
        offset: Option<i32>,
        limit: Option<i32>,
        format: Option<String>,
    ) -> PyResult<Vec<HouseCommunication>> {
        let mut params = HashMap::new();
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }

        let endpoint = format!(
            "/house-communication/{}/{}",
            congress,
            communication_type.to_lowercase()
        );
        let response: HouseCommunicationsResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.house_communications)
    }

    #[pyo3(signature = (congress, communication_type, communication_number, format=None))]
    pub fn get_house_communication(
        &self,
        congress: i32,
        communication_type: String,
        communication_number: i32,
        format: Option<String>,
    ) -> PyResult<HouseCommunication> {
        let mut params = HashMap::new();
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }

        let endpoint = format!(
            "/house-communication/{}/{}/{}",
            congress,
            communication_type.to_lowercase(),
            communication_number
        );
        let response: HouseCommunicationDetailResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.house_communication)
    }

    #[pyo3(signature = (offset=None, limit=None, format=None))]
    pub fn list_senate_communications(
        &self,
        offset: Option<i32>,
        limit: Option<i32>,
        format: Option<String>,
    ) -> PyResult<Vec<SenateCommunication>> {
        let mut params = HashMap::new();
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }

        let response: SenateCommunicationsResponse = self
            .client
            .get("/senate-communication", Some(params))
            .map_err(api_py_err)?;

        Ok(response.senate_communications)
    }

    #[pyo3(signature = (congress, offset=None, limit=None, format=None))]
    pub fn list_senate_communications_by_congress(
        &self,
        congress: i32,
        offset: Option<i32>,
        limit: Option<i32>,
        format: Option<String>,
    ) -> PyResult<Vec<SenateCommunication>> {
        let mut params = HashMap::new();
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }

        let endpoint = format!("/senate-communication/{}", congress);
        let response: SenateCommunicationsResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.senate_communications)
    }

    #[pyo3(signature = (congress, communication_type, offset=None, limit=None, format=None))]
    pub fn list_senate_communications_by_type(
        &self,
        congress: i32,
        communication_type: String,
        offset: Option<i32>,
        limit: Option<i32>,
        format: Option<String>,
    ) -> PyResult<Vec<SenateCommunication>> {
        let mut params = HashMap::new();
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }

        let endpoint = format!(
            "/senate-communication/{}/{}",
            congress,
            communication_type.to_lowercase()
        );
        let response: SenateCommunicationsResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.senate_communications)
    }

    #[pyo3(signature = (congress, communication_type, communication_number, format=None))]
    pub fn get_senate_communication(
        &self,
        congress: i32,
        communication_type: String,
        communication_number: i32,
        format: Option<String>,
    ) -> PyResult<SenateCommunication> {
        let mut params = HashMap::new();
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }

        let endpoint = format!(
            "/senate-communication/{}/{}/{}",
            congress,
            communication_type.to_lowercase(),
            communication_number
        );
        let response: SenateCommunicationDetailResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.senate_communication)
    }

    #[pyo3(signature = (offset=None, limit=None, format=None))]
    pub fn list_house_requirements(
        &self,
        offset: Option<i32>,
        limit: Option<i32>,
        format: Option<String>,
    ) -> PyResult<Vec<HouseRequirement>> {
        let mut params = HashMap::new();
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }

        let response: HouseRequirementsResponse = self
            .client
            .get("/house-requirement", Some(params))
            .map_err(api_py_err)?;

        Ok(response.house_requirements)
    }

    #[pyo3(signature = (requirement_number, format=None))]
    pub fn get_house_requirement(
        &self,
        requirement_number: i32,
        format: Option<String>,
    ) -> PyResult<HouseRequirement> {
        let mut params = HashMap::new();
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }

        let endpoint = format!("/house-requirement/{}", requirement_number);
        let response: HouseRequirementDetailResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.house_requirement)
    }

    #[pyo3(signature = (requirement_number, offset=None, limit=None, format=None))]
    pub fn get_house_requirement_matching_communications(
        &self,
        requirement_number: i32,
        offset: Option<i32>,
        limit: Option<i32>,
        format: Option<String>,
    ) -> PyResult<Vec<HouseCommunication>> {
        let mut params = HashMap::new();
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }

        let endpoint = format!(
            "/house-requirement/{}/matching-communications",
            requirement_number
        );
        let response: MatchingCommunicationsResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.matching_communications)
    }

    #[pyo3(signature = (congress, nomination_number, format=None, offset=None, limit=None))]
    pub fn get_nomination_actions(
        &self,
        congress: i32,
        nomination_number: String,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> PyResult<Vec<Action>> {
        let params = build_offset_limit_format_params(format, offset, limit);

        let endpoint = format!("/nomination/{}/{}/actions", congress, nomination_number);
        let response: ActionsResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.actions)
    }

    #[pyo3(signature = (congress, nomination_number, format=None))]
    pub fn get_nomination_committees(
        &self,
        congress: i32,
        nomination_number: String,
        format: Option<String>,
    ) -> PyResult<Vec<NominationCommittee>> {
        let params = build_format_params(format);

        let endpoint = format!("/nomination/{}/{}/committees", congress, nomination_number);
        let response: NominationCommitteesResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.committees)
    }

    #[pyo3(signature = (congress, nomination_number, format=None, offset=None, limit=None))]
    pub fn get_nomination_hearings(
        &self,
        congress: i32,
        nomination_number: String,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> PyResult<Vec<NominationHearing>> {
        let mut params = HashMap::new();
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }

        let endpoint = format!("/nomination/{}/{}/hearings", congress, nomination_number);
        let response: NominationHearingsResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.hearings)
    }

    #[pyo3(signature = (congress, nomination_number, ordinal, format=None, offset=None, limit=None))]
    pub fn get_nomination_ordinal(
        &self,
        congress: i32,
        nomination_number: String,
        ordinal: String,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> PyResult<Vec<Nominee>> {
        let mut params = HashMap::new();
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }

        let endpoint = format!("/nomination/{}/{}/{}", congress, nomination_number, ordinal);
        let response: NomineesResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.nominees)
    }

    #[pyo3(signature = (congress, treaty_number, format=None, offset=None, limit=None))]
    pub fn get_treaty_actions(
        &self,
        congress: i32,
        treaty_number: String,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> PyResult<Vec<Action>> {
        let mut params = HashMap::new();
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }

        let endpoint = format!("/treaty/{}/{}/actions", congress, treaty_number);
        let response: ActionsResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.actions)
    }

    #[pyo3(signature = (congress, treaty_number, format=None))]
    pub fn get_treaty_committees(
        &self,
        congress: i32,
        treaty_number: String,
        format: Option<String>,
    ) -> PyResult<Vec<NominationCommittee>> {
        let mut params = HashMap::new();
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }

        let endpoint = format!("/treaty/{}/{}/committees", congress, treaty_number);
        let response: TreatyCommitteesResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.treaty_committees)
    }

    #[pyo3(signature = (congress, treaty_number, treaty_suffix, format=None))]
    pub fn get_treaty_part(
        &self,
        congress: i32,
        treaty_number: String,
        treaty_suffix: String,
        format: Option<String>,
    ) -> PyResult<Vec<Treaty>> {
        let params = build_format_params(format);

        let endpoint = format!("/treaty/{}/{}/{}", congress, treaty_number, treaty_suffix);
        let response: TreatyPartDetailResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.treaty)
    }

    #[pyo3(signature = (congress, treaty_number, treaty_suffix, format=None, offset=None, limit=None))]
    pub fn get_treaty_part_actions(
        &self,
        congress: i32,
        treaty_number: String,
        treaty_suffix: String,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> PyResult<Vec<Action>> {
        let params = build_offset_limit_format_params(format, offset, limit);

        let endpoint = format!(
            "/treaty/{}/{}/{}/actions",
            congress, treaty_number, treaty_suffix
        );
        let response: ActionsResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.actions)
    }

    #[pyo3(signature = (congress, bill_type, offset=None, limit=None, format=None))]
    pub fn list_summaries_by_bill_type(
        &self,
        congress: i32,
        bill_type: String,
        offset: Option<i32>,
        limit: Option<i32>,
        format: Option<String>,
    ) -> PyResult<Vec<SummaryItem>> {
        let mut params = HashMap::new();
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }

        let endpoint = format!("/summaries/{}/{}", congress, bill_type.to_lowercase());
        let response: SummariesListResponse = self
            .client
            .get(&endpoint, Some(params))
            .map_err(api_py_err)?;

        Ok(response.summaries)
    }
}

#[pyfunction]
pub fn configure_client_retries(
    mut client: PyRefMut<'_, CDGPythonClient>,
    retry_attempts: i32,
    retry_base_delay_ms: i32,
) -> PyResult<()> {
    client.set_retry_config(retry_attempts, retry_base_delay_ms)
}

#[pyfunction]
pub fn get_client_retry_config(client: PyRef<'_, CDGPythonClient>) -> (i32, i32) {
    (client.retry_attempts(), client.retry_base_delay_ms())
}

fn request_params(offset: Option<i32>, limit: Option<i32>) -> Option<HashMap<String, String>> {
    let mut params = HashMap::new();

    if let Some(offset) = offset {
        params.insert("offset".to_string(), offset.to_string());
    }

    if let Some(limit) = limit {
        params.insert("limit".to_string(), limit.to_string());
    }

    if params.is_empty() {
        None
    } else {
        Some(params)
    }
}

#[pymethods]
impl AsyncClientCore {
    #[pyo3(signature = (api_key, timeout_seconds=None, user_agent=None))]
    #[new]
    pub fn new(
        api_key: String,
        timeout_seconds: Option<f64>,
        user_agent: Option<String>,
    ) -> PyResult<Self> {
        if api_key.trim().is_empty() {
            return Err(api_py_err(ApiError::MissingApiKey));
        }

        let mut client =
            CongressApiClient::new(api_key, RetryConfig::default()).map_err(api_py_err)?;
        client
            .set_timeout_seconds(timeout_seconds)
            .map_err(api_py_err)?;
        client.set_user_agent(user_agent).map_err(api_py_err)?;

        Ok(Self { client })
    }

    pub fn configure_retries(
        &mut self,
        retry_attempts: i32,
        retry_base_delay_ms: i32,
    ) -> PyResult<()> {
        if retry_attempts < 1 {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "retry_attempts must be at least 1",
            ));
        }

        if retry_base_delay_ms < 0 {
            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                "retry_base_delay_ms cannot be negative",
            ));
        }

        self.client.retry_config.max_attempts = retry_attempts as u32;
        self.client.retry_config.base_delay_ms = retry_base_delay_ms as u64;

        Ok(())
    }

    pub fn get_retry_config(&self) -> (i32, i32) {
        (
            self.client.retry_config.max_attempts as i32,
            self.client.retry_config.base_delay_ms as i32,
        )
    }

    #[pyo3(signature = (timeout_seconds=None))]
    pub fn configure_timeout(&mut self, timeout_seconds: Option<f64>) -> PyResult<()> {
        self.client
            .set_timeout_seconds(timeout_seconds)
            .map_err(api_py_err)
    }

    pub fn get_timeout(&self) -> Option<f64> {
        self.client.timeout_seconds()
    }

    #[pyo3(signature = (user_agent=None))]
    pub fn configure_user_agent(&mut self, user_agent: Option<String>) -> PyResult<()> {
        self.client.set_user_agent(user_agent).map_err(api_py_err)
    }

    pub fn get_user_agent(&self) -> Option<String> {
        self.client.user_agent()
    }

    #[pyo3(signature = (handler=None))]
    fn _set_log_handler(&mut self, handler: Option<Py<PyAny>>) {
        self.client.set_log_handler(handler);
    }

    fn _clear_log_handler(&mut self) {
        self.client.clear_log_handler();
    }

    fn _logging_enabled(&self) -> bool {
        self.client.logging_enabled()
    }

    #[pyo3(signature = (path_or_url, offset=None, limit=None))]
    pub fn fetch_page<'py>(
        &self,
        py: Python<'py>,
        path_or_url: String,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let params = request_params(offset, limit);

        future_into_py(py, async move {
            let response = client
                .get_value_from_absolute_or_relative_url_async(&path_or_url, params)
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let page = build_api_page(py, response, offset, limit)?;
                Ok(Py::new(py, page)?.into_any())
            })
        })
    }

    #[pyo3(signature = (format=None, offset=None, limit=None, from_date_time=None, to_date_time=None))]
    pub fn list_amendments<'py>(
        &self,
        py: Python<'py>,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
        from_date_time: Option<String>,
        to_date_time: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        if let Some(from) = from_date_time {
            params.insert("fromDateTime".to_string(), from);
        }
        if let Some(to) = to_date_time {
            params.insert("toDateTime".to_string(), to);
        }

        future_into_py(py, async move {
            let response: AmendmentsResponse = client
                .get_async("/amendment", Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.amendments {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (congress, format=None, offset=None, limit=None, from_date_time=None, to_date_time=None))]
    pub fn list_amendments_by_congress<'py>(
        &self,
        py: Python<'py>,
        congress: i32,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
        from_date_time: Option<String>,
        to_date_time: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        if let Some(from) = from_date_time {
            params.insert("fromDateTime".to_string(), from);
        }
        if let Some(to) = to_date_time {
            params.insert("toDateTime".to_string(), to);
        }
        let endpoint = format!("/amendment/{}", congress);

        future_into_py(py, async move {
            let response: AmendmentsResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.amendments {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (congress, amendment_type, format=None, offset=None, limit=None, from_date_time=None, to_date_time=None))]
    pub fn list_amendments_by_type<'py>(
        &self,
        py: Python<'py>,
        congress: i32,
        amendment_type: String,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
        from_date_time: Option<String>,
        to_date_time: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        if let Some(from) = from_date_time {
            params.insert("fromDateTime".to_string(), from);
        }
        if let Some(to) = to_date_time {
            params.insert("toDateTime".to_string(), to);
        }
        let endpoint = format!("/amendment/{}/{}", congress, amendment_type.to_lowercase());

        future_into_py(py, async move {
            let response: AmendmentsResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.amendments {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (congress, amendment_type, amendment_number, format=None))]
    pub fn get_amendment<'py>(
        &self,
        py: Python<'py>,
        congress: i32,
        amendment_type: String,
        amendment_number: String,
        format: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        let endpoint = format!(
            "/amendment/{}/{}/{}",
            congress,
            amendment_type.to_lowercase(),
            amendment_number
        );

        future_into_py(py, async move {
            let response: AmendmentDetailResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                Ok(Py::new(py, response.amendment)?.into_any())
            })
        })
    }

    #[pyo3(signature = (congress, amendment_type, amendment_number, format=None, offset=None, limit=None))]
    pub fn get_amendment_actions<'py>(
        &self,
        py: Python<'py>,
        congress: i32,
        amendment_type: String,
        amendment_number: String,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        let endpoint = format!(
            "/amendment/{}/{}/{}/actions",
            congress,
            amendment_type.to_lowercase(),
            amendment_number
        );

        future_into_py(py, async move {
            let response: ActionsResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.actions {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (congress, amendment_type, amendment_number, format=None, offset=None, limit=None))]
    pub fn get_amendment_amendments<'py>(
        &self,
        py: Python<'py>,
        congress: i32,
        amendment_type: String,
        amendment_number: String,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        let endpoint = format!(
            "/amendment/{}/{}/{}/amendments",
            congress,
            amendment_type.to_lowercase(),
            amendment_number
        );

        future_into_py(py, async move {
            let response: AmendmentsResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.amendments {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (congress, amendment_type, amendment_number, format=None, offset=None, limit=None))]
    pub fn get_amendment_cosponsors<'py>(
        &self,
        py: Python<'py>,
        congress: i32,
        amendment_type: String,
        amendment_number: String,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        let endpoint = format!(
            "/amendment/{}/{}/{}/cosponsors",
            congress,
            amendment_type.to_lowercase(),
            amendment_number
        );

        future_into_py(py, async move {
            let response: CosponsorsResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.cosponsors {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (congress, amendment_type, amendment_number, format=None, offset=None, limit=None))]
    pub fn get_amendment_text<'py>(
        &self,
        py: Python<'py>,
        congress: i32,
        amendment_type: String,
        amendment_number: String,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        let endpoint = format!(
            "/amendment/{}/{}/{}/text",
            congress,
            amendment_type.to_lowercase(),
            amendment_number
        );

        future_into_py(py, async move {
            let response: TextVersionsResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.text_versions {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (format=None, offset=None, limit=None, from_date_time=None, to_date_time=None))]
    pub fn list_bills<'py>(
        &self,
        py: Python<'py>,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
        from_date_time: Option<String>,
        to_date_time: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        if let Some(from) = from_date_time {
            params.insert("fromDateTime".to_string(), from);
        }
        if let Some(to) = to_date_time {
            params.insert("toDateTime".to_string(), to);
        }

        future_into_py(py, async move {
            let response: BillsResponse = client
                .get_async("/bill", Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.bills {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (congress, format=None, offset=None, limit=None, from_date_time=None, to_date_time=None))]
    pub fn list_bills_by_congress<'py>(
        &self,
        py: Python<'py>,
        congress: i32,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
        from_date_time: Option<String>,
        to_date_time: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        if let Some(from) = from_date_time {
            params.insert("fromDateTime".to_string(), from);
        }
        if let Some(to) = to_date_time {
            params.insert("toDateTime".to_string(), to);
        }
        let endpoint = format!("/bill/{}", congress);

        future_into_py(py, async move {
            let response: BillsResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.bills {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (congress, bill_type, format=None, offset=None, limit=None, from_date_time=None, to_date_time=None))]
    pub fn list_bills_by_type<'py>(
        &self,
        py: Python<'py>,
        congress: i32,
        bill_type: String,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
        from_date_time: Option<String>,
        to_date_time: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        if let Some(from) = from_date_time {
            params.insert("fromDateTime".to_string(), from);
        }
        if let Some(to) = to_date_time {
            params.insert("toDateTime".to_string(), to);
        }
        let endpoint = format!("/bill/{}/{}", congress, bill_type);

        future_into_py(py, async move {
            let response: BillsResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.bills {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (congress=None, bill_type=None, format=None, offset=None, limit=None, from_date_time=None, to_date_time=None))]
    pub fn get_bills<'py>(
        &self,
        py: Python<'py>,
        congress: Option<i32>,
        bill_type: Option<String>,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
        from_date_time: Option<String>,
        to_date_time: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        if let Some(from) = from_date_time {
            params.insert("fromDateTime".to_string(), from);
        }
        if let Some(to) = to_date_time {
            params.insert("toDateTime".to_string(), to);
        }
        let endpoint = match (congress, bill_type) {
            (Some(c), Some(bt)) => format!("/bill/{}/{}", c, bt),
            (Some(c), None) => format!("/bill/{}", c),
            (None, _) => "/bill".to_string(),
        };

        future_into_py(py, async move {
            let response: BillsResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.bills {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (congress, bill_type, bill_number, format=None, offset=None, limit=None, from_date_time=None, to_date_time=None))]
    pub fn get_bill<'py>(
        &self,
        py: Python<'py>,
        congress: i32,
        bill_type: String,
        bill_number: String,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
        from_date_time: Option<String>,
        to_date_time: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        if let Some(from) = from_date_time {
            params.insert("fromDateTime".to_string(), from);
        }
        if let Some(to) = to_date_time {
            params.insert("toDateTime".to_string(), to);
        }
        let endpoint = format!("/bill/{}/{}/{}", congress, bill_type, bill_number);

        future_into_py(py, async move {
            let response: BillDetailResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                Ok(Py::new(py, response.bill)?.into_any())
            })
        })
    }

    #[pyo3(signature = (congress, bill_type, bill_number, format=None, offset=None, limit=None, from_date_time=None, to_date_time=None))]
    pub fn get_bill_detail<'py>(
        &self,
        py: Python<'py>,
        congress: i32,
        bill_type: String,
        bill_number: String,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
        from_date_time: Option<String>,
        to_date_time: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        self.get_bill(
            py,
            congress,
            bill_type,
            bill_number,
            format,
            offset,
            limit,
            from_date_time,
            to_date_time,
        )
    }

    #[pyo3(signature = (congress, bill_type, bill_number, format=None, offset=None, limit=None))]
    pub fn get_bill_actions<'py>(
        &self,
        py: Python<'py>,
        congress: i32,
        bill_type: String,
        bill_number: String,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        let endpoint = format!("/bill/{}/{}/{}/actions", congress, bill_type, bill_number);

        future_into_py(py, async move {
            let response: ActionsResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.actions {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (congress, bill_type, bill_number, format=None, offset=None, limit=None))]
    pub fn get_bill_amendments<'py>(
        &self,
        py: Python<'py>,
        congress: i32,
        bill_type: String,
        bill_number: String,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        let endpoint = format!(
            "/bill/{}/{}/{}/amendments",
            congress, bill_type, bill_number
        );

        future_into_py(py, async move {
            let response: AmendmentsResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.amendments {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (congress, bill_type, bill_number, format=None, offset=None, limit=None))]
    pub fn get_bill_committees<'py>(
        &self,
        py: Python<'py>,
        congress: i32,
        bill_type: String,
        bill_number: String,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        let endpoint = format!(
            "/bill/{}/{}/{}/committees",
            congress, bill_type, bill_number
        );

        future_into_py(py, async move {
            let response: CommitteesResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.committees {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (congress, bill_type, bill_number, format=None, offset=None, limit=None))]
    pub fn get_bill_cosponsors<'py>(
        &self,
        py: Python<'py>,
        congress: i32,
        bill_type: String,
        bill_number: String,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        let endpoint = format!(
            "/bill/{}/{}/{}/cosponsors",
            congress, bill_type, bill_number
        );

        future_into_py(py, async move {
            let response: CosponsorsResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.cosponsors {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (congress, bill_type, bill_number, format=None, offset=None, limit=None))]
    pub fn get_related_bills<'py>(
        &self,
        py: Python<'py>,
        congress: i32,
        bill_type: String,
        bill_number: String,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        let endpoint = format!(
            "/bill/{}/{}/{}/relatedbills",
            congress, bill_type, bill_number
        );

        future_into_py(py, async move {
            let response: RelatedBillsResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;
            let related_bills = response.related_bills.unwrap_or_default();

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in related_bills {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (congress, bill_type, bill_number, format=None, offset=None, limit=None))]
    pub fn get_bill_subjects<'py>(
        &self,
        py: Python<'py>,
        congress: i32,
        bill_type: String,
        bill_number: String,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        let endpoint = format!("/bill/{}/{}/{}/subjects", congress, bill_type, bill_number);

        future_into_py(py, async move {
            let response: SubjectsResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;
            let subjects = response.legislative_subjects.unwrap_or_default();

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in subjects {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (congress, bill_type, bill_number, format=None, offset=None, limit=None))]
    pub fn get_bill_summaries<'py>(
        &self,
        py: Python<'py>,
        congress: i32,
        bill_type: String,
        bill_number: String,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        let endpoint = format!("/bill/{}/{}/{}/summaries", congress, bill_type, bill_number);

        future_into_py(py, async move {
            let response: SummariesResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.summaries {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (congress, bill_type, bill_number, format=None, offset=None, limit=None))]
    pub fn get_bill_text<'py>(
        &self,
        py: Python<'py>,
        congress: i32,
        bill_type: String,
        bill_number: String,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        let endpoint = format!("/bill/{}/{}/{}/text", congress, bill_type, bill_number);

        future_into_py(py, async move {
            let response: TextVersionsResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.text_versions {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (congress, bill_type, bill_number, format=None, offset=None, limit=None))]
    pub fn get_bill_titles<'py>(
        &self,
        py: Python<'py>,
        congress: i32,
        bill_type: String,
        bill_number: String,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        let endpoint = format!("/bill/{}/{}/{}/titles", congress, bill_type, bill_number);

        future_into_py(py, async move {
            let response: TitlesResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.titles {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (format=None, offset=None, limit=None))]
    pub fn list_congresses<'py>(
        &self,
        py: Python<'py>,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }

        future_into_py(py, async move {
            let response: CongressesResponse = client
                .get_async("/congress", Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.congresses {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (congress, format=None))]
    pub fn get_congress<'py>(
        &self,
        py: Python<'py>,
        congress: i32,
        format: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        let endpoint = format!("/congress/{}", congress);

        future_into_py(py, async move {
            let response: CongressResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                Ok(Py::new(py, response.congress)?.into_any())
            })
        })
    }

    #[pyo3(signature = (format=None))]
    pub fn get_current_congress<'py>(
        &self,
        py: Python<'py>,
        format: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }

        future_into_py(py, async move {
            let response: CongressResponse = client
                .get_async("/congress/current", Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                Ok(Py::new(py, response.congress)?.into_any())
            })
        })
    }

    #[pyo3(signature = (format=None, offset=None, limit=None, from_date_time=None, to_date_time=None, current_member=None))]
    pub fn list_members<'py>(
        &self,
        py: Python<'py>,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
        from_date_time: Option<String>,
        to_date_time: Option<String>,
        current_member: Option<bool>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        if let Some(from) = from_date_time {
            params.insert("fromDateTime".to_string(), from);
        }
        if let Some(to) = to_date_time {
            params.insert("toDateTime".to_string(), to);
        }
        if let Some(cm) = current_member {
            params.insert("currentMember".to_string(), cm.to_string());
        }

        future_into_py(py, async move {
            let response: MembersResponse = client
                .get_async("/member", Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.members {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    pub fn get_member<'py>(
        &self,
        py: Python<'py>,
        bioguide_id: String,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let endpoint = format!("/member/{}", bioguide_id);

        future_into_py(py, async move {
            let response: MemberResponse = client
                .get_async(&endpoint, None)
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                Ok(Py::new(py, response.member)?.into_any())
            })
        })
    }

    #[pyo3(signature = (congress, format=None, offset=None, limit=None, current_member=None))]
    pub fn list_members_by_congress<'py>(
        &self,
        py: Python<'py>,
        congress: i32,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
        current_member: Option<bool>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        if let Some(cm) = current_member {
            params.insert("currentMember".to_string(), cm.to_string());
        }
        let endpoint = format!("/member/congress/{}", congress);

        future_into_py(py, async move {
            let response: MembersResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.members {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (bioguide_id, format=None, offset=None, limit=None))]
    pub fn get_member_sponsored_legislation<'py>(
        &self,
        py: Python<'py>,
        bioguide_id: String,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        let endpoint = format!("/member/{}/sponsored-legislation", bioguide_id);

        future_into_py(py, async move {
            let response: SponsoredLegislationResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.sponsored_legislation {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (bioguide_id, format=None, offset=None, limit=None))]
    pub fn get_member_cosponsored_legislation<'py>(
        &self,
        py: Python<'py>,
        bioguide_id: String,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        let endpoint = format!("/member/{}/cosponsored-legislation", bioguide_id);

        future_into_py(py, async move {
            let response: CosponsoredLegislationResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.cosponsored_legislation {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (state_code, format=None, limit=None, current_member=None))]
    pub fn list_members_by_state<'py>(
        &self,
        py: Python<'py>,
        state_code: String,
        format: Option<String>,
        limit: Option<i32>,
        current_member: Option<bool>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        if let Some(cm) = current_member {
            params.insert("currentMember".to_string(), cm.to_string());
        }
        let endpoint = format!("/member/{}", state_code);

        future_into_py(py, async move {
            let response: MembersResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.members {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (state_code, district, format=None, current_member=None))]
    pub fn list_members_by_state_district<'py>(
        &self,
        py: Python<'py>,
        state_code: String,
        district: i32,
        format: Option<String>,
        current_member: Option<bool>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(cm) = current_member {
            params.insert("currentMember".to_string(), cm.to_string());
        }
        let endpoint = format!("/member/{}/{}", state_code, district);

        future_into_py(py, async move {
            let response: MembersResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.members {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (format=None, offset=None, limit=None))]
    pub fn list_committees<'py>(
        &self,
        py: Python<'py>,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }

        future_into_py(py, async move {
            let response: CommitteesListResponse = client
                .get_async("/committee", Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.committees {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (chamber, committee_code, format=None))]
    pub fn get_committee<'py>(
        &self,
        py: Python<'py>,
        chamber: String,
        committee_code: String,
        format: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        let endpoint = format!("/committee/{}/{}", chamber, committee_code);

        future_into_py(py, async move {
            let response: CommitteeDetailResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                Ok(Py::new(py, response.committee)?.into_any())
            })
        })
    }

    #[pyo3(signature = (chamber, offset=None, limit=None, format=None))]
    pub fn list_committees_by_chamber<'py>(
        &self,
        py: Python<'py>,
        chamber: String,
        offset: Option<i32>,
        limit: Option<i32>,
        format: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        let endpoint = format!("/committee/{}", chamber);

        future_into_py(py, async move {
            let response: CommitteesListResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.committees {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (congress, offset=None, limit=None, format=None))]
    pub fn list_committees_by_congress<'py>(
        &self,
        py: Python<'py>,
        congress: i32,
        offset: Option<i32>,
        limit: Option<i32>,
        format: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        let endpoint = format!("/committee/{}", congress);

        future_into_py(py, async move {
            let response: CommitteesListResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.committees {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (congress, chamber, offset=None, limit=None, format=None))]
    pub fn list_committees_by_congress_and_chamber<'py>(
        &self,
        py: Python<'py>,
        congress: i32,
        chamber: String,
        offset: Option<i32>,
        limit: Option<i32>,
        format: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        let endpoint = format!("/committee/{}/{}", congress, chamber);

        future_into_py(py, async move {
            let response: CommitteesListResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.committees {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (chamber, committee_code, offset=None, limit=None, format=None))]
    pub fn get_committee_bills<'py>(
        &self,
        py: Python<'py>,
        chamber: String,
        committee_code: String,
        offset: Option<i32>,
        limit: Option<i32>,
        format: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        let endpoint = format!("/committee/{}/{}/bills", chamber, committee_code);

        future_into_py(py, async move {
            let response: CommitteeBillsResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.bills {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (offset=None, limit=None, format=None))]
    pub fn list_laws<'py>(
        &self,
        py: Python<'py>,
        offset: Option<i32>,
        limit: Option<i32>,
        format: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }

        future_into_py(py, async move {
            let response: LawsResponse = client
                .get_async("/law", Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.bills {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (congress, offset=None, limit=None, format=None))]
    pub fn list_laws_by_congress<'py>(
        &self,
        py: Python<'py>,
        congress: i32,
        offset: Option<i32>,
        limit: Option<i32>,
        format: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        let endpoint = format!("/law/{}", congress);

        future_into_py(py, async move {
            let response: LawsResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.bills {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (congress, law_type, offset=None, limit=None, format=None))]
    pub fn list_laws_by_type<'py>(
        &self,
        py: Python<'py>,
        congress: i32,
        law_type: String,
        offset: Option<i32>,
        limit: Option<i32>,
        format: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        let endpoint = format!("/law/{}/{}", congress, law_type);

        future_into_py(py, async move {
            let response: LawsResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.bills {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (congress, law_type, law_number, format=None))]
    pub fn get_law<'py>(
        &self,
        py: Python<'py>,
        congress: i32,
        law_type: String,
        law_number: String,
        format: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        let endpoint = format!(
            "/law/{}/{}/{}",
            congress,
            law_type.to_lowercase(),
            law_number
        );

        future_into_py(py, async move {
            let response: LawDetailResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                Ok(Py::new(py, response.bill)?.into_any())
            })
        })
    }

    #[pyo3(signature = (offset=None, limit=None, sort=None, format=None))]
    pub fn list_hearings<'py>(
        &self,
        py: Python<'py>,
        offset: Option<i32>,
        limit: Option<i32>,
        sort: Option<String>,
        format: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(s) = sort {
            params.insert("sort".to_string(), s);
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }

        future_into_py(py, async move {
            let response: HearingsResponse = client
                .get_async("/hearing", Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.hearings {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (congress, chamber, jacket_number, format=None))]
    pub fn get_hearing<'py>(
        &self,
        py: Python<'py>,
        congress: i32,
        chamber: String,
        jacket_number: i32,
        format: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        let endpoint = format!(
            "/hearing/{}/{}/{}",
            congress,
            chamber.to_lowercase(),
            jacket_number
        );

        future_into_py(py, async move {
            let response: HearingDetailResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                Ok(Py::new(py, response.hearing)?.into_any())
            })
        })
    }

    #[pyo3(signature = (offset=None, limit=None, format=None))]
    pub fn list_summaries<'py>(
        &self,
        py: Python<'py>,
        offset: Option<i32>,
        limit: Option<i32>,
        format: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }

        future_into_py(py, async move {
            let response: SummariesListResponse = client
                .get_async("/summaries", Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.summaries {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (congress, offset=None, limit=None, format=None))]
    pub fn list_summaries_by_congress<'py>(
        &self,
        py: Python<'py>,
        congress: i32,
        offset: Option<i32>,
        limit: Option<i32>,
        format: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        let endpoint = format!("/summaries/{}", congress);

        future_into_py(py, async move {
            let response: SummariesListResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.summaries {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (congress, bill_type, offset=None, limit=None, format=None))]
    pub fn list_summaries_by_bill_type<'py>(
        &self,
        py: Python<'py>,
        congress: i32,
        bill_type: String,
        offset: Option<i32>,
        limit: Option<i32>,
        format: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        let endpoint = format!("/summaries/{}/{}", congress, bill_type.to_lowercase());

        future_into_py(py, async move {
            let response: SummariesListResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.summaries {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (offset=None, limit=None, from_date_time=None, to_date_time=None, format=None))]
    pub fn list_crs_reports<'py>(
        &self,
        py: Python<'py>,
        offset: Option<i32>,
        limit: Option<i32>,
        from_date_time: Option<String>,
        to_date_time: Option<String>,
        format: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(from) = from_date_time {
            params.insert("fromDateTime".to_string(), from);
        }
        if let Some(to) = to_date_time {
            params.insert("toDateTime".to_string(), to);
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }

        future_into_py(py, async move {
            let response: CrsReportsResponse = client
                .get_async("/crsreport", Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.crs_reports {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (report_number, format=None))]
    pub fn get_crs_report<'py>(
        &self,
        py: Python<'py>,
        report_number: String,
        format: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        let endpoint = format!("/crsreport/{}", report_number);

        future_into_py(py, async move {
            let response: CrsReportDetailResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                Ok(Py::new(py, response.report)?.into_any())
            })
        })
    }

    #[pyo3(signature = (offset=None, limit=None, from_date=None, to_date=None, sort=None, format=None))]
    pub fn list_house_votes<'py>(
        &self,
        py: Python<'py>,
        offset: Option<i32>,
        limit: Option<i32>,
        from_date: Option<String>,
        to_date: Option<String>,
        sort: Option<String>,
        format: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(from) = from_date {
            params.insert("fromDateTime".to_string(), from);
        }
        if let Some(to) = to_date {
            params.insert("toDateTime".to_string(), to);
        }
        if let Some(s) = sort {
            params.insert("sort".to_string(), s);
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        let endpoint = ("/house-vote").to_string();

        future_into_py(py, async move {
            let response: HouseVotesResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.votes {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (congress, offset=None, limit=None, from_date=None, to_date=None, sort=None, format=None))]
    pub fn list_house_votes_by_congress<'py>(
        &self,
        py: Python<'py>,
        congress: i32,
        offset: Option<i32>,
        limit: Option<i32>,
        from_date: Option<String>,
        to_date: Option<String>,
        sort: Option<String>,
        format: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(from) = from_date {
            params.insert("fromDateTime".to_string(), from);
        }
        if let Some(to) = to_date {
            params.insert("toDateTime".to_string(), to);
        }
        if let Some(s) = sort {
            params.insert("sort".to_string(), s);
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        let endpoint = (format!("/house-vote/{}", congress)).to_string();

        future_into_py(py, async move {
            let response: HouseVotesResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.votes {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (congress, session, offset=None, limit=None, from_date=None, to_date=None, sort=None, format=None))]
    pub fn list_house_votes_by_session<'py>(
        &self,
        py: Python<'py>,
        congress: i32,
        session: i32,
        offset: Option<i32>,
        limit: Option<i32>,
        from_date: Option<String>,
        to_date: Option<String>,
        sort: Option<String>,
        format: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(from) = from_date {
            params.insert("fromDateTime".to_string(), from);
        }
        if let Some(to) = to_date {
            params.insert("toDateTime".to_string(), to);
        }
        if let Some(s) = sort {
            params.insert("sort".to_string(), s);
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        let endpoint = (format!("/house-vote/{}/{}", congress, session)).to_string();

        future_into_py(py, async move {
            let response: HouseVotesResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.votes {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (congress, session, vote_number, format=None))]
    pub fn get_house_vote<'py>(
        &self,
        py: Python<'py>,
        congress: i32,
        session: i32,
        vote_number: i32,
        format: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        let endpoint =
            (format!("/house-vote/{}/{}/{}", congress, session, vote_number)).to_string();

        future_into_py(py, async move {
            let response: HouseVoteDetailResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                Ok(Py::new(py, response.vote)?.into_any())
            })
        })
    }

    #[pyo3(signature = (congress, session, vote_number, offset=None, limit=None, format=None))]
    pub fn get_house_vote_members<'py>(
        &self,
        py: Python<'py>,
        congress: i32,
        session: i32,
        vote_number: i32,
        offset: Option<i32>,
        limit: Option<i32>,
        format: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        let endpoint = (format!(
            "/house-vote/{}/{}/{}/members",
            congress, session, vote_number
        ))
        .to_string();

        future_into_py(py, async move {
            let response: HouseVoteMembersResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                Ok(Py::new(py, response.vote)?.into_any())
            })
        })
    }

    #[pyo3(signature = (offset=None, limit=None, from_date=None, to_date=None, sort=None, format=None))]
    pub fn list_committee_reports<'py>(
        &self,
        py: Python<'py>,
        offset: Option<i32>,
        limit: Option<i32>,
        from_date: Option<String>,
        to_date: Option<String>,
        sort: Option<String>,
        format: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(from) = from_date {
            params.insert("fromDateTime".to_string(), from);
        }
        if let Some(to) = to_date {
            params.insert("toDateTime".to_string(), to);
        }
        if let Some(s) = sort {
            params.insert("sort".to_string(), s);
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        let endpoint = ("/committee-report").to_string();

        future_into_py(py, async move {
            let response: CommitteeReportsResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.reports {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (congress, offset=None, limit=None, from_date=None, to_date=None, sort=None, format=None))]
    pub fn list_committee_reports_by_congress<'py>(
        &self,
        py: Python<'py>,
        congress: i32,
        offset: Option<i32>,
        limit: Option<i32>,
        from_date: Option<String>,
        to_date: Option<String>,
        sort: Option<String>,
        format: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(from) = from_date {
            params.insert("fromDateTime".to_string(), from);
        }
        if let Some(to) = to_date {
            params.insert("toDateTime".to_string(), to);
        }
        if let Some(s) = sort {
            params.insert("sort".to_string(), s);
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        let endpoint = (format!("/committee-report/{}", congress)).to_string();

        future_into_py(py, async move {
            let response: CommitteeReportsResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.reports {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (congress, report_type, offset=None, limit=None, from_date=None, to_date=None, sort=None, format=None))]
    pub fn list_committee_reports_by_type<'py>(
        &self,
        py: Python<'py>,
        congress: i32,
        report_type: String,
        offset: Option<i32>,
        limit: Option<i32>,
        from_date: Option<String>,
        to_date: Option<String>,
        sort: Option<String>,
        format: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(from) = from_date {
            params.insert("fromDateTime".to_string(), from);
        }
        if let Some(to) = to_date {
            params.insert("toDateTime".to_string(), to);
        }
        if let Some(s) = sort {
            params.insert("sort".to_string(), s);
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        let endpoint = (format!("/committee-report/{}/{}", congress, report_type)).to_string();

        future_into_py(py, async move {
            let response: CommitteeReportsResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.reports {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (congress, report_type, report_number, format=None))]
    pub fn get_committee_report<'py>(
        &self,
        py: Python<'py>,
        congress: i32,
        report_type: String,
        report_number: i32,
        format: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        let endpoint = (format!(
            "/committee-report/{}/{}/{}",
            congress, report_type, report_number
        ))
        .to_string();

        future_into_py(py, async move {
            let response: CommitteeReportDetailResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                Ok(Py::new(py, response.report)?.into_any())
            })
        })
    }

    #[pyo3(signature = (congress, report_type, report_number, format=None))]
    pub fn get_committee_report_text<'py>(
        &self,
        py: Python<'py>,
        congress: i32,
        report_type: String,
        report_number: i32,
        format: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        let endpoint = (format!(
            "/committee-report/{}/{}/{}/text",
            congress, report_type, report_number
        ))
        .to_string();

        future_into_py(py, async move {
            let response: CommitteeReportTextResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.text {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (offset=None, limit=None, from_date=None, to_date=None, sort=None, format=None))]
    pub fn list_committee_prints<'py>(
        &self,
        py: Python<'py>,
        offset: Option<i32>,
        limit: Option<i32>,
        from_date: Option<String>,
        to_date: Option<String>,
        sort: Option<String>,
        format: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(from) = from_date {
            params.insert("fromDateTime".to_string(), from);
        }
        if let Some(to) = to_date {
            params.insert("toDateTime".to_string(), to);
        }
        if let Some(s) = sort {
            params.insert("sort".to_string(), s);
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        let endpoint = ("/committee-print").to_string();

        future_into_py(py, async move {
            let response: CommitteePrintsResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.committee_prints {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (congress, offset=None, limit=None, from_date=None, to_date=None, sort=None, format=None))]
    pub fn list_committee_prints_by_congress<'py>(
        &self,
        py: Python<'py>,
        congress: i32,
        offset: Option<i32>,
        limit: Option<i32>,
        from_date: Option<String>,
        to_date: Option<String>,
        sort: Option<String>,
        format: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(from) = from_date {
            params.insert("fromDateTime".to_string(), from);
        }
        if let Some(to) = to_date {
            params.insert("toDateTime".to_string(), to);
        }
        if let Some(s) = sort {
            params.insert("sort".to_string(), s);
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        let endpoint = (format!("/committee-print/{}", congress)).to_string();

        future_into_py(py, async move {
            let response: CommitteePrintsResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.committee_prints {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (congress, chamber, offset=None, limit=None, from_date=None, to_date=None, sort=None, format=None))]
    pub fn list_committee_prints_by_chamber<'py>(
        &self,
        py: Python<'py>,
        congress: i32,
        chamber: String,
        offset: Option<i32>,
        limit: Option<i32>,
        from_date: Option<String>,
        to_date: Option<String>,
        sort: Option<String>,
        format: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(from) = from_date {
            params.insert("fromDateTime".to_string(), from);
        }
        if let Some(to) = to_date {
            params.insert("toDateTime".to_string(), to);
        }
        if let Some(s) = sort {
            params.insert("sort".to_string(), s);
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        let endpoint = (format!("/committee-print/{}/{}", congress, chamber)).to_string();

        future_into_py(py, async move {
            let response: CommitteePrintsResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.committee_prints {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (congress, chamber, jacket_number, format=None))]
    pub fn get_committee_print<'py>(
        &self,
        py: Python<'py>,
        congress: i32,
        chamber: String,
        jacket_number: i32,
        format: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        let endpoint = (format!(
            "/committee-print/{}/{}/{}",
            congress, chamber, jacket_number
        ))
        .to_string();

        future_into_py(py, async move {
            let response: CommitteePrintDetailResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                Ok(Py::new(py, response.committee_print)?.into_any())
            })
        })
    }

    #[pyo3(signature = (congress, chamber, jacket_number, format=None))]
    pub fn get_committee_print_text<'py>(
        &self,
        py: Python<'py>,
        congress: i32,
        chamber: String,
        jacket_number: i32,
        format: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        let endpoint = (format!(
            "/committee-print/{}/{}/{}/text",
            congress, chamber, jacket_number
        ))
        .to_string();

        future_into_py(py, async move {
            let response: CommitteePrintTextResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.text {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (offset=None, limit=None, sort=None, format=None))]
    pub fn list_nominations<'py>(
        &self,
        py: Python<'py>,
        offset: Option<i32>,
        limit: Option<i32>,
        sort: Option<String>,
        format: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(s) = sort {
            params.insert("sort".to_string(), s);
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        let endpoint = ("/nomination").to_string();

        future_into_py(py, async move {
            let response: NominationsResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.nominations {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (congress, offset=None, limit=None, sort=None, format=None))]
    pub fn list_nominations_by_congress<'py>(
        &self,
        py: Python<'py>,
        congress: i32,
        offset: Option<i32>,
        limit: Option<i32>,
        sort: Option<String>,
        format: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(s) = sort {
            params.insert("sort".to_string(), s);
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        let endpoint = (format!("/nomination/{}", congress)).to_string();

        future_into_py(py, async move {
            let response: NominationsResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.nominations {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (congress, nomination_number, format=None))]
    pub fn get_nomination<'py>(
        &self,
        py: Python<'py>,
        congress: i32,
        nomination_number: String,
        format: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        let endpoint = (format!("/nomination/{}/{}", congress, nomination_number)).to_string();

        future_into_py(py, async move {
            let response: NominationDetailResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                Ok(Py::new(py, response.nomination)?.into_any())
            })
        })
    }

    #[pyo3(signature = (congress, nomination_number, offset=None, limit=None, format=None))]
    pub fn get_nomination_nominees<'py>(
        &self,
        py: Python<'py>,
        congress: i32,
        nomination_number: String,
        offset: Option<i32>,
        limit: Option<i32>,
        format: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        let endpoint =
            (format!("/nomination/{}/{}/nominees", congress, nomination_number)).to_string();

        future_into_py(py, async move {
            let response: NomineesResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.nominees {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (offset=None, limit=None, sort=None, format=None))]
    pub fn list_treaties<'py>(
        &self,
        py: Python<'py>,
        offset: Option<i32>,
        limit: Option<i32>,
        sort: Option<String>,
        format: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(s) = sort {
            params.insert("sort".to_string(), s);
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        let endpoint = ("/treaty").to_string();

        future_into_py(py, async move {
            let response: TreatiesResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.treaties {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (congress, offset=None, limit=None, sort=None, format=None))]
    pub fn list_treaties_by_congress<'py>(
        &self,
        py: Python<'py>,
        congress: i32,
        offset: Option<i32>,
        limit: Option<i32>,
        sort: Option<String>,
        format: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(s) = sort {
            params.insert("sort".to_string(), s);
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        let endpoint = (format!("/treaty/{}", congress)).to_string();

        future_into_py(py, async move {
            let response: TreatiesResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.treaties {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (congress, treaty_number, format=None))]
    pub fn get_treaty<'py>(
        &self,
        py: Python<'py>,
        congress: i32,
        treaty_number: String,
        format: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        let endpoint = (format!("/treaty/{}/{}", congress, treaty_number)).to_string();

        future_into_py(py, async move {
            let response: TreatyDetailResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                Ok(Py::new(py, response.treaty)?.into_any())
            })
        })
    }

    #[pyo3(signature = (congress, offset=None, limit=None, sort=None, format=None))]
    pub fn list_hearings_by_congress<'py>(
        &self,
        py: Python<'py>,
        congress: i32,
        offset: Option<i32>,
        limit: Option<i32>,
        sort: Option<String>,
        format: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(s) = sort {
            params.insert("sort".to_string(), s);
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        let endpoint = (format!("/hearing/{}", congress)).to_string();

        future_into_py(py, async move {
            let response: HearingsResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.hearings {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (congress, chamber, offset=None, limit=None, format=None))]
    pub fn list_hearings_by_chamber<'py>(
        &self,
        py: Python<'py>,
        congress: i32,
        chamber: String,
        offset: Option<i32>,
        limit: Option<i32>,
        format: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        let endpoint = (format!("/hearing/{}/{}", congress, chamber.to_lowercase())).to_string();

        future_into_py(py, async move {
            let response: HearingsResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.hearings {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (offset=None, limit=None, format=None))]
    pub fn list_congressional_records<'py>(
        &self,
        py: Python<'py>,
        offset: Option<i32>,
        limit: Option<i32>,
        format: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(off) = offset {
            params.insert("offset".to_string(), off.to_string());
        }
        if let Some(lim) = limit {
            params.insert("limit".to_string(), lim.to_string());
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        let endpoint = ("/daily-congressional-record").to_string();

        future_into_py(py, async move {
            let response: DailyCongressionalRecordsResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.daily_congressional_record {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (congress, state_code, district, format=None, offset=None, limit=None, current_member=None))]
    pub fn list_members_by_congress_state_district<'py>(
        &self,
        py: Python<'py>,
        congress: i32,
        state_code: String,
        district: i32,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
        current_member: Option<bool>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        if let Some(cm) = current_member {
            params.insert("currentMember".to_string(), cm.to_string());
        }
        let endpoint =
            (format!("/member/congress/{}/{}/{}", congress, state_code, district)).to_string();

        future_into_py(py, async move {
            let response: MembersResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.members {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (congress, chamber, committee_code, format=None))]
    pub fn get_committee_by_congress<'py>(
        &self,
        py: Python<'py>,
        congress: i32,
        chamber: String,
        committee_code: String,
        format: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        let endpoint = (format!(
            "/committee/{}/{}/{}",
            congress,
            chamber.to_lowercase(),
            committee_code
        ))
        .to_string();

        future_into_py(py, async move {
            let response: CommitteeDetailResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                Ok(Py::new(py, response.committee)?.into_any())
            })
        })
    }

    #[pyo3(signature = (chamber, committee_code, format=None, offset=None, limit=None))]
    pub fn get_committee_house_communications<'py>(
        &self,
        py: Python<'py>,
        chamber: String,
        committee_code: String,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        let endpoint = (format!(
            "/committee/{}/{}/house-communication",
            chamber.to_lowercase(),
            committee_code
        ))
        .to_string();

        future_into_py(py, async move {
            let response: HouseCommunicationsResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.house_communications {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (chamber, committee_code, format=None, offset=None, limit=None))]
    pub fn get_committee_senate_communications<'py>(
        &self,
        py: Python<'py>,
        chamber: String,
        committee_code: String,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        let endpoint = (format!(
            "/committee/{}/{}/senate-communication",
            chamber.to_lowercase(),
            committee_code
        ))
        .to_string();

        future_into_py(py, async move {
            let response: SenateCommunicationsResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.senate_communications {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (chamber, committee_code, format=None, offset=None, limit=None))]
    pub fn get_committee_nominations<'py>(
        &self,
        py: Python<'py>,
        chamber: String,
        committee_code: String,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        let endpoint = (format!(
            "/committee/{}/{}/nominations",
            chamber.to_lowercase(),
            committee_code
        ))
        .to_string();

        future_into_py(py, async move {
            let response: NominationsResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.nominations {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (chamber, committee_code, format=None, offset=None, limit=None))]
    pub fn get_committee_reports<'py>(
        &self,
        py: Python<'py>,
        chamber: String,
        committee_code: String,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        let endpoint = (format!(
            "/committee/{}/{}/reports",
            chamber.to_lowercase(),
            committee_code
        ))
        .to_string();

        future_into_py(py, async move {
            let response: CommitteeReportsResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.reports {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (offset=None, limit=None, from_date_time=None, to_date_time=None, format=None))]
    pub fn list_committee_meetings<'py>(
        &self,
        py: Python<'py>,
        offset: Option<i32>,
        limit: Option<i32>,
        from_date_time: Option<String>,
        to_date_time: Option<String>,
        format: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        if let Some(from) = from_date_time {
            params.insert("fromDateTime".to_string(), from);
        }
        if let Some(to) = to_date_time {
            params.insert("toDateTime".to_string(), to);
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        let endpoint = ("/committee-meeting").to_string();

        future_into_py(py, async move {
            let response: CommitteeMeetingsResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.committee_meetings {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (congress, offset=None, limit=None, from_date_time=None, to_date_time=None, format=None))]
    pub fn list_committee_meetings_by_congress<'py>(
        &self,
        py: Python<'py>,
        congress: i32,
        offset: Option<i32>,
        limit: Option<i32>,
        from_date_time: Option<String>,
        to_date_time: Option<String>,
        format: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        if let Some(from) = from_date_time {
            params.insert("fromDateTime".to_string(), from);
        }
        if let Some(to) = to_date_time {
            params.insert("toDateTime".to_string(), to);
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        let endpoint = (format!("/committee-meeting/{}", congress)).to_string();

        future_into_py(py, async move {
            let response: CommitteeMeetingsResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.committee_meetings {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (congress, chamber, offset=None, limit=None, from_date_time=None, to_date_time=None, format=None))]
    pub fn list_committee_meetings_by_chamber<'py>(
        &self,
        py: Python<'py>,
        congress: i32,
        chamber: String,
        offset: Option<i32>,
        limit: Option<i32>,
        from_date_time: Option<String>,
        to_date_time: Option<String>,
        format: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        if let Some(from) = from_date_time {
            params.insert("fromDateTime".to_string(), from);
        }
        if let Some(to) = to_date_time {
            params.insert("toDateTime".to_string(), to);
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        let endpoint =
            (format!("/committee-meeting/{}/{}", congress, chamber.to_lowercase())).to_string();

        future_into_py(py, async move {
            let response: CommitteeMeetingsResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.committee_meetings {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (congress, chamber, event_id, format=None))]
    pub fn get_committee_meeting<'py>(
        &self,
        py: Python<'py>,
        congress: i32,
        chamber: String,
        event_id: String,
        format: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        let endpoint = (format!(
            "/committee-meeting/{}/{}/{}",
            congress,
            chamber.to_lowercase(),
            event_id
        ))
        .to_string();

        future_into_py(py, async move {
            let response: CommitteeMeetingDetailResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                Ok(Py::new(py, response.committee_meeting)?.into_any())
            })
        })
    }

    #[pyo3(signature = (offset=None, limit=None, format=None))]
    pub fn list_bound_congressional_records<'py>(
        &self,
        py: Python<'py>,
        offset: Option<i32>,
        limit: Option<i32>,
        format: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        let endpoint = ("/bound-congressional-record").to_string();

        future_into_py(py, async move {
            let response: BoundCongressionalRecordsResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.bound_congressional_record {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (year, offset=None, limit=None, format=None))]
    pub fn list_bound_congressional_records_by_year<'py>(
        &self,
        py: Python<'py>,
        year: i32,
        offset: Option<i32>,
        limit: Option<i32>,
        format: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        let endpoint = (format!("/bound-congressional-record/{}", year)).to_string();

        future_into_py(py, async move {
            let response: BoundCongressionalRecordsResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.bound_congressional_record {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (year, month, offset=None, limit=None, format=None))]
    pub fn list_bound_congressional_records_by_month<'py>(
        &self,
        py: Python<'py>,
        year: i32,
        month: i32,
        offset: Option<i32>,
        limit: Option<i32>,
        format: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        let endpoint = (format!("/bound-congressional-record/{}/{}", year, month)).to_string();

        future_into_py(py, async move {
            let response: BoundCongressionalRecordsResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.bound_congressional_record {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (year, month, day, offset=None, limit=None, format=None))]
    pub fn get_bound_congressional_record<'py>(
        &self,
        py: Python<'py>,
        year: i32,
        month: i32,
        day: i32,
        offset: Option<i32>,
        limit: Option<i32>,
        format: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        let endpoint =
            (format!("/bound-congressional-record/{}/{}/{}", year, month, day)).to_string();

        future_into_py(py, async move {
            let response: BoundCongressionalRecordsResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.bound_congressional_record {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (offset=None, limit=None, format=None))]
    pub fn list_congressional_record<'py>(
        &self,
        py: Python<'py>,
        offset: Option<i32>,
        limit: Option<i32>,
        format: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        let endpoint = ("/congressional-record").to_string();

        future_into_py(py, async move {
            let response: CongressionalRecordResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                Ok(Py::new(py, response.results)?.into_any())
            })
        })
    }

    #[pyo3(signature = (volume_number, offset=None, limit=None, format=None))]
    pub fn list_daily_congressional_records_by_volume<'py>(
        &self,
        py: Python<'py>,
        volume_number: i32,
        offset: Option<i32>,
        limit: Option<i32>,
        format: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        let endpoint = (format!("/daily-congressional-record/{}", volume_number)).to_string();

        future_into_py(py, async move {
            let response: DailyCongressionalRecordsResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.daily_congressional_record {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (volume_number, issue_number, format=None))]
    pub fn get_daily_congressional_record_issue<'py>(
        &self,
        py: Python<'py>,
        volume_number: i32,
        issue_number: String,
        format: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        let endpoint = (format!(
            "/daily-congressional-record/{}/{}",
            volume_number, issue_number
        ))
        .to_string();

        future_into_py(py, async move {
            let response: DailyCongressionalRecordIssueResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                Ok(Py::new(py, response.issue)?.into_any())
            })
        })
    }

    #[pyo3(signature = (volume_number, issue_number, offset=None, limit=None, format=None))]
    pub fn get_daily_congressional_record_articles<'py>(
        &self,
        py: Python<'py>,
        volume_number: i32,
        issue_number: String,
        offset: Option<i32>,
        limit: Option<i32>,
        format: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        let endpoint = (format!(
            "/daily-congressional-record/{}/{}/articles",
            volume_number, issue_number
        ))
        .to_string();

        future_into_py(py, async move {
            let response: DailyCongressionalRecordArticlesResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.articles {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (offset=None, limit=None, format=None))]
    pub fn list_house_communications<'py>(
        &self,
        py: Python<'py>,
        offset: Option<i32>,
        limit: Option<i32>,
        format: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        let endpoint = ("/house-communication").to_string();

        future_into_py(py, async move {
            let response: HouseCommunicationsResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.house_communications {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (congress, offset=None, limit=None, format=None))]
    pub fn list_house_communications_by_congress<'py>(
        &self,
        py: Python<'py>,
        congress: i32,
        offset: Option<i32>,
        limit: Option<i32>,
        format: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        let endpoint = (format!("/house-communication/{}", congress)).to_string();

        future_into_py(py, async move {
            let response: HouseCommunicationsResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.house_communications {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (congress, communication_type, offset=None, limit=None, format=None))]
    pub fn list_house_communications_by_type<'py>(
        &self,
        py: Python<'py>,
        congress: i32,
        communication_type: String,
        offset: Option<i32>,
        limit: Option<i32>,
        format: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        let endpoint = (format!(
            "/house-communication/{}/{}",
            congress,
            communication_type.to_lowercase()
        ))
        .to_string();

        future_into_py(py, async move {
            let response: HouseCommunicationsResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.house_communications {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (congress, communication_type, communication_number, format=None))]
    pub fn get_house_communication<'py>(
        &self,
        py: Python<'py>,
        congress: i32,
        communication_type: String,
        communication_number: i32,
        format: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        let endpoint = (format!(
            "/house-communication/{}/{}/{}",
            congress,
            communication_type.to_lowercase(),
            communication_number
        ))
        .to_string();

        future_into_py(py, async move {
            let response: HouseCommunicationDetailResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                Ok(Py::new(py, response.house_communication)?.into_any())
            })
        })
    }

    #[pyo3(signature = (offset=None, limit=None, format=None))]
    pub fn list_senate_communications<'py>(
        &self,
        py: Python<'py>,
        offset: Option<i32>,
        limit: Option<i32>,
        format: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        let endpoint = ("/senate-communication").to_string();

        future_into_py(py, async move {
            let response: SenateCommunicationsResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.senate_communications {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (congress, offset=None, limit=None, format=None))]
    pub fn list_senate_communications_by_congress<'py>(
        &self,
        py: Python<'py>,
        congress: i32,
        offset: Option<i32>,
        limit: Option<i32>,
        format: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        let endpoint = (format!("/senate-communication/{}", congress)).to_string();

        future_into_py(py, async move {
            let response: SenateCommunicationsResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.senate_communications {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (congress, communication_type, offset=None, limit=None, format=None))]
    pub fn list_senate_communications_by_type<'py>(
        &self,
        py: Python<'py>,
        congress: i32,
        communication_type: String,
        offset: Option<i32>,
        limit: Option<i32>,
        format: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        let endpoint = (format!(
            "/senate-communication/{}/{}",
            congress,
            communication_type.to_lowercase()
        ))
        .to_string();

        future_into_py(py, async move {
            let response: SenateCommunicationsResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.senate_communications {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (congress, communication_type, communication_number, format=None))]
    pub fn get_senate_communication<'py>(
        &self,
        py: Python<'py>,
        congress: i32,
        communication_type: String,
        communication_number: i32,
        format: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        let endpoint = (format!(
            "/senate-communication/{}/{}/{}",
            congress,
            communication_type.to_lowercase(),
            communication_number
        ))
        .to_string();

        future_into_py(py, async move {
            let response: SenateCommunicationDetailResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                Ok(Py::new(py, response.senate_communication)?.into_any())
            })
        })
    }

    #[pyo3(signature = (offset=None, limit=None, format=None))]
    pub fn list_house_requirements<'py>(
        &self,
        py: Python<'py>,
        offset: Option<i32>,
        limit: Option<i32>,
        format: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        let endpoint = ("/house-requirement").to_string();

        future_into_py(py, async move {
            let response: HouseRequirementsResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.house_requirements {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (requirement_number, format=None))]
    pub fn get_house_requirement<'py>(
        &self,
        py: Python<'py>,
        requirement_number: i32,
        format: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        let endpoint = (format!("/house-requirement/{}", requirement_number)).to_string();

        future_into_py(py, async move {
            let response: HouseRequirementDetailResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                Ok(Py::new(py, response.house_requirement)?.into_any())
            })
        })
    }

    #[pyo3(signature = (requirement_number, offset=None, limit=None, format=None))]
    pub fn get_house_requirement_matching_communications<'py>(
        &self,
        py: Python<'py>,
        requirement_number: i32,
        offset: Option<i32>,
        limit: Option<i32>,
        format: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        let endpoint = (format!(
            "/house-requirement/{}/matching-communications",
            requirement_number
        ))
        .to_string();

        future_into_py(py, async move {
            let response: MatchingCommunicationsResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.matching_communications {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (congress, nomination_number, format=None, offset=None, limit=None))]
    pub fn get_nomination_actions<'py>(
        &self,
        py: Python<'py>,
        congress: i32,
        nomination_number: String,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        let endpoint =
            (format!("/nomination/{}/{}/actions", congress, nomination_number)).to_string();

        future_into_py(py, async move {
            let response: ActionsResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.actions {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (congress, nomination_number, format=None))]
    pub fn get_nomination_committees<'py>(
        &self,
        py: Python<'py>,
        congress: i32,
        nomination_number: String,
        format: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        let endpoint =
            (format!("/nomination/{}/{}/committees", congress, nomination_number)).to_string();

        future_into_py(py, async move {
            let response: NominationCommitteesResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.committees {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (congress, nomination_number, format=None, offset=None, limit=None))]
    pub fn get_nomination_hearings<'py>(
        &self,
        py: Python<'py>,
        congress: i32,
        nomination_number: String,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        let endpoint =
            (format!("/nomination/{}/{}/hearings", congress, nomination_number)).to_string();

        future_into_py(py, async move {
            let response: NominationHearingsResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.hearings {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (congress, nomination_number, ordinal, format=None, offset=None, limit=None))]
    pub fn get_nomination_ordinal<'py>(
        &self,
        py: Python<'py>,
        congress: i32,
        nomination_number: String,
        ordinal: String,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        let endpoint =
            (format!("/nomination/{}/{}/{}", congress, nomination_number, ordinal)).to_string();

        future_into_py(py, async move {
            let response: NomineesResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.nominees {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (congress, treaty_number, format=None, offset=None, limit=None))]
    pub fn get_treaty_actions<'py>(
        &self,
        py: Python<'py>,
        congress: i32,
        treaty_number: String,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        let endpoint = (format!("/treaty/{}/{}/actions", congress, treaty_number)).to_string();

        future_into_py(py, async move {
            let response: ActionsResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.actions {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (congress, treaty_number, format=None))]
    pub fn get_treaty_committees<'py>(
        &self,
        py: Python<'py>,
        congress: i32,
        treaty_number: String,
        format: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        let endpoint = (format!("/treaty/{}/{}/committees", congress, treaty_number)).to_string();

        future_into_py(py, async move {
            let response: TreatyCommitteesResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.treaty_committees {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (congress, treaty_number, treaty_suffix, format=None))]
    pub fn get_treaty_part<'py>(
        &self,
        py: Python<'py>,
        congress: i32,
        treaty_number: String,
        treaty_suffix: String,
        format: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        let endpoint =
            (format!("/treaty/{}/{}/{}", congress, treaty_number, treaty_suffix)).to_string();

        future_into_py(py, async move {
            let response: TreatyPartDetailResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.treaty {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }

    #[pyo3(signature = (congress, treaty_number, treaty_suffix, format=None, offset=None, limit=None))]
    pub fn get_treaty_part_actions<'py>(
        &self,
        py: Python<'py>,
        congress: i32,
        treaty_number: String,
        treaty_suffix: String,
        format: Option<String>,
        offset: Option<i32>,
        limit: Option<i32>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let client = self.client.clone();
        let mut params = HashMap::new();
        if let Some(f) = format {
            params.insert("format".to_string(), f);
        }
        if let Some(o) = offset {
            params.insert("offset".to_string(), o.to_string());
        }
        if let Some(l) = limit {
            params.insert("limit".to_string(), l.to_string());
        }
        let endpoint = (format!(
            "/treaty/{}/{}/{}/actions",
            congress, treaty_number, treaty_suffix
        ))
        .to_string();

        future_into_py(py, async move {
            let response: ActionsResponse = client
                .get_async(&endpoint, Some(params))
                .await
                .map_err(api_py_err)?;

            Python::with_gil(|py| -> PyResult<Py<PyAny>> {
                let list = PyList::empty(py);
                for item in response.actions {
                    list.append(Py::new(py, item)?)?;
                }
                Ok(list.unbind().into_any())
            })
        })
    }
}

fn api_py_err(error: ApiError) -> PyErr {
    match error {
        ApiError::RequestFailed(error) => {
            PyErr::new::<crate::CDGRequestError, _>(error.to_string())
        }
        ApiError::HttpError {
            status_code,
            url,
            message,
        } => {
            let full_message = if let Some(url) = url {
                format!("{} ({})", message, url)
            } else {
                message
            };

            match status_code {
                401 | 403 => PyErr::new::<crate::CDGAuthError, _>(full_message),
                404 => PyErr::new::<crate::CDGNotFoundError, _>(full_message),
                429 => PyErr::new::<crate::CDGRateLimitError, _>(full_message),
                500..=599 => PyErr::new::<crate::CDGServerError, _>(full_message),
                _ => PyErr::new::<crate::CDGHttpError, _>(full_message),
            }
        }
        ApiError::DeserializationError {
            context,
            message,
            response_preview,
        } => PyErr::new::<crate::CDGDeserializationError, _>(format!(
            "Failed to deserialize response from {}: {}\nResponse preview: {}",
            context, message, response_preview
        )),
        ApiError::InvalidUrl(message) => PyErr::new::<crate::CDGInvalidUrlError, _>(message),
        ApiError::ConfigurationError(message) => {
            PyErr::new::<crate::CDGConfigurationError, _>(message)
        }
        ApiError::ApiError(message) => PyErr::new::<crate::CDGClientError, _>(message),
        ApiError::MissingApiKey => PyErr::new::<crate::CDGConfigurationError, _>("Missing API key"),
    }
}

fn build_api_page(
    py: Python<'_>,
    response: Value,
    requested_offset: Option<i32>,
    requested_limit: Option<i32>,
) -> PyResult<ApiPage> {
    let pagination = response
        .get("pagination")
        .and_then(|value| value.as_object());

    let count = pagination
        .and_then(|value| value.get("count"))
        .and_then(|value| value.as_i64())
        .and_then(|value| i32::try_from(value).ok());

    let next_url = pagination
        .and_then(|value| value.get("next"))
        .and_then(|value| value.as_str())
        .map(|value| value.to_string());

    let previous_url = pagination
        .and_then(|value| value.get("previous"))
        .and_then(|value| value.as_str())
        .map(|value| value.to_string());

    let (item_key, items) = extract_items(&response);

    Ok(ApiPage {
        items: json_value_to_py(py, &items)?,
        raw_response: json_value_to_py(py, &response)?,
        item_key,
        count,
        next_url,
        previous_url,
        offset: requested_offset,
        limit: requested_limit,
    })
}

fn extract_items(response: &Value) -> (Option<String>, Value) {
    let Some(object) = response.as_object() else {
        return (None, Value::Array(Vec::new()));
    };

    let array_fields: Vec<(&String, &Value)> = object
        .iter()
        .filter(|(key, value)| *key != "pagination" && *key != "request" && value.is_array())
        .collect();

    if array_fields.len() == 1 {
        let (key, value) = array_fields[0];
        return (Some(key.clone()), value.clone());
    }

    for preferred_key in [
        "bills",
        "amendments",
        "members",
        "committees",
        "reports",
        "committeePrints",
        "nominations",
        "treaties",
        "hearings",
        "congresses",
        "congressionalRecord",
        "dailyCongressionalRecord",
        "boundCongressionalRecord",
        "houseVotes",
        "houseCommunications",
        "senateCommunications",
        "houseRequirements",
        "summaries",
        "crsReports",
        "actions",
        "titles",
        "textVersions",
        "articles",
        "nominees",
    ] {
        if let Some(value) = object.get(preferred_key).filter(|value| value.is_array()) {
            return (Some(preferred_key.to_string()), value.clone());
        }
    }

    if let Some((key, value)) = array_fields.first() {
        return (Some((*key).clone()), (*value).clone());
    }

    (None, Value::Array(Vec::new()))
}

fn json_value_to_py(py: Python<'_>, value: &Value) -> PyResult<Py<PyAny>> {
    match value {
        Value::Null => Ok(py.None()),
        Value::Bool(value) => Ok(PyBool::new(py, *value).to_owned().unbind().into_any()),
        Value::Number(value) => {
            if let Some(int_value) = value.as_i64() {
                Ok(int_value.into_pyobject(py)?.unbind().into_any())
            } else if let Some(uint_value) = value.as_u64() {
                Ok(uint_value.into_pyobject(py)?.unbind().into_any())
            } else if let Some(float_value) = value.as_f64() {
                Ok(float_value.into_pyobject(py)?.unbind().into_any())
            } else {
                Ok(py.None())
            }
        }
        Value::String(value) => Ok(value.into_pyobject(py)?.unbind().into_any()),
        Value::Array(values) => {
            let list = PyList::empty(py);
            for value in values {
                list.append(json_value_to_py(py, value)?)?;
            }
            Ok(list.unbind().into_any())
        }
        Value::Object(values) => {
            let dict = PyDict::new(py);
            for (key, value) in values {
                dict.set_item(key, json_value_to_py(py, value)?)?;
            }
            Ok(dict.unbind().into_any())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{build_api_page, ApiError, CongressApiClient, RetryConfig};
    use chrono::{Duration as ChronoDuration, Utc};
    use pyo3::types::{PyAnyMethods, PyList};
    use pyo3::Python;
    use reqwest::header::{HeaderMap, HeaderValue, RETRY_AFTER};
    use reqwest::StatusCode;
    use serde_json::{json, Value};
    use std::io::{Read, Write};
    use std::net::TcpListener;
    use std::thread;

    fn http_response(status: &str, body: &str) -> String {
        format!(
            "HTTP/1.1 {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            status,
            body.as_bytes().len(),
            body
        )
    }

    fn spawn_test_server(responses: Vec<String>) -> (String, thread::JoinHandle<()>) {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind test server");
        let addr = listener.local_addr().expect("test server addr");
        let handle = thread::spawn(move || {
            for response in responses {
                let (mut stream, _) = listener.accept().expect("accept");
                let mut buffer = [0_u8; 4096];
                let _ = stream.read(&mut buffer);
                stream
                    .write_all(response.as_bytes())
                    .expect("write response");
                stream.flush().expect("flush response");
            }
        });

        (format!("http://{}", addr), handle)
    }

    fn test_client(base_url: String, retry_config: RetryConfig) -> CongressApiClient {
        let mut client = CongressApiClient::new("test_api_key".to_string(), retry_config)
            .expect("construct test client");
        client.base_url = base_url;
        client
    }

    #[test]
    fn default_retry_config_matches_public_defaults() {
        let config = RetryConfig::default();

        assert_eq!(config.max_attempts, 3);
        assert_eq!(config.base_delay_ms, 1000);
    }

    #[test]
    fn get_retries_service_unavailable_then_succeeds() {
        let (base_url, handle) = spawn_test_server(vec![
            http_response("503 Service Unavailable", "{}"),
            http_response("200 OK", r#"{"ok":true}"#),
        ]);
        let client = test_client(
            base_url,
            RetryConfig {
                max_attempts: 2,
                base_delay_ms: 0,
            },
        );

        let value: Value = client
            .get("/bill", None)
            .expect("request succeeds after retry");

        assert_eq!(value["ok"], true);
        handle.join().expect("server thread");
    }

    #[test]
    fn get_retries_too_many_requests_then_succeeds() {
        let (base_url, handle) = spawn_test_server(vec![
            http_response("429 Too Many Requests", "{}"),
            http_response("200 OK", r#"{"ok":true}"#),
        ]);
        let client = test_client(
            base_url,
            RetryConfig {
                max_attempts: 2,
                base_delay_ms: 0,
            },
        );

        let value: Value = client
            .get("/bill", None)
            .expect("request succeeds after retry");

        assert_eq!(value["ok"], true);
        handle.join().expect("server thread");
    }

    #[test]
    fn retryable_statuses_cover_transient_failures() {
        assert!(CongressApiClient::should_retry_status(
            StatusCode::TOO_MANY_REQUESTS
        ));
        assert!(CongressApiClient::should_retry_status(
            StatusCode::INTERNAL_SERVER_ERROR
        ));
        assert!(CongressApiClient::should_retry_status(
            StatusCode::BAD_GATEWAY
        ));
        assert!(CongressApiClient::should_retry_status(
            StatusCode::SERVICE_UNAVAILABLE
        ));
        assert!(CongressApiClient::should_retry_status(
            StatusCode::GATEWAY_TIMEOUT
        ));
        assert!(!CongressApiClient::should_retry_status(
            StatusCode::NOT_FOUND
        ));
    }

    #[test]
    fn retry_after_delay_prefers_header_seconds() {
        let mut headers = HeaderMap::new();
        headers.insert(RETRY_AFTER, HeaderValue::from_static("7"));

        assert_eq!(
            CongressApiClient::retry_after_delay(&headers),
            Some(std::time::Duration::from_secs(7))
        );
    }

    #[test]
    fn retry_after_delay_accepts_http_date_values() {
        let mut headers = HeaderMap::new();
        let retry_at = (Utc::now() + ChronoDuration::seconds(60)).to_rfc2822();
        headers.insert(
            RETRY_AFTER,
            HeaderValue::from_str(&retry_at).expect("valid retry-after date"),
        );

        let delay = CongressApiClient::retry_after_delay(&headers).expect("http-date retry delay");

        assert!(delay >= std::time::Duration::from_secs(58));
        assert!(delay <= std::time::Duration::from_secs(60));
    }

    #[test]
    fn shared_runtime_is_reused() {
        let first = CongressApiClient::shared_runtime().expect("first runtime") as *const _;
        let second = CongressApiClient::shared_runtime().expect("second runtime") as *const _;

        assert_eq!(first, second);
    }

    #[test]
    fn build_api_page_defaults_items_to_empty_list() {
        Python::with_gil(|py| {
            let page = build_api_page(py, json!({"request": {"path": "/bill"}}), None, None)
                .expect("build api page");
            let items = page.items.bind(py);

            assert!(items.is_instance_of::<PyList>());
            assert_eq!(items.len().expect("list length"), 0);
            assert_eq!(page.item_key, None);
        });
    }

    #[test]
    fn get_returns_status_error_for_non_success_responses() {
        let (base_url, handle) = spawn_test_server(vec![http_response(
            "404 Not Found",
            r#"{"error":"missing"}"#,
        )]);
        let client = test_client(base_url, RetryConfig::default());

        let error = client
            .get::<Value>("/bill", None)
            .expect_err("status error");

        match error {
            ApiError::HttpError {
                status_code,
                url,
                message,
            } => {
                assert_eq!(status_code, 404);
                assert!(url.expect("url").contains("/bill"));
                assert!(message.contains("API returned status: 404"));
            }
            other => panic!("unexpected error: {other:?}"),
        }

        handle.join().expect("server thread");
    }

    #[test]
    fn get_includes_response_preview_for_invalid_json() {
        let (base_url, handle) = spawn_test_server(vec![http_response("200 OK", r#"{"bills":["#)]);
        let client = test_client(base_url, RetryConfig::default());

        let error = client
            .get::<Value>("/bill", None)
            .expect_err("invalid json should fail");

        match error {
            ApiError::DeserializationError {
                context,
                message,
                response_preview,
            } => {
                assert!(context.contains("endpoint '/bill'"));
                assert!(!message.is_empty());
                assert!(response_preview.contains(r#"{"bills":["#));
            }
            other => panic!("unexpected error: {other:?}"),
        }

        handle.join().expect("server thread");
    }
}
