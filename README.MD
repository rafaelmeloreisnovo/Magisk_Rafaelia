![](docs/images/logo.png)

[![Downloads](https://img.shields.io/badge/dynamic/json?color=green&label=Downloads&query=totalString&url=https%3A%2F%2Fraw.githubusercontent.com%2Ftopjohnwu%2Fmagisk-files%2Fcount%2Fcount.json&cacheSeconds=1800)](https://raw.githubusercontent.com/topjohnwu/magisk-files/count/count.json)

#### This is not an officially supported Google product

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

- **[Activation Guide](docs/ACTIVATION_GUIDE.md)** - ✓ **Start here!** Enable RAFAELIA features
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
# Clone with submodules
git clone --recurse-submodules https://github.com/rafaelmeloreisnovo/Magisk_Rafaelia.git
cd Magisk_Rafaelia

# Install Magisk NDK
python3 build.py ndk

# Build everything
python3 build.py -v all
```

See [COMO_OBTER_APK.md](COMO_OBTER_APK.md) or [HOW_TO_GET_APK.md](HOW_TO_GET_APK.md) for detailed instructions.

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
