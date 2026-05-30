#!/usr/bin/env python3
import os, requests, json

api_key = os.getenv("CONGRESS_API_KEY")
url = "https://api.congress.gov/v3/house-vote/119/1/240"
r = requests.get(url, params={"api_key": api_key, "format": "json"})
print(json.dumps(r.json(), indent=2))
