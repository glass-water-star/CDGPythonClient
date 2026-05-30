import functools
import logging
from urllib.parse import parse_qs, urlparse

from .cdg_python_client import *

_NativeCDGPythonClient = CDGPythonClient
_NativeAsyncClientCore = AsyncClientCore
_native_configure_client_retries = configure_client_retries
_native_get_client_retry_config = get_client_retry_config

Title = BillTitle

def _resolve_link_url(link_or_url, *, url_index=0):
    if isinstance(link_or_url, str):
        return link_or_url

    url = getattr(link_or_url, "url", None)
    if not url:
        urls = getattr(link_or_url, "urls", None)
        if urls is not None:
            try:
                return urls[url_index]
            except IndexError as exc:
                raise IndexError(
                    f"url_index {url_index} is out of range for link object"
                ) from exc

    if not url:
        raise ValueError("Expected a URL string or an object with 'url' or 'urls'")

    return url


def _resolve_api_segments_and_query(link_or_url, *, url_index=0):
    parsed = urlparse(_resolve_link_url(link_or_url, url_index=url_index))
    path = parsed.path

    if not path:
        raise ValueError("Expected a link with a non-empty path")

    if path == "/v3":
        path = "/"
    elif path.startswith("/v3/"):
        path = path[3:]

    segments = [segment for segment in path.split("/") if segment]
    query = {
        key: values[-1]
        for key, values in parse_qs(parsed.query, keep_blank_values=True).items()
        if values
    }
    return segments, query


def _query_int(query, key):
    value = query.get(key)
    if value is None:
        return None

    try:
        return int(value)
    except ValueError as exc:
        raise ValueError(f"{key} query parameter must be an integer") from exc


def _linked_request_context(link_or_url, *, offset=None, limit=None, url_index=0):
    segments, query = _resolve_api_segments_and_query(link_or_url, url_index=url_index)
    format_value = query.get("format")

    if offset is None:
        offset = _query_int(query, "offset")
    if limit is None:
        limit = _query_int(query, "limit")

    return segments, format_value, offset, limit


def _parse_amendment_actions_link(segments):
    if len(segments) != 5 or segments[0] != "amendment" or segments[4] != "actions":
        raise ValueError("Expected an amendment actions link")
    return int(segments[1]), segments[2], segments[3]


def _parse_amendment_text_link(segments):
    if len(segments) != 5 or segments[0] != "amendment" or segments[4] != "text":
        raise ValueError("Expected an amendment text link")
    return int(segments[1]), segments[2], segments[3]


def _parse_committee_resource_link(segments, resource_name):
    if not segments or segments[0] != "committee":
        raise ValueError(f"Expected a committee {resource_name} link")
    if len(segments) == 4 and segments[3] == resource_name:
        return segments[1], segments[2]
    if len(segments) == 5 and segments[4] == resource_name:
        return segments[2], segments[3]
    raise ValueError(f"Expected a committee {resource_name} link")


def _parse_committee_report_text_link(segments):
    if (
        len(segments) != 5
        or segments[0] != "committee-report"
        or segments[4] != "text"
    ):
        raise ValueError("Expected a committee report text link")
    return int(segments[1]), segments[2], int(segments[3])


def _parse_committee_print_text_link(segments):
    if (
        len(segments) != 5
        or segments[0] != "committee-print"
        or segments[4] != "text"
    ):
        raise ValueError("Expected a committee print text link")
    return int(segments[1]), segments[2], int(segments[3])


def _parse_daily_congressional_record_articles_link(segments):
    if (
        len(segments) != 4
        or segments[0] != "daily-congressional-record"
        or segments[3] != "articles"
    ):
        raise ValueError("Expected a daily congressional record articles link")
    return int(segments[1]), segments[2]


def _parse_treaty_actions_link(segments):
    if len(segments) != 4 or segments[0] != "treaty" or segments[3] != "actions":
        raise ValueError("Expected a treaty actions link")
    return int(segments[1]), segments[2]


def _parse_treaty_committees_link(segments):
    if len(segments) != 4 or segments[0] != "treaty" or segments[3] != "committees":
        raise ValueError("Expected a treaty committees link")
    return int(segments[1]), segments[2]


