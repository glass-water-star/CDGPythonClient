# Examples

This folder contains example scripts demonstrating the Congress.gov API client.

## Quick Example

Run a quick example to verify the implementation works:

```bash
# First, build the package
pip install maturin
maturin develop --release

# Then run the example script
export CONGRESS_API_KEY="YOUR_API_KEY"
python examples/test_bills.py
```

Replace `YOUR_API_KEY` with your actual Congress.gov API key. Get one at https://api.data.gov/signup/

## Comprehensive Tests

For more detailed testing of all API endpoints:

```bash
export CONGRESS_API_KEY="your_api_key"
python examples/test_bills.py
```

## Example Scripts

- **test_bills.py** - Example script covering Bills API endpoint usage
