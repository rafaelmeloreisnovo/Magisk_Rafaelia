# RAFAELIA Architecture Audit

Status legend used in this audit:

| Token | Meaning |
| --- | --- |
| FATO_OBSERVADO | Directly observed repository fact. |
| CÓDIGO_EXISTENTE | Code path exists and contains the described implementation. |
| DOCUMENTAÇÃO_EXISTENTE | Existing documentation or report states the claim. |
| RELATÓRIO_EXISTENTE | A repository report records a previous result; it is not current proof. |
| HIPÓTESE | Plausible interpretation that still needs proof. |
| IMPLEMENTAÇÃO_PARCIAL | Some code exists, but scope is incomplete or not fully coupled. |
| TOKEN_VAZIO | Required evidence/source is absent or not identifiable in this repository. |
| RISCO | Operational, safety, privacy, or maintainability risk. |
| PRÓXIMO_TESTE | Concrete validation step recommended before promoting a claim. |
| DOCUMENTADO_NÃO_ACOPLADO | Documentation or manifest describes a capability without a verified runtime call path. |
| CÓDIGO_SEM_PROVA_DE_EXECUÇÃO_ATUAL | Code exists, but no current build/test/log artifact was produced in this audit. |

## Audit boundary

- This audit was read-only for runtime/system behavior. No root scripts, boot patching scripts, uninstallers, `/data/adb` actions, partition operations, Magisk module installation, or device commands were executed.
- The only write operations were documentation updates inside the repository.
- No secrets or token values were copied into this document. When a field or origin is absent, this document uses `TOKEN_VAZIO` instead of inventing provenance.
- Existing Magisk/GPL notices and RAFCODE-Φ / Rafael Melo Reis attribution were preserved where they already existed.

## Repository architecture map

