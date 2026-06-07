# RAFAELIA Governance and Privacy Audit

## Governance stance

This repository contains RAFAELIA governance, ethics, state matrix, manifest, audit, and telemetry documentation. Documentation is useful context, but it is not execution proof. This audit uses explicit labels and `TOKEN_VAZIO` to avoid converting aspirational language into unsupported claims.

## Claim classification policy

| Claim source | Allowed classification | Promotion rule |
| --- | --- | --- |
| Markdown only | DOCUMENTAÇÃO_EXISTENTE | Do not state as implemented unless code path exists. |
| Manifest/data only | DOCUMENTAÇÃO_EXISTENTE or FATO_OBSERVADO for file existence | Do not state as enforced unless runtime code consumes it. |
| Code exists | CÓDIGO_EXISTENTE | If no current build/test/log, add CÓDIGO_SEM_PROVA_DE_EXECUÇÃO_ATUAL. |
| Historical report | RELATÓRIO_EXISTENTE | Must be revalidated before claiming current success. |
| Missing origin | TOKEN_VAZIO | Do not infer. Ask for provenance or add implementation/test. |
| Partial mechanism | IMPLEMENTAÇÃO_PARCIAL | State exact boundary and next test. |

## Privacy and data locality

| Data / path | Evidence | Data type | Privacy status | Risk |
| --- | --- | --- | --- | --- |
| `/data/adb/magisk/rafaelia_audit/*.jsonl` | `native/src/core/rafaelia_audit.rs`, `tools/rafaelia/activate_rafaelia.sh` | Operation names, contexts, success/error fields, timestamps, optional hashes. | Local file path in Magisk area. | Root-readable operational logs can reveal device behavior; Rust rotation not observed. |
| `/data/adb/magisk/rafaelia_metrics/*.jsonl` | `native/src/core/rafaelia_telemetry.rs`, `tools/rafaelia/metrics_collector.sh` | CPU/memory/I/O/network counters, timestamps. | Local telemetry; no external upload path found in audited code. | Continuous collection can grow logs and expose usage patterns. |
| `/data/adb/RAFAELIA_MANIFEST.json` | `tools/rafaelia/activate_rafaelia.sh`, repository `RAFAELIA_MANIFEST.json` | Signature, timestamp, version, component booleans/configuration. | Local manifest. | Manifest may overstate components if not synchronized with code. |
| `/data/local/tmp/rafaelia` | `tools/rafaelia/activate_rafaelia.sh` | Operational tool staging path. | Local temp path. | Executable scripts in tmp path require root caution. |
| Magisk DB / module paths | `native/src/core/lib.rs`, `native/src/core/bootstages.rs`, scripts | Root, denylist, Zygisk, module state. | Magisk internal data. | High sensitivity; avoid logging secret values. |

## Permissions and hardening observations

| Item | Evidence | Status |
| --- | --- | --- |
| Secure Magisk dir 700 | `native/src/core/bootstages.rs` applies 0700 to `SECURE_DIR`. | CÓDIGO_EXISTENTE |
| RAFAELIA audit/metrics dirs 700 | `tools/rafaelia/activate_rafaelia.sh` sets chmod 700 after creating directories. | CÓDIGO_EXISTENTE in shell activation; not proof for Rust-created dirs. |
| Manifest 600 | `tools/rafaelia/activate_rafaelia.sh` sets chmod 600 on `/data/adb/RAFAELIA_MANIFEST.json`. | CÓDIGO_EXISTENTE in shell activation. |
| SELinux | `native/src/sepolicy/api.cpp`, `native/src/core/bootstages.rs`, `tools/rafaelia/integrity_checker.sh` | Magisk SELinux code/checks exist; RAFAELIA-specific enforcement beyond documented checks is TOKEN_VAZIO. |
| `ro.magisk.version` | `tools/rafaelia/integrity_checker.sh` reads Magisk properties. | CÓDIGO_EXISTENTE checker. |
| `module.prop` | `docs/guides.md`, Magisk module structure. | DOCUMENTAÇÃO_EXISTENTE; runtime module metadata support belongs to Magisk base. |

## TOKEN_VAZIO list

| Missing / unproven item | Why it is TOKEN_VAZIO | Next step |
| --- | --- | --- |
| Current successful build for this branch | This audit did not run native/app builds. | Run CI or local build in prepared Android SDK/NDK environment. |
| Current APK artifact validity | Workflows exist; no current artifact was generated here. | Download/upload artifact from current workflow run and verify zip/APK. |
| RAFAELIA state matrix runtime enforcement | Matrix files exist; runtime consumption/enforcement call path was not proven. | Add tests showing code loads and enforces matrix rules. |
| SHA3/Blake3 audit verification | Audit entries have optional hash fields; this audit did not find algorithm implementation in audit logging path. | Implement or document exact hash source; add tests. |
| Anomaly detection | Telemetry sampling exists; detector logic not proven. | Add detector path/tests or remove claim. |
| RAFAELIA-specific seccomp/eBPF | General hardening language exists; no verified RAFAELIA seccomp/eBPF runtime path in this audit. | Link to code/tests or mark as roadmap. |
| Performance/battery improvement | No benchmark artifact in this audit. | Run before/after measurement plan. |

## Security risks

- Root scripts lack dry-run mode and can write under `/data/adb` or `/data/local/tmp`.
- Magisk boot patch and uninstall scripts are operationally destructive outside controlled flows.
- Rust audit/telemetry JSONL rotation is not documented as implemented; long-running devices may accumulate logs.
- Dual telemetry can exist: Rust daemon telemetry plus shell collector. Running both without coordination can duplicate sampling and writes.
- Low-level hook code uses executable memory, page protection changes, and process memory primitives; these require ABI/API/hardening tests.
- Logical rollback in `rafaelia_audit.rs` should not be described as full physical rollback.

## Governance next steps

1. Keep README claims conservative and link to traceability docs.
2. Add CI checks that fail when README claims lack a traceability-matrix row.
3. Add dry-run/mock modes to root scripts before using them in CI.
4. Add log rotation/retention policy for Rust audit and telemetry.
5. Add privacy statement for local telemetry fields and retention.
