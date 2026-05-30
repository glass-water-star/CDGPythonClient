#!/usr/bin/env python3
"""Benchmark the Rust/PyO3 client against a simple requests baseline."""

from __future__ import annotations

import argparse
import os
import statistics
import sys
import time
from collections.abc import Callable
from pathlib import Path
from typing import Any

try:
    import requests
except ImportError as exc:  # pragma: no cover - convenience for manual use
    raise SystemExit(
        "The benchmark script requires `requests`. Install dev dependencies first with "
        "`python -m pip install -e \".[dev]\"`."
    ) from exc

try:
    from cdg_python_client import CDGPythonClient
except ImportError as exc:  # pragma: no cover - convenience for manual use
    raise SystemExit(
        "cdg_python_client is not installed. Build it first with `maturin develop --release`."
    ) from exc


BASE_URL = "https://api.congress.gov/v3"


def load_api_key() -> str | None:
    for key in ("CONGRESS_API_KEY", "API_KEY"):
        value = os.environ.get(key)
        if value and value != "your_api_key_here":
            return value

    env_path = Path(__file__).resolve().parents[1] / ".env"
    if not env_path.exists():
        return None

    for raw_line in env_path.read_text().splitlines():
        line = raw_line.strip()
        if not line or line.startswith("#") or "=" not in line:
            continue
        key, value = line.split("=", 1)
        key = key.strip()
        value = value.strip().strip('"').strip("'")
        if key in {"CONGRESS_API_KEY", "API_KEY"} and value and value != "your_api_key_here":
            return value

    return None


def requests_json(
    session: requests.Session,
    api_key: str,
    path: str,
    *,
    params: dict[str, Any] | None = None,
) -> Any:
    merged_params = {"api_key": api_key, "format": "json"}
    if params:
        merged_params.update(params)

    response = session.get(f"{BASE_URL}{path}", params=merged_params, timeout=30)
    response.raise_for_status()
    return response.json()


def rust_fetch_page(client: CDGPythonClient, path: str, *, limit: int | None = None) -> Any:
    return client.fetch_page(path, limit=limit)


def rust_list_bills(client: CDGPythonClient, *, limit: int) -> Any:
    return client.list_bills(limit=limit)


def rust_get_bill(client: CDGPythonClient, *, congress: int, bill_type: str, bill_number: str) -> Any:
    return client.get_bill(congress=congress, bill_type=bill_type, bill_number=bill_number)


def baseline_fetch_page(session: requests.Session, api_key: str, path: str, *, limit: int | None = None) -> Any:
    return requests_json(session, api_key, path, params={"limit": limit} if limit is not None else None)


def baseline_list_bills(session: requests.Session, api_key: str, *, limit: int) -> Any:
    return requests_json(session, api_key, "/bill", params={"limit": limit})


def baseline_get_bill(
    session: requests.Session,
    api_key: str,
    *,
    congress: int,
    bill_type: str,
    bill_number: str,
) -> Any:
    return requests_json(session, api_key, f"/bill/{congress}/{bill_type}/{bill_number}")


def run_case(
    name: str,
    rust_callable: Callable[[], Any],
    baseline_callable: Callable[[], Any],
    *,
    iterations: int,
    warmups: int,
) -> dict[str, float]:
    for _ in range(warmups):
        rust_callable()
        baseline_callable()

    rust_samples: list[float] = []
    baseline_samples: list[float] = []

    for _ in range(iterations):
        start = time.perf_counter()
        rust_callable()
        rust_samples.append(time.perf_counter() - start)

        start = time.perf_counter()
        baseline_callable()
        baseline_samples.append(time.perf_counter() - start)

    rust_mean = statistics.mean(rust_samples)
    baseline_mean = statistics.mean(baseline_samples)

    return {
        "name": name,
        "rust_mean_ms": rust_mean * 1000,
        "baseline_mean_ms": baseline_mean * 1000,
        "rust_median_ms": statistics.median(rust_samples) * 1000,
        "baseline_median_ms": statistics.median(baseline_samples) * 1000,
        "speedup": baseline_mean / rust_mean if rust_mean else float("inf"),
    }


def print_results(results: list[dict[str, float]]) -> None:
    header = (
        f"{'case':<18} {'rust mean ms':>14} {'requests mean ms':>18} "
        f"{'rust median ms':>16} {'requests median ms':>20} {'speedup':>10}"
    )
    print(header)
    print("-" * len(header))

    for row in results:
        print(
            f"{row['name']:<18} "
            f"{row['rust_mean_ms']:>14.2f} "
            f"{row['baseline_mean_ms']:>18.2f} "
            f"{row['rust_median_ms']:>16.2f} "
            f"{row['baseline_median_ms']:>20.2f} "
            f"{row['speedup']:>9.2f}x"
        )


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Benchmark the Rust/PyO3 client against a simple requests baseline."
    )
    parser.add_argument("--iterations", type=int, default=5, help="timed iterations per case")
    parser.add_argument("--warmups", type=int, default=1, help="warmup iterations per case")
    parser.add_argument(
        "--bill-limit",
        type=int,
        default=20,
        help="limit used for list-bills/fetch-page benchmark cases",
    )
    parser.add_argument("--congress", type=int, default=118, help="bill congress for detail benchmark")
    parser.add_argument("--bill-type", default="hr", help="bill type for detail benchmark")
    parser.add_argument("--bill-number", default="1", help="bill number for detail benchmark")
    return parser.parse_args()


def main() -> None:
    args = parse_args()
    api_key = load_api_key()
    if not api_key:
        raise SystemExit(
            "No API key found. Set CONGRESS_API_KEY or API_KEY, or place one in the repo .env file."
        )

    client = CDGPythonClient(api_key=api_key)
    session = requests.Session()

    cases = [
        run_case(
            "fetch_page",
            lambda: rust_fetch_page(client, "/bill", limit=args.bill_limit),
            lambda: baseline_fetch_page(session, api_key, "/bill", limit=args.bill_limit),
            iterations=args.iterations,
            warmups=args.warmups,
        ),
        run_case(
            "list_bills",
            lambda: rust_list_bills(client, limit=args.bill_limit),
            lambda: baseline_list_bills(session, api_key, limit=args.bill_limit),
            iterations=args.iterations,
            warmups=args.warmups,
        ),
        run_case(
            "get_bill",
            lambda: rust_get_bill(
                client,
                congress=args.congress,
                bill_type=args.bill_type,
                bill_number=args.bill_number,
            ),
            lambda: baseline_get_bill(
                session,
                api_key,
                congress=args.congress,
                bill_type=args.bill_type,
                bill_number=args.bill_number,
            ),
            iterations=args.iterations,
            warmups=args.warmups,
        ),
    ]

    print(
        "Benchmarking live Congress.gov API calls. Results are environment-dependent and include "
        "network latency, JSON decoding, and client-side conversion overhead.\n"
    )
    print_results(cases)


if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        sys.exit(130)
