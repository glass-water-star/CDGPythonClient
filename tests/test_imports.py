"""Test basic import statements for cdg_python_client."""

import logging

import pytest


def test_import_main_client():
    """Test that CDGPythonClient can be imported."""
    from cdg_python_client import (
        AsyncCDGPythonClient,
        CDGAuthError,
        CDGClientError,
        CDGDeserializationError,
        CDGHttpError,
        CDGInvalidUrlError,
        CDGNotFoundError,
        CDGRateLimitError,
        CDGRequestError,
        CDGServerError,
        CDGPythonClient,
    )

    assert CDGPythonClient is not None
    assert AsyncCDGPythonClient is not None
    assert CDGClientError is not None
    assert CDGRequestError is not None
    assert CDGHttpError is not None
    assert CDGAuthError is not None
    assert CDGNotFoundError is not None
    assert CDGRateLimitError is not None
    assert CDGServerError is not None
    assert CDGDeserializationError is not None
    assert CDGInvalidUrlError is not None


def test_exception_hierarchy():
    """Test that the public exception hierarchy is wired as expected."""
    from cdg_python_client import (
        CDGAuthError,
        CDGClientError,
        CDGConfigurationError,
        CDGDeserializationError,
        CDGHttpError,
        CDGInvalidUrlError,
        CDGNotFoundError,
        CDGRateLimitError,
        CDGRequestError,
        CDGServerError,
    )

    assert issubclass(CDGConfigurationError, CDGClientError)
    assert issubclass(CDGInvalidUrlError, CDGClientError)
    assert issubclass(CDGRequestError, CDGClientError)
    assert issubclass(CDGHttpError, CDGClientError)
    assert issubclass(CDGAuthError, CDGHttpError)
    assert issubclass(CDGNotFoundError, CDGHttpError)
    assert issubclass(CDGRateLimitError, CDGHttpError)
    assert issubclass(CDGServerError, CDGHttpError)
    assert issubclass(CDGDeserializationError, CDGClientError)


def test_import_all_models():
    """Test that all model classes can be imported."""
    from cdg_python_client import (
        Bill,
        BillDetail,
        LatestAction,
        Law,
        Sponsor,
        PolicyArea,
        Action,
        Amendment,
        Committee,
        Cosponsor,
        RelatedBill,
        RelationshipDetail,
        Subject,
        Summary,
        TextVersion,
        TextFormat,
        Title,
    )
    
    # Verify all classes are not None
    assert Bill is not None
    assert BillDetail is not None
    assert LatestAction is not None
    assert Law is not None
    assert Sponsor is not None
    assert PolicyArea is not None
    assert Action is not None
    assert Amendment is not None
    assert Committee is not None
    assert Cosponsor is not None
    assert RelatedBill is not None
    assert RelationshipDetail is not None
    assert Subject is not None
    assert Summary is not None
    assert TextVersion is not None
    assert TextFormat is not None
    assert Title is not None


def test_client_instantiation():
    """Test that CDGPythonClient can be instantiated."""
    from cdg_python_client import CDGPythonClient
    
    client = CDGPythonClient(api_key="test_key")
    assert client is not None


def test_client_retry_settings_can_be_configured():
    """Test that CDGPythonClient accepts custom retry settings."""
    from cdg_python_client import (
        CDGPythonClient,
        configure_client_retries,
        get_client_retry_config,
    )

    client = CDGPythonClient(api_key="test_key")
    configure_client_retries(client, 5, 250)

    assert get_client_retry_config(client) == (5, 250)


def test_client_timeout_and_user_agent_can_be_configured():
    """Test that CDGPythonClient accepts timeout and user-agent configuration."""
    from cdg_python_client import CDGPythonClient

    client = CDGPythonClient(
        api_key="test_key",
        timeout_seconds=2.5,
        user_agent="cdg-python-client-tests/1.0",
    )

    assert client.get_timeout() == 2.5
    assert client.get_user_agent() == "cdg-python-client-tests/1.0"

    client.configure_timeout(5.0)
    client.configure_user_agent("cdg-python-client-tests/2.0")

    assert client.get_timeout() == 5.0
    assert client.get_user_agent() == "cdg-python-client-tests/2.0"

    client.configure_timeout(None)
    client.configure_user_agent(None)

    assert client.get_timeout() is None
    assert client.get_user_agent() is None


