# Integration Tests

These tests call the real Congress.gov API and validate the returned data.

## Setup

1. Get an API key from [api.data.gov](https://api.data.gov/signup/)

2. Add the API key to the repository `.env` file or export it directly:
   ```bash
   # .env
   CONGRESS_API_KEY=your_api_key_here

   # or shell
   export CONGRESS_API_KEY=your_api_key_here
   ```

## Running Tests

Run all integration tests:
```bash
pytest test_integ/ -v
```

The integration fixtures cache identical live requests for the duration of the
test session and apply a small delay between unique live calls by default. This
keeps the suite closer to a coverage pass than a load test while still hitting
real endpoints.

Run specific test file:
```bash
pytest test_integ/test_bills.py -v
```

Run specific test:
```bash
pytest test_integ/test_bills.py::TestBillsList::test_list_bills -v
```

## Test Coverage

The integration tests cover:

- **Bills**: Listing, filtering, getting details, and all sub-endpoints (actions, amendments, committees, cosponsors, subjects, summaries, text, titles)
- **Amendments**: Listing and filtering by congress
- **Members**: Listing, getting details, sponsored/cosponsored legislation
- **Committees**: Listing

## Note

If neither `CONGRESS_API_KEY` nor `API_KEY` is set in the environment or `.env`, all integration tests will be automatically skipped.

## API Load Controls

By default, unique live requests are spaced by 100ms and repeated identical
requests are served from an in-memory session cache.

You can tune the pacing if needed:

```bash
CDG_INTEG_MIN_DELAY_MS=250 pytest test_integ/ -v
```

Set `CDG_INTEG_MIN_DELAY_MS=0` to disable the delay while keeping request
caching.
