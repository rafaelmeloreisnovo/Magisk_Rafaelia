# 🚀 Quick Start: Get Magisk_Rafaelia APK

## ⚡ Fastest Way (2 clicks)

1. Go to [Actions](../../actions/workflows/build.yml?query=is%3Asuccess)
2. Click latest successful build → Download artifacts at bottom

✅ **Done!** Extract ZIP and you'll find:
- `app-debug.apk` - For testing (signed with debug key)
- `app-release.apk` - For distribution (signed)
- `app-unsigned.apk` - **UNSIGNED** (sign it yourself!)

---

## 🛠️ Build Locally (3 commands)

```bash
git clone --recurse-submodules https://github.com/rafaelmeloreisnovo/Magisk_Rafaelia.git
cd Magisk_Rafaelia
python3 build.py ndk && python3 build.py -v all
```

Find APK in `out/app-debug.apk`

---

## 📖 Detailed Guides

- 🇧🇷 [Portuguese Guide](COMO_OBTER_APK.md) - Guia completo em português
- 🇬🇧 [English Guide](HOW_TO_GET_APK.md) - Complete guide in English

---

**Quick Links:**
- [📦 Download from Actions](../../actions)
- [📚 Full README](README.MD)
- [🔧 Build Documentation](docs/build.md)
