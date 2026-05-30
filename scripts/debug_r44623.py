#!/usr/bin/env python3
"""Debug script to check R44623 specifically."""

import os
import requests
import json

api_key = os.getenv("CONGRESS_API_KEY")
if not api_key:
    print("CONGRESS_API_KEY not set")
    exit(1)

report_id = "R44623"
detail_url = f"https://api.congress.gov/v3/crsreport/{report_id}"
detail_response = requests.get(detail_url, params={"api_key": api_key, "format": "json"})

print(f"Status: {detail_response.status_code}")

if detail_response.status_code == 200:
    detail_data = detail_response.json()
    print(f"\nFull response:")
    print(json.dumps(detail_data, indent=2))
else:
    print(f"Error: {detail_response.text}")