| Area | Evidence path | Type | Observed behavior | Status | Gap / next test |
| --- | --- | --- | --- | --- | --- |
| Magisk base | `README.MD`, `native/src/core/lib.rs`, `native/src/init/rootdir.cpp`, `native/src/init/mount.cpp`, `native/src/sepolicy/api.cpp`, `scripts/boot_patch.sh` | CÓDIGO_EXISTENTE + DOCUMENTAÇÃO_EXISTENTE | Repository contains Magisk app/native/init/sepolicy/boot patch structure and documentation describing MagiskSU, modules, MagiskBoot, and Zygisk. | CÓDIGO_SEM_PROVA_DE_EXECUÇÃO_ATUAL | Run current native/app build in CI or controlled host. |
| Core daemon bridge | `native/src/core/lib.rs` | CÓDIGO_EXISTENTE | Rust core exposes Magisk daemon request codes including `POST_FS_DATA`, `LATE_START`, `BOOT_COMPLETE`, `DENYLIST`, `REMOVE_MODULES`, and `ZYGISK`; it also exposes `ZygiskConfig` and process flags. | CÓDIGO_SEM_PROVA_DE_EXECUÇÃO_ATUAL | Build and run daemon integration tests or emulator smoke test. |
| Boot lifecycle | `native/src/core/bootstages.rs` | CÓDIGO_EXISTENTE | `post_fs_data` initializes RAFAELIA audit/telemetry, sets Magisk directories, handles safe mode, initializes denylist/modules, and logs RAFAELIA stage records; `late_start` and `boot_complete` also log stage activity and boot snapshot if telemetry is available. | IMPLEMENTAÇÃO_PARCIAL | Requires Android boot test to prove runtime order and log creation. |
| Zygisk / NativeBridge | `native/src/core/zygisk/daemon.rs`, `native/src/core/zygisk/hook.cpp`, `native/src/core/zygisk/module.cpp`, `native/src/core/zygisk/api.hpp`, `native/src/core/zygisk/jni_hooks.hpp` | CÓDIGO_EXISTENTE | Code configures `ro.dalvik.vm.native.bridge` with `libzygisk.so`, uses Unix sockets and FD passing, exposes module specialization callbacks, and contains PLT/JNI hook infrastructure. | CÓDIGO_SEM_PROVA_DE_EXECUÇÃO_ATUAL | Validate on controlled device with Zygisk enabled and logcat/zygote crash monitoring. |
| Low-level syscalls/memory | `native/src/base/lowlevel.c`, `native/src/core/zygisk/lowlevel_inject.cpp` | CÓDIGO_EXISTENTE | Code wraps direct syscalls (`openat`, `read`, `write`, `close`, `mmap`, `munmap`, `mprotect`, `process_vm_readv`, `process_vm_writev`) and includes hook helpers using RWX/RX transitions, barriers, cache flush, GOT backup/restore, `ScopedHook`, `MemoryProtection`, and byte-wise integrity comparison. | CÓDIGO_SEM_PROVA_DE_EXECUÇÃO_ATUAL | Compile per-ABI and add host/unit tests for non-device-safe helpers where possible. |
| RAFAELIA audit | `native/src/core/rafaelia_audit.rs` | CÓDIGO_EXISTENTE | Creates `/data/adb/magisk/rafaelia_audit`, opens JSONL session logs, buffers audit entries with `VecDeque`, limits history to `MAX_AUDIT_HISTORY`, creates logical `RollbackPoint` records, and exposes global audit logging. | IMPLEMENTAÇÃO_PARCIAL | Rollback is logical/history-based, not physical boot/module rollback. Need Android log validation and log rotation design. |
| RAFAELIA telemetry | `native/src/core/rafaelia_telemetry.rs` | CÓDIGO_EXISTENTE | Samples `/proc/stat`, `/proc/meminfo`, `/proc/diskstats`, and `/proc/net/dev`; writes JSONL under `/data/adb/magisk/rafaelia_metrics`; maintains bounded in-memory history; starts a collection thread. | IMPLEMENTAÇÃO_PARCIAL | Requires Android runtime test, lifecycle/stop validation, and log growth test. |
| Shell operation layer | `tools/rafaelia/activate_rafaelia.sh`, `tools/rafaelia/metrics_collector.sh`, `tools/rafaelia/integrity_checker.sh` | CÓDIGO_EXISTENTE | Root-oriented scripts create directories/manifests, run a shell metrics collector, and inspect boot/modules/db/audit/manifest/SELinux/properties. | RISCO | Scripts write to `/data/adb` and `/data/local/tmp`, require root, and do not expose a dry-run mode. Do not run in CI without sandbox/mock. |
| Boot patch/uninstall/addon | `scripts/boot_patch.sh`, `scripts/uninstaller.sh`, `scripts/addon.d.sh` | CÓDIGO_EXISTENTE | Existing Magisk operational scripts manipulate boot images, ramdisk, addon.d, partitions, and removal paths. | RISCO | Device/system destructive; only execute in controlled Magisk workflows. |
| Build/CI | `build.py`, `native/CMakeLists.txt`, `app/*.gradle.kts`, `.github/workflows/*.yml`, `scripts/pre_ci_validate.py`, `BUILD_SUCCESS.md` | CÓDIGO_EXISTENTE + RELATÓRIO_EXISTENTE | Build scripts and workflows exist for native/app builds and APK artifacts. `BUILD_SUCCESS.md` records a previous native multi-ABI result and an APK network limitation. | RELATÓRIO_EXISTENTE | This audit did not run full builds. Current validity requires CI rerun. |
| Governance/privacy docs | `docs/governance/*`, `docs/RAFAELIA_*`, `RAFAELIA_MANIFEST.json` | DOCUMENTAÇÃO_EXISTENTE | Governance, ethics, security, matrix, manifest, telemetry, and audit documentation exist. | DOCUMENTADO_NÃO_ACOPLADO where not linked to runtime | Claims must cite code, tests, workflows, or reports before being promoted. |

## Confirmed vs unproven RAFAELIA claims

