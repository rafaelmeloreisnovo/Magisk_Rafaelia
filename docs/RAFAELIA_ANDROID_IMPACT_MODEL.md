# RAFAELIA Android Impact Model

This is an estimation model, not a benchmark report. It must not be used as proof of performance, battery impact, stability, or safety until measured on real target devices.

## What changes when enabled

| Layer | Observed code path | Expected effect | Evidence status |
| --- | --- | --- | --- |
| Boot audit | `native/src/core/bootstages.rs`, `native/src/core/rafaelia_audit.rs` | Boot stages can emit JSONL audit entries under `/data/adb/magisk/rafaelia_audit`. | CÓDIGO_EXISTENTE; execution proof pending. |
| Telemetry thread | `native/src/core/bootstages.rs`, `native/src/core/rafaelia_telemetry.rs` | A background thread can sample `/proc` every 1000 ms in daemon context. | CÓDIGO_EXISTENTE; benchmark pending. |
| Shell metrics collector | `tools/rafaelia/metrics_collector.sh` | If started, a shell loop samples `/proc` and writes metrics. | CÓDIGO_EXISTENTE; not automatically proven as running. |
| Zygisk | `native/src/core/zygisk/*` | NativeBridge/property/hooks/module loading can affect zygote-derived processes when Zygisk is enabled. | CÓDIGO_EXISTENTE; device validation pending. |
| Root scripts | `tools/rafaelia/*.sh`, `scripts/*.sh` | Scripts can create/remove/modify Magisk paths, boot artifacts, modules, addon.d state. | RISCO; do not run casually. |

## Cautious impact estimates

These are qualitative ranges because the repository does not include current before/after benchmarks:

| Resource | Estimate | Why | Required proof |
| --- | --- | --- | --- |
| RAM | Low to moderate incremental daemon memory if Rust telemetry thread and bounded histories are active; additional shell process memory if shell collector runs. | In-memory histories are bounded (`MAX_AUDIT_HISTORY`, `MAX_METRICS_HISTORY`), but thread/process overhead exists. | `dumpsys meminfo`, `procrank`/`showmap`, daemon RSS before/after. |
| CPU | Usually low if interval remains 1s and parsing is lightweight; may increase on low-end devices or if both Rust and shell collectors run. | `/proc` parsing occurs periodically. | `top -H`, `simpleperf`, trace around boot and idle. |
| I/O | Low to moderate depending on JSONL frequency and rotation; risk grows if logs are not capped. | Audit and telemetry write JSONL; Rust rotation not observed. | Log file size over 24h, `iostat`/`/proc/diskstats` deltas. |
| Battery | Unknown; likely tied to wakeups, CPU parsing, and writes. | Periodic collection and shell loops can prevent deep idle if misconfigured. | `dumpsys batterystats`, Battery Historian, idle drain test. |
| Boot time | Unknown; audit/telemetry init adds work in `post_fs_data`; Zygisk hooks can affect zygote startup. | Code runs in boot path. | Boot timestamp comparison over repeated runs. |
| Stability | Unknown; Zygisk low-level hooks and RWX/RX transitions are sensitive to API/ABI/hardening changes. | Hooking and process memory operations carry inherent risk. | Controlled device matrix, crash-loop rollback validation, logcat tombstones. |

## Measurement plan

Run only on controlled test devices/emulators with backups and known-good recovery path.

1. Baseline without RAFAELIA shell activation and with current Magisk/Zygisk settings recorded.
2. Enable one layer at a time: Rust boot observability, shell metrics collector, Zygisk module behavior.
3. Repeat each scenario at least three boots and compare medians/ranges rather than one run.
4. Capture before/after:
   - `adb shell getprop sys.boot_completed` timestamps via host loop.
   - `adb shell top -b -n 1 -H`.
   - `adb shell dumpsys meminfo` for Magisk/zygote/system_server if available.
   - `adb shell dumpsys batterystats` before/after controlled idle window.
   - `adb logcat -d` filtered for Magisk/Zygisk/RAFAELIA tags.
   - `adb shell dmesg` if permitted.
   - JSONL file sizes under `/data/adb/magisk/rafaelia_audit` and `/data/adb/magisk/rafaelia_metrics`.
   - Magisk/Zygisk/module state and denylist settings.
5. Record device model, Android API level, kernel, Magisk version, Zygisk enabled/disabled, module list, battery state, thermal state.

## Safe commands suggested for future tests

These commands are suggestions only and were not run in this audit:

```bash
adb shell getprop ro.build.version.sdk
adb shell getprop ro.magisk.version
adb shell getprop ro.dalvik.vm.native.bridge
adb shell getprop sys.boot_completed
adb shell top -b -n 1 -H
adb shell dumpsys batterystats --reset
adb shell dumpsys batterystats
adb shell logcat -d | grep -E 'Magisk|Zygisk|RAFAELIA'
adb shell du -h /data/adb/magisk/rafaelia_audit /data/adb/magisk/rafaelia_metrics
```

Do not run boot patch, uninstall, or activation scripts on a personal/production device without understanding the impact.
