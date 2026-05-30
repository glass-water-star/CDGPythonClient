#!/usr/bin/env python3
"""Debug script to check actual CRS report API responses."""

import os
import requests
import json

api_key = os.getenv("CONGRESS_API_KEY")
if not api_key:
    print("CONGRESS_API_KEY not set")
    exit(1)

# First, get a list to find a valid report ID
list_url = "https://api.congress.gov/v3/crsreport"
response = requests.get(list_url, params={"api_key": api_key, "limit": 1, "format": "json"})
print(f"List response status: {response.status_code}")

if response.status_code == 200:
    data = response.json()
    print(f"\nList response keys: {list(data.keys())}")
    
    if "CRSReports" in data and len(data["CRSReports"]) > 0:
        report_id = data["CRSReports"][0].get("id")
        print(f"\nFound report ID: {report_id}")
        
        # Now get the detail
        detail_url = f"https://api.congress.gov/v3/crsreport/{report_id}"
        detail_response = requests.get(detail_url, params={"api_key": api_key, "format": "json"})
        print(f"\nDetail response status: {detail_response.status_code}")
        
        if detail_response.status_code == 200:
            detail_data = detail_response.json()
            print(f"\nDetail response structure:")
            print(json.dumps(detail_data, indent=2)[:2000])  # Print first 2000 chars
        else:
            print(f"Detail error: {detail_response.text}")
else:
    print(f"List error: {response.text}")