| Claim | Evidence | Classification | Status |
| --- | --- | --- | --- |
| 56 × 18 = 1008 state matrix exists as repository data/documentation. | `docs/RAFAELIA_STATE_MATRIX.csv`, `docs/RAFAELIA_FRAMEWORK.md`, `docs/RAFAELIA_PRIMITIVES.json` | DOCUMENTAÇÃO_EXISTENTE | FATO_OBSERVADO as documentation/data; runtime enforcement is TOKEN_VAZIO unless call path is added/proven. |
| Full audit system exists. | `native/src/core/rafaelia_audit.rs` | CÓDIGO_EXISTENTE | IMPLEMENTAÇÃO_PARCIAL; JSONL and in-memory history exist, but “full” must not imply complete security without tests. |
| SHA3/Blake3 verified logging. | README/older docs claim variants; audit code stores optional hashes but this audit did not find SHA3/Blake3 computation in `rafaelia_audit.rs`. | DOCUMENTAÇÃO_EXISTENTE | DOCUMENTADO_NÃO_ACOPLADO / TOKEN_VAZIO for implemented hash algorithm. |
| Real-time telemetry exists. | `native/src/core/rafaelia_telemetry.rs`; `tools/rafaelia/metrics_collector.sh` | CÓDIGO_EXISTENTE | IMPLEMENTAÇÃO_PARCIAL; local sampling code exists, execution proof pending. |
| Anomaly detection. | Some documentation mentions monitoring/anomaly concepts. | DOCUMENTAÇÃO_EXISTENTE | TOKEN_VAZIO unless a concrete detector call path is identified. |
| Security hardening via SELinux/seccomp/eBPF. | Magisk SELinux code exists; shell checker queries SELinux. This audit did not verify RAFAELIA-specific seccomp/eBPF runtime integration. | CÓDIGO_EXISTENTE + TOKEN_VAZIO | Magisk SELinux integration exists; RAFAELIA-specific seccomp/eBPF is DOCUMENTADO_NÃO_ACOPLADO if claimed. |
| Ethical computing / governance. | `docs/governance/*`, `docs/RAFAELIA_*` | DOCUMENTAÇÃO_EXISTENTE | Documentation exists; runtime enforcement is TOKEN_VAZIO unless linked to tests/gates. |
| Baremetal optimization. | `native/src/base/lowlevel.c`, `native/src/core/zygisk/lowlevel_inject.cpp`, several docs. | CÓDIGO_EXISTENTE + DOCUMENTAÇÃO_EXISTENTE | Low-level primitives exist; performance claims require benchmarks. |
| Rollback. | `RollbackPoint` in audit; Zygisk daemon property restore after repeated crashes. | CÓDIGO_EXISTENTE | IMPLEMENTAÇÃO_PARCIAL: logical audit rollback and Zygisk native-bridge rollback exist; not a full boot image/module rollback. |
| Checksums/integrity. | `tools/rafaelia/integrity_checker.sh`, `.github/scripts/generate_manifest.sh`, manifest docs. | CÓDIGO_EXISTENTE | IMPLEMENTAÇÃO_PARCIAL; verify current algorithms and artifacts in CI/device. |

## Architecture flow observed in code

1. Android/Magisk boot reaches Magisk daemon request handling.
2. `POST_FS_DATA` calls `init_rafaelia_observability`, which calls `init_global_audit`, `init_global_telemetry(1000)`, and `start_global_telemetry`.
3. `POST_FS_DATA` continues Magisk environment setup, safe-mode handling, script execution, denylist initialization, module handling, mount cleanup, and RAFAELIA stage logging.
4. `LATE_START` executes service scripts, module service scripts, and logs a RAFAELIA stage record.
5. `BOOT_COMPLETE` resets bootloop count, logs a telemetry snapshot if one is present, and logs a RAFAELIA stage record.
6. Zygisk behavior is controlled separately through `ZygiskConfig`, NativeBridge property changes, zygiskd sockets, FD passing, denylist flags, and module file descriptors.

## Operational limits

- Code presence is not proof of successful execution on Android.
- Documentation presence is not proof of runtime coupling.
- Existing build reports are historical repository artifacts and must be revalidated.
- Root/Magisk scripts can affect `/data/adb`, modules, boot images, and system state; they require controlled devices, backups, and explicit operator consent.
