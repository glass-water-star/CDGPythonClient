# Copilot instructions for CDGPythonClient

## Build and test commands

- `maturin develop --release` builds the Rust extension and installs it into the active Python environment. Use this after changing Rust code that backs the Python package.
- `maturin develop` is the faster debug build for local iteration.
- `maturin build --release` builds a wheel for distribution.
- `python -m pip install .` builds and installs the package through the PEP 517 path; use this when validating installability rather than only local development builds.
- `pytest tests -q` runs the local smoke tests that validate imports and the exposed Python surface without calling the real API.
- `pytest tests/test_api_spec_coverage.py -q` verifies that the implemented endpoint templates in `src/client.rs` still cover every documented path in `docs/swagger.json`.
- `pytest tests/test_imports.py::test_import_main_client -q` runs a single local smoke test.
- `export CONGRESS_API_KEY=your_api_key && pytest test_integ/ -v` runs the live integration suite against Congress.gov.
- `export CONGRESS_API_KEY=your_api_key && pytest test_integ/test_bills.py::TestBillsList::test_list_bills -v` runs a single live integration test.
- `python tests/package_smoke.py` is the installed-package smoke check used by release automation after wheel/sdist installation from a temporary directory.
- `python scripts/benchmark_live_api.py --iterations 5 --warmups 1` runs the simple live benchmark harness comparing the Rust/PyO3 client against a `requests` baseline. It expects dev dependencies plus a real API key.
- `cargo test --lib` runs the Rust transport/error tests for retry behavior and malformed payload handling.

## High-level architecture

- The Rust crate under `src/` is the source of truth for the library. `src/client.rs` contains both the internal `CongressApiClient` HTTP layer and the `#[pyclass] CDGPythonClient` methods exposed to Python.
- `CongressApiClient` uses blocking `reqwest`, always appends the API key, retries HTTP 503 responses with short backoff, and deserializes JSON into typed Rust response structs before converting failures to Python exceptions.
- Endpoint families are split by domain into separate Rust modules such as `bills.rs`, `members.rs`, `committees.rs`, `laws.rs`, `hearings.rs`, and `crsreport.rs`. Those files define the serde-backed `#[pyclass]` models plus the internal response wrapper structs each client method deserializes into.
- `src/lib.rs` is the PyO3 module registration layer. Any new class that should be importable from Python must be added there.
- The Python package is intentionally thin: `cdg_python_client/__init__.py` just re-exports the compiled extension, while `cdg_python_client/__init__.pyi` is the typing/public-API layer that editors and type checkers rely on.
- Tests are split by purpose: `tests/` is the fast, no-network smoke suite for imports and public surface checks; `test_integ/` exercises live Congress.gov endpoints and is skipped when `CONGRESS_API_KEY` is not set.
- One-off investigation helpers belong under `scripts/` instead of the repository root so they do not get confused with distributable package files.

## Key conventions

- Keep the Python-facing API snake_case even when Congress.gov returns camelCase or fields named `type`. The Rust models handle translation with `serde(rename = ...)`, and Python consumers should continue to see names like `bill_type`, `origin_chamber`, `update_date`, and `from_date_time`.
- When adding or changing an endpoint, the usual touch points are: the domain model file in `src/`, the method in `src/client.rs`, the PyO3 registration in `src/lib.rs`, and the public typing surface in `cdg_python_client/__init__.pyi`.
- Most API fields are intentionally optional because Congress.gov responses vary by endpoint and by record. Preserve `Option<T>` usage unless the API consistently guarantees a field.
- Match real API payloads instead of trusting the Swagger docs. This codebase already carries endpoint-specific accommodations, such as the law detail endpoint using the originating bill type/number and CRS report parsing accepting values that may arrive as either strings or integers.
- `docs/swagger.json` is the local reference used for endpoint coverage checks. When Congress.gov updates the spec or this client intentionally keeps a non-swagger path for compatibility, update `tests/test_api_spec_coverage.py` alongside the client changes.
- Surface request and deserialization failures clearly. The current public Python API maps them into typed exceptions such as `CDGConfigurationError`, `CDGRequestError`, `CDGHttpError`, `CDGNotFoundError`, `CDGRateLimitError`, `CDGServerError`, `CDGDeserializationError`, and `CDGInvalidUrlError`.
- Integration tests are the main place to capture endpoint quirks and regressions. When behavior changes for a live endpoint, update or extend `test_integ/` alongside the Rust and typing changes.
- Python pagination ergonomics should stay consistent across sync and async clients. If you add page-level helpers on `CDGPythonClient`, mirror the corresponding item/page iteration helpers on `AsyncCDGPythonClient` and update the stubs/tests together.
- Sync and async clients should keep configuration surfaces aligned. Retry, timeout, user-agent, and logging changes should behave consistently across `CDGPythonClient` and `AsyncCDGPythonClient`.
- Packaging uses maturin mixed-project mode with `module-name = "cdg_python_client.cdg_python_client"`. Release automation currently targets CPython `abi3` wheels plus an sdist; PyPy is treated as a source-build path unless dedicated PyPy wheel jobs are added.
