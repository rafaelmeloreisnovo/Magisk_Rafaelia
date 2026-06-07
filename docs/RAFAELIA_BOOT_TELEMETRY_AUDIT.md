# RAFAELIA Boot and Telemetry Audit

## Boot lifecycle observed

| Stage | Path | Observed call path | Status | Gap |
| --- | --- | --- | --- | --- |
| `post_fs_data` | `native/src/core/bootstages.rs` | Calls `init_rafaelia_observability`, `preserve_stub_apk`, Magisk environment setup, safe-mode check, common scripts, denylist initialization, module handling, mount cleanup, and `log_rafaelia_stage("POST_FS_DATA", ...)`. | CÓDIGO_EXISTENTE | Needs Android boot proof. |
| RAFAELIA observability init | `native/src/core/bootstages.rs` | Calls `init_global_audit()`, `init_global_telemetry(1000)`, and `start_global_telemetry()`. | CÓDIGO_EXISTENTE | Test startup failure handling and duplicate initialization. |
| Safe mode | `native/src/core/bootstages.rs` | Uses bootloop count, safemode properties, and key combo; disables modules and sets `ZygiskConfig` to 0 on safe mode. | CÓDIGO_EXISTENTE | Test only in disposable environment. |
| `late_start` | `native/src/core/bootstages.rs` | Executes common service scripts and module service scripts, then logs `LATE_START`. | CÓDIGO_EXISTENTE | Device boot log proof pending. |
| `boot_complete` | `native/src/core/bootstages.rs` | Resets bootloop count, obtains `get_global_snapshot()`, logs `BOOT_COMPLETE_TELEMETRY` if present, then logs `BOOT_COMPLETE`. | CÓDIGO_EXISTENTE | Verify JSONL entry and snapshot content on device. |

Expected flow from code, not from runtime proof:

`Android boot → MagiskD → POST_FS_DATA → RAFAELIA audit/telemetry init/start → Magisk scripts/modules/denylist → LATE_START → BOOT_COMPLETE → optional telemetry snapshot → JSONL audit/metrics files`

## RAFAELIA audit system

| Component | Path | Observed behavior | Status | Risk / next test |
| --- | --- | --- | --- | --- |
| Audit directory | `native/src/core/rafaelia_audit.rs` | Creates `/data/adb/magisk/rafaelia_audit`. | CÓDIGO_EXISTENTE | Directory permissions are not explicitly set by Rust audit init; activation script sets 700. |
| JSONL session file | `native/src/core/rafaelia_audit.rs` | Opens `audit_<session>.jsonl` append file and writes escaped JSON lines. | CÓDIGO_EXISTENTE | Need log growth/rotation strategy. |
| In-memory history | `native/src/core/rafaelia_audit.rs` | Maintains bounded `VecDeque` of audit entries. | CÓDIGO_EXISTENTE | Test high-volume behavior. |
| Rollback | `native/src/core/rafaelia_audit.rs` | Stores rollback point metadata and truncates audit history on rollback. | IMPLEMENTAÇÃO_PARCIAL | This is not physical rollback of boot image/modules/files. |
| Global audit | `native/src/core/rafaelia_audit.rs`, `native/src/core/bootstages.rs` | Exposes global audit init and `log_global_operation`; boot stages call it. | CÓDIGO_EXISTENTE | Verify lock contention and failure behavior. |

## RAFAELIA telemetry system

