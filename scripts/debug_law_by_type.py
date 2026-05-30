#!/usr/bin/env python3
"""
Debug script to test the /law/{congress}/{lawType} endpoint using requests.
Tests listing laws filtered by congress and law type (public or private).
"""

import os
import requests
import json

def test_law_by_type():
    api_key = os.getenv('CONGRESS_API_KEY')
    if not api_key:
        print("Error: CONGRESS_API_KEY environment variable not set")
        return
    
    base_url = "https://api.congress.gov/v3"
    
    print("=" * 80)
    print("Testing /law/{congress}/{lawType} endpoint")
    print("=" * 80)
    
    # Test 1: Get public laws for Congress 118
    print("\n1. Testing /law/118/pub")
    print("-" * 80)
    url = f"{base_url}/law/118/pub?api_key={api_key}&limit=3"
    print(f"URL: {url}")
    
    response = requests.get(url)
    print(f"Status Code: {response.status_code}")
    
    if response.status_code == 200:
        data = response.json()
        print(f"\nResponse structure:")
        print(f"  Keys: {list(data.keys())}")
        
        if 'bills' in data:
            print(f"  Number of bills: {len(data['bills'])}")
            if data['bills']:
                first_bill = data['bills'][0]
                print(f"\n  First bill structure:")
                print(f"    Keys: {list(first_bill.keys())}")
                print(f"    Congress: {first_bill.get('congress')}")
                print(f"    Bill Type: {first_bill.get('type')}")
                print(f"    Bill Number: {first_bill.get('number')}")
                print(f"    Title: {first_bill.get('title', 'N/A')[:100]}...")
                
                if 'laws' in first_bill and first_bill['laws']:
                    print(f"    Laws:")
                    for law in first_bill['laws']:
                        print(f"      - {law.get('type')} {law.get('number')}")
        
        print(f"\nFull JSON response (first bill only):")
        if 'bills' in data and data['bills']:
            print(json.dumps(data['bills'][0], indent=2))
    else:
        print(f"Error: {response.text}")
    
    # Test 2: Get private laws for Congress 118
    print("\n\n2. Testing /law/118/priv")
    print("-" * 80)
    url = f"{base_url}/law/118/priv?api_key={api_key}&limit=3"
    print(f"URL: {url}")
    
    response = requests.get(url)
    print(f"Status Code: {response.status_code}")
    
    if response.status_code == 200:
        data = response.json()
        print(f"\nResponse structure:")
        print(f"  Keys: {list(data.keys())}")
        
        if 'bills' in data:
            print(f"  Number of bills: {len(data['bills'])}")
            if data['bills']:
                print(f"\n  Private laws found:")
                for bill in data['bills']:
                    print(f"    - {bill.get('type')}-{bill.get('number')}")
                    if 'laws' in bill and bill['laws']:
                        for law in bill['laws']:
                            print(f"      Became: {law.get('type')} {law.get('number')}")
            else:
                print("  No private laws found for Congress 118")
    else:
        print(f"Error: {response.text}")
    
    # Test 3: Try invalid law type
    print("\n\n3. Testing /law/118/invalid (should fail)")
    print("-" * 80)
    url = f"{base_url}/law/118/invalid?api_key={api_key}&limit=1"
    print(f"URL: {url}")
    
    response = requests.get(url)
    print(f"Status Code: {response.status_code}")
    print(f"Response: {response.text[:200]}")
    
    print("\n" + "=" * 80)
    print("Debug test completed")
    print("=" * 80)

if __name__ == "__main__":
    test_law_by_type()
