#!/usr/bin/env python3
import os, requests, json

api_key = os.getenv("CONGRESS_API_KEY")
url = "https://api.congress.gov/v3/house-vote/118/2/135/members"
r = requests.get(url, params={"api_key": api_key, "format": "json", "limit": 2})
print(json.dumps(r.json(), indent=2))