def _parse_treaty_part_link(segments):
    if (
        len(segments) != 4
        or segments[0] != "treaty"
        or segments[3] in {"actions", "committees"}
    ):
        raise ValueError("Expected a treaty part link")
    return int(segments[1]), segments[2], segments[3]


def _parse_treaty_part_actions_link(segments):
    if len(segments) != 5 or segments[0] != "treaty" or segments[4] != "actions":
        raise ValueError("Expected a treaty part actions link")
    return int(segments[1]), segments[2], segments[3]


def _call_typed_link_method(
    client,
    link_or_url,
    parser,
    method_name,
    *,
    offset=None,
    limit=None,
    url_index=0,
    supports_offset=True,
    supports_limit=True,
):
    segments, format_value, offset, limit = _linked_request_context(
        link_or_url,
        offset=offset,
        limit=limit,
        url_index=url_index,
    )
    kwargs = {}
    if format_value is not None:
        kwargs["format"] = format_value
    if supports_offset and offset is not None:
        kwargs["offset"] = offset
    if supports_limit and limit is not None:
        kwargs["limit"] = limit
    return getattr(client, method_name)(*parser(segments), **kwargs)


async def _call_typed_link_method_async(
    client,
    link_or_url,
    parser,
    method_name,
    *,
    offset=None,
    limit=None,
    url_index=0,
    supports_offset=True,
    supports_limit=True,
):
    segments, format_value, offset, limit = _linked_request_context(
        link_or_url,
        offset=offset,
        limit=limit,
        url_index=url_index,
    )
    kwargs = {}
    if format_value is not None:
        kwargs["format"] = format_value
    if supports_offset and offset is not None:
        kwargs["offset"] = offset
    if supports_limit and limit is not None:
        kwargs["limit"] = limit
    return await getattr(client, method_name)(*parser(segments), **kwargs)


def _normalize_bill_number(bill_number):
    if isinstance(bill_number, bool) or not isinstance(bill_number, (int, str)):
        raise TypeError("bill_number must be a string or integer")

    return str(bill_number)


def _normalize_bill_number_arguments(args, kwargs, *, bill_number_index=2):
    normalized_args = list(args)
    normalized_kwargs = kwargs

    if "bill_number" in kwargs:
        normalized_kwargs = dict(kwargs)
        normalized_kwargs["bill_number"] = _normalize_bill_number(kwargs["bill_number"])
    elif len(normalized_args) > bill_number_index:
        normalized_args[bill_number_index] = _normalize_bill_number(
            normalized_args[bill_number_index]
        )

    return tuple(normalized_args), normalized_kwargs


def _wrap_sync_bill_number_method(method, *, bill_number_index=2):
    @functools.wraps(method)
    def _wrapped(*args, **kwargs):
        args, kwargs = _normalize_bill_number_arguments(
            args,
            kwargs,
            bill_number_index=bill_number_index,
        )
        return method(*args, **kwargs)

    return _wrapped


def _client_follow_link(self, link_or_url, *, offset=None, limit=None, url_index=0):
    """Fetch a page from a raw API URL or an object exposing ``url`` / ``urls``."""
    return self.fetch_page(
        _resolve_link_url(link_or_url, url_index=url_index),
        offset=offset,
        limit=limit,
    )


def _client_fetch_pages(self, path_or_url, *, offset=None, limit=None, max_pages=None):
    """Yield successive ApiPage objects starting from a path or URL."""
    if max_pages is not None and max_pages <= 0:
        return

    current_page = self.fetch_page(
        path_or_url,
        offset=offset,
        limit=limit,
    )
    yielded_pages = 0

    while True:
        yield current_page
        yielded_pages += 1

        if max_pages is not None and yielded_pages >= max_pages:
            return

        if not current_page.has_next():
            return

        current_page = self.fetch_page(current_page.next_url)


def _client_iter_items(self, path_or_url, *, offset=None, limit=None, max_pages=None, max_items=None):
    """Yield items across successive pages for a path or URL."""
    if max_items is not None and max_items <= 0:
        return

    remaining_items = max_items

    for page in self.fetch_pages(
        path_or_url,
        offset=offset,
        limit=limit,
        max_pages=max_pages,
    ):
        items = [] if page.items is None else list(page.items)

        if remaining_items is None:
            for item in items:
                yield item
            continue

        if remaining_items <= 0:
            return

        for item in items[:remaining_items]:
            yield item

        remaining_items -= len(items[:remaining_items])
        if remaining_items <= 0:
            return


