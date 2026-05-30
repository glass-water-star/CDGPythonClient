# Examples

This folder contains example scripts demonstrating the Congress.gov API client.

## Quick Test

Run a quick test to verify the implementation works:

```bash
# First, build the package
pip install maturin
maturin develop --release

# Then run the quick test
python examples/quick_test.py YOUR_API_KEY
```

Replace `YOUR_API_KEY` with your actual Congress.gov API key. Get one at https://api.data.gov/signup/

## Comprehensive Tests

For more detailed testing of all API endpoints:

```bash
export CONGRESS_API_KEY="your_api_key"
python examples/test_bills.py
```

## Example Scripts

- **quick_test.py** - Simple script to quickly verify the implementation works
- **test_bills.py** - Comprehensive test of all Bills API endpoints
