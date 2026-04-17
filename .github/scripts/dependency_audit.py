#!/usr/bin/env python3
"""Dependency and CI action obsolescence audit for Magisk_Rafaelia."""

from __future__ import annotations

import re
import sys
from dataclasses import dataclass
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
WORKFLOWS = ROOT / ".github" / "workflows"
ACTION_DEFINITIONS = ROOT / ".github" / "actions"
WRAPPER = ROOT / "app" / "gradle" / "wrapper" / "gradle-wrapper.properties"
VERSIONS = ROOT / "app" / "gradle" / "libs.versions.toml"

# Minimum majors aligned with the project's current supported baseline.
MIN_ACTION_MAJOR = {
    "actions/checkout": 4,
    "actions/setup-java": 4,
    "actions/setup-python": 5,
    "actions/upload-artifact": 4,
    "actions/download-artifact": 4,
    "android-actions/setup-android": 3,
    "gradle/actions/setup-gradle": 4,
    "actions/first-interaction": 3,
    "gacts/run-and-post-run": 1,
}

MIN_GRADLE_MAJOR = 9
MIN_AGP_MAJOR = 8
MIN_KOTLIN_MAJOR = 2


@dataclass
class Finding:
    level: str
    location: str
    message: str


def parse_major(version: str) -> int | None:
    m = re.match(r"v(\d+)", version)
    return int(m.group(1)) if m else None


def _scan_uses_references(file: Path) -> list[tuple[int, str, str]]:
    refs: list[tuple[int, str, str]] = []
    for idx, line in enumerate(file.read_text(encoding="utf-8").splitlines(), start=1):
        m = re.search(r"uses:\s*([\w./-]+)@(v\d+)", line)
        if m:
            refs.append((idx, m.group(1), m.group(2)))
    return refs


def audit_workflows() -> list[Finding]:
    findings: list[Finding] = []

    files = sorted(WORKFLOWS.glob("*.yml")) + sorted(ACTION_DEFINITIONS.glob("**/action.yml"))
    for file in files:
        for idx, action, version in _scan_uses_references(file):
            if action in MIN_ACTION_MAJOR:
                major = parse_major(version)
                if major is not None and major < MIN_ACTION_MAJOR[action]:
                    findings.append(
                        Finding(
                            "ERROR",
                            f"{file.relative_to(ROOT)}:{idx}",
                            f"{action}@{version} is obsolete (min supported v{MIN_ACTION_MAJOR[action]}).",
                        )
                    )
    return findings


def audit_gradle_wrapper() -> list[Finding]:
    findings: list[Finding] = []
    content = WRAPPER.read_text(encoding="utf-8")
    m = re.search(r"gradle-(\d+)\.(\d+)\.(\d+)-", content)
    if not m:
        findings.append(Finding("ERROR", str(WRAPPER.relative_to(ROOT)), "Could not parse Gradle wrapper version."))
        return findings
    major = int(m.group(1))
    if major < MIN_GRADLE_MAJOR:
        findings.append(
            Finding(
                "ERROR",
                str(WRAPPER.relative_to(ROOT)),
                f"Gradle {major}.x is below required baseline {MIN_GRADLE_MAJOR}.x.",
            )
        )
    return findings


def audit_versions_toml() -> list[Finding]:
    findings: list[Finding] = []
    content = VERSIONS.read_text(encoding="utf-8")

    agp = re.search(r'^android\s*=\s*"(\d+)\.', content, flags=re.MULTILINE)
    kotlin = re.search(r'^kotlin\s*=\s*"(\d+)\.', content, flags=re.MULTILINE)

    if agp and int(agp.group(1)) < MIN_AGP_MAJOR:
        findings.append(
            Finding(
                "ERROR",
                str(VERSIONS.relative_to(ROOT)),
                f"AGP major {agp.group(1)} is below required baseline {MIN_AGP_MAJOR}.",
            )
        )
    if kotlin and int(kotlin.group(1)) < MIN_KOTLIN_MAJOR:
        findings.append(
            Finding(
                "ERROR",
                str(VERSIONS.relative_to(ROOT)),
                f"Kotlin major {kotlin.group(1)} is below required baseline {MIN_KOTLIN_MAJOR}.",
            )
        )
    return findings


def main() -> int:
    findings: list[Finding] = []
    findings.extend(audit_workflows())
    findings.extend(audit_gradle_wrapper())
    findings.extend(audit_versions_toml())

    if not findings:
        print("✅ Dependency audit passed: no obsolete baselines detected.")
        return 0

    errors = [f for f in findings if f.level == "ERROR"]
    for finding in findings:
        marker = "::error::" if finding.level == "ERROR" else "::warning::"
        print(f"{marker}{finding.location} - {finding.message}")

    return 1 if errors else 0


if __name__ == "__main__":
    sys.exit(main())