def _normalize_log_level(level):
    if isinstance(level, int):
        return level

    if isinstance(level, str):
        normalized = getattr(logging, level.upper(), None)
        if isinstance(normalized, int):
            return normalized

    raise ValueError("level must be a logging level name or integer value")


def _coerce_log_handler(target, *, level):
    log_level = _normalize_log_level(level)

    if callable(target) and not hasattr(target, "log"):
        return target

    if hasattr(target, "log"):
        def _emit(event, *, _logger=target, _level=log_level):
            payload = dict(event)
            _logger.log(
                _level,
                "Congress.gov API %s",
                payload.get("event", "event"),
                extra={f"cdg_{key}": value for key, value in payload.items()},
            )

        return _emit

    if callable(target):
        return target

    raise TypeError("target must be a callable or a logger-like object with a .log(...) method")


def _client_configure_logging(self, target=None, *, level="INFO"):
    """Enable optional per-client request logging using a logger-like object or callback."""
    if target is None:
        self._clear_log_handler()
        return

    self._set_log_handler(_coerce_log_handler(target, level=level))


def _client_disable_logging(self):
    """Disable optional request logging for this client."""
    self._clear_log_handler()


def _client_is_logging_enabled(self):
    """Return whether optional request logging is currently enabled."""
    return self._logging_enabled()


def _public_surface_names(*objects):
    names = set()

    for obj in objects:
        if obj is None:
            continue
        names.update(name for name in dir(obj) if not name.startswith("_"))

    return names


_BILL_NUMBER_METHOD_NAMES = (
    "get_bill",
    "get_bill_detail",
    "get_bill_actions",
    "get_bill_amendments",
    "get_bill_committees",
    "get_bill_cosponsors",
    "get_related_bills",
    "get_bill_subjects",
    "get_bill_summaries",
    "get_bill_text",
    "get_bill_titles",
)

