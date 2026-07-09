import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent.parent))

from governance.download_strategy import (
    ContentKind,
    DownloadCandidate,
    DownloadState,
    RollbackPlan,
    SafetyFlag,
    WatchdogPolicy,
    build_download_strategy,
    decision_manifest,
    recognize_content,
    sha256_bytes,
)


class TestDownloadStrategy(unittest.TestCase):
    def test_recognizes_apk_by_extension(self):
        self.assertEqual(recognize_content("release.apk", b"PK\x03\x04"), ContentKind.APK)

    def test_ready_decision_with_failover_mirror(self):
        payload = b'{"ok": true}'
        digest = sha256_bytes([payload])
        decision = build_download_strategy(
            "manifest.json",
            payload[:8],
            [payload],
            [
                DownloadCandidate("https://b.example/manifest.json", digest, len(payload), priority=20),
                DownloadCandidate("https://a.example/manifest.json", digest, len(payload), priority=10),
            ],
            WatchdogPolicy(max_retries=1),
            now=0.0,
        )

        self.assertEqual(decision.state, DownloadState.READY)
        self.assertEqual(decision.selected_url, "https://a.example/manifest.json")
        self.assertEqual(decision.failover_urls, ("https://b.example/manifest.json",))
        self.assertEqual(decision.flags, ())
        self.assertEqual(decision_manifest(decision)["sha256"], digest)

    def test_failover_when_primary_hash_mismatches(self):
        payload = b"safe text"
        digest = sha256_bytes([payload])
        decision = build_download_strategy(
            "notes.txt",
            payload,
            [payload],
            [
                DownloadCandidate("https://bad.example/notes.txt", "0" * 64, len(payload), priority=1),
                DownloadCandidate("https://good.example/notes.txt", digest, len(payload), priority=2),
            ],
            WatchdogPolicy(),
            now=0.0,
        )

        self.assertEqual(decision.state, DownloadState.FAILOVER)
        self.assertEqual(decision.selected_url, "https://good.example/notes.txt")
        self.assertIn(SafetyFlag.DIGEST_MISMATCH, decision.flags)
        self.assertTrue(any("sha256 mismatch" in reason for reason in decision.reasons))

    def test_rollback_when_no_safe_route_remains(self):
        rollback = RollbackPlan("/snap/last", "/opt/app", "integrity failure", True)
        decision = build_download_strategy(
            "payload.bin",
            b"\x00\x01",
            [b"\x00\x01"],
            [DownloadCandidate("https://bad.example/payload.bin", "f" * 64, 2)],
            WatchdogPolicy(allowed_kinds=(ContentKind.TEXT,)),
            rollback=rollback,
            now=0.0,
        )

        self.assertEqual(decision.state, DownloadState.ROLLBACK)
        self.assertEqual(decision.rollback, rollback)
        self.assertIn(SafetyFlag.CONTENT_BLOCKED, decision.flags)
        self.assertIn(SafetyFlag.DIGEST_MISMATCH, decision.flags)
        self.assertIsNone(decision.selected_url)

    def test_rejects_insecure_scheme_even_with_valid_digest(self):
        payload = b"safe text"
        digest = sha256_bytes([payload])
        decision = build_download_strategy(
            "notes.txt",
            payload,
            [payload],
            [DownloadCandidate("http://bad.example/notes.txt", digest, len(payload))],
            WatchdogPolicy(allowed_schemes=("https",)),
            now=0.0,
        )

        self.assertEqual(decision.state, DownloadState.REJECTED)
        self.assertIn(SafetyFlag.URL_SCHEME_BLOCKED, decision.flags)
        self.assertIsNone(decision.selected_url)

    def test_requires_compatible_mirror_count_for_enterprise_rollout(self):
        payload = b'{"ok": true}'
        digest = sha256_bytes([payload])
        rollback = RollbackPlan("/snap/stable", "/srv/app", "mirror quorum failure", True)
        decision = build_download_strategy(
            "manifest.json",
            payload,
            [payload],
            [
                DownloadCandidate(
                    "https://arm.example/manifest.json",
                    digest,
                    len(payload),
                    compatibility=("arm32", "neon"),
                ),
                DownloadCandidate(
                    "https://generic.example/manifest.json",
                    digest,
                    len(payload),
                    priority=2,
                    compatibility=("generic",),
                ),
            ],
            WatchdogPolicy(
                min_viable_mirrors=2,
                required_compatibility=("arm32", "neon"),
            ),
            rollback=rollback,
            now=0.0,
        )

        self.assertEqual(decision.state, DownloadState.ROLLBACK)
        self.assertIn(SafetyFlag.MIN_MIRRORS_UNMET, decision.flags)
        self.assertIsNone(decision.selected_url)
        self.assertEqual(decision.rollback, rollback)

    def test_manifest_exports_stable_flags_for_audit(self):
        payload = b"safe text"
        digest = sha256_bytes([payload])
        decision = build_download_strategy(
            "notes.txt",
            payload,
            [payload],
            [DownloadCandidate("http://bad.example/notes.txt", digest, len(payload))],
            WatchdogPolicy(allowed_schemes=("https",)),
            now=0.0,
        )
        manifest = decision_manifest(decision)

        self.assertEqual(manifest["flags"], [SafetyFlag.URL_SCHEME_BLOCKED.value, SafetyFlag.MIN_MIRRORS_UNMET.value])
        self.assertEqual(manifest["state"], DownloadState.REJECTED.value)


if __name__ == "__main__":
    unittest.main()
