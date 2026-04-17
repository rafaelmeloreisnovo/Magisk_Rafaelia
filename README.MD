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

**Magisk_Rafaelia** includes the comprehensive **RAFAELIA Framework** for advanced auditing, monitoring, and security:

- **1008 State Matrix**: Complete operational state coverage (56 primitives × 18 contexts)
- **Full Audit System**: SHA3/Blake3 verified logging with rollback support ✓ **IMPLEMENTED**
- **Real-time Telemetry**: CPU, memory, I/O monitoring with anomaly detection ✓ **IMPLEMENTED**
- **Security Hardening**: SELinux, seccomp, eBPF integration
- **Ethical Computing**: Transparency, accountability, and safety guarantees
- **Baremetal Optimization**: Hardware-specific SIMD, cache, and assembly optimizations ✓ **NEW**

### 🚀 Quick Start

**Activate RAFAELIA** (requires root):
```bash
adb push tools/rafaelia /data/local/tmp/
adb shell
su
cd /data/local/tmp/rafaelia
./activate_rafaelia.sh activate
```

See [Activation Guide](docs/ACTIVATION_GUIDE.md) for detailed instructions.

### RAFAELIA Documentation

#### 📚 Start Here
- **[RAFAELIA Index](docs/RAFAELIA_INDEX.md)** - ✓ **Complete navigation guide** to all documentation
- **[Activation Guide](docs/ACTIVATION_GUIDE.md)** - ✓ Enable RAFAELIA features (practical start)

#### 🏛️ Meta-Architecture (Theoretical Foundation)
- **[Meta-Architecture](docs/RAFAELIA_META_ARCHITECTURE.md)** - ✓ **NEW!** 30 holistic analyses
  - Livro I: Hardware as Metaphor (ICE, 4-64 bit spectrum, DSP, ADC)
  - Livro II: Data Nexus (Interleaving, Permutation, Fractal Geometry)
  - Livro III: Software Abstraction (Yin-Yang model, CLIMEX, PLIMEX)
- **[Toolkit Analysis](docs/RAFAELIA_TOOLKIT_ANALYSIS.md)** - ✓ **NEW!** retro_feed.py, bootctl, futility

#### 🎯 Baremetal & Optimization (NEW!)
- **[Baremetal Architecture Analysis](BAREMETAL_ARCHITECTURE_ANALYSIS.md)** - ✓ **NEW!** Pre-CI validation & transformation strategy
- **[Hardware Optimization Guide](HARDWARE_OPTIMIZATION_GUIDE.md)** - ✓ **NEW!** SIMD, cache, assembly optimizations
- **[Implementation Summary](BAREMETAL_IMPLEMENTATION_SUMMARY.md)** - ✓ **NEW!** Complete status & roadmap

#### 🚀 Performance Boosters (NEW!)
- **[Performance Boosters Guide](docs/PERFORMANCE_BOOSTERS.md)** - ✓ **NEW!** Complete guide to all performance boosters
  - 16 distinct booster types (Software, Hardware, Mathematical, Cognitive)
  - Detailed benchmarks and performance gains (up to 6x improvement)
  - Usage examples and activation instructions
- **[Guia de Boosters PT-BR](docs/BOOSTERS_PERFORMANCE_PT.md)** - ✓ **NEW!** Versão completa em português

#### 🔧 Implementation & Operations
- [Framework Overview](docs/RAFAELIA_FRAMEWORK.md) - Complete specification
- [Summary](docs/RAFAELIA_SUMMARY.md) - Quick overview and features
- [Implementation Guide](docs/RAFAELIA_IMPLEMENTATION_GUIDE.md) - Step-by-step guide
- [State Matrix](docs/RAFAELIA_STATE_MATRIX.csv) - All 1008 states
- [Primitives](docs/RAFAELIA_PRIMITIVES.json) - Detailed primitive specs
- [Audit System](docs/RAFAELIA_AUDIT_SYSTEM.md) - Audit and rollback
- [Telemetry](docs/RAFAELIA_TELEMETRY.md) - Monitoring and observability
- [Operational Checklist](docs/RAFAELIA_CHECKLIST.md) - Daily/weekly/monthly tasks

### Implementation Status

✅ **Core Modules** (Rust)
- Audit system: `native/src/core/rafaelia_audit.rs`
- Telemetry system: `native/src/core/rafaelia_telemetry.rs`

✅ **Tools** (Python & Shell)
- State validator, metrics collector, integrity checker
- Audit analyzer, activation script

🔄 **In Progress**
- FFI bindings for C++ integration
- Daemon integration
- Comprehensive testing

**Philosophy**: VAZIO → VERBO → CHEIO → RETRO (Empty → Action → Full → Feedback)

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
