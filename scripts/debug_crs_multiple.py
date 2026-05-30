#!/usr/bin/env python3
"""Debug script to check multiple CRS report structures."""

import os
import requests
import json

api_key = os.getenv("CONGRESS_API_KEY")
if not api_key:
    print("CONGRESS_API_KEY not set")
    exit(1)

# Get a few reports
list_url = "https://api.congress.gov/v3/crsreport"
response = requests.get(list_url, params={"api_key": api_key, "limit": 5, "format": "json"})

if response.status_code == 200:
    data = response.json()
    reports = data.get("CRSReports", [])
    
    print(f"Testing {len(reports)} reports:\n")
    
    for i, report in enumerate(reports):
        report_id = report.get("id")
        print(f"\n{i+1}. Testing report ID: {report_id}")
        
        # Try to get detail
        detail_url = f"https://api.congress.gov/v3/crsreport/{report_id}"
        detail_response = requests.get(detail_url, params={"api_key": api_key, "format": "json"})
        
        if detail_response.status_code == 200:
            detail_data = detail_response.json()
            crs_report = detail_data.get("CRSReport", {})
            
            print(f"   Status: OK")
            print(f"   Keys: {list(crs_report.keys())}")
            
            # Check for optional fields
            if "title" in crs_report:
                print(f"   Has title: {crs_report['title'] is not None}")
            if "authors" in crs_report:
                print(f"   Authors count: {len(crs_report['authors']) if crs_report['authors'] else 0}")
            if "topics" in crs_report:
                print(f"   Topics count: {len(crs_report['topics']) if crs_report['topics'] else 0}")
        else:
            print(f"   Error: {detail_response.status_code}")
            print(f"   {detail_response.text[:200]}")
else:
    print(f"List error: {response.text}")
