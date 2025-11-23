# Third-Party Licenses
# Licenças de Terceiros

This document lists all third-party libraries, tools, and dependencies used in Magisk_Rafaelia, along with their respective licenses and copyright information.

---

## Git Submodules

### 1. selinux
- **Author:** John Wu (@topjohnwu)
- **Repository:** https://github.com/topjohnwu/selinux
- **License:** GNU General Public License v3.0 (GPL-3.0)
- **Purpose:** SELinux policy manipulation and management
- **Copyright:** © 2017-2025 John Wu

### 2. lz4
- **Author:** Yann Collet and LZ4 contributors
- **Repository:** https://github.com/lz4/lz4
- **License:** BSD-2-Clause (library) / GPL-2.0 (CLI tools)
- **Purpose:** Fast compression/decompression library
- **Copyright:** © 2011-2020, Yann Collet

**BSD-2-Clause License:**
```
Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

1. Redistributions of source code must retain the above copyright notice,
   this list of conditions and the following disclaimer.
2. Redistributions in binary form must reproduce the above copyright notice,
   this list of conditions and the following disclaimer in the documentation
   and/or other materials provided with the distribution.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
POSSIBILITY OF SUCH DAMAGE.
```

### 3. libcxx
- **Author:** LLVM Project (fork by John Wu)
- **Repository:** https://github.com/topjohnwu/libcxx
- **License:** Apache License 2.0 with LLVM Exception
- **Purpose:** LLVM C++ Standard Library implementation
- **Copyright:** © 2003-2025 LLVM Project

**Apache-2.0 with LLVM Exception:**
```
Licensed under the Apache License v2.0 with LLVM Exceptions.
You may obtain a copy of the License at:
https://llvm.org/LICENSE.txt
```

### 4. cxx-rs
- **Author:** David Tolnay (fork by John Wu)
- **Repository:** https://github.com/topjohnwu/cxx
- **License:** Apache-2.0 OR MIT
- **Purpose:** Safe Rust/C++ interoperability
- **Copyright:** © David Tolnay

**Dual License (Apache-2.0 OR MIT):**
Users may choose either license for use.

### 5. lsplt (LSPlt)
- **Author:** LSPosed Team
- **Repository:** https://github.com/LSPosed/LSPlt
- **License:** GNU General Public License v3.0 (GPL-3.0)
- **Purpose:** Android native hooking framework
- **Copyright:** © LSPosed Team

### 6. system_properties
- **Author:** John Wu (@topjohnwu)
- **Repository:** https://github.com/topjohnwu/system_properties
- **License:** GNU General Public License v3.0 (GPL-3.0)
- **Purpose:** Android system properties access library
- **Copyright:** © 2017-2025 John Wu

### 7. crt0
- **Author:** John Wu (@topjohnwu)
- **Repository:** https://github.com/topjohnwu/crt0
- **License:** GNU General Public License v3.0 (GPL-3.0)
- **Purpose:** Minimal C runtime for Android
- **Copyright:** © 2017-2025 John Wu

---

## Embedded Libraries

### xz-embedded
- **Author:** Lasse Collin, Igor Pavlov
- **License:** Public Domain (mostly) / Various (see specific files)
- **Purpose:** XZ compression/decompression
- **Location:** `native/src/external/xz-embedded/`
- **Note:** LZMA SDK by Igor Pavlov is in the public domain

### XZ Utils
- **Author:** Lasse Collin and others
- **License:** Public Domain / GPL-2.0+ (various components)
- **Purpose:** LZMA/XZ compression support
- **Website:** https://tukaani.org/xz/

---

## Build Tools and Development Dependencies

### Android NDK
- **Provider:** Google LLC / Android Open Source Project (AOSP)
- **License:** Apache License 2.0
- **Purpose:** Native development kit for Android
- **Website:** https://developer.android.com/ndk
- **Copyright:** © 2005-2025 The Android Open Source Project

### Rust Toolchain
- **Provider:** Rust Foundation
- **License:** Apache-2.0 OR MIT
- **Purpose:** Rust programming language compiler and tools
- **Website:** https://www.rust-lang.org/
- **Copyright:** © 2010-2025 The Rust Project Developers

### Python
- **Provider:** Python Software Foundation
- **License:** Python Software Foundation License (PSF)
- **Purpose:** Python interpreter and standard library
- **Version:** 3.x
- **Website:** https://www.python.org/
- **Copyright:** © 2001-2025 Python Software Foundation

### Gradle
- **Provider:** Gradle Inc.
- **License:** Apache License 2.0
- **Purpose:** Build automation tool
- **Website:** https://gradle.org/

### Android SDK
- **Provider:** Google LLC
- **License:** Android Software Development Kit License
- **Purpose:** Android development tools
- **Website:** https://developer.android.com/

