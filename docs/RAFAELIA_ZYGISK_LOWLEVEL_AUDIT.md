# RAFAELIA Zygisk and Low-Level Audit

This document maps repository evidence for Zygisk, NativeBridge, hooks, low-level memory operations, and syscall-level helpers. No device Zygisk execution was performed during this audit.

## Zygisk / NativeBridge traceability

| Item | Path | Function / class / region | What the code does | Runtime coupling | Gap / next test |
| --- | --- | --- | --- | --- | --- |
| NativeBridge property | `native/src/core/zygisk/daemon.rs` | `NBPROP`, `ZYGISKLDR`, `set_prop`, `restore_prop` | Targets `ro.dalvik.vm.native.bridge` and prefixes/restores `libzygisk.so`. | Coupled to MagiskD Zygisk state and Android properties. | Test property before/after on controlled device. |
| libzygisk loader | `native/src/core/zygisk/daemon.rs`, `native/src/core/zygisk/hook.cpp` | `ZYGISKLDR`, hook bootstrap comments/code | Uses `libzygisk.so` as native bridge loader. | Coupled through NativeBridge property. | Confirm loader is present in built artifacts. |
| Unix socket / FD passing | `native/src/core/zygisk/daemon.rs`, `native/src/core/socket.rs` | `UnixStream::pair`, `send_fds`, `read_decodable` | Creates socket pair, forks zygiskd, sends module/client FDs. | Coupled to `ZygiskRequest::ConnectCompanion` and zygiskd startup. | Device log and fd leak test. |
| Denylist/process flags | `native/src/core/zygisk/daemon.rs`, `native/src/core/lib.rs` | `update_deny_flags`, `ZygiskStateFlags`, `zygisk_should_load_module` | Computes root/denylist/Magisk-app flags and only sends module FDs when policy allows. | Coupled to daemon database/settings and process query. | Test denylist matrix on controlled device. |
| Crash rollback | `native/src/core/zygisk/daemon.rs` | `reset`, `start_count > 3`, `restore_prop` | Restores native bridge property after repeated zygote crashes. | Coupled to Zygisk reset path. | Force crash only on lab device/emulator; verify property restoration. |
| Unloaded marker | `native/src/core/zygisk/daemon.rs` | failed module handling | Creates `unloaded` marker under a failed module directory after system_server reports failed modules. | Coupled to system_server failed module report. | Test with intentionally failing module in disposable environment. |
| PLT bootstrap hooks | `native/src/core/zygisk/hook.cpp` | hook bootstrap and PLT hook calls | Comments and code describe hooks across `libandroid_runtime`, `libart`, and `libnative_bridge`. | Coupled to zygote startup symbol availability. | Verify symbols on target Android versions. |
| JNI hooks | `native/src/core/zygisk/jni_hooks.hpp`, `native/src/core/zygisk/api.hpp` | zygote method hook table; `hookJniNativeMethods` | Replaces zygote JNI native method entries and exposes module JNI hook API. | Coupled to Android zygote method signatures. | Signature compatibility test per Android API level. |
| Module API entry | `native/src/core/zygisk/api.hpp` | `REGISTER_ZYGISK_MODULE`, `zygisk_module_entry`, callback table | Defines module entry macro and callback pointers. | Coupled to third-party Zygisk module ABI. | Build sample module against headers. |
| Specialization callbacks | `native/src/core/zygisk/api.hpp`, `native/src/core/zygisk/module.cpp` | `preAppSpecialize`, `postAppSpecialize`, `preServerSpecialize`, `postServerSpecialize` | Exposes and invokes lifecycle callbacks around app/server specialization. | Coupled to module loading and zygote specialization path. | Device trace with test module emitting logcat markers. |
| `android_dlopen_ext` | `native/src/core/zygisk/hook.cpp` | symbol hook region | Hook infrastructure references dynamic loading interception. | Coupled to linker/libdl symbol resolution. | Verify on API targets with symbol availability. |
| `fork`, `unshare`, `selinux_android_setcontext`, `dlclose`, `pthread_attr_destroy` | `native/src/core/zygisk/hook.cpp` | PLT hook targets | These are part of the Zygisk lifecycle interception set in hook code/comments. | Coupled to zygote/libandroid_runtime/libart behavior. | ABI/API specific integration test. |

## Low-level / memory traceability

