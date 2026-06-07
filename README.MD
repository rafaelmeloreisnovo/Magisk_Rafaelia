![](docs/images/logo.png)

[![Downloads](https://img.shields.io/badge/dynamic/json?color=green&label=Downloads&query=totalString&url=https%3A%2F%2Fraw.githubusercontent.com%2Ftopjohnwu%2Fmagisk-files%2Fcount%2Fcount.json&cacheSeconds=1800)](https://raw.githubusercontent.com/topjohnwu/magisk-files/count/count.json)

#### This is not an officially supported Google product

---

## 🌿 Two-Branch Adaptive Strategy

**Magisk_Rafaelia** uses an evolutionary two-branch workflow for optimal development:

- **`main`**: Stable, production-ready releases
- **`develop`**: Active development and integration

This cognitive-optimized approach:
- ✅ Minimizes mental overhead
- ✅ Ensures code quality through clear gates
- ✅ Enables rapid iteration on develop
- ✅ Maintains stability on main

**Contributing?** See [CONTRIBUTING.md](CONTRIBUTING.md) and [Branching Strategy](docs/BRANCHING_STRATEGY.md)

---

## 🌍 Multilingual Documentation / Documentação Multilíngue

**Documentation available in 91 languages!** / **Documentação disponível em 91 idiomas!**

📚 [**View All Languages / Ver Todos os Idiomas**](docs/LANGUAGES.md) 

📘 **Engineering documentation hub**: [docs/README.md](docs/README.md)  
🧭 **Build/Release/CI source of truth**: [docs/ENGINEERING_PLAYBOOK.md](docs/ENGINEERING_PLAYBOOK.md)

Popular languages / Idiomas populares:
- 🇬🇧 [English](docs/languages/en/README.md)
- 🇧🇷 [Português (Brasil)](docs/languages/pt-BR/README.md)
- 🇪🇸 [Español](docs/languages/es/README.md)
- 🇨🇳 [简体中文](docs/languages/zh-CN/README.md)
- 🇯🇵 [日本語](docs/languages/ja/README.md)
- 🇩🇪 [Deutsch](docs/languages/de/README.md)
- 🇫🇷 [Français](docs/languages/fr/README.md)
- 🇷🇺 [Русский](docs/languages/ru/README.md)

---

## 🚀 Quick Start

**Want the APK?**
- 🇧🇷 [**OBTER APK RÁPIDO**](OBTER_APK_RAPIDO.md) ← Comece aqui! (Português)
- 🇬🇧 [**GET APK QUICK**](GET_APK_QUICK.md) ← Start here! (English)

**Need help compiling for ARM64 or fixing CI checks?**
- 🔧 [**Guia de Troubleshooting ARM64**](GUIA_COMPILACAO_ARM64_TROUBLESHOOTING.md) ← Como acompanhar até compilar! (PT-BR)
- 🔧 [**ARM64 Troubleshooting Guide**](ARM64_COMPILATION_TROUBLESHOOTING_GUIDE.md) ← Track to compilation! (EN)
- 📊 [**CI Quick Reference**](CI_MONITORING_QUICKREF.md) ← Monitor checks
- 📈 [**Workflow Diagrams**](ARM64_COMPILATION_WORKFLOW_DIAGRAM.md) ← Visual guide

---

## Introduction

Magisk is a suite of open source software for customizing Android, supporting devices higher than Android 6.0.<br>
Some highlight features:

- **MagiskSU**: Provide root access for applications
- **Magisk Modules**: Modify read-only partitions by installing modules
- **MagiskBoot**: The most complete tool for unpacking and repacking Android boot images
- **Zygisk**: Run code in every Android applications' processes

## Downloads

**Magisk_Rafaelia APK:**
- 📱 **[Como Obter o APK (Português)](COMO_OBTER_APK.md)** - Guia completo em português
- 📱 **[How to Get APK (English)](HOW_TO_GET_APK.md)** - Complete guide in English
- 🔄 [GitHub Actions Artifacts](../../actions) - Download pre-built APKs

**Official Magisk:**
- [Github](https://github.com/topjohnwu/Magisk/releases) is the only source where you can get official Magisk information and downloads.

## RAFAELIA Framework

**Magisk_Rafaelia / PCR_Rafaelia_Code_seed** is a Magisk-derived repository with an added RAFAELIA documentation, audit, telemetry, tooling, and governance layer. The repository contains code for Magisk boot/init/root/SELinux/modules, Zygisk/NativeBridge hooks, low-level memory/syscall helpers, Rust RAFAELIA audit and telemetry modules, and root-oriented operational scripts.

This README intentionally separates observed code from unproven claims:

- **CÓDIGO_EXISTENTE:** Rust audit logging exists in `native/src/core/rafaelia_audit.rs`; Rust telemetry exists in `native/src/core/rafaelia_telemetry.rs`; boot-stage integration calls these modules in `native/src/core/bootstages.rs`; Zygisk/NativeBridge code exists under `native/src/core/zygisk/`; low-level syscall/memory helpers exist in `native/src/base/lowlevel.c` and `native/src/core/zygisk/lowlevel_inject.cpp`.
- **DOCUMENTAÇÃO_EXISTENTE:** RAFAELIA state matrix, primitives, framework, audit, telemetry, governance, and build documents exist under `docs/` and repository root files such as `BUILD_SUCCESS.md`.
- **RELATÓRIO_EXISTENTE:** `BUILD_SUCCESS.md` records a previous native multi-ABI build and an APK build limitation caused by network access; it must be treated as historical until current CI/build logs confirm it again.
- **DOCUMENTADO_NÃO_ACOPLADO / TOKEN_VAZIO:** Claims such as SHA3/Blake3 audit verification, anomaly detection, performance gains, RAFAELIA-specific seccomp/eBPF integration, and runtime enforcement of the 56 × 18 state matrix require direct code/test/log evidence before being promoted.

### Safety notice for root/Magisk operations

The RAFAELIA activation and integrity scripts are designed for a rooted Android/Magisk environment and reference paths such as `/data/adb`, `/data/adb/magisk`, and `/data/local/tmp/rafaelia`. Boot patching, uninstall, addon.d, module, Zygisk, and `/data/adb` operations can change device boot behavior or Magisk state. Do **not** run root scripts, boot patching scripts, uninstallers, or module operations on a production/personal device unless you understand the impact, have backups, and are using a controlled test environment.

### Conservative audit documentation

Start with these documents for traceable, non-marketing status:

- [RAFAELIA Architecture Audit](docs/RAFAELIA_ARCHITECTURE_AUDIT.md) — architecture map, evidence labels, confirmed/unproven claims, and limits.
- [RAFAELIA Zygisk and Low-Level Audit](docs/RAFAELIA_ZYGISK_LOWLEVEL_AUDIT.md) — NativeBridge, Zygisk hooks, module callbacks, low-level syscalls, memory protection, and hook risks.
- [RAFAELIA Boot and Telemetry Audit](docs/RAFAELIA_BOOT_TELEMETRY_AUDIT.md) — boot stages, audit/telemetry JSONL, `/proc` collection, and script safety review.
- [RAFAELIA Android Impact Model](docs/RAFAELIA_ANDROID_IMPACT_MODEL.md) — cautious impact estimates and a before/after measurement plan.
- [RAFAELIA Governance and Privacy Audit](docs/RAFAELIA_GOVERNANCE_AND_PRIVACY.md) — anti-inference policy, privacy paths, `TOKEN_VAZIO` list, and root/Magisk risks.
- [RAFAELIA Documentation Status](docs/RAFAELIA_DOCUMENTATION_STATUS.md) — files read, documentation updated, pending validation.
- [RAFAELIA Traceability Matrix](docs/RAFAELIA_TRACEABILITY_MATRIX.md) — claim-by-claim map: `ID | Claim | Tipo | Caminho | Evidência | Status | Lacuna | Próximo teste`.

### Existing RAFAELIA documentation

These documents remain useful as design/specification context, but each technical claim should be cross-checked against the audit matrix before being treated as implemented or validated:

- [RAFAELIA Index](docs/RAFAELIA_INDEX.md)
- [Activation Guide](docs/ACTIVATION_GUIDE.md)
- [Framework Overview](docs/RAFAELIA_FRAMEWORK.md)
- [State Matrix](docs/RAFAELIA_STATE_MATRIX.csv)
- [Primitives](docs/RAFAELIA_PRIMITIVES.json)
- [Audit System](docs/RAFAELIA_AUDIT_SYSTEM.md)
- [Telemetry](docs/RAFAELIA_TELEMETRY.md)
- [Operational Checklist](docs/RAFAELIA_CHECKLIST.md)

**Philosophy preserved as project context:** VAZIO → VERBO → CHEIO → RETRO (Empty → Action → Full → Feedback).

## Building

To build Magisk_Rafaelia locally:

```bash
# Clone with submodules (use develop for latest features, main for stable)
git clone --recurse-submodules https://github.com/rafaelmeloreisnovo/Magisk_Rafaelia.git
cd Magisk_Rafaelia

# Checkout desired branch
git checkout develop  # For development
# or
git checkout main     # For stable release

# (Optional) Configure for ARM64 only (faster builds)
cp config.prop.sample config.prop
# Edit config.prop and uncomment: abiList=arm64-v8a

# Install Magisk NDK
python3 build.py ndk

# Build everything
python3 build.py -v all
```

### 🔧 Troubleshooting & Monitoring

**Need help fixing compilation errors or tracking CI checks?**

- 🇧🇷 [**Guia Completo de Troubleshooting**](GUIA_COMPILACAO_ARM64_TROUBLESHOOTING.md) - Como acompanhar até compilar para ARM64, arrumando cada vez que checks falham (Português)
- 🇬🇧 [**Complete Troubleshooting Guide**](ARM64_COMPILATION_TROUBLESHOOTING_GUIDE.md) - How to track ARM64 compilation and fix failing checks (English)
- 📊 [**CI/CD Quick Reference**](CI_MONITORING_QUICKREF.md) - Monitor GitHub Actions status and logs

### 📚 Build Documentation

**Build Guides:**
- 🇧🇷 [**Como Compilar para ARM64**](COMPILAR_ARM64.md) - Guia rápido em português
- 🇬🇧 [**How to Build for ARM64**](BUILD_ARM64.md) - Quick guide in English
- [COMO_OBTER_APK.md](COMO_OBTER_APK.md) - Guia completo em português
- [HOW_TO_GET_APK.md](HOW_TO_GET_APK.md) - Complete guide in English

**Note:** By default, builds include all architectures. For ARM64-only builds (recommended for modern devices like RMX3834), copy `config.prop.sample` to `config.prop` and set `abiList=arm64-v8a`.

## Useful Links

- [Installation Instruction](https://topjohnwu.github.io/Magisk/install.html)
- [Building and Development](docs/build.md)
- [Magisk Documentation](https://topjohnwu.github.io/Magisk/)
- [Zygisk module sample](https://github.com/topjohnwu/zygisk-module-sample)

## Bug Reports

**Only bug reports from Debug builds will be accepted.**

For installation issues, upload both boot image and install logs.<br>
For Magisk issues, upload boot logcat or dmesg.<br>
For Magisk app crashes, record and upload the logcat when the crash occurs.

## Translation Contributions

Default string resources for the Magisk app and its stub APK are located here:

- `app/core/src/main/res/values/strings.xml`
- `app/stub/src/main/res/values/strings.xml`

Translate each and place them in the respective locations (`[module]/src/main/res/values-[lang]/strings.xml`).

---

## 📋 Governance & Standards

**Magisk_Rafaelia** operates under a comprehensive governance framework defined in **[ativar.txt v999](ativar.txt)**.

### Applied International Standards

- **ISO**: 9001 (Quality), 27001 (Security), 27002 (Security Controls), 27018 (Privacy), 25010 (Software Quality), 8000 (Data Quality)
- **IEEE**: 830 (Requirements), 1012 (V&V), 12207 (Lifecycle), 14764 (Maintenance), 1633 (Reliability), 42010 (Architecture)
- **NIST**: Cybersecurity Framework, SP 800-53 (Security Controls), SP 800-207 (Zero Trust), AI Risk Management Framework
- **ITU-T**: X Series (Data Networks), Y.3500 (Cloud Computing), F.747 (Multimedia)
- **W3C**: JSON, YAML, Web Architecture
- **ABNT**: NBR ISO/IEC standards

### Ethica[8] Framework

All development follows eight ethical principles:

1. **Transparency** 🔍 - Open communication and documentation
2. **Accountability** 📋 - Clear responsibility and ownership
3. **Fairness** ⚖️ - Equitable treatment for all
4. **Privacy** 🔒 - Respect for personal information
5. **Security** 🛡️ - Protection of systems and data
6. **Reliability** 🔧 - Dependable operation
7. **Safety** 🛟 - No harm to users or systems
8. **Sustainability** ♻️ - Long-term responsibility

### Documentation

- **Governance Framework**: [ativar.txt](ativar.txt)
- **Implementation Guide**: [GOVERNANCE_IMPLEMENTATION.md](GOVERNANCE_IMPLEMENTATION.md)
- **Ethics Framework**: [Ethica[8]](docs/governance/ethics/ethica-8-framework.md)
- **Security Policy**: [ISMS Policy](docs/governance/security/isms-policy.md)

### Continuous Improvement

The project operates with an **Infinite Feedback Loop** for perpetual enhancement:
- Automated quality gates and security scanning
- Regular compliance audits
- Community feedback integration
- Continuous learning and adaptation

---

## License

    Magisk, including all git submodules are free software:
    you can redistribute it and/or modify it under the terms of the
    GNU General Public License as published by the Free Software Foundation,
    either version 3 of the License, or (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.
