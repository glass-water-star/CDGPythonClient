#!/usr/bin/env python3
"""Debug script to check house votes API responses."""

import os
import requests
import json

api_key = os.getenv("CONGRESS_API_KEY")
if not api_key:
    print("CONGRESS_API_KEY not set")
    exit(1)

# Test list endpoint
print("=" * 80)
print("LIST HOUSE VOTES")
print("=" * 80)
list_url = "https://api.congress.gov/v3/house-vote"
list_response = requests.get(list_url, params={"api_key": api_key, "format": "json", "limit": 1})
print(f"Status: {list_response.status_code}\n")
if list_response.status_code == 200:
    data = list_response.json()
    print(json.dumps(data, indent=2))
    
    # Get first vote for detail test
    if data.get("votes") and len(data["votes"]) > 0:
        first_vote = data["votes"][0]
        congress = first_vote.get("congress")
        session = first_vote.get("session")
        vote_number = first_vote.get("number")
        
        if congress and session and vote_number:
            print("\n" + "=" * 80)
            print(f"DETAIL: Congress {congress}, Session {session}, Vote {vote_number}")
            print("=" * 80)
            detail_url = f"https://api.congress.gov/v3/house-vote/{congress}/{session}/{vote_number}"
            detail_response = requests.get(detail_url, params={"api_key": api_key, "format": "json"})
            print(f"Status: {detail_response.status_code}\n")
            if detail_response.status_code == 200:
                print(json.dumps(detail_response.json(), indent=2))
                
            # Also get members endpoint
            print("\n" + "=" * 80)
            print(f"MEMBERS: Congress {congress}, Session {session}, Vote {vote_number}")
            print("=" * 80)
            members_url = f"https://api.congress.gov/v3/house-vote/{congress}/{session}/{vote_number}/members"
            members_response = requests.get(members_url, params={"api_key": api_key, "format": "json", "limit": 2})
            print(f"Status: {members_response.status_code}\n")
            if members_response.status_code == 200:
                print(json.dumps(members_response.json(), indent=2))
else:
    print(f"Error: {list_response.text}")
