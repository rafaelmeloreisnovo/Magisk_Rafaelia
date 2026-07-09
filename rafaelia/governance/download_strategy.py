"""Enterprise-safe download strategy primitives for RAFAELIA governance.

This module is a decision layer, not a downloader.  It performs no network I/O,
creates no subprocesses, and mutates no system state.  Callers provide artifact
metadata plus a bounded content stream; the module classifies the artifact,
validates compatibility/security constraints, chooses a deterministic primary
mirror, exposes bounded failover mirrors, and returns a rollback-ready audit
manifest.
"""

from __future__ import annotations

import hashlib
import time
from dataclasses import dataclass, field
from enum import Enum
from pathlib import Path
from typing import Dict, Iterable, List, Mapping, Optional, Sequence, Tuple
from urllib.parse import urlparse


class DownloadState(str, Enum):
    """Operational state produced by the download strategy."""

    READY = "READY"
    FAILOVER = "FAILOVER"
    ROLLBACK = "ROLLBACK"
    REJECTED = "REJECTED"


class ContentKind(str, Enum):
    """Recognized artifact families allowed by the secure download pipeline."""

    APK = "apk"
    ZIP = "zip"
    JSON = "json"
    TEXT = "text"
    BINARY = "binary"
    UNKNOWN = "unknown"


class SafetyFlag(str, Enum):
    """Reason flags kept stable for automation, dashboards, and rollback gates."""

    CONTENT_BLOCKED = "CONTENT_BLOCKED"
    DIGEST_MISMATCH = "DIGEST_MISMATCH"
    HASH_FORMAT_INVALID = "HASH_FORMAT_INVALID"
    NO_CANDIDATES = "NO_CANDIDATES"
    SIZE_LIMIT = "SIZE_LIMIT"
    URL_SCHEME_BLOCKED = "URL_SCHEME_BLOCKED"
    WATCHDOG_TIMEOUT = "WATCHDOG_TIMEOUT"
    MIN_MIRRORS_UNMET = "MIN_MIRRORS_UNMET"


_MAGIC_TABLE: Tuple[Tuple[bytes, ContentKind], ...] = (
    (b"PK\x03\x04", ContentKind.ZIP),
    (b"{", ContentKind.JSON),
    (b"[", ContentKind.JSON),
)

_EXTENSION_TABLE: Mapping[str, ContentKind] = {
    ".apk": ContentKind.APK,
    ".zip": ContentKind.ZIP,
    ".json": ContentKind.JSON,
    ".txt": ContentKind.TEXT,
    ".md": ContentKind.TEXT,
}

_HEX_DIGITS = frozenset("0123456789abcdefABCDEF")


@dataclass(frozen=True)
class DownloadCandidate:
    """Single immutable download source description."""

    url: str
    expected_sha256: str
    size_bytes: int
    priority: int = 100
    compatibility: Tuple[str, ...] = ()


@dataclass(frozen=True)
class WatchdogPolicy:
    """Bounded operational guardrails for a download transaction."""

    timeout_seconds: float = 30.0
    max_retries: int = 2
    max_size_bytes: int = 512 * 1024 * 1024
    min_viable_mirrors: int = 1
    required_compatibility: Tuple[str, ...] = ()
    allowed_schemes: Tuple[str, ...] = ("https", "file")
    allowed_kinds: Tuple[ContentKind, ...] = (
        ContentKind.APK,
        ContentKind.ZIP,
        ContentKind.JSON,
        ContentKind.TEXT,
    )


@dataclass(frozen=True)
class RollbackPlan:
    """Explicit rollback data for the caller's deployment layer."""

    snapshot_path: str
    restore_path: str
    reason: str
    required: bool


@dataclass(frozen=True)
class StrategyDecision:
    """Result of validating a content sample and its candidate mirrors."""

    state: DownloadState
    kind: ContentKind
    selected_url: Optional[str]
    failover_urls: Tuple[str, ...]
    digest: str
    flags: Tuple[SafetyFlag, ...] = field(default_factory=tuple)
    reasons: Tuple[str, ...] = field(default_factory=tuple)
    rollback: Optional[RollbackPlan] = None


def recognize_content(name: str, prefix: bytes) -> ContentKind:
    """Recognize a download artifact from extension and a small content prefix."""

    suffix = Path(name).suffix.lower()
    if suffix == ".apk":
        return ContentKind.APK
    if suffix in _EXTENSION_TABLE:
        return _EXTENSION_TABLE[suffix]
    trimmed = prefix.lstrip()[:4]
    for magic, kind in _MAGIC_TABLE:
        if trimmed.startswith(magic):
            return kind
    if prefix and all(byte in b"\t\n\r" or 32 <= byte <= 126 for byte in prefix[:128]):
        return ContentKind.TEXT
    return ContentKind.BINARY if prefix else ContentKind.UNKNOWN


def sha256_bytes(chunks: Iterable[bytes]) -> str:
    """Compute SHA-256 incrementally from byte chunks."""

    digest = hashlib.sha256()
    for chunk in chunks:
        digest.update(chunk)
    return digest.hexdigest()


def _valid_sha256(value: str) -> bool:
    return len(value) == 64 and all(character in _HEX_DIGITS for character in value)