class CDGPythonClient:
    """Thin Python wrapper over the native Rust client."""

    def __init__(self, api_key, *, timeout_seconds=None, user_agent=None):
        self._client = _NativeCDGPythonClient(
            api_key=api_key,
            timeout_seconds=timeout_seconds,
            user_agent=user_agent,
        )

    def __getattr__(self, name):
        attr = getattr(self._client, name)
        if callable(attr) and name in _BILL_NUMBER_METHOD_NAMES:
            return _wrap_sync_bill_number_method(attr)
        return attr

    def __dir__(self):
        names = {name for name in super().__dir__() if not name.startswith("_")}
        names.update(_public_surface_names(getattr(self, "_client", None)))
        return sorted(names)

    def follow_link(self, link_or_url, *, offset=None, limit=None, url_index=0):
        return _client_follow_link(
            self,
            link_or_url,
            offset=offset,
            limit=limit,
            url_index=url_index,
        )

    def fetch_pages(self, path_or_url, *, offset=None, limit=None, max_pages=None):
        return _client_fetch_pages(
            self,
            path_or_url,
            offset=offset,
            limit=limit,
            max_pages=max_pages,
        )

    def get_amendment_actions_by_link(self, link_or_url, *, offset=None, limit=None, url_index=0):
        return _call_typed_link_method(
            self,
            link_or_url,
            _parse_amendment_actions_link,
            "get_amendment_actions",
            offset=offset,
            limit=limit,
            url_index=url_index,
        )

    def get_amendment_text_by_link(self, link_or_url, *, offset=None, limit=None, url_index=0):
        return _call_typed_link_method(
            self,
            link_or_url,
            _parse_amendment_text_link,
            "get_amendment_text",
            offset=offset,
            limit=limit,
            url_index=url_index,
        )

    def get_committee_bills_by_link(self, link_or_url, *, offset=None, limit=None, url_index=0):
        return _call_typed_link_method(
            self,
            link_or_url,
            lambda segments: _parse_committee_resource_link(segments, "bills"),
            "get_committee_bills",
            offset=offset,
            limit=limit,
            url_index=url_index,
        )

    def get_committee_reports_by_link(self, link_or_url, *, offset=None, limit=None, url_index=0):
        return _call_typed_link_method(
            self,
            link_or_url,
            lambda segments: _parse_committee_resource_link(segments, "reports"),
            "get_committee_reports",
            offset=offset,
            limit=limit,
            url_index=url_index,
        )

    def get_committee_nominations_by_link(
        self,
        link_or_url,
        *,
        offset=None,
        limit=None,
        url_index=0,
    ):
        return _call_typed_link_method(
            self,
            link_or_url,
            lambda segments: _parse_committee_resource_link(segments, "nominations"),
            "get_committee_nominations",
            offset=offset,
            limit=limit,
            url_index=url_index,
        )

    def get_committee_report_text_by_link(
        self,
        link_or_url,
        *,
        url_index=0,
    ):
        return _call_typed_link_method(
            self,
            link_or_url,
            _parse_committee_report_text_link,
            "get_committee_report_text",
            url_index=url_index,
            supports_offset=False,
            supports_limit=False,
        )

    def get_committee_print_text_by_link(
        self,
        link_or_url,
        *,
        url_index=0,
    ):
        return _call_typed_link_method(
            self,
            link_or_url,
            _parse_committee_print_text_link,
            "get_committee_print_text",
            url_index=url_index,
            supports_offset=False,
            supports_limit=False,
        )

    def get_daily_congressional_record_articles_by_link(
        self,
        link_or_url,
        *,
        offset=None,
        limit=None,
        url_index=0,
    ):
        return _call_typed_link_method(
            self,
            link_or_url,
            _parse_daily_congressional_record_articles_link,
            "get_daily_congressional_record_articles",
            offset=offset,
            limit=limit,
            url_index=url_index,
        )

    def get_treaty_actions_by_link(self, link_or_url, *, offset=None, limit=None, url_index=0):
        return _call_typed_link_method(
            self,
            link_or_url,
            _parse_treaty_actions_link,
            "get_treaty_actions",
            offset=offset,
            limit=limit,
            url_index=url_index,
        )

    def get_treaty_committees_by_link(self, link_or_url, *, url_index=0):
        return _call_typed_link_method(
            self,
            link_or_url,
            _parse_treaty_committees_link,
            "get_treaty_committees",
            url_index=url_index,
            supports_offset=False,
            supports_limit=False,
        )

    def get_treaty_part_by_link(self, link_or_url, *, url_index=0):
        return _call_typed_link_method(
            self,
            link_or_url,
            _parse_treaty_part_link,
            "get_treaty_part",
            url_index=url_index,
            supports_offset=False,
            supports_limit=False,
        )

    def get_treaty_part_actions_by_link(
        self,
        link_or_url,
        *,
        offset=None,
        limit=None,
        url_index=0,
    ):
        return _call_typed_link_method(
            self,
            link_or_url,
            _parse_treaty_part_actions_link,
            "get_treaty_part_actions",
            offset=offset,
            limit=limit,
            url_index=url_index,
        )

    def iter_items(
        self,
        path_or_url,
        *,
        offset=None,
        limit=None,
        max_pages=None,
        max_items=None,
    ):
        return _client_iter_items(
            self,
            path_or_url,
            offset=offset,
            limit=limit,
            max_pages=max_pages,
            max_items=max_items,
        )

    def configure_logging(self, target=None, *, level="INFO"):
        return _client_configure_logging(self._client, target, level=level)

    def disable_logging(self):
        return _client_disable_logging(self._client)

    def is_logging_enabled(self):
        return _client_is_logging_enabled(self._client)


class _AsyncSyncClientProxy:
    """Sync-facing proxy that keeps shared async client configuration aligned."""

    def __init__(self, async_client):
        self._async_client = async_client

    def configure_timeout(self, timeout_seconds=None):
        self._async_client.configure_timeout(timeout_seconds)

    def get_timeout(self):
        return self._async_client._client.get_timeout()

    def configure_user_agent(self, user_agent=None):
        self._async_client.configure_user_agent(user_agent)

    def get_user_agent(self):
        return self._async_client._client.get_user_agent()

    def configure_logging(self, target=None, *, level="INFO"):
        self._async_client.configure_logging(target, level=level)

    def disable_logging(self):
        self._async_client.disable_logging()

    def is_logging_enabled(self):
        return self._async_client.is_logging_enabled()

    def __getattr__(self, name):
        return getattr(self._async_client._client, name)