def test_client_configuration_validation_uses_typed_errors():
    """Test that invalid client configuration raises package-specific config errors."""
    from cdg_python_client import CDGConfigurationError, CDGPythonClient

    with pytest.raises(CDGConfigurationError):
        CDGPythonClient(api_key="")

    client = CDGPythonClient(api_key="test_key")

    with pytest.raises(CDGConfigurationError):
        client.configure_timeout(0)

    with pytest.raises(CDGConfigurationError):
        client.configure_user_agent("   ")


def test_client_has_pagination_helpers():
    """Test that pagination helpers are exposed on the client."""
    from cdg_python_client import CDGPythonClient

    client = CDGPythonClient(api_key="test_key")

    assert hasattr(client, "fetch_page")
    assert hasattr(client, "fetch_pages")
    assert hasattr(client, "iter_items")
    assert hasattr(client, "follow_link")


def test_invalid_absolute_url_raises_typed_exception():
    """Test that invalid absolute URLs raise the package-specific URL error."""
    from cdg_python_client import CDGInvalidUrlError, CDGPythonClient

    client = CDGPythonClient(api_key="test_key")

    with pytest.raises(CDGInvalidUrlError):
        client.fetch_page("https://example.com/not-congress")


def test_async_client_instantiation_and_retry_configuration(async_client):
    """Test that AsyncCDGPythonClient wraps the sync client and exposes retry helpers."""
    assert hasattr(async_client.sync_client, "list_bills")
    assert hasattr(async_client.sync_client, "configure_timeout")
    assert hasattr(async_client, "_native_client")
    async_client.configure_retries(4, 300)
    assert async_client.get_retry_config() == (4, 300)


def test_async_client_timeout_and_user_agent_stay_in_sync():
    """Test that async client mirrors timeout and user-agent config to both backends."""
    from cdg_python_client import AsyncCDGPythonClient

    client = AsyncCDGPythonClient(
        api_key="test_key",
        timeout_seconds=1.25,
        user_agent="cdg-python-client-async-tests/1.0",
    )

    assert client.get_timeout() == 1.25
    assert client.get_user_agent() == "cdg-python-client-async-tests/1.0"
    assert client.sync_client.get_timeout() == 1.25
    assert client.sync_client.get_user_agent() == "cdg-python-client-async-tests/1.0"

    client.configure_timeout(3.5)
    client.configure_user_agent("cdg-python-client-async-tests/2.0")

    assert client.get_timeout() == 3.5
    assert client.get_user_agent() == "cdg-python-client-async-tests/2.0"
    assert client.sync_client.get_timeout() == 3.5
    assert client.sync_client.get_user_agent() == "cdg-python-client-async-tests/2.0"

    client.sync_client.configure_timeout(4.25)
    client.sync_client.configure_user_agent("cdg-python-client-async-tests/3.0")

    assert client.get_timeout() == 4.25
    assert client.get_user_agent() == "cdg-python-client-async-tests/3.0"


def test_async_sync_client_logging_stays_in_sync():
    """Test that sync_client logging helpers keep the async wrapper state aligned."""
    from cdg_python_client import AsyncCDGPythonClient

    client = AsyncCDGPythonClient(api_key="test_key")

    client.sync_client.configure_logging(lambda event: None)
    assert client.is_logging_enabled() is True

    client.sync_client.disable_logging()
    assert client.is_logging_enabled() is False


def test_logger_target_adapter_adds_structured_extra_fields():
    """Test that logger-like targets receive the normalized event payload via `extra`."""
    import cdg_python_client

    captured = []

    class FakeLogger:
        def log(self, level, msg, *args, **kwargs):
            captured.append((level, msg, args, kwargs))

    emitter = cdg_python_client._coerce_log_handler(FakeLogger(), level="warning")
    emitter(
        {
            "event": "request_success",
            "url": "https://api.congress.gov/v3/bill?limit=5",
            "path": "/v3/bill",
            "status_code": 200,
            "attempt": 1,
        }
    )

    assert captured == [
        (
            logging.WARNING,
            "Congress.gov API %s",
            ("request_success",),
            {
                "extra": {
                    "cdg_event": "request_success",
                    "cdg_url": "https://api.congress.gov/v3/bill?limit=5",
                    "cdg_path": "/v3/bill",
                    "cdg_status_code": 200,
                    "cdg_attempt": 1,
                }
            },
        )
    ]


