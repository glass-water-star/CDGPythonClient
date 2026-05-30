"""Smoke-check an installed package from outside the repository checkout."""

from __future__ import annotations

import asyncio
import importlib
import os
import pathlib


def main() -> None:
    import cdg_python_client
    from cdg_python_client import (
        AsyncCDGPythonClient,
        Bill,
        CDGPythonClient,
        Title,
        configure_client_retries,
        get_client_retry_config,
    )

    extension = importlib.import_module("cdg_python_client.cdg_python_client")

    client = CDGPythonClient(api_key="test_key")
    configure_client_retries(client, 5, 250)

    assert get_client_retry_config(client) == (5, 250)
    assert Title is getattr(extension, "BillTitle")
    assert getattr(cdg_python_client, "Bill") is Bill
    assert getattr(extension, "CDGPythonClient") is not CDGPythonClient
    assert isinstance(client._client, getattr(extension, "CDGPythonClient"))
    assert getattr(cdg_python_client, "AsyncCDGPythonClient") is AsyncCDGPythonClient

    async def exercise_async_surface() -> None:
        class FakePage:
            def __init__(self, items, next_url):
                self.items = items
                self.next_url = next_url

            def has_next(self):
                return self.next_url is not None

        pages = {
            "/bill": FakePage(
                [
                    {"url": "https://api.congress.gov/v3/bill/118/hr/1"},
                    {"url": "https://api.congress.gov/v3/bill/118/hr/2"},
                ],
                "next-1",
            ),
            "next-1": FakePage([{"url": "https://api.congress.gov/v3/bill/118/hr/3"}], None),
            "https://api.congress.gov/v3/bill": FakePage(
                [{"url": "https://api.congress.gov/v3/bill/118/hr/4"}],
                None,
            ),
        }

        class FakeSyncClient:
            def get_timeout(self):
                return None

            def get_user_agent(self):
                return None

            def is_logging_enabled(self):
                return False

        class FakeNativeClient:
            def configure_retries(self, retry_attempts, retry_base_delay_ms):
                assert (retry_attempts, retry_base_delay_ms) == (3, 1000)

            def configure_timeout(self, timeout_seconds=None):
                assert timeout_seconds is None

            def configure_user_agent(self, user_agent=None):
                assert user_agent is None

            def _set_log_handler(self, handler=None):
                raise AssertionError("logging should stay disabled")

            def _clear_log_handler(self):
                pass

            async def list_bills(self, limit=None):
                return [f"native-limit={limit}"]

            async def fetch_page(self, path_or_url, offset=None, limit=None):
                return pages[path_or_url]

        async_client = AsyncCDGPythonClient(api_key="test_key")
        async_client._client = FakeSyncClient()
        async_client._native_client = FakeNativeClient()

        assert "list_bills" in dir(async_client)
        assert "follow_link" in dir(async_client)
        assert await async_client.list_bills(limit=5) == ["native-limit=5"]
        assert (await async_client.follow_link("https://api.congress.gov/v3/bill", limit=1)).items == [
            {"url": "https://api.congress.gov/v3/bill/118/hr/4"}
        ]

        pages_seen = []
        async for page in async_client.fetch_pages("/bill", limit=2, max_pages=2):
            pages_seen.append(page.items)

        flattened = []
        async for item in async_client.iter_items("/bill", limit=2, max_items=2):
            flattened.append(item)

        assert pages_seen == [
            [
                {"url": "https://api.congress.gov/v3/bill/118/hr/1"},
                {"url": "https://api.congress.gov/v3/bill/118/hr/2"},
            ],
            [{"url": "https://api.congress.gov/v3/bill/118/hr/3"}],
        ]
        assert flattened == [
            {"url": "https://api.congress.gov/v3/bill/118/hr/1"},
            {"url": "https://api.congress.gov/v3/bill/118/hr/2"},
        ]

    asyncio.run(exercise_async_surface())

    package_path = pathlib.Path(cdg_python_client.__file__).resolve()
    if os.environ.get("CDG_EXPECT_SITE_PACKAGES") == "1":
        repo_root = pathlib.Path(os.environ["CDG_REPO_ROOT"]).resolve()
        try:
            package_path.relative_to(repo_root)
        except ValueError:
            pass
        else:
            raise AssertionError(
                f"Expected an installed wheel import, but loaded package from repository checkout: {package_path}"
            )

    print(f"Imported package from: {package_path}")
    print(f"Export count: {len(cdg_python_client.__all__)}")


if __name__ == "__main__":
    main()