---

## Runtime Dependencies

### Android OS Components
- **Provider:** Android Open Source Project (AOSP)
- **License:** Apache License 2.0 (primarily)
- **Components:** System libraries, frameworks, tools
- **Copyright:** © 2005-2025 The Android Open Source Project

---

## Rust Crates (Dependencies)

The following Rust crates are used in the native code:

### Core Crates
- **libc** - MIT OR Apache-2.0 - C library bindings
- **cfg-if** - MIT OR Apache-2.0 - Conditional compilation
- **num-traits** - MIT OR Apache-2.0 - Numeric trait definitions
- **num-derive** - MIT OR Apache-2.0 - Numeric trait derivation

### Cryptography
- **blake3** - CC0-1.0 OR Apache-2.0 - BLAKE3 hashing
- **sha3** - MIT OR Apache-2.0 - SHA-3 hashing

### Serialization
- **serde** - MIT OR Apache-2.0 - Serialization framework
- **serde_json** - MIT OR Apache-2.0 - JSON serialization
- **toml** - MIT OR Apache-2.0 - TOML parsing

### Error Handling
- **anyhow** - MIT OR Apache-2.0 - Error handling
- **thiserror** - MIT OR Apache-2.0 - Error derive macros

### Logging
- **log** - MIT OR Apache-2.0 - Logging facade
- **env_logger** - MIT OR Apache-2.0 - Environment logger

### Async Runtime
- **tokio** - MIT - Async runtime (if used)

*Note: Exact crate versions and complete dependency tree can be found in `Cargo.toml` and `Cargo.lock` files.*

---

## Python Libraries (for RAFAELIA tools)

### Standard Library
- **Python Standard Library** - PSF License - Included with Python

### Potential Third-Party Libraries
*Note: Check specific requirements.txt or pyproject.toml files for exact dependencies*

Common Python dependencies that may be used:
- **hashlib** - PSF (standard library) - Cryptographic hashing
- **json** - PSF (standard library) - JSON handling
- **subprocess** - PSF (standard library) - Process execution

---

## Build Scripts and Configuration

### Shell Scripts
- Various shell scripts in the repository
- **License:** GPL-3.0 (as part of this project)
- **Authors:** John Wu (original), Rafael Melo Reis (modifications)

---

## Documentation Tools

### Markdown
- **Format:** Markdown (no license, it's a standard)
- **Tools:** Various Markdown processors (MIT/BSD/Apache typically)

---

## License Compatibility Analysis

### GPL-3.0 Compatibility

**Compatible Licenses (can be combined with GPL-3.0):**
- ✅ GPL-3.0 (same license)
- ✅ GPL-2.0 (can be upgraded to GPL-3.0)
- ✅ Apache-2.0 (explicitly compatible with GPL-3.0)
- ✅ MIT (permissive, compatible)
- ✅ BSD-2-Clause (permissive, compatible)
- ✅ Public Domain (fully permissive)

**Note:** When combining code under different licenses, the resulting work must be distributed under GPL-3.0 due to its copyleft requirements.

---

## Full License Texts

Complete license texts for all referenced licenses can be found at:

- **GPL-3.0:** [LICENSE](LICENSE) file in this repository
- **Apache-2.0:** https://www.apache.org/licenses/LICENSE-2.0
- **MIT:** https://opensource.org/licenses/MIT
- **BSD-2-Clause:** https://opensource.org/licenses/BSD-2-Clause
- **LLVM Exception:** https://llvm.org/LICENSE.txt
- **PSF License:** https://docs.python.org/3/license.html

---

## Acknowledgments

We acknowledge and thank all the open source projects and their maintainers whose work makes Magisk_Rafaelia possible. The open source community's collaborative spirit enables innovation and progress.

Special thanks to:
- **John Wu** - For Magisk and related libraries
- **AOSP/Google** - For Android platform
- **Rust Foundation** - For Rust language and tools
- **LLVM Project** - For compiler infrastructure
- **All library authors** - For their excellent work

---

## Compliance

This project strives to comply with all license requirements including:
- ✅ Preserving copyright notices
- ✅ Including license texts
- ✅ Documenting modifications
- ✅ Maintaining source code availability (GPL-3.0)
- ✅ Proper attribution

If you believe there is a license compliance issue, please open an issue at:
https://github.com/rafaelmeloreisnovo/Magisk_Rafaelia/issues

---

## Updates

This document should be updated whenever:
- New dependencies are added
- Existing dependencies are updated
- License information changes

**Last Updated:** November 23, 2025  
**Document Version:** 1.0

---

**Maintained by:** Rafael Melo Reis (rafaelmeloreisnovo)  
**Contact:** Via GitHub issues or repository