def test_invalid_logging_level_is_rejected():
    """Test that invalid logging level names raise a clear error."""
    import cdg_python_client

    class FakeLogger:
        def log(self, level, msg, *args, **kwargs):
            raise AssertionError("should not be called")

    with pytest.raises(ValueError, match="level must be a logging level name or integer value"):
        cdg_python_client._coerce_log_handler(FakeLogger(), level="not-a-level")


def test_async_logging_helpers_keep_callback_state_aligned():
    """Test that async logging helpers keep the sync facade and wrapper state aligned."""
    from cdg_python_client import AsyncCDGPythonClient

    client = AsyncCDGPythonClient(api_key="test_key")
    callback = lambda event: None

    assert client.is_logging_enabled() is False
    client.configure_logging(callback)
    assert client.is_logging_enabled() is True
    assert client.sync_client.is_logging_enabled() is True

    client.disable_logging()
    assert client.is_logging_enabled() is False
    assert client.sync_client.is_logging_enabled() is False


def test_follow_link_uses_string_or_url_attribute():
    """Test that client.follow_link accepts raw URLs and objects with a url attribute."""
    import cdg_python_client

    calls = []

    class FakeClient:
        follow_link = cdg_python_client.CDGPythonClient.follow_link

        def fetch_page(self, path_or_url, offset=None, limit=None):
            calls.append((path_or_url, offset, limit))
            return "page"

    client = FakeClient()
    assert client.follow_link("https://api.congress.gov/v3/bill", limit=10) == "page"

    class Link:
        url = "https://api.congress.gov/v3/member"

    assert client.follow_link(Link(), offset=20) == "page"
    assert calls == [
        ("https://api.congress.gov/v3/bill", None, 10),
        ("https://api.congress.gov/v3/member", 20, None),
    ]


def test_fetch_pages_uses_fetch_page():
    """Test that client.fetch_pages chains through client.fetch_page and next_url."""
    import cdg_python_client

    class FakePage:
        def __init__(self, items, next_url):
            self.items = items
            self.next_url = next_url

        def has_next(self):
            return self.next_url is not None

    pages = {
        "/bill": FakePage([1, 2], "next-1"),
        "next-1": FakePage([3, 4], None),
    }

    calls = []

    class FakeClient:
        fetch_pages = cdg_python_client.CDGPythonClient.fetch_pages

        def fetch_page(self, path_or_url, offset=None, limit=None):
            calls.append((path_or_url, offset, limit))
            return pages[path_or_url]

    page_items = [page.items for page in FakeClient().fetch_pages("/bill", limit=2)]
    assert page_items == [[1, 2], [3, 4]]
    assert calls == [
        ("/bill", None, 2),
        ("next-1", None, None),
    ]

    calls.clear()
    assert list(FakeClient().fetch_pages("/bill", limit=2, max_pages=0)) == []
    assert list(FakeClient().fetch_pages("/bill", limit=2, max_pages=-1)) == []
    assert calls == []


def test_iter_items_flattens_pages_and_honors_max_items():
    """Test that client.iter_items flattens page items consistently."""
    import cdg_python_client

    class FakePage:
        def __init__(self, items, next_url):
            self.items = items
            self.next_url = next_url

        def has_next(self):
            return self.next_url is not None

    pages = {
        "/bill": FakePage([1, 2], "next-1"),
        "next-1": FakePage([3, 4], None),
    }

    class FakeClient:
        iter_items = cdg_python_client.CDGPythonClient.iter_items
        fetch_pages = cdg_python_client.CDGPythonClient.fetch_pages
        calls = []

        def fetch_page(self, path_or_url, offset=None, limit=None):
            self.calls.append((path_or_url, offset, limit))
            return pages[path_or_url]

    assert list(FakeClient().iter_items("/bill", limit=2)) == [1, 2, 3, 4]
    assert list(FakeClient().iter_items("/bill", limit=2, max_items=3)) == [1, 2, 3]

    client = FakeClient()
    client.calls = []
    assert list(client.iter_items("/bill", limit=2, max_items=0)) == []
    assert list(client.iter_items("/bill", limit=2, max_items=-1)) == []
    assert client.calls == []


