# Integration Tests

These tests call the real Congress.gov API and validate the returned data.

## Setup

1. Get an API key from [api.data.gov](https://api.data.gov/signup/)

2. Set the environment variable:
   ```bash
   export CONGRESS_API_KEY=your_api_key_here
   ```

## Running Tests

Run all integration tests:
```bash
pytest tests/test_integ/ -v
```

Run specific test file:
```bash
pytest tests/test_integ/test_bills.py -v
```

Run specific test:
```bash
pytest tests/test_integ/test_bills.py::TestBillsList::test_list_bills -v
```

## Test Coverage

The integration tests cover:

- **Bills**: Listing, filtering, getting details, and all sub-endpoints (actions, amendments, committees, cosponsors, subjects, summaries, text, titles)
- **Amendments**: Listing and filtering by congress
- **Members**: Listing, getting details, sponsored/cosponsored legislation
- **Committees**: Listing

## Note

If the `CONGRESS_API_KEY` environment variable is not set, all integration tests will be automatically skipped.
