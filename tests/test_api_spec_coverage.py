"""Guard against drift between the documented Congress.gov API and the client."""

from __future__ import annotations

import json
import re
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
CLIENT_SOURCE = REPO_ROOT / "src" / "client.rs"
SWAGGER_PATH = REPO_ROOT / "docs" / "swagger.json"

KNOWN_EXTRA_CLIENT_PATHS = {
    "/law",
    "/nomination/{param}/{param}/nominees",
}


def _normalize_spec_path(path: str) -> str:
    return re.sub(r"\{[^}/]+\}", "{param}", path)


def _normalize_client_path(path: str) -> str:
    return path.replace("{}", "{param}")


def _normalize_param_name(name: str) -> str:
    return re.sub(r"(?<!^)(?=[A-Z])", "_", name).lower()


def _extract_client_paths(source: str) -> set[str]:
    paths: set[str] = set()

    for pattern in (
        r'format!\(\s*"([^"]+)"',
        r'\.get\(\s*"([^"]+)"',
    ):
        for match in re.finditer(pattern, source, re.MULTILINE | re.DOTALL):
            candidate = match.group(1)
            if not candidate.startswith("/"):
                continue

            normalized = _normalize_client_path(candidate)
            if normalized == "/{param}":
                continue

            paths.add(normalized)

    return paths


def _extract_spec_paths(swagger_text: str) -> set[str]:
    spec = json.loads(swagger_text)
    return {_normalize_spec_path(path) for path in spec["paths"]}


def _extract_client_methods(source: str) -> list[dict[str, object]]:
    methods: list[dict[str, object]] = []
    lines = source.splitlines()
    signature_re = re.compile(r'#\[pyo3\(signature = \((?P<signature>.*)\)\)\]')
    fn_re = re.compile(r"\s*pub fn (?P<name>\w+)")
    path_re = re.compile(r'(?:format!\(\s*"([^"]+)"|\.get\(\s*"([^"]+)")')

    index = 0
    while index < len(lines):
        signature_match = signature_re.search(lines[index])
        if signature_match is None:
            index += 1
            continue

        fn_index = index + 1
        while fn_index < len(lines) and "pub fn " not in lines[fn_index]:
            fn_index += 1
        if fn_index >= len(lines):
            break

        fn_match = fn_re.search(lines[fn_index])
        if fn_match is None:
            index = fn_index + 1
            continue

        signature_params = {
            token.split("=", 1)[0].strip().lstrip("*")
            for token in signature_match.group("signature").split(",")
            if token.strip()
        }

        body_lines: list[str] = []
        brace_balance = 0
        saw_body_start = False
        body_index = fn_index
        while body_index < len(lines):
            line = lines[body_index]
            body_lines.append(line)
            brace_balance += line.count("{")
            brace_balance -= line.count("}")
            if "{" in line:
                saw_body_start = True
            if saw_body_start and brace_balance <= 0:
                break
            body_index += 1

        body_window = "\n".join(body_lines)
        paths = {
            _normalize_client_path(candidate)
            for match in path_re.finditer(body_window)
            for candidate in match.groups()
            if candidate and candidate.startswith("/")
        }

        if paths:
            methods.append(
                {
                    "name": fn_match.group("name"),
                    "signature_params": signature_params,
                    "paths": paths,
                }
            )

        index = fn_index + 1

    return methods


def _resolve_spec_parameter(spec: dict[str, object], parameter: dict[str, object]) -> dict[str, object]:
    if "$ref" not in parameter:
        return parameter

    ref_name = str(parameter["$ref"]).split("/")[-1]
    return spec["components"]["parameters"][ref_name]


def _extract_spec_parameters(swagger_text: str) -> dict[str, dict[str, set[str]]]:
    spec = json.loads(swagger_text)
    spec_parameters: dict[str, dict[str, set[str]]] = {}

    for path, operations in spec["paths"].items():
        normalized_path = _normalize_spec_path(path)
        operation_params: set[str] = set()
        required_params: set[str] = set()

        for operation in operations.values():
            for parameter in operation.get("parameters", []):
                resolved = _resolve_spec_parameter(spec, parameter)
                normalized_name = _normalize_param_name(resolved["name"])
                operation_params.add(normalized_name)
                if resolved.get("required"):
                    required_params.add(normalized_name)

        spec_parameters[normalized_path] = {
            "all": operation_params,
            "required": required_params,
        }

    return spec_parameters


def test_client_covers_all_documented_swagger_paths() -> None:
    client_paths = _extract_client_paths(CLIENT_SOURCE.read_text())
    spec_paths = _extract_spec_paths(SWAGGER_PATH.read_text())

    missing_paths = sorted(spec_paths - client_paths)
    assert not missing_paths, (
        "Documented API paths are missing from the client:\n"
        + "\n".join(missing_paths)
    )


def test_client_only_has_expected_non_swagger_paths() -> None:
    client_paths = _extract_client_paths(CLIENT_SOURCE.read_text())
    spec_paths = _extract_spec_paths(SWAGGER_PATH.read_text())

    unexpected_extra_paths = sorted(client_paths - spec_paths - KNOWN_EXTRA_CLIENT_PATHS)
    assert not unexpected_extra_paths, (
        "Client exposes unexpected non-swagger paths:\n"
        + "\n".join(unexpected_extra_paths)
    )


def test_client_method_signatures_cover_required_swagger_parameters() -> None:
    client_methods = _extract_client_methods(CLIENT_SOURCE.read_text())
    spec_parameters = _extract_spec_parameters(SWAGGER_PATH.read_text())

    methods_by_path: dict[str, list[dict[str, object]]] = {}
    for method in client_methods:
        for path in method["paths"]:
            methods_by_path.setdefault(path, []).append(method)

    missing_parameter_coverage: list[str] = []
    for path, expected in sorted(spec_parameters.items()):
        candidate_methods = methods_by_path.get(path, [])
        if not candidate_methods:
            continue

        required_params = expected["required"]
        if any(required_params.issubset(method["signature_params"]) for method in candidate_methods):
            continue

        covered = sorted(
            {
                param
                for method in candidate_methods
                for param in method["signature_params"]
            }
        )
        missing_parameter_coverage.append(
            f"{path}: required {sorted(required_params)}, covered {covered}"
        )

    assert not missing_parameter_coverage, (
        "Client methods are missing required Swagger parameters:\n"
        + "\n".join(missing_parameter_coverage)
    )