def test_typed_link_helpers_dispatch_to_existing_methods():
    """Test that typed link helpers parse API URLs and reuse the typed client methods."""
    import cdg_python_client

    calls = []

    class TreatyPartsLink:
        urls = [
            "https://api.congress.gov/v3/treaty/118/12/A?format=json",
            "https://api.congress.gov/v3/treaty/118/12/B?format=json",
        ]

    class FakeClient:
        get_committee_bills_by_link = cdg_python_client.CDGPythonClient.get_committee_bills_by_link
        get_committee_report_text_by_link = (
            cdg_python_client.CDGPythonClient.get_committee_report_text_by_link
        )
        get_treaty_part_by_link = cdg_python_client.CDGPythonClient.get_treaty_part_by_link
        get_daily_congressional_record_articles_by_link = (
            cdg_python_client.CDGPythonClient.get_daily_congressional_record_articles_by_link
        )

        def get_committee_bills(self, chamber, committee_code, format=None, offset=None, limit=None):
            calls.append(("committee_bills", chamber, committee_code, format, offset, limit))
            return ["committee-bills"]

        def get_committee_report_text(self, congress, report_type, report_number, format=None):
            calls.append(("committee_report_text", congress, report_type, report_number, format))
            return ["report-text"]

        def get_treaty_part(self, congress, treaty_number, treaty_suffix, format=None):
            calls.append(("treaty_part", congress, treaty_number, treaty_suffix, format))
            return ["treaty-part"]

        def get_daily_congressional_record_articles(
            self,
            volume_number,
            issue_number,
            format=None,
            offset=None,
            limit=None,
        ):
            calls.append(("daily_articles", volume_number, issue_number, format, offset, limit))
            return ["daily-articles"]

    client = FakeClient()

    assert client.get_committee_bills_by_link(
        "https://api.congress.gov/v3/committee/house/hsif00/bills?format=json&offset=4&limit=2"
    ) == ["committee-bills"]
    assert client.get_committee_report_text_by_link(
        "https://api.congress.gov/v3/committee-report/118/hrpt/12/text?format=json"
    ) == ["report-text"]
    assert client.get_treaty_part_by_link(TreatyPartsLink(), url_index=1) == ["treaty-part"]
    assert client.get_daily_congressional_record_articles_by_link(
        "https://api.congress.gov/v3/daily-congressional-record/170/12/articles?limit=3",
        offset=1,
    ) == ["daily-articles"]

    assert calls == [
        ("committee_bills", "house", "hsif00", "json", 4, 2),
        ("committee_report_text", 118, "hrpt", 12, "json"),
        ("treaty_part", 118, "12", "B", "json"),
        ("daily_articles", 170, "12", None, 1, 3),
    ]


def test_typed_link_helpers_reject_mismatched_paths():
    """Test that typed link helpers fail clearly for incompatible resource URLs."""
    import cdg_python_client

    class FakeClient:
        get_treaty_actions_by_link = cdg_python_client.CDGPythonClient.get_treaty_actions_by_link

        def get_treaty_actions(self, congress, treaty_number, format=None, offset=None, limit=None):
            raise AssertionError("should not be called")

    with pytest.raises(ValueError, match="Expected a treaty actions link"):
        FakeClient().get_treaty_actions_by_link("https://api.congress.gov/v3/committee/house/hsif00/bills")


def test_normalize_bill_number_accepts_strings_and_integers():
    """Test that bill numbers normalize to strings for wrapper compatibility."""
    import cdg_python_client

    assert cdg_python_client._normalize_bill_number("1") == "1"
    assert cdg_python_client._normalize_bill_number(1) == "1"

    with pytest.raises(TypeError):
        cdg_python_client._normalize_bill_number(1.5)

    with pytest.raises(TypeError):
        cdg_python_client._normalize_bill_number(True)