class AsyncCDGPythonClient:
    """Async facade over the Rust-backed client."""

    def __init__(self, api_key, *, timeout_seconds=None, user_agent=None):
        self._client = CDGPythonClient(
            api_key=api_key,
            timeout_seconds=timeout_seconds,
            user_agent=user_agent,
        )
        self._native_client = _NativeAsyncClientCore(
            api_key=api_key,
            timeout_seconds=timeout_seconds,
            user_agent=user_agent,
        )
        self._retry_config = get_client_retry_config(self._client)
        self._log_handler = None
        self._sync_client_proxy = _AsyncSyncClientProxy(self)

    @property
    def sync_client(self):
        """Access a synchronous facade that keeps shared config aligned."""
        return self._sync_client_proxy

    def _sync_native_runtime_config(self):
        try:
            retry_attempts, retry_base_delay_ms = get_client_retry_config(self._client)
        except TypeError:
            retry_attempts, retry_base_delay_ms = getattr(self, "_retry_config", (3, 1000))
        self._native_client.configure_retries(retry_attempts, retry_base_delay_ms)
        get_timeout = getattr(self._client, "get_timeout", None)
        get_user_agent = getattr(self._client, "get_user_agent", None)
        is_logging_enabled = getattr(self._client, "is_logging_enabled", None)

        self._native_client.configure_timeout(None if get_timeout is None else get_timeout())
        self._native_client.configure_user_agent(
            None if get_user_agent is None else get_user_agent()
        )

        if (
            is_logging_enabled is not None
            and is_logging_enabled()
            and self._log_handler is not None
        ):
            self._native_client._set_log_handler(self._log_handler)
        else:
            self._native_client._clear_log_handler()

    def configure_retries(self, retry_attempts, retry_base_delay_ms):
        configure_client_retries(self._client, retry_attempts, retry_base_delay_ms)
        self._retry_config = (retry_attempts, retry_base_delay_ms)
        self._native_client.configure_retries(retry_attempts, retry_base_delay_ms)

    def get_retry_config(self):
        try:
            return get_client_retry_config(self._client)
        except TypeError:
            return getattr(self, "_retry_config", (3, 1000))

    def configure_timeout(self, timeout_seconds=None):
        self._client.configure_timeout(timeout_seconds)
        self._native_client.configure_timeout(timeout_seconds)

    def get_timeout(self):
        get_timeout = getattr(self._client, "get_timeout", None)
        return None if get_timeout is None else get_timeout()

    def configure_user_agent(self, user_agent=None):
        self._client.configure_user_agent(user_agent)
        self._native_client.configure_user_agent(user_agent)

    def get_user_agent(self):
        get_user_agent = getattr(self._client, "get_user_agent", None)
        return None if get_user_agent is None else get_user_agent()

    def configure_logging(self, target=None, *, level="INFO"):
        self._client.configure_logging(target, level=level)
        self._log_handler = None if target is None else _coerce_log_handler(target, level=level)
        self._native_client._set_log_handler(self._log_handler)

    def disable_logging(self):
        self._client.disable_logging()
        self._log_handler = None
        self._native_client._clear_log_handler()

    def is_logging_enabled(self):
        is_logging_enabled = getattr(self._client, "is_logging_enabled", None)
        return False if is_logging_enabled is None else is_logging_enabled()

    async def fetch_page(self, path_or_url, offset=None, limit=None):
        self._sync_native_runtime_config()
        return await self._native_client.fetch_page(path_or_url, offset, limit)

    async def follow_link(self, link_or_url, *, offset=None, limit=None, url_index=0):
        return await self.fetch_page(
            _resolve_link_url(link_or_url, url_index=url_index),
            offset=offset,
            limit=limit,
        )

    async def get_amendment_actions_by_link(
        self,
        link_or_url,
        *,
        offset=None,
        limit=None,
        url_index=0,
    ):
        return await _call_typed_link_method_async(
            self,
            link_or_url,
            _parse_amendment_actions_link,
            "get_amendment_actions",
            offset=offset,
            limit=limit,
            url_index=url_index,
        )

    async def get_amendment_text_by_link(
        self,
        link_or_url,
        *,
        offset=None,
        limit=None,
        url_index=0,
    ):
        return await _call_typed_link_method_async(
            self,
            link_or_url,
            _parse_amendment_text_link,
            "get_amendment_text",
            offset=offset,
            limit=limit,
            url_index=url_index,
        )

    async def get_committee_bills_by_link(
        self,
        link_or_url,
        *,
        offset=None,
        limit=None,
        url_index=0,
    ):
        return await _call_typed_link_method_async(
            self,
            link_or_url,
            lambda segments: _parse_committee_resource_link(segments, "bills"),
            "get_committee_bills",
            offset=offset,
            limit=limit,
            url_index=url_index,
        )

    async def get_committee_reports_by_link(
        self,
        link_or_url,
        *,
        offset=None,
        limit=None,
        url_index=0,
    ):
        return await _call_typed_link_method_async(
            self,
            link_or_url,
            lambda segments: _parse_committee_resource_link(segments, "reports"),
            "get_committee_reports",
            offset=offset,
            limit=limit,
            url_index=url_index,
        )

    async def get_committee_nominations_by_link(
        self,
        link_or_url,
        *,
        offset=None,
        limit=None,
        url_index=0,
    ):
        return await _call_typed_link_method_async(
            self,
            link_or_url,
            lambda segments: _parse_committee_resource_link(segments, "nominations"),
            "get_committee_nominations",
            offset=offset,
            limit=limit,
            url_index=url_index,
        )

    async def get_committee_report_text_by_link(self, link_or_url, *, url_index=0):
        return await _call_typed_link_method_async(
            self,
            link_or_url,
            _parse_committee_report_text_link,
            "get_committee_report_text",
            url_index=url_index,
            supports_offset=False,
            supports_limit=False,
        )

    async def get_committee_print_text_by_link(self, link_or_url, *, url_index=0):
        return await _call_typed_link_method_async(
            self,
            link_or_url,
            _parse_committee_print_text_link,
            "get_committee_print_text",
            url_index=url_index,
            supports_offset=False,
            supports_limit=False,
        )

    async def get_daily_congressional_record_articles_by_link(
        self,
        link_or_url,
        *,
        offset=None,
        limit=None,
        url_index=0,
    ):
        return await _call_typed_link_method_async(
            self,
            link_or_url,
            _parse_daily_congressional_record_articles_link,
            "get_daily_congressional_record_articles",
            offset=offset,
            limit=limit,
            url_index=url_index,
        )

    async def get_treaty_actions_by_link(
        self,
        link_or_url,
        *,
        offset=None,
        limit=None,
        url_index=0,
    ):
        return await _call_typed_link_method_async(
            self,
            link_or_url,
            _parse_treaty_actions_link,
            "get_treaty_actions",
            offset=offset,
            limit=limit,
            url_index=url_index,
        )

    async def get_treaty_committees_by_link(self, link_or_url, *, url_index=0):
        return await _call_typed_link_method_async(
            self,
            link_or_url,
            _parse_treaty_committees_link,
            "get_treaty_committees",
            url_index=url_index,
            supports_offset=False,
            supports_limit=False,
        )

    async def get_treaty_part_by_link(self, link_or_url, *, url_index=0):
        return await _call_typed_link_method_async(
            self,
            link_or_url,
            _parse_treaty_part_link,
            "get_treaty_part",
            url_index=url_index,
            supports_offset=False,
            supports_limit=False,
        )

    async def get_treaty_part_actions_by_link(
        self,
        link_or_url,
        *,
        offset=None,
        limit=None,
        url_index=0,
    ):
        return await _call_typed_link_method_async(
            self,
            link_or_url,
            _parse_treaty_part_actions_link,
            "get_treaty_part_actions",
            offset=offset,
            limit=limit,
            url_index=url_index,
        )

    async def fetch_pages(self, path_or_url, *, offset=None, limit=None, max_pages=None):
        if max_pages is not None and max_pages <= 0:
            return

        current_page = await self.fetch_page(
            path_or_url,
            offset=offset,
            limit=limit,
        )
        yielded_pages = 0

        while True:
            yield current_page
            yielded_pages += 1

            if max_pages is not None and yielded_pages >= max_pages:
                return

            if not current_page.has_next():
                return

            current_page = await self.fetch_page(current_page.next_url)

    async def iter_items(
        self,
        path_or_url,
        *,
        offset=None,
        limit=None,
        max_pages=None,
        max_items=None,
    ):
        if max_items is not None and max_items <= 0:
            return

        remaining_items = max_items

        async for page in self.fetch_pages(
            path_or_url,
            offset=offset,
            limit=limit,
            max_pages=max_pages,
        ):
            items = [] if page.items is None else list(page.items)

            if remaining_items is None:
                for item in items:
                    yield item
                continue

            if remaining_items <= 0:
                return

            for item in items[:remaining_items]:
                yield item

            remaining_items -= len(items[:remaining_items])
            if remaining_items <= 0:
                return

    async def _call_native(self, name, *args, **kwargs):
        native_attr = getattr(self._native_client, name, None)
        if not callable(native_attr):
            raise AttributeError(name)
        self._sync_native_runtime_config()
        return await native_attr(*args, **kwargs)

    def __getattr__(self, name):
        native_attr = getattr(self._native_client, name, None)
        if callable(native_attr):
            @functools.wraps(native_attr)
            async def _native_method(*args, **kwargs):
                if name in _BILL_NUMBER_METHOD_NAMES:
                    args, kwargs = _normalize_bill_number_arguments(args, kwargs)
                return await self._call_native(name, *args, **kwargs)

            return _native_method

        return getattr(self._client, name)

    def __dir__(self):
        names = {
            name for name in super().__dir__()
            if not name.startswith("_")
        }
        names.update(
            _public_surface_names(
                getattr(self, "_client", None),
                getattr(self, "_native_client", None),
            )
        )
        names.add("sync_client")
        return sorted(names)


