"""Safe download strategy primitives for RAFAELIA governance.

The module is intentionally side-effect free: it does not open sockets, mutate
system state, or allocate unbounded buffers.  It classifies planned artifacts,
checks integrity metadata, selects failover mirrors, and returns an explicit
rollback plan that callers can execute in their own deployment layer.
"""

from __future__ import annotations

import hashlib
import time
from dataclasses import dataclass, field
from enum import Enum
from pathlib import Path
from typing import Dict, Iterable, List, Mapping, Optional, Sequence, Tuple


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


@dataclass(frozen=True)
class DownloadCandidate:
    """Single immutable download source description."""

    url: str
    expected_sha256: str
    size_bytes: int
    priority: int = 100


@dataclass(frozen=True)
class WatchdogPolicy:
    """Bounded operational guardrails for a download transaction."""

    timeout_seconds: float = 30.0
    max_retries: int = 2
    max_size_bytes: int = 512 * 1024 * 1024
    allowed_kinds: Tuple[ContentKind, ...] = (
        ContentKind.APK,
        ContentKind.ZIP,
        ContentKind.JSON,
        ContentKind.TEXT,
    )


@dataclass(frozen=True)
class RollbackPlan:
    """Explicit rollback commands/data for the caller's deployment layer."""

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

    The function is deterministic except for the optional watchdog timestamp.
    It returns REJECTED for unsafe content, FAILOVER when the best candidate is
    unusable but another candidate remains, READY when all checks pass, and
    ROLLBACK when no safe route remains and a rollback plan is available.
    """

    clock_value = time.monotonic() if now is None else now
    started_at = clock_value
    kind = recognize_content(name, content_prefix)
    digest = sha256_bytes(content_chunks)
    ordered = tuple(sorted(candidates, key=lambda item: (item.priority, item.url)))
    reasons: List[str] = []

    if kind not in policy.allowed_kinds:
        reasons.append(f"content kind {kind.value} is not allowed")
    if not ordered:
        reasons.append("no download candidates supplied")
    elapsed = (time.monotonic() if now is None else clock_value) - started_at
    if elapsed > policy.timeout_seconds:
        reasons.append("watchdog timeout before validation completed")

    viable: List[DownloadCandidate] = []
    for candidate in ordered:
        if candidate.size_bytes > policy.max_size_bytes:
            reasons.append(f"candidate too large: {candidate.url}")
            continue
        if candidate.expected_sha256.lower() != digest:
            reasons.append(f"sha256 mismatch: {candidate.url}")
            continue
        viable.append(candidate)

    if reasons and not viable:
        state = DownloadState.ROLLBACK if rollback and rollback.required else DownloadState.REJECTED
        return StrategyDecision(
            state=state,
            kind=kind,
            selected_url=None,
            failover_urls=(),
            digest=digest,
            reasons=tuple(reasons),
            rollback=rollback if state is DownloadState.ROLLBACK else None,
        )

    if not viable:
        state = DownloadState.ROLLBACK if rollback and rollback.required else DownloadState.REJECTED
        return StrategyDecision(state, kind, None, (), digest, tuple(reasons), rollback)

    selected = viable[0]
    failovers = tuple(candidate.url for candidate in viable[1 : policy.max_retries + 1])
    state = DownloadState.FAILOVER if reasons else DownloadState.READY
    return StrategyDecision(
        state=state,
        kind=kind,
        selected_url=selected.url,
        failover_urls=failovers,
        digest=digest,
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
        "reasons": list(decision.reasons),
        "rollback": None if decision.rollback is None else {
            "snapshot_path": decision.rollback.snapshot_path,
            "restore_path": decision.rollback.restore_path,
            "reason": decision.rollback.reason,
            "required": decision.rollback.required,
        },
    }