@pytest.mark.asyncio
async def test_async_client_wraps_sync_methods_and_pagination():
    """Test that AsyncCDGPythonClient exposes awaitable methods and async page iteration."""
    import cdg_python_client

    class FakePage:
        def __init__(self, items, next_url):
            self.items = items
            self.next_url = next_url

        def has_next(self):
            return self.next_url is not None

    pages = {
        "/bill": FakePage([1, 2], "next-1"),
        "next-1": FakePage([3], None),
        "https://api.congress.gov/v3/bill": FakePage(["linked"], None),
    }

    class FakeNativeClient:
        def __init__(self):
            self.retry_config = (3, 1000)
            self.timeout = None
            self.user_agent = None
            self.logging_enabled = False
            self.fetch_calls = []

        def configure_retries(self, retry_attempts, retry_base_delay_ms):
            self.retry_config = (retry_attempts, retry_base_delay_ms)

        def configure_timeout(self, timeout_seconds=None):
            self.timeout = timeout_seconds

        def configure_user_agent(self, user_agent=None):
            self.user_agent = user_agent

        def _set_log_handler(self, handler=None):
            self.logging_enabled = handler is not None

        def _clear_log_handler(self):
            self.logging_enabled = False

        async def list_bills(self, limit=None):
            return [f"native-limit={limit}"]

        async def get_amendment_text(self, congress, amendment_type, amendment_number, limit=None):
            return [f"{congress}-{amendment_type}-{amendment_number}-text-limit={limit}"]

        async def get_bill_actions(self, congress, bill_type, bill_number, limit=None):
            return [f"{congress}-{bill_type}-{bill_number}-limit={limit}"]

        async def get_member_sponsored_legislation(self, bioguide_id, limit=None):
            return [f"{bioguide_id}-sponsored-limit={limit}"]

        async def get_committee_bills(self, chamber, committee_code, limit=None):
            return [f"{chamber}-{committee_code}-bills-limit={limit}"]

        async def list_laws(self, limit=None):
            return [f"native-laws-limit={limit}"]

        async def list_house_votes(self, limit=None):
            return [f"native-house-votes-limit={limit}"]

        async def fetch_page(self, path_or_url, offset=None, limit=None):
            self.fetch_calls.append((path_or_url, offset, limit))
            return pages[path_or_url]

    class FakeSyncClient:
        def get_timeout(self):
            return None

        def get_user_agent(self):
            return None

        def is_logging_enabled(self):
            return False

        def list_bills(self, limit=None):
            raise AssertionError("native async method should be preferred")

        def get_amendment_text(self, congress, amendment_type, amendment_number, limit=None):
            raise AssertionError("native async method should be preferred")

        def get_bill_actions(self, congress, bill_type, bill_number, limit=None):
            raise AssertionError("native async method should be preferred")

        def get_member_sponsored_legislation(self, bioguide_id, limit=None):
            raise AssertionError("native async method should be preferred")

        def get_committee_bills(self, chamber, committee_code, limit=None):
            raise AssertionError("native async method should be preferred")

        def list_laws(self, limit=None):
            raise AssertionError("native async method should be preferred")

        def list_house_votes(self, limit=None):
            raise AssertionError("native async method should be preferred")

        def follow_link(self, link_or_url, *, offset=None, limit=None, url_index=0):
            raise AssertionError("async follow_link should use native fetch_page")

    async_client = object.__new__(cdg_python_client.AsyncCDGPythonClient)
    async_client._client = FakeSyncClient()
    async_client._native_client = FakeNativeClient()
    async_client._retry_config = (3, 1000)
    async_client._log_handler = None
    async_client._sync_client_proxy = None

    amendment_text = await async_client.get_amendment_text(118, "samdt", "1", limit=2)
    bills = await async_client.list_bills(limit=5)
    actions = await async_client.get_bill_actions(118, "hr", 1, limit=4)
    sponsored = await async_client.get_member_sponsored_legislation("A000360", limit=6)
    committee_bills = await async_client.get_committee_bills("house", "hsag", limit=7)
    house_votes = await async_client.list_house_votes(limit=8)
    laws = await async_client.list_laws(limit=3)
    first_page = await async_client.fetch_page("/bill", limit=2)
    linked = await async_client.follow_link("https://api.congress.gov/v3/bill", limit=10)
    pages_seen = []

    async for page in async_client.fetch_pages("/bill", limit=2):
        pages_seen.append(page.items)

    flattened = []
    async for item in async_client.iter_items("/bill", limit=2, max_items=2):
        flattened.append(item)

    assert amendment_text == ["118-samdt-1-text-limit=2"]
    assert bills == ["native-limit=5"]
    assert actions == ["118-hr-1-limit=4"]
    assert sponsored == ["A000360-sponsored-limit=6"]
    assert committee_bills == ["house-hsag-bills-limit=7"]
    assert house_votes == ["native-house-votes-limit=8"]
    assert laws == ["native-laws-limit=3"]
    assert first_page.items == [1, 2]
    assert linked.items == ["linked"]
    assert pages_seen == [[1, 2], [3]]
    assert flattened == [1, 2]
    assert async_client._native_client.fetch_calls == [
        ("/bill", None, 2),
        ("https://api.congress.gov/v3/bill", None, 10),
        ("/bill", None, 2),
        ("next-1", None, None),
        ("/bill", None, 2),
    ]