| Item | Path | Function / class / region | Observed behavior | Status | Gap / next test |
| --- | --- | --- | --- | --- | --- |
| Direct syscalls | `native/src/base/lowlevel.c` | syscall wrappers | Wraps low-level Linux syscalls instead of libc convenience calls for selected operations. | CÓDIGO_EXISTENTE | Compile for every ABI; add syscall error-path tests where safe. |
| `openat/read/write/close` | `native/src/base/lowlevel.c` | low-level file wrappers | Provides direct syscall wrappers for basic file I/O. | CÓDIGO_EXISTENTE | Host smoke tests against temp files. |
| `mmap/munmap/mprotect` | `native/src/base/lowlevel.c`, `native/src/core/zygisk/lowlevel_inject.cpp` | memory map/protection helpers | Allocates, frees, and changes page protection; hook helpers temporarily use writable/executable page permissions then restore RX. | CÓDIGO_EXISTENTE + RISCO | Ensure W^X policy compatibility and failure rollback. |
| `process_vm_readv/writev` | `native/src/base/lowlevel.c`, `native/src/core/zygisk/lowlevel_inject.cpp` | target memory read/write helpers | Provides process memory read/write primitives. | CÓDIGO_EXISTENTE + RISCO | Only test in controlled process; document permission requirements. |
| Memory barriers | `native/src/base/lowlevel.c`, `native/src/core/zygisk/lowlevel_inject.cpp` | barrier helpers / barrier calls | Uses barriers before/after hook writes. | CÓDIGO_EXISTENTE | Add architecture-specific compile checks. |
| I-cache flush | `native/src/base/lowlevel.c`, `native/src/core/zygisk/lowlevel_inject.cpp` | `lowlevel_flush_icache`, shellcode injection | Flushes instruction cache after code modification where architecture supports it. | CÓDIGO_EXISTENTE | ABI test on ARM/ARM64/x86/x86_64. |
| RWX/RX transition | `native/src/core/zygisk/lowlevel_inject.cpp` | hook install and restore helpers | Temporarily changes a page to writable/executable, writes pointer/code, then restores read/execute. | RISCO | Validate on devices with hardened memory policies. |
| GOT backup/restore | `native/src/core/zygisk/lowlevel_inject.cpp` | `GOTBackup` | Saves original GOT entry and can restore it. | CÓDIGO_EXISTENTE | Unit-test in controlled shared library fixture. |
| `ScopedHook` | `native/src/core/zygisk/lowlevel_inject.cpp` | RAII hook class | Installs hook and restores on destruction. | CÓDIGO_EXISTENTE | Test destructor rollback on error paths. |
| `MemoryProtection` | `native/src/core/zygisk/lowlevel_inject.cpp` | RAII protection class | Restores previous protection in destructor. | CÓDIGO_EXISTENTE | Test nested/failed protection behavior. |
| Atomic CAS | `native/src/core/zygisk/lowlevel_inject.cpp` | atomic install helper | Uses compare-and-swap for pointer replacement. | CÓDIGO_EXISTENTE | Multi-threaded fixture test. |
| Shellcode allocation | `native/src/base/lowlevel.c`, `native/src/core/zygisk/lowlevel_inject.cpp` | executable allocation helpers | Allocates executable memory and copies code. | RISCO | Avoid production use without explicit policy review. |
| Integrity byte-check | `native/src/core/zygisk/lowlevel_inject.cpp` | `verify_code_integrity` | Compares memory bytes against expected content. | CÓDIGO_EXISTENTE | Test with known buffers. |

## RAFAELIA memory model observed in Rust

| Item | Path | Evidence | Status |
| --- | --- | --- | --- |
| `VecDeque` audit history | `native/src/core/rafaelia_audit.rs` | `audit_history: Arc<Mutex<VecDeque<AuditEntry>>>` and bounded push/pop behavior. | CÓDIGO_EXISTENTE |
| `MAX_AUDIT_HISTORY` | `native/src/core/rafaelia_audit.rs` | Constant set to `1000`; old entries popped from front. | CÓDIGO_EXISTENTE |
| `AUDIT_BUFFER_SIZE` | `native/src/core/rafaelia_audit.rs` | Constant set to `8192` for `BufWriter`. | CÓDIGO_EXISTENTE |
| `RollbackPoint` | `native/src/core/rafaelia_audit.rs` | Stores ID, timestamp, primitive, context, session snapshot bytes, and audit index. | IMPLEMENTAÇÃO_PARCIAL: logical rollback only. |
| `MetricsSnapshot` | `native/src/core/rafaelia_telemetry.rs` | Snapshot groups CPU, memory, I/O, network metrics and JSON conversion. | CÓDIGO_EXISTENTE |
| `MAX_METRICS_HISTORY` | `native/src/core/rafaelia_telemetry.rs` | Constant bounds in-memory metric history. | CÓDIGO_EXISTENTE |
| `/proc` collection | `native/src/core/rafaelia_telemetry.rs` | Reads `/proc/stat`, `/proc/meminfo`, `/proc/diskstats`, `/proc/net/dev`. | CÓDIGO_EXISTENTE |
| Telemetry thread | `native/src/core/rafaelia_telemetry.rs` | `thread::spawn` loop controlled by atomic `running`. | CÓDIGO_EXISTENTE |
| JSONL | `native/src/core/rafaelia_audit.rs`, `native/src/core/rafaelia_telemetry.rs` | `to_jsonl` output and line writes. | CÓDIGO_EXISTENTE |
| Log rotation | Rust telemetry/audit code | No Rust rotation code was identified in this audit. Shell collector has rotation logic. | RISCO / TOKEN_VAZIO for Rust rotation. |