def _candidate_scheme(candidate: DownloadCandidate) -> str:
    parsed = urlparse(candidate.url)
    if parsed.scheme:
        return parsed.scheme.lower()
    return "file" if candidate.url.startswith("/") else ""


def _is_compatible(candidate: DownloadCandidate, required: Tuple[str, ...]) -> bool:
    if not required:
        return True
    supported = frozenset(candidate.compatibility)
    return all(flag in supported for flag in required)


def _flag(flags: List[SafetyFlag], flag: SafetyFlag) -> None:
    if flag not in flags:
        flags.append(flag)


def _terminal_state(rollback: Optional[RollbackPlan]) -> DownloadState:
    return DownloadState.ROLLBACK if rollback and rollback.required else DownloadState.REJECTED


def build_download_strategy(
    name: str,
    content_prefix: bytes,
    content_chunks: Iterable[bytes],
    candidates: Sequence[DownloadCandidate],
    policy: WatchdogPolicy,
    rollback: Optional[RollbackPlan] = None,
    now: Optional[float] = None,
) -> StrategyDecision:
    """Validate artifact metadata and choose a primary source plus failovers.

    The function is deterministic when ``now`` is supplied.  It returns
    ``READY`` when all guardrails pass, ``FAILOVER`` when at least one candidate
    was rejected but a safe route remains, ``ROLLBACK`` when no safe route
    remains and rollback is mandatory, otherwise ``REJECTED``.
    """

    clock_value = time.monotonic() if now is None else now
    started_at = clock_value
    kind = recognize_content(name, content_prefix)
    digest = sha256_bytes(content_chunks)
    ordered = tuple(sorted(candidates, key=lambda item: (item.priority, item.url)))
    flags: List[SafetyFlag] = []
    reasons: List[str] = []

    if kind not in policy.allowed_kinds:
        _flag(flags, SafetyFlag.CONTENT_BLOCKED)
        reasons.append(f"content kind {kind.value} is not allowed")
    if not ordered:
        _flag(flags, SafetyFlag.NO_CANDIDATES)
        reasons.append("no download candidates supplied")
    elapsed = (time.monotonic() if now is None else clock_value) - started_at
    if elapsed > policy.timeout_seconds:
        _flag(flags, SafetyFlag.WATCHDOG_TIMEOUT)
        reasons.append("watchdog timeout before validation completed")

    viable: List[DownloadCandidate] = []
    for candidate in ordered:
        if candidate.size_bytes > policy.max_size_bytes:
            _flag(flags, SafetyFlag.SIZE_LIMIT)
            reasons.append(f"candidate too large: {candidate.url}")
            continue
        if _candidate_scheme(candidate) not in policy.allowed_schemes:
            _flag(flags, SafetyFlag.URL_SCHEME_BLOCKED)
            reasons.append(f"candidate URL scheme blocked: {candidate.url}")
            continue
        if not _valid_sha256(candidate.expected_sha256):
            _flag(flags, SafetyFlag.HASH_FORMAT_INVALID)
            reasons.append(f"sha256 format invalid: {candidate.url}")
            continue
        if candidate.expected_sha256.lower() != digest:
            _flag(flags, SafetyFlag.DIGEST_MISMATCH)
            reasons.append(f"sha256 mismatch: {candidate.url}")
            continue
        if not _is_compatible(candidate, policy.required_compatibility):
            _flag(flags, SafetyFlag.MIN_MIRRORS_UNMET)
            reasons.append(f"candidate lacks required compatibility: {candidate.url}")
            continue
        viable.append(candidate)

    if len(viable) < policy.min_viable_mirrors:
        _flag(flags, SafetyFlag.MIN_MIRRORS_UNMET)
        reasons.append("minimum viable mirror count was not met")

    if not viable or len(viable) < policy.min_viable_mirrors:
        state = _terminal_state(rollback)
        return StrategyDecision(
            state=state,
            kind=kind,
            selected_url=None,
            failover_urls=(),
            digest=digest,
            flags=tuple(flags),
            reasons=tuple(reasons),
            rollback=rollback if state is DownloadState.ROLLBACK else None,
        )

    selected = viable[0]
    failovers = tuple(candidate.url for candidate in viable[1 : policy.max_retries + 1])
    state = DownloadState.FAILOVER if flags else DownloadState.READY
    return StrategyDecision(
        state=state,
        kind=kind,
        selected_url=selected.url,
        failover_urls=failovers,
        digest=digest,
        flags=tuple(flags),
        reasons=tuple(reasons),
        rollback=None,
    )


def decision_manifest(decision: StrategyDecision) -> Dict[str, object]:
    """Serialize a strategy decision into an audit-friendly manifest."""

    return {
        "state": decision.state.value,
        "kind": decision.kind.value,
        "selected_url": decision.selected_url,
        "failover_urls": list(decision.failover_urls),
        "sha256": decision.digest,
        "flags": [flag.value for flag in decision.flags],
        "reasons": list(decision.reasons),
        "rollback": None if decision.rollback is None else {
            "snapshot_path": decision.rollback.snapshot_path,
            "restore_path": decision.rollback.restore_path,
            "reason": decision.rollback.reason,
            "required": decision.rollback.required,
        },
    }