@pytest.mark.asyncio
async def test_async_typed_link_helpers_dispatch_to_async_methods():
    """Test that async typed link helpers preserve typed async dispatch."""
    import cdg_python_client

    calls = []

    class FakeAsyncClient:
        get_amendment_actions_by_link = (
            cdg_python_client.AsyncCDGPythonClient.get_amendment_actions_by_link
        )
        get_treaty_committees_by_link = (
            cdg_python_client.AsyncCDGPythonClient.get_treaty_committees_by_link
        )

        async def get_amendment_actions(
            self,
            congress,
            amendment_type,
            amendment_number,
            format=None,
            offset=None,
            limit=None,
        ):
            calls.append(
                (
                    "amendment_actions",
                    congress,
                    amendment_type,
                    amendment_number,
                    format,
                    offset,
                    limit,
                )
            )
            return ["actions"]

        async def get_treaty_committees(self, congress, treaty_number, format=None):
            calls.append(("treaty_committees", congress, treaty_number, format))
            return ["committees"]

    client = FakeAsyncClient()

    assert await client.get_amendment_actions_by_link(
        "https://api.congress.gov/v3/amendment/118/hamdt/12/actions?format=json&limit=4"
    ) == ["actions"]
    assert await client.get_treaty_committees_by_link(
        "https://api.congress.gov/v3/treaty/118/12/committees?format=json"
    ) == ["committees"]

    assert calls == [
        ("amendment_actions", 118, "hamdt", "12", "json", None, 4),
        ("treaty_committees", 118, "12", "json"),
    ]


@pytest.mark.asyncio
async def test_async_pagination_short_circuits_zero_limits():
    """Test that async pagination helpers avoid fetching when limits are zero or negative."""
    import cdg_python_client

    calls = []

    class FakeNativeClient:
        def configure_retries(self, retry_attempts, retry_base_delay_ms):
            pass

        def configure_timeout(self, timeout_seconds=None):
            pass

        def configure_user_agent(self, user_agent=None):
            pass

        def _set_log_handler(self, handler=None):
            pass

        def _clear_log_handler(self):
            pass

        async def fetch_page(self, path_or_url, offset=None, limit=None):
            calls.append((path_or_url, offset, limit))
            raise AssertionError("fetch_page should not be called")

    class FakeSyncClient:
        def get_timeout(self):
            return None

        def get_user_agent(self):
            return None

        def is_logging_enabled(self):
            return False

    async_client = object.__new__(cdg_python_client.AsyncCDGPythonClient)
    async_client._client = FakeSyncClient()
    async_client._native_client = FakeNativeClient()
    async_client._retry_config = (3, 1000)
    async_client._log_handler = None
    async_client._sync_client_proxy = None

    pages_seen = []
    async for page in async_client.fetch_pages("/bill", limit=2, max_pages=0):
        pages_seen.append(page)
    assert pages_seen == []

    items_seen = []
    async for item in async_client.iter_items("/bill", limit=2, max_items=0):
        items_seen.append(item)
    assert items_seen == []
    assert calls == []


