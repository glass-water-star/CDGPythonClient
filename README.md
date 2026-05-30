# Congress.gov Python Client (CDG Python Client)

A high-performance Python client for the Congress.gov API v3, implemented in Rust using PyO3 for maximum speed and efficiency.

## Overview

This library provides comprehensive access to the U.S. Congress.gov API, allowing you to query bills, laws, members, committees, hearings, votes, and more. Built with Rust and PyO3, it offers native Python bindings with exceptional performance.

## Features

- **🚀 Fast**: Built with Rust for optimal performance
- **📦 Comprehensive**: Full coverage of Congress.gov API v3 endpoints
- **🔒 Type-safe**: Complete type hints for excellent IDE support
- **🐍 Pythonic**: Simple, intuitive Python interface
- **⚡ Zero-copy**: Efficient data structures with minimal overhead

## Supported API Endpoints

- **Bills** - Legislation tracking and details
- **Amendments** - Bill amendments
- **Laws** - Enacted public and private laws
- **Members** - Congressional members and their activities
- **Committees** - Committee information and bills
- **Committee Reports** - Official committee reports
- **Committee Prints** - Committee publications
- **Committee Meetings** - Scheduled and historical committee meetings
- **Hearings** - Congressional hearings
- **House Votes** - House roll call votes (BETA)
- **House Communications** - Executive and other House communications
- **Senate Communications** - Executive and other Senate communications
- **House Requirements** - House reporting requirements and matching communications
- **Nominations** - Presidential nominations
- **Treaties** - Treaty information
- **Daily Congressional Record** - Daily issue listings, issue detail, and articles
- **Bound Congressional Record** - Bound record listings and day detail
- **Congressional Record** - Daily Congressional Record
- **Summaries** - Bill summaries
- **CRS Reports** - Congressional Research Service reports

## Installation

### From PyPI

Published releases are intended to include prebuilt wheels for supported platforms, so most users should only need:

```bash
python -m pip install cdg_python_client
```

### Build from source

You only need Rust when installing from a checkout or when a wheel is not available for your platform.

