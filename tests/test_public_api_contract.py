"""Contract tests for the top-level Python package surface."""

from __future__ import annotations

import ast
import importlib
import inspect
from pathlib import Path

import cdg_python_client


PACKAGE_DIR = Path(cdg_python_client.__file__).resolve().parent
ALIASES = {"Title": "BillTitle"}
WRAPPER_ONLY_EXPORTS = {
    "CDGPythonClient",
    "AsyncCDGPythonClient",
    "configure_client_retries",
    "get_client_retry_config",
}


def _extract_export_list(path: Path) -> list[str]:
    module = ast.parse(path.read_text())

    for node in module.body:
        if not isinstance(node, ast.Assign):
            continue

        if not any(isinstance(target, ast.Name) and target.id == "__all__" for target in node.targets):
            continue

        if not isinstance(node.value, ast.List):
            raise AssertionError(f"{path} defines __all__ in an unexpected format")

        return [ast.literal_eval(element) for element in node.value.elts]

    raise AssertionError(f"{path} does not define __all__")


def test_runtime_exports_match_wrapper_and_stub() -> None:
    wrapper_exports = _extract_export_list(PACKAGE_DIR / "__init__.py")
    stub_exports = _extract_export_list(PACKAGE_DIR / "__init__.pyi")

    assert cdg_python_client.__all__ == wrapper_exports == stub_exports


def test_all_runtime_exports_exist() -> None:
    for export_name in cdg_python_client.__all__:
        assert hasattr(cdg_python_client, export_name), export_name


def test_wrapper_exports_resolve_to_compiled_module() -> None:
    extension = importlib.import_module("cdg_python_client.cdg_python_client")

    for export_name in cdg_python_client.__all__:
        if export_name in WRAPPER_ONLY_EXPORTS:
            continue
        compiled_name = ALIASES.get(export_name, export_name)
        assert hasattr(extension, compiled_name), compiled_name
        assert getattr(cdg_python_client, export_name) is getattr(extension, compiled_name)


def test_python_wrappers_hold_native_clients() -> None:
    extension = importlib.import_module("cdg_python_client.cdg_python_client")

    client = cdg_python_client.CDGPythonClient(api_key="test_key")
    async_client = cdg_python_client.AsyncCDGPythonClient(api_key="test_key")

    assert isinstance(client._client, extension.CDGPythonClient)
    assert isinstance(async_client._native_client, extension.AsyncClientCore)


def test_async_client_dir_covers_public_sync_surface() -> None:
    sync_client = cdg_python_client.CDGPythonClient(api_key="test_key")
    async_client = cdg_python_client.AsyncCDGPythonClient(api_key="test_key")

    sync_methods = {
        name
        for name in dir(sync_client)
        if not name.startswith("_") and callable(getattr(sync_client, name))
    }
    async_names = set(dir(async_client))
    missing = sorted(name for name in sync_methods if name not in async_names)

    assert missing == []


def test_async_client_exposes_stable_sync_and_async_method_shapes() -> None:
    async_client = cdg_python_client.AsyncCDGPythonClient(api_key="test_key")

    assert not inspect.iscoroutinefunction(async_client.configure_timeout)
    assert inspect.iscoroutinefunction(async_client.fetch_page)
    assert inspect.iscoroutinefunction(async_client.follow_link)
    assert inspect.iscoroutinefunction(async_client.list_bills)


def test_async_client_signature_parity_for_representative_methods() -> None:
    sync_client = cdg_python_client.CDGPythonClient(api_key="test_key")
    async_client = cdg_python_client.AsyncCDGPythonClient(api_key="test_key")

    representative_methods = [
        "fetch_page",
        "follow_link",
        "get_amendment_actions_by_link",
        "get_committee_bills_by_link",
        "fetch_pages",
        "iter_items",
        "list_bills",
        "get_bill",
        "list_members",
        "get_member_sponsored_legislation",
        "list_committee_reports",
        "get_committee_report",
        "list_house_votes",
        "list_nominations",
        "get_treaty_part_actions",
        "get_treaty_part_by_link",
        "list_house_communications",
        "get_house_requirement_matching_communications",
        "list_crs_reports",
    ]

    for method_name in representative_methods:
        sync_signature = inspect.signature(getattr(sync_client, method_name))
        async_signature = inspect.signature(getattr(async_client, method_name))
        assert async_signature == sync_signature, method_name