@pytest.mark.asyncio
async def test_async_fetch_page_syncs_runtime_config_to_native_client():
    """Test that async fetches mirror sync-side config and logging state into the native core."""
    import cdg_python_client

    sync_calls = []
    native_calls = []

    class FakeNativeClient:
        def configure_retries(self, retry_attempts, retry_base_delay_ms):
            native_calls.append(("configure_retries", retry_attempts, retry_base_delay_ms))

        def configure_timeout(self, timeout_seconds=None):
            native_calls.append(("configure_timeout", timeout_seconds))

        def configure_user_agent(self, user_agent=None):
            native_calls.append(("configure_user_agent", user_agent))

        def _set_log_handler(self, handler=None):
            native_calls.append(("set_log_handler", handler))

        def _clear_log_handler(self):
            native_calls.append(("clear_log_handler",))

        async def fetch_page(self, path_or_url, offset=None, limit=None):
            native_calls.append(("fetch_page", path_or_url, offset, limit))
            return {"path_or_url": path_or_url, "offset": offset, "limit": limit}

    class FakeSyncClient:
        def get_timeout(self):
            sync_calls.append(("get_timeout",))
            return 2.5

        def get_user_agent(self):
            sync_calls.append(("get_user_agent",))
            return "cdg-tests/1.0"

        def is_logging_enabled(self):
            sync_calls.append(("is_logging_enabled",))
            return True

    async_client = object.__new__(cdg_python_client.AsyncCDGPythonClient)
    async_client._client = FakeSyncClient()
    async_client._native_client = FakeNativeClient()
    async_client._retry_config = (4, 250)
    async_client._log_handler = object()
    async_client._sync_client_proxy = None

    page = await async_client.fetch_page("/bill", offset=20, limit=5)

    assert page == {"path_or_url": "/bill", "offset": 20, "limit": 5}
    assert sync_calls == [
        ("get_timeout",),
        ("get_user_agent",),
        ("is_logging_enabled",),
    ]
    assert native_calls == [
        ("configure_retries", 4, 250),
        ("configure_timeout", 2.5),
        ("configure_user_agent", "cdg-tests/1.0"),
        ("set_log_handler", async_client._log_handler),
        ("fetch_page", "/bill", 20, 5),
    ]


def test_async_native_client_covers_sync_endpoint_surface():
    """Test that all public sync endpoint methods have native async counterparts."""
    from cdg_python_client import AsyncCDGPythonClient, CDGPythonClient

    sync_client = CDGPythonClient(api_key="test_key")
    async_client = AsyncCDGPythonClient(api_key="test_key")

    excluded = {
        "configure_logging",
        "configure_timeout",
        "configure_user_agent",
        "disable_logging",
        "fetch_pages",
        "follow_link",
        "get_timeout",
        "get_user_agent",
        "is_logging_enabled",
        "iter_items",
        "retry_attempts",
        "retry_base_delay_ms",
        "set_retry_config",
    }

    endpoint_methods = {
        name
        for name in dir(sync_client)
        if not name.startswith("_")
        and name not in excluded
        and not name.endswith("_by_link")
        and callable(getattr(sync_client, name))
    }
    missing = sorted(
        name for name in endpoint_methods if not hasattr(async_client._native_client, name)
    )

    assert missing == []


@pytest.mark.asyncio
async def test_async_wrapper_uses_native_dispatch_for_unlisted_native_methods(async_client):
    """Test that wrapper dispatch prefers any callable on the native client, not a stale allowlist."""

    calls = []

    class NativeClient:
        async def list_house_votes(self, *args, **kwargs):
            calls.append(("native", args, kwargs))
            return "native-result"

    class SyncClient:
        def list_house_votes(self, *args, **kwargs):
            calls.append(("sync", args, kwargs))
            return "sync-result"

    async_client._native_client = NativeClient()
    async_client._client = SyncClient()
    async_client._sync_native_runtime_config = lambda: calls.append(("sync-config", (), {}))

    result = await async_client.list_house_votes(offset=5)

    assert result == "native-result"
    assert calls == [("sync-config", (), {}), ("native", (), {"offset": 5})]


