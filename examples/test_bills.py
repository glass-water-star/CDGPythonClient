#!/usr/bin/env python3
"""
Example script demonstrating the Bills API client functionality.
"""

import os
import sys

try:
    from cdg_python_client import CDGPythonClient
except ImportError:
    print("Error: cdg_python_client not found. Please build the package first:")
    print("  maturin develop --release")
    sys.exit(1)


def main():
    # Get API key from environment
    api_key = os.environ.get("CONGRESS_API_KEY")
    if not api_key:
        print("Error: CONGRESS_API_KEY environment variable not set")
        print("Get your API key at: https://api.data.gov/signup/")
        sys.exit(1)
    
    print("Initializing Congress.gov API client...")
    client = CDGPythonClient(api_key=api_key)
    
    print("\n" + "="*80)
    print("TEST 1: List Recent Bills")
    print("="*80)
    try:
        bills = client.list_bills(limit=5)
        print(f"Retrieved {len(bills)} bills:")
        for i, bill in enumerate(bills, 1):
            print(f"\n{i}. {bill.bill_type.upper()} {bill.number}")
            print(f"   Title: {bill.title}")
            print(f"   Congress: {bill.congress}")
            if bill.latest_action:
                print(f"   Latest Action: {bill.latest_action.text}")
    except Exception as e:
        print(f"Error: {e}")

    print("\n" + "="*80)
    print("TEST 9: Iterate Across Pages")
    print("="*80)
    try:
        for i, bill in enumerate(client.iter_items("/bill", limit=2, max_items=4), 1):
            print(f"{i}. {bill.get('url')}")
    except Exception as e:
        print(f"Error: {e}")
    
    print("\n" + "="*80)
    print("TEST 2: Get Bills from 118th Congress")
    print("="*80)
    try:
        bills = client.list_bills_by_congress(congress=118, limit=3)
        print(f"Retrieved {len(bills)} bills from the 118th Congress:")
        for i, bill in enumerate(bills, 1):
            print(f"\n{i}. {bill.bill_type.upper()} {bill.number}: {bill.title}")
    except Exception as e:
        print(f"Error: {e}")
    
    print("\n" + "="*80)
    print("TEST 3: Get Senate Bills from 118th Congress")
    print("="*80)
    try:
        bills = client.list_bills_by_type(congress=118, bill_type="s", limit=3)
        print(f"Retrieved {len(bills)} Senate bills:")
        for i, bill in enumerate(bills, 1):
            print(f"\n{i}. S {bill.number}: {bill.title}")
    except Exception as e:
        print(f"Error: {e}")
    
    print("\n" + "="*80)
    print("TEST 4: Get Detailed Bill Information")
    print("="*80)
    try:
        # Get a specific bill (adjust these values as needed)
        bill = client.get_bill(congress=118, bill_type="hr", bill_number=1)
        print(f"Bill: {bill.bill_type.upper()} {bill.number}")
        print(f"Title: {bill.title}")
        print(f"Introduced: {bill.introduced_date}")
        print(f"Origin Chamber: {bill.origin_chamber}")
        if bill.sponsors:
            print(f"Sponsors: {len(bill.sponsors)}")
            for sponsor in bill.sponsors[:3]:  # Show first 3
                print(f"  - {sponsor.full_name} ({sponsor.party}-{sponsor.state})")
        if bill.policy_area:
            print(f"Policy Area: {bill.policy_area.name}")
    except Exception as e:
        print(f"Error: {e}")
    
    print("\n" + "="*80)
    print("TEST 5: Get Bill Actions")
    print("="*80)
    try:
        actions = client.get_bill_actions(
            congress=118,
            bill_type="hr",
            bill_number=1,
            limit=5
        )
        print(f"Retrieved {len(actions)} actions:")
        for i, action in enumerate(actions, 1):
            print(f"\n{i}. {action.action_date}")
            print(f"   Type: {action.action_type}")
            print(f"   {action.text[:100]}..." if len(action.text or "") > 100 else f"   {action.text}")
    except Exception as e:
        print(f"Error: {e}")
    
    print("\n" + "="*80)
    print("TEST 6: Get Bill Cosponsors")
    print("="*80)
    try:
        cosponsors = client.get_bill_cosponsors(
            congress=118,
            bill_type="hr",
            bill_number=1,
            limit=5
        )
        print(f"Retrieved {len(cosponsors)} cosponsors:")
        for i, cosponsor in enumerate(cosponsors, 1):
            original = "Original" if cosponsor.is_original_cosponsor else "Added"
            print(f"{i}. {cosponsor.full_name} ({cosponsor.party}-{cosponsor.state}) - {original}")
            print(f"   Sponsored: {cosponsor.sponsorship_date}")
    except Exception as e:
        print(f"Error: {e}")
    
    print("\n" + "="*80)
    print("TEST 7: Get Bill Subjects")
    print("="*80)
    try:
        subjects = client.get_bill_subjects(
            congress=118,
            bill_type="hr",
            bill_number=1
        )
        print(f"Retrieved {len(subjects)} subjects:")
        for i, subject in enumerate(subjects[:10], 1):  # Show first 10
            print(f"{i}. {subject.name}")
    except Exception as e:
        print(f"Error: {e}")
    
    print("\n" + "="*80)
    print("TEST 8: Get Bill Summaries")
    print("="*80)
    try:
        summaries = client.get_bill_summaries(
            congress=118,
            bill_type="hr",
            bill_number=1
        )
        print(f"Retrieved {len(summaries)} summaries:")
        for i, summary in enumerate(summaries, 1):
            print(f"\n{i}. {summary.action_desc} ({summary.action_date})")
            print(f"   Version: {summary.version_code}")
            if summary.text:
                preview = summary.text[:200] + "..." if len(summary.text) > 200 else summary.text
                print(f"   {preview}")
    except Exception as e:
        print(f"Error: {e}")
    
    print("\n" + "="*80)
    print("All tests completed!")
    print("="*80)


if __name__ == "__main__":
    main()
