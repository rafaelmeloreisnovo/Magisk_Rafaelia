# RAFAELIA Documentation Status

## Files read or sampled during audit

| Area | Paths |
| --- | --- |
| Root documentation | `README.MD`, `BUILD_SUCCESS.md`, `docs/RAFAELIA_FRAMEWORK.md`, `docs/ENGINEERING_PLAYBOOK.md`, `docs/boot.md`, `docs/ota.md`, `docs/build.md` |
| RAFAELIA docs/data | `docs/RAFAELIA_AUDIT_SYSTEM.md`, `docs/RAFAELIA_TELEMETRY.md`, `docs/RAFAELIA_STATE_MATRIX.csv`, `docs/RAFAELIA_PRIMITIVES.json`, `RAFAELIA_MANIFEST.json` |
| Rust core | `native/src/core/lib.rs`, `native/src/core/bootstages.rs`, `native/src/core/rafaelia_audit.rs`, `native/src/core/rafaelia_telemetry.rs`, `native/src/core/zygisk/daemon.rs` |
| Zygisk/low-level | `native/src/core/zygisk/hook.cpp`, `native/src/core/zygisk/module.cpp`, `native/src/core/zygisk/api.hpp`, `native/src/core/zygisk/jni_hooks.hpp`, `native/src/core/zygisk/lowlevel_inject.cpp`, `native/src/base/lowlevel.c` |
| Magisk native | `native/src/core/su/su.cpp`, `native/src/init/rootdir.cpp`, `native/src/init/mount.cpp`, `native/src/sepolicy/api.cpp` |
| Scripts | `tools/rafaelia/activate_rafaelia.sh`, `tools/rafaelia/metrics_collector.sh`, `tools/rafaelia/integrity_checker.sh`, `scripts/boot_patch.sh`, `scripts/uninstaller.sh`, `scripts/addon.d.sh`, `scripts/pre_ci_validate.py` |
| Build/CI | `build.py`, `native/CMakeLists.txt`, `app/*.gradle.kts`, `app/gradle/libs.versions.toml`, `.github/workflows/*.yml` |

## Documentation created or updated in this audit

| File | Purpose |
| --- | --- |
| `docs/RAFAELIA_ARCHITECTURE_AUDIT.md` | Overall architecture, status legend, confirmed/unproven claim map. |
| `docs/RAFAELIA_ZYGISK_LOWLEVEL_AUDIT.md` | Zygisk NativeBridge/hooks/module lifecycle and low-level memory/syscall traceability. |
| `docs/RAFAELIA_BOOT_TELEMETRY_AUDIT.md` | Boot stages, audit/telemetry behavior, scripts audit. |
| `docs/RAFAELIA_ANDROID_IMPACT_MODEL.md` | Cautious Android impact model and measurement plan. |
| `docs/RAFAELIA_GOVERNANCE_AND_PRIVACY.md` | Anti-inference policy, privacy paths, TOKEN_VAZIO list, risks. |
| `docs/RAFAELIA_DOCUMENTATION_STATUS.md` | Audit inventory and documentation status. |
| `docs/RAFAELIA_TRACEABILITY_MATRIX.md` | Claim-to-evidence matrix. |
| `README.MD` | Updated to point users to conservative audit docs and remove overclaiming in the RAFAELIA overview. |

## Existing documentation status

| Existing file | Status | Notes |
| --- | --- | --- |
| `README.MD` | Updated | Previous RAFAELIA section used strong implementation/performance language; it now distinguishes code, docs, and pending validation. |
| `BUILD_SUCCESS.md` | RELATÓRIO_EXISTENTE | Records previous native multi-ABI build and APK network limitation; needs current validation. |
| `docs/RAFAELIA_FRAMEWORK.md` | DOCUMENTAÇÃO_EXISTENTE | Useful framework context; runtime coupling must be checked claim-by-claim. |
| `docs/RAFAELIA_AUDIT_SYSTEM.md` | DOCUMENTAÇÃO_EXISTENTE | Should be reconciled with actual `rafaelia_audit.rs` capabilities. |
| `docs/RAFAELIA_TELEMETRY.md` | DOCUMENTAÇÃO_EXISTENTE | Should be reconciled with Rust telemetry and shell collector behavior. |
| `docs/ENGINEERING_PLAYBOOK.md` | DOCUMENTAÇÃO_EXISTENTE | Build/CI guidance exists; current pipeline proof requires a fresh run. |
| `docs/boot.md`, `docs/ota.md` | DOCUMENTAÇÃO_EXISTENTE | Magisk boot/OTA docs exist; they are not RAFAELIA runtime proof by themselves. |

## Safe checks run in this audit

| Command | Result | Notes |
| --- | --- | --- |
| `find . -maxdepth 5 -type f` | PASS | Inventory only; no destructive action. |
| `rg ...` searches | PASS | Static text/code search only. |
| `python3 -m py_compile scripts/pre_ci_validate.py build.py` | PASS | Host Python syntax check only. |
| `python3 scripts/pre_ci_validate.py --skip-slow` | PASS | Safe host validation passed. It reported permission issues and uncommitted documentation changes as informational pass details; no root/device scripts were run. |
| Audited Markdown local link/path review | PASS | Checked the new audit docs and README local links, excluding the repository-relative GitHub Actions link. |

## Pending points

- Current native build for `magisk`, `magiskboot`, `magiskinit`, `magiskpolicy`.
- Current Android APK build and artifact validation.
- Zygisk runtime test on controlled device/emulator.
- Mockable/dry-run mode for root scripts.
- Rust audit/telemetry log rotation or retention policy.
- Runtime proof that RAFAELIA state matrix/manifest govern actual decisions.