@pytest.mark.asyncio
async def test_async_follow_link_uses_native_fetch_page_after_url_resolution():
    """Test that async follow_link resolves Python link helpers, then uses native fetch_page."""
    import cdg_python_client

    calls = []

    class FakeSyncClient:
        def get_timeout(self):
            return None

        def get_user_agent(self):
            return None

        def is_logging_enabled(self):
            return False

    class FakeNativeClient:
        def configure_retries(self, retry_attempts, retry_base_delay_ms):
            calls.append(("configure_retries", retry_attempts, retry_base_delay_ms))

        def configure_timeout(self, timeout_seconds=None):
            calls.append(("configure_timeout", timeout_seconds))

        def configure_user_agent(self, user_agent=None):
            calls.append(("configure_user_agent", user_agent))

        def _set_log_handler(self, handler=None):
            calls.append(("set_log_handler", handler))

        def _clear_log_handler(self):
            calls.append(("clear_log_handler",))

        async def fetch_page(self, path_or_url, offset=None, limit=None):
            calls.append(("fetch_page", path_or_url, offset, limit))
            return {"path_or_url": path_or_url, "offset": offset, "limit": limit}

    class Link:
        urls = [
            "https://api.congress.gov/v3/bill",
            "https://api.congress.gov/v3/member",
        ]

    async_client = object.__new__(cdg_python_client.AsyncCDGPythonClient)
    async_client._client = FakeSyncClient()
    async_client._native_client = FakeNativeClient()
    async_client._retry_config = (3, 1000)
    async_client._log_handler = None
    async_client._sync_client_proxy = None

    link = Link()
    page = await async_client.follow_link(link, offset=10, limit=5, url_index=1)

    assert page == {
        "path_or_url": "https://api.congress.gov/v3/member",
        "offset": 10,
        "limit": 5,
    }
    assert calls[-1] == ("fetch_page", "https://api.congress.gov/v3/member", 10, 5)


def test_async_client_dir_includes_dynamic_surface(async_client):
    """Test that dir(async_client) exposes the discoverable sync and native surfaces."""
    names = set(dir(async_client))

    assert "sync_client" in names
    assert "fetch_page" in names
    assert "follow_link" in names
    assert "list_bills" in names
    assert "list_house_votes" in names
    assert "configure_timeout" in names


def test_client_has_bill_methods():
    """Test that CDGPythonClient has all bill-related methods."""
    from cdg_python_client import CDGPythonClient
    
    client = CDGPythonClient(api_key="test_key")
    
    # Check for bill methods
    assert hasattr(client, "list_bills")
    assert hasattr(client, "list_bills_by_congress")
    assert hasattr(client, "list_bills_by_type")
    assert hasattr(client, "get_bill")
    assert hasattr(client, "get_bill_actions")
    assert hasattr(client, "get_bill_amendments")
    assert hasattr(client, "get_bill_committees")
    assert hasattr(client, "get_bill_cosponsors")
    assert hasattr(client, "get_related_bills")
    assert hasattr(client, "get_bill_subjects")
    assert hasattr(client, "get_bill_summaries")
    assert hasattr(client, "get_bill_text")
    assert hasattr(client, "get_bill_titles")


def test_client_has_amendment_methods():
    """Test that CDGPythonClient has amendment-related methods."""
    from cdg_python_client import CDGPythonClient
    
    client = CDGPythonClient(api_key="test_key")
    
    # Check for amendment methods
    assert hasattr(client, "list_amendments")
    assert hasattr(client, "list_amendments_by_congress")


def test_client_has_member_methods():
    """Test that CDGPythonClient has member-related methods."""
    from cdg_python_client import CDGPythonClient
    
    client = CDGPythonClient(api_key="test_key")
    
    # Check for member methods
    assert hasattr(client, "list_members")
    assert hasattr(client, "get_member")
    assert hasattr(client, "list_members_by_congress")
    assert hasattr(client, "get_member_sponsored_legislation")
    assert hasattr(client, "get_member_cosponsored_legislation")


def test_client_has_committee_methods():
    """Test that CDGPythonClient has committee-related methods."""
    from cdg_python_client import CDGPythonClient
    
    client = CDGPythonClient(api_key="test_key")
    
    # Check for committee methods
    assert hasattr(client, "list_committees")


def test_all_exports():
    """Test that __all__ contains all expected exports."""
    import cdg_python_client
    
    expected_exports = [
        "CDGPythonClient",
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
        "Bill",
        "BillDetail",
        "LatestAction",
        "Law",
        "Sponsor",
        "PolicyArea",
        "Action",
        "Amendment",
        "Committee",
        "Cosponsor",
        "RelatedBill",
        "RelationshipDetail",
        "Subject",
        "Summary",
        "TextVersion",
        "TextFormat",
        "Title",
    ]
    
    for export in expected_exports:
        assert export in cdg_python_client.__all__
        assert hasattr(cdg_python_client, export)