def _unwrap_native_client(client):
    if isinstance(client, AsyncCDGPythonClient):
        return client._client._client
    if isinstance(client, CDGPythonClient):
        return client._client
    return client


def configure_client_retries(client, retry_attempts, retry_base_delay_ms):
    return _native_configure_client_retries(
        _unwrap_native_client(client),
        retry_attempts,
        retry_base_delay_ms,
    )


def get_client_retry_config(client):
    return _native_get_client_retry_config(_unwrap_native_client(client))


__all__ = [
    "CDGPythonClient",
    "AsyncCDGPythonClient",
    "CDGClientError",
    "CDGConfigurationError",
    "CDGInvalidUrlError",
    "CDGRequestError",
    "CDGHttpError",
    "CDGAuthError",
    "CDGNotFoundError",
    "CDGRateLimitError",
    "CDGServerError",
    "CDGDeserializationError",
    "ApiPage",
    "configure_client_retries",
    "get_client_retry_config",
    "ActionCommittee",
    "Bill",
    "BillDetail",
    "LatestAction",
    "Law",
    "Sponsor",
    "PolicyArea",
    "Action",
    "Amendment",
    "AmendmentDetail",
    "BoundCongressionalRecord",
    "Committee",
    "CommitteeMeeting",
    "CommitteeMeetingLocation",
    "CommitteeMeetingVideo",
    "CommunicationType",
    "CongressionalRecord",
    "CongressionalRecordIssue",
    "CongressionalRecordLink",
    "CongressionalRecordLinks",
    "CongressionalRecordPdfItem",
    "Cosponsor",
    "CountLink",
    "RelatedBill",
    "RelationshipDetail",
    "HouseCommunication",
    "HouseRequirement",
    "Subject",
    "Summary",
    "TextVersion",
    "TextFormat",
    "Title",
    "Congress",
    "Session",
    "HouseVote",
    "HouseVoteDetail",
    "HouseVoteMembers",
    "MemberVote",
    "Party",
    "VoteParty",
    "Hearing",
    "HearingDate",
    "HearingFormat",
    "HearingCommittee",
    "AssociatedMeeting",
    "CrsReport",
    "CrsReportDetail",
    "CrsReportAuthor",
    "CrsReportFormat",
    "CrsReportTopic",
    "CrsReportRelatedMaterial",
    "LawItem",
    "LawDetail",
    "DailyCongressionalRecord",
    "DailyCongressionalRecordArticle",
    "DailyCongressionalRecordArticleGroup",
    "DailyCongressionalRecordFullIssue",
    "DailyCongressionalRecordIssue",
    "MatchingRequirement",
    "Nomination",
    "NominationCommittee",
    "NominationCommitteeActivity",
    "NominationHearing",
    "NominationType",
    "Nominee",
    "RecordResource",
    "RecordSection",
    "RecordTextLink",
    "SenateCommunication",
    "SourceSystem",
    "Treaty",
    "TreatyCountryParty",
    "TreatyIndexTerm",
    "TreatyParts",
]
