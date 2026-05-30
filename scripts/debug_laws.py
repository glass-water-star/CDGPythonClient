#!/usr/bin/env python3
import os, requests, json

api_key = os.getenv("CONGRESS_API_KEY")

# print("\n2. /law/{congress}/{lawType}")
# url = "https://api.congress.gov/v3/law/100/priv"
# r = requests.get(url, params={"api_key": api_key, "format": "json", "limit": 10})
# print(f"Status: {r.status_code}")
# if r.status_code == 200:
#     data = r.json()
#     print(f"Keys: {list(data.keys())}")
#     print(json.dumps(data, indent=2))  # Print first 2000 chars
# for i in range(1,500):
#     url = f"https://api.congress.gov/v3/law/118/priv/{i}"
#     r = requests.get(url, params={"api_key": api_key, "format": "json"})
#     print(f"Status: {r.status_code}")
#     if r.status_code == 200:
#         data = r.json()
#         print(f"Keys: {list(data.keys())}")
#         print(json.dumps(data, indent=2))  # Print first 2000 chars


r = requests.get("https://api.congress.gov/v3/bill/100/s/423?format=json", params={"api_key": api_key, "format": "json", "limit": 10})
print(r.json())