- Python 3.8 or higher
- Rust toolchain
- A Congress.gov API key ([get one here](https://api.data.gov/signup/))

```bash
# Clone the repository
git clone https://github.com/glass-water-star/CDGPythonClient.git
cd CDGPythonClient

# Build and install with pip
python -m pip install .
```

### Development install

```bash
python -m pip install -e ".[dev]"
maturin develop --release
```

### PyPy

PyO3 0.23 supports PyPy builds, and this package does not use any CPython-only FFI APIs. PyPy builds are interpreter-specific, so build against a PyPy interpreter directly:

```bash
# Source install with PyPy
PYO3_PYTHON=pypy3 pip install .

# Or build a PyPy wheel explicitly
maturin build --release -i pypy3
```

For Linux wheel publishing, build inside a manylinux environment (or use `maturin --zig`) so the resulting wheel is broadly portable.

## Quick Start

```python
from cdg_python_client import CDGPythonClient

# Initialize the client with your API key
client = CDGPythonClient(api_key="your_api_key_here")

# Get recent bills
bills = client.list_bills(limit=10)
for bill in bills:
    print(f"{bill.bill_type} {bill.number}: {bill.title}")

# Get details for a specific bill (bill_number accepts either 1 or "1")
bill = client.get_bill(congress=118, bill_type="hr", bill_number="1")
print(f"Title: {bill.title}")
print(f"Sponsor: {bill.sponsors[0].full_name if bill.sponsors else 'N/A'}")
```

### Async Quick Start

```python
import asyncio

from cdg_python_client import AsyncCDGPythonClient


async def main():
    client = AsyncCDGPythonClient(api_key="your_api_key_here")

    bills = await client.list_bills(limit=10)
    for bill in bills:
        print(f"{bill.bill_type} {bill.number}: {bill.title}")

    bill = await client.get_bill(congress=118, bill_type="hr", bill_number="1")
    print(f"Title: {bill.title}")


asyncio.run(main())
```

## Usage Examples

### Working with Bills

```python
# List bills from the current congress
bills = client.list_bills_by_congress(118, limit=50)

# Get bills by type
senate_bills = client.list_bills_by_type(
    congress=118,
    bill_type="s",  # Senate bills
    limit=100
)

# Get bill actions
actions = client.get_bill_actions(
    congress=118,
    bill_type="hr",
    bill_number="1"
)

# Get bill cosponsors
cosponsors = client.get_bill_cosponsors(
    congress=118,
    bill_type="hr",
    bill_number="1"
)
```

### Working with Laws

> **⚠️ Known Issue**: The Congress.gov API endpoint `/law/{congress}/{lawType}/{lawNumber}` has inconsistent parameter documentation. The swagger documentation claims it uses `lawType` (pub/priv) and `lawNumber`, but the actual API expects `billType` (hr/s/etc.) and `billNumber`. This library correctly implements the actual API behavior, not the documented behavior.

```python
# List all laws from Congress 118
laws = client.list_laws_by_congress(118, limit=50)

# Get public laws only (lawType: "pub" or "priv")
public_laws = client.list_laws_by_type(118, "pub", limit=25)

# Get private laws
private_laws = client.list_laws_by_type(118, "priv", limit=25)

# Get specific law details (by original bill information)
# NOTE: Despite the parameter names, you must provide the BILL type and number,
# not the law type and number. For example, to get Public Law 118-4,
# you need to know it was originally HR 346.
law = client.get_law(
    congress=118,
    law_type="hr",     # Bill type (NOT "pub"/"priv")
    law_number="346"   # Bill number (NOT law number like "4")
)
```

### Working with Members

```python
# List all current members
members = client.list_members(limit=100)

# Get members from a specific congress
members_118 = client.list_members_by_congress(118, limit=100)

# Get member details
member = client.get_member(bioguide_id="B000944")

# Get member's sponsored legislation
sponsored = client.get_member_sponsored_legislation(
    bioguide_id="B000944",
    limit=50
)

# Get member's cosponsored legislation
cosponsored = client.get_member_cosponsored_legislation(
    bioguide_id="B000944",
    limit=50
)
```

### Working with Committees

```python
# List committees by chamber
house_committees = client.list_committees_by_chamber("house", limit=50)
senate_committees = client.list_committees_by_chamber("senate", limit=50)

# Get committee details
committee = client.get_committee(
    chamber="house",
    committee_code="hsif00"  # House Energy and Commerce
)

# Get bills referred to a committee
bills = client.get_committee_bills(
    chamber="house",
    committee_code="hsif00",
    limit=100
)
```

### Working with House Votes (BETA)

```python
# List recent House votes
votes = client.list_house_votes(limit=20)

# Get votes from a specific congress and session
votes_118 = client.list_house_votes_by_session(
    congress=118,
    session=1,
    limit=50
)

# Get vote details
vote = client.get_house_vote(
    congress=118,
    session=1,
    vote_number=100
)

# Get how members voted
members = client.get_house_vote_members(
    congress=118,
    session=1,
    vote_number=100
)
```

### Working with Hearings

```python
# List all hearings
hearings = client.list_hearings(limit=50)

# Get hearings by congress
hearings_118 = client.list_hearings_by_congress(118, limit=50)

# Get hearings by chamber
house_hearings = client.list_hearings_by_chamber(
    congress=118,
    chamber="house",
    limit=50
)

# Get specific hearing
hearing = client.get_hearing(
    congress=118,
    chamber="house",
    jacket_number=51075
)
```

### Working with CRS Reports

```python
# List CRS reports
reports = client.list_crs_reports(limit=25)

# Get CRS report details
report = client.get_crs_report(report_number="R47641")
```

## API Reference

### CDGPythonClient

The main client class providing access to all Congress.gov API endpoints.

#### Initialization

```python
client = CDGPythonClient(api_key="your_api_key")
```

Retry behavior:

- Retryable transport send failures are retried automatically.
- HTTP `429 Too Many Requests` and transient `5xx` responses (`500`, `502`, `503`, `504`) are retried.
- By default, the client makes up to 3 total attempts.
- The delay uses exponential backoff with jitter starting from `retry_base_delay_ms`; when the
  API returns a numeric `Retry-After` header, that delay is honored instead.
- Non-retryable HTTP errors and deserialization errors are returned immediately.

The client raises package-specific exceptions instead of generic runtime errors. The most useful
ones to catch are `CDGRequestError`, `CDGHttpError`, `CDGAuthError`, `CDGNotFoundError`,
`CDGRateLimitError`, `CDGServerError`, `CDGDeserializationError`, and `CDGInvalidUrlError`.

You can also configure per-client HTTP behavior with optional constructor arguments or instance
methods:

```python
from cdg_python_client import CDGPythonClient

client = CDGPythonClient(
    api_key="your_api_key",
    timeout_seconds=10.0,
    user_agent="my-app/1.0",
)

client.configure_timeout(5.0)
client.configure_user_agent("my-app/1.1")
```

You can customize the retry settings after constructing the client:

```python
from cdg_python_client import CDGPythonClient, configure_client_retries

client = CDGPythonClient(api_key="your_api_key")
configure_client_retries(client, 5, 500)
```

#### Optional request logging

Logging is **disabled by default** and is configured per client instance. You can provide either:

- a callback that accepts one event dictionary, or
- a standard-library logger-like object with a `.log(...)` method.

```python
import logging

from cdg_python_client import CDGPythonClient

client = CDGPythonClient(api_key="your_api_key")

# Callback target
client.configure_logging(lambda event: print(event["event"], event["url"]))

# Or a stdlib logger target
logger = logging.getLogger("cdg")
client.configure_logging(logger, level="INFO")
```

Each event contains structured fields including:

- `event` (`request_start`, `request_retry`, `request_success`, `request_http_error`, `request_decode_error`, `request_transport_error`)
- `method`
- redacted `url`
- `path`
- `attempt`
- optional `status_code`, `elapsed_ms`, `error`, `offset`, and `limit`

The client never logs the API key; logged URLs are redacted before the event is emitted.

`AsyncCDGPythonClient` exposes the same `configure_logging(...)`, `disable_logging()`, and
`is_logging_enabled()` methods and uses the same event payload shape.

#### Pagination and follow-up fetch helpers

Both `CDGPythonClient` and `AsyncCDGPythonClient` expose the same pagination story. The sync
client returns pages directly, while the async client exposes awaitable `fetch_page(...)`,
async-iterable `fetch_pages(...)`, and awaitable `follow_link(...)`.

```python
from cdg_python_client import CDGPythonClient

client = CDGPythonClient(api_key="your_api_key")

# Fetch a page from a relative API path
page = client.fetch_page("/bill", limit=5)
print(page.item_key)   # "bills"
print(page.count)      # total result count when provided by the API
print(page.next_url)   # next page URL when available
print(page.items[0]["url"])  # raw JSON-like page items, not typed Bill objects

if page.has_next():
    next_page = client.fetch_page(page.next_url)

# Iterate items directly across pages
for bill in client.iter_items("/bill", limit=5, max_items=10):
    print(bill)

# Iterate pages from a single client entry point
for page in client.fetch_pages("/bill", limit=250, max_pages=2):
    for bill in page.items:
        ...

# Follow a resource link as a raw page
page = client.follow_link("https://api.congress.gov/v3/bill?limit=5")
print(page.items[0]["url"])

# Follow known linked resources and keep typed models
amendment = client.get_amendment(118, "hamdt", "1")
actions = client.get_amendment_actions_by_link(amendment.actions, limit=5)
text_versions = client.get_amendment_text_by_link(amendment.text_versions, limit=5)
```

```python
from cdg_python_client import AsyncCDGPythonClient

client = AsyncCDGPythonClient(api_key="your_api_key")
page = await client.fetch_page("/bill", limit=5)
amendment = await client.get_amendment(118, "hamdt", "1")
actions = await client.get_amendment_actions_by_link(amendment.actions, limit=5)
```

Typed `*_by_link(...)` helpers are available for the linked resources already exposed by the API
models, including amendment actions/text, committee bills/reports/nominations, committee
report/print text links, daily congressional record articles, and treaty actions/committees/parts.

#### Bill Operations

- `list_bills(offset=None, limit=None, ...)` - List all bills sorted by latest action
- `list_bills_by_congress(congress, ...)` - List bills from specific congress
- `list_bills_by_type(congress, bill_type, ...)` - List bills by type (hr, s, hjres, etc.)
- `get_bill(congress, bill_type, bill_number)` - Get detailed bill information
- `get_bill_actions(congress, bill_type, bill_number, ...)` - Get bill actions
- `get_bill_amendments(congress, bill_type, bill_number, ...)` - Get bill amendments
- `get_bill_committees(congress, bill_type, bill_number, ...)` - Get bill committees
- `get_bill_cosponsors(congress, bill_type, bill_number, ...)` - Get bill cosponsors
- `get_related_bills(congress, bill_type, bill_number, ...)` - Get related bills
- `get_bill_subjects(congress, bill_type, bill_number, ...)` - Get bill subjects
- `get_bill_summaries(congress, bill_type, bill_number, ...)` - Get bill summaries
- `get_bill_text(congress, bill_type, bill_number, ...)` - Get bill text versions
- `get_bill_titles(congress, bill_type, bill_number, ...)` - Get bill titles

#### Amendment Operations

- `list_amendments(...)` - List all amendments
- `list_amendments_by_congress(congress, ...)` - List amendments by congress

#### Law Operations

- `list_laws(...)` - List all laws
- `list_laws_by_congress(congress, ...)` - List laws from specific congress
- `list_laws_by_type(congress, law_type, ...)` - List laws by type ("pub" or "priv")
- `get_law(congress, law_type, law_number)` - Get specific law details

#### Member Operations

- `list_members(...)` - List all members
- `list_members_by_congress(congress, ...)` - List members from specific congress
- `list_members_by_state(state, ...)` - List members by state
- `get_member(bioguide_id)` - Get member details
- `get_member_sponsored_legislation(bioguide_id, ...)` - Get member's sponsored bills
- `get_member_cosponsored_legislation(bioguide_id, ...)` - Get member's cosponsored bills

#### Committee Operations

- `list_committees_by_chamber(chamber, ...)` - List committees by chamber
- `list_committees_by_congress(congress, ...)` - List committees by congress
- `list_committees_by_congress_and_chamber(congress, chamber, ...)` - Combined filter
- `get_committee(chamber, committee_code)` - Get committee details
- `get_committee_bills(chamber, committee_code, ...)` - Get committee's bills

#### Committee Report Operations

- `list_committee_reports(...)` - List all committee reports
- `list_committee_reports_by_congress(congress, ...)` - List reports by congress
- `list_committee_reports_by_type(congress, report_type, ...)` - List reports by type
- `get_committee_report(congress, report_type, report_number)` - Get report details
- `get_committee_report_text(congress, report_type, report_number)` - Get report text

#### Committee Print Operations

- `list_committee_prints(...)` - List all committee prints
- `list_committee_prints_by_congress(congress, ...)` - List prints by congress
- `list_committee_prints_by_chamber(congress, chamber, ...)` - List prints by chamber
- `get_committee_print(congress, chamber, jacket_number)` - Get print details
- `get_committee_print_text(congress, chamber, jacket_number)` - Get print text

#### Hearing Operations

- `list_hearings(...)` - List all hearings
- `list_hearings_by_congress(congress, ...)` - List hearings by congress
- `list_hearings_by_chamber(congress, chamber, ...)` - List hearings by chamber
- `get_hearing(congress, chamber, jacket_number)` - Get hearing details

#### House Vote Operations (BETA)

- `list_house_votes(...)` - List all House votes
- `list_house_votes_by_congress(congress, ...)` - List votes by congress
- `list_house_votes_by_session(congress, session, ...)` - List votes by session
- `get_house_vote(congress, session, vote_number)` - Get vote details
- `get_house_vote_members(congress, session, vote_number, ...)` - Get member votes

#### Nomination Operations

- `list_nominations(...)` - List all nominations
- `list_nominations_by_congress(congress, ...)` - List nominations by congress
- `get_nomination(congress, nomination_number)` - Get nomination details
- `get_nomination_nominees(congress, nomination_number, ...)` - Get nominees

#### Treaty Operations

- `list_treaties(...)` - List all treaties
- `list_treaties_by_congress(congress, ...)` - List treaties by congress
- `get_treaty(congress, treaty_number)` - Get treaty details

#### Congressional Record Operations

- `list_congressional_records(...)` - List daily Congressional Records

#### Summary Operations

- `list_summaries(...)` - List all summaries
- `list_summaries_by_congress(congress, ...)` - List summaries by congress

#### CRS Report Operations

- `list_crs_reports(...)` - List CRS reports
- `get_crs_report(report_number)` - Get CRS report details

### Bill Types

- `hr` - House Bill
- `s` - Senate Bill
- `hjres` - House Joint Resolution
- `sjres` - Senate Joint Resolution
- `hconres` - House Concurrent Resolution
- `sconres` - Senate Concurrent Resolution
- `hres` - House Simple Resolution
- `sres` - Senate Simple Resolution

### Common Parameters

Most list methods support these optional parameters:
- `offset` (int) - Pagination offset
- `limit` (int) - Number of results to return (default varies by endpoint)
- `format` (str) - Response format ("json" or "xml")
- `from_date` / `to_date` (str) - Date range filters (format: YYYY-MM-DD)
- `sort` (str) - Sort order

## Data Structures

The library returns Python objects with attributes for easy access:

- **Bill/BillDetail** - Bill information with sponsors, actions, status
- **LawItem/LawDetail** - Law information (bills that became laws)
- **Action** - Legislative actions
- **Amendment** - Bill amendments
- **Committee** - Committee information
- **Cosponsor/Sponsor** - Legislator information
- **HouseVote** - House vote information
- **Hearing** - Hearing information
- **Nomination** - Presidential nomination
- **Treaty** - Treaty information
- **CrsReport** - Congressional Research Service report

All objects use optional fields (can be `None`) since API responses vary by endpoint and data availability.

## Development

### Building

```bash
# Development build (faster, includes debug info)
maturin develop

# Release build (optimized)
maturin develop --release

# Build wheel for distribution
maturin build --release
```

### Testing

```bash
# Build the extension into the active environment first
maturin develop --release

# Run local/unit tests
pytest tests -v

# Build a wheel, install it into an isolated virtualenv, and run the package smoke test
python tests/wheel_smoke.py

# Set your API key in .env or the shell
echo 'CONGRESS_API_KEY="your_api_key"' > .env

# Or export it for the current shell
export CONGRESS_API_KEY="your_api_key"

# Run integration tests
pytest test_integ/ -v

# Run the default configured pytest selection
pytest -v

# Run specific test file
pytest test_integ/test_bills.py -v

# Run a single integration test
pytest test_integ/test_bills.py::TestBillsList::test_list_bills -v

# Run examples
python examples/test_bills.py
```

### Project Structure

```
CDGPythonClient/
├── src/                    # Rust source code
│   ├── bills.rs           # Bill data structures
│   ├── laws.rs            # Law data structures
│   ├── members.rs         # Member data structures
│   ├── committees.rs      # Committee data structures
│   ├── house_votes.rs     # House vote data structures
│   ├── hearings.rs        # Hearing data structures
│   ├── nominations.rs     # Nomination data structures
│   ├── treaties.rs        # Treaty data structures
│   ├── congressional_record.rs
│   ├── summaries.rs
│   ├── sessions.rs
│   ├── client.rs          # Main API client
│   └── lib.rs             # Library entry point
├── cdg_python_client/     # Python package
│   ├── __init__.py
│   └── __init__.pyi       # Type stubs
├── test_integ/            # Integration tests
├── examples/              # Example scripts
├── scripts/               # One-off API/debug probes
├── docs/                  # Documentation
└── pyproject.toml         # Python project config
```

### Release automation

GitHub Actions now builds CPython wheels for Linux, macOS, and Windows plus an sdist on pull requests and pushes. Tags matching `v*` can publish those artifacts to PyPI once trusted publishing is enabled for this repository in PyPI.

Wheel and sdist artifacts are smoke-tested from clean temporary virtual environments outside the repository checkout so local files cannot mask packaging issues.
The source-build test job also runs on a small CPython matrix (`3.8`, `3.12`, `3.13`), and the Linux `abi3` wheel is install-smoke-tested across multiple supported CPython versions (`3.8`, `3.10`, `3.12`, `3.13`).

`pytest tests/test_api_spec_coverage.py -q` compares `src/client.rs` against `docs/swagger.json` and fails if a documented Congress.gov path is missing from the client. It also tracks the small set of intentional non-swagger paths that are kept for compatibility or live-API reasons.

PyPy source builds are documented above. If you want to publish PyPy wheels as well, extend the release workflow with explicit PyPy interpreter jobs.

Release notes are drafted automatically with Release Drafter based on merged pull requests and labels.

## Performance

Built with Rust and PyO3, this library offers:
- **Native speed**: Rust performance with Python convenience
- **Low overhead**: Minimal memory footprint
- **Zero-copy**: Efficient data transfer between Rust and Python
- **Concurrent**: Safe handling of concurrent requests

You can run the included live benchmark harness to compare the Rust/PyO3 client against a simple `requests` baseline on your own machine:

```bash
python -m pip install -e ".[dev]"
python scripts/benchmark_live_api.py --iterations 5 --warmups 1
```

The script benchmarks a small set of representative Congress.gov calls (`fetch_page`, `list_bills`, and `get_bill`) and prints mean/median timings plus a relative speedup ratio. Because it measures real API traffic, the exact numbers depend on network conditions and the remote service; use it as a reproducible end-to-end comparison rather than a fixed universal claim.

## Contributing

Contributions are welcome! Here's how you can help:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests (`pytest test_integ/ -v`)
5. Commit your changes (`git commit -m 'Add amazing feature'`)
6. Push to the branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

### Development Guidelines

- Follow Rust best practices for the Rust code
- Ensure Python type hints are complete
- Add integration tests for new endpoints
- Update documentation for API changes
- Keep the README updated

## License

See [LICENSE](LICENSE) file for details.

## Resources

- **Congress.gov API Documentation**: [GitHub](https://github.com/LibraryOfCongress/api.congress.gov/)
- **Get an API Key**: [api.data.gov](https://api.data.gov/signup/)
- **PyO3 Documentation**: [pyo3.rs](https://pyo3.rs/)
- **Maturin Documentation**: [maturin.rs](https://www.maturin.rs/)

## Troubleshooting

### Import Errors

If you get import errors after installation:
```bash
# Rebuild in release mode
maturin develop --release
```

### API Key Issues

Ensure your API key is valid and properly set:
```python
# Test your API key
client = CDGPythonClient(api_key="your_key")
bills = client.list_bills(limit=1)
```

### Known API Issues

#### Law Detail Endpoint (`/law/{congress}/{lawType}/{lawNumber}`)

The Congress.gov API documentation for this endpoint is incorrect. The swagger documentation states it accepts:
- `lawType`: "pub" or "priv"  
- `lawNumber`: Law number (e.g., "4" for Public Law 118-4)

However, the actual API requires:
- `lawType`: Bill type (e.g., "hr", "s", "hjres", "sjres")
- `lawNumber`: Bill number (e.g., "346" for HR 346)

**This library implements the actual API behavior**, so you must provide bill information, not law information:

```python
# CORRECT: Use the original bill type and number
law = client.get_law(118, "hr", "346")  # Gets law for HR 346

# INCORRECT: Do not use law type and law number
law = client.get_law(118, "pub", "4")   # This will return 404
```

To find which bill became a specific law, first list laws and check the nested `laws` array:
```python
laws = client.list_laws_by_type(118, "pub", limit=10)
for law in laws:
    print(f"Bill {law.bill_type} {law.number} became {law.laws}")
```

### Rate Limiting

The Congress.gov API has rate limits. The client already retries transient `429` and server-side
responses with backoff, but if you continue to encounter rate limits:
- Reduce request frequency
- Contact api.data.gov for higher limits if needed

## Support

For issues, questions, or contributions:
- **Issues**: [GitHub Issues](https://github.com/glass-water-star/CDGPythonClient/issues)
- **Discussions**: [GitHub Discussions](https://github.com/glass-water-star/CDGPythonClient/discussions)

## Acknowledgments

- Library of Congress for providing the Congress.gov API
- PyO3 team for the excellent Rust-Python bindings
- Maturin team for the build tool

---

**Note**: This library is not officially affiliated with Congress.gov or the Library of Congress. It is an independent client implementation of their public API.