| Component | Path | Observed behavior | Status | Risk / next test |
| --- | --- | --- | --- | --- |
| Metrics directory | `native/src/core/rafaelia_telemetry.rs` | Creates `/data/adb/magisk/rafaelia_metrics`. | CÓDIGO_EXISTENTE | Directory permissions are not explicitly set by Rust telemetry init; activation script sets 700. |
| Collection interval | `native/src/core/bootstages.rs`, `native/src/core/rafaelia_telemetry.rs` | Boot init passes `1000` ms; collector sleeps for configured interval. | CÓDIGO_EXISTENTE | Measure CPU/I/O at 1s interval. |
| CPU | `native/src/core/rafaelia_telemetry.rs`; `tools/rafaelia/metrics_collector.sh` | Reads `/proc/stat` and computes delta usage. | CÓDIGO_EXISTENTE | Validate first-sample semantics and multi-core math. |
| Memory | `native/src/core/rafaelia_telemetry.rs`; `tools/rafaelia/metrics_collector.sh` | Reads `/proc/meminfo`. | CÓDIGO_EXISTENTE | Compare with `dumpsys meminfo`/`top`. |
| I/O | `native/src/core/rafaelia_telemetry.rs`; `tools/rafaelia/metrics_collector.sh` | Reads `/proc/diskstats` and computes deltas. | CÓDIGO_EXISTENTE | Device-specific block device filtering may be needed. |
| Network | `native/src/core/rafaelia_telemetry.rs`; `tools/rafaelia/metrics_collector.sh` | Reads `/proc/net/dev`, skips loopback, sums interfaces. | CÓDIGO_EXISTENTE | Validate interface filtering. |
| JSONL | `native/src/core/rafaelia_telemetry.rs`; `tools/rafaelia/metrics_collector.sh` | Writes structured metric lines. | CÓDIGO_EXISTENTE | Add current schema docs/tests. |
| Rotation | `tools/rafaelia/metrics_collector.sh` | Shell collector includes `rotate_logs`; Rust collector rotation was not found. | IMPLEMENTAÇÃO_PARCIAL / RISCO | Add explicit Rust rotation or size cap. |

## Operational scripts audit

| Script | Purpose | Inputs | Outputs / touched paths | Requires root | Writes `/data/adb` | CI safe | Dry-run | Risk / next test |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `tools/rafaelia/activate_rafaelia.sh` | Activate shell-layer RAFAELIA directories, manifest, tools, services, integrity check. | Command argument: `activate`, `status`, `stop`, `help`. | `/data/adb/magisk/rafaelia_audit`, `/data/adb/magisk/rafaelia_metrics`, `/data/adb/RAFAELIA_MANIFEST.json`, `/data/local/tmp/rafaelia`. | Yes (`id -u`). | Yes. | No, unless sandboxed/mocked. | No. | Root writes and service start; add dry-run/mock mode before CI use. |
| `tools/rafaelia/metrics_collector.sh` | Continuous or one-shot shell metrics collection. | Command and environment variables such as interval/sample count. | `/data/adb/magisk/rafaelia_metrics`, `/proc/*` reads, JSON output. | Directory path implies Android/root context; script itself creates target dir. | Yes. | No for default path; possible with refactor to configurable output root. | No. | Continuous loop can grow logs and consume battery/I/O. |
| `tools/rafaelia/integrity_checker.sh` | Checks boot/modules/database/audit/manifest/SELinux/properties. | Check argument: `full`, `boot`, `modules`, `database`, `audit`, `manifest`. | Reads Magisk paths, manifest, system properties, SELinux status. | Yes (`id -u`). | Mostly reads; may depend on root-only paths. | No. | No. | Should support non-root mock input for CI. |
| `scripts/boot_patch.sh` | Magisk boot image patch path. | Boot image path/environment sourced by installer. | Boot/ramdisk working files; can repack images. | Device/install context. | Indirect. | No. | No. | Destructive if misused; do not run outside controlled workflow. |
| `scripts/uninstaller.sh` | Magisk uninstall/restore path. | Installer args/environment. | Boot image restoration, cache/data/metadata/persist cleanup, optional reboot. | Yes/device context. | Yes/removes. | No. | No. | Destructive removal path. |
| `scripts/addon.d.sh` | OTA addon.d survival/restore integration. | addon.d command context. | `/data/adb/magisk`, addon.d backup/restore hooks. | Device OTA context. | Yes. | No. | No. | System OTA integration risk. |
| `scripts/pre_ci_validate.py` | Host-side pre-CI checks for dependencies, security, style, tests, build readiness. | `--strict`, `--fix`, `--skip-slow`. | Reads repo; with `--fix` may chmod scripts/Python files. | No. | No. | Yes for default validation; do not use `--fix` if audit must be read-only. | Not applicable. | Some checks depend on optional tools such as cargo-audit. |
