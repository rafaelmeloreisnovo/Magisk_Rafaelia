#!/usr/bin/env python3
"""
Generate multilingual documentation structure for Magisk Rafaelia
Creates directories and README files for all 91 supported languages
"""

import json
import os
from pathlib import Path

def load_languages():
    """Load language configuration from JSON"""
    script_dir = Path(__file__).parent
    languages_file = script_dir / 'languages.json'
    
    with open(languages_file, 'r', encoding='utf-8') as f:
        data = json.load(f)
    
    return data['languages']

def create_readme_content(lang_info, is_primary=False):
    """Generate README content for a specific language"""
    code = lang_info['code']
    name = lang_info['name']
    native = lang_info['native']
    flag = lang_info['flag']
    
    # Base template
    content = f"""# {flag} Magisk Rafaelia - {native}

[← Back to Language Selection](../../LANGUAGES.md) | [🇬🇧 English](../en/README.md) | [🇧🇷 Português](../pt-BR/README.md)

---

## {native} / {name}

Welcome to the {name} documentation for Magisk Rafaelia.

**Language Code**: `{code}` {flag}

---

## 🚀 Quick Start / Início Rápido

**Magisk Rafaelia** is a customized version of Magisk with enhanced features and the RAFAELIA Framework.

### What is Magisk?

Magisk is a suite of open source software for customizing Android, supporting devices higher than Android 6.0.

**Key Features**:
- **MagiskSU**: Root access for applications
- **Magisk Modules**: Modify read-only partitions
- **MagiskBoot**: Complete tool for boot images
- **Zygisk**: Run code in every Android app process

### RAFAELIA Framework

**Magisk Rafaelia** includes the comprehensive **RAFAELIA Framework**:

- **1008 State Matrix**: Complete operational coverage
- **Full Audit System**: SHA3/Blake3 verified logging
- **Real-time Telemetry**: CPU, memory, I/O monitoring
- **Security Hardening**: SELinux, seccomp, eBPF integration

---

## 📱 Download / Baixar

### Get the APK / Obter o APK

**Quick Download Options**:
- 🇧🇷 [OBTER APK RÁPIDO](../../../OBTER_APK_RAPIDO.md) (Português)
- 🇬🇧 [GET APK QUICK](../../../GET_APK_QUICK.md) (English)

**Detailed Guides**:
- 📱 [Como Obter o APK (Português)](../../../COMO_OBTER_APK.md)
- 📱 [How to Get APK (English)](../../../HOW_TO_GET_APK.md)
- 🔄 [GitHub Actions Artifacts](../../../../actions)

---

## 📚 Documentation / Documentação

### Core Documentation
- [Installation Instructions](../../install.md)
- [FAQ - Frequently Asked Questions](../../faq.md)
- [Building Magisk](../../build.md)
- [Developer Guides](../../guides.md)

### RAFAELIA Framework Documentation
- [RAFAELIA Index](../../RAFAELIA_INDEX.md) - Complete navigation
- [Activation Guide](../../ACTIVATION_GUIDE.md) - Enable RAFAELIA features
- [Framework Overview](../../RAFAELIA_FRAMEWORK.md)
- [Implementation Guide](../../RAFAELIA_IMPLEMENTATION_GUIDE.md)
- [State Matrix](../../RAFAELIA_STATE_MATRIX.csv)
- [Audit System](../../RAFAELIA_AUDIT_SYSTEM.md)
- [Telemetry](../../RAFAELIA_TELEMETRY.md)

---

## 🔧 Building / Compilar

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

See the [building guide](../../build.md) for detailed instructions.

---

## 🌍 Other Languages / Outros Idiomas

**Available in 91 languages** - [View all languages](../../LANGUAGES.md)

Popular languages:
- 🇬🇧 [English](../en/README.md)
- 🇧🇷 [Português (Brasil)](../pt-BR/README.md)
- 🇪🇸 [Español](../es/README.md)
- 🇨🇳 [简体中文](../zh-CN/README.md)
- 🇯🇵 [日本語](../ja/README.md)
- 🇩🇪 [Deutsch](../de/README.md)
- 🇫🇷 [Français](../fr/README.md)
- 🇷🇺 [Русский](../ru/README.md)

---

## 🤝 Contributing / Contribuir

Contributions are welcome! See [CONTRIBUTING.md](../../../CONTRIBUTING.md)

---

## 📄 License / Licença

Magisk, including all git submodules are free software:
you can redistribute it and/or modify it under the terms of the
GNU General Public License as published by the Free Software Foundation,
either version 3 of the License, or (at your option) any later version.

See [LICENSE](../../../LICENSE) for details.

---

## 🔗 Useful Links / Links Úteis

- [Main Repository](https://github.com/rafaelmeloreisnovo/Magisk_Rafaelia)
- [Official Magisk](https://github.com/topjohnwu/Magisk)
- [Magisk Documentation](https://topjohnwu.github.io/Magisk/)

---

**Note**: This documentation is automatically generated. For the most accurate and up-to-date information, please refer to the [English documentation](../en/README.md) or the [main README](../../../README.MD).
"""
    
    return content

def create_language_structure(base_dir):
    """Create directory structure and README files for all languages"""
    languages = load_languages()
    languages_dir = base_dir / 'languages'
    
    print(f"Creating language directories in: {languages_dir}")
    
    # Create base languages directory
    languages_dir.mkdir(exist_ok=True)
    
    created_count = 0
    for lang in languages:
        code = lang['code']
        lang_dir = languages_dir / code
        
        # Create language directory
        lang_dir.mkdir(exist_ok=True)
        
        # Create README.md
        readme_path = lang_dir / 'README.md'
        content = create_readme_content(lang)
        
        with open(readme_path, 'w', encoding='utf-8') as f:
            f.write(content)
        
        created_count += 1
        print(f"  ✓ Created: {code} - {lang['native']}")
    
    print(f"\n✅ Successfully created {created_count} language directories and README files")
    return created_count

def main():
    """Main execution"""
    script_dir = Path(__file__).parent
    docs_dir = script_dir
    
    print("=" * 60)
    print("Magisk Rafaelia - Multilingual Documentation Generator")
    print("=" * 60)
    print()
    
    try:
        count = create_language_structure(docs_dir)
        print()
        print("=" * 60)
        print(f"✅ SUCCESS: Generated documentation for {count} languages")
        print("=" * 60)
        print()
        print("Next steps:")
        print("  1. View the language index: docs/LANGUAGES.md")
        print("  2. Browse language-specific docs: docs/languages/<code>/README.md")
        print("  3. Update main README.md to link to multilingual docs")
        
    except Exception as e:
        print(f"❌ ERROR: {e}")
        import traceback
        traceback.print_exc()
        return 1
    
    return 0

if __name__ == '__main__':
    exit(main())
