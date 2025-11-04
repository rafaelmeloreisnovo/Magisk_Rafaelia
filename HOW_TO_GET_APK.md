# How to Get Compiled Magisk_Rafaelia APK

## 📱 Options to Get the APK

There are **3 main ways** to obtain the compiled Magisk_Rafaelia APK:

---

## 🎯 Option 1: Download from GitHub Actions Artifacts (Easiest)

This is the **quickest and easiest** way for users who don't want to compile locally.

### Steps:

1. Go to the repository [Actions](../../actions) page
2. Click on a successful build workflow (with ✅)
3. Scroll down to the **"Artifacts"** section
4. Download the file with the commit hash (example: `abc123def`)
5. Extract the downloaded ZIP file
6. Inside you'll find:
   - `app-release.apk` - Signed release APK
   - `app-debug.apk` - Debug APK
   - Compiled native binaries
   - RAFAELIA manifest

### Visual Example:
```
GitHub → Actions → Build Workflow (✅ Success) → Artifacts → Download
```

---

## 🏗️ Option 2: Build Locally (Complete)

For developers who want to build the complete project.

### Requirements:

- **Operating System**: Linux x64, macOS (Intel/ARM), or Windows x64
- **Python**: 3.8 or higher
- **Git**: With symbolic link support (Windows: enable during installation)
- **Android Studio**: Installed and configured
- **Java**: JDK 17 (can use the one from Android Studio)
- **Disk Space**: ~10 GB free

### Step by Step:

1. **Clone repository with submodules:**
```bash
git clone --recurse-submodules https://github.com/rafaelmeloreisnovo/Magisk_Rafaelia.git
cd Magisk_Rafaelia
```

2. **Set environment variables:**
```bash
# Linux/macOS
export ANDROID_HOME=/path/to/android/sdk
export ANDROID_STUDIO=/path/to/android/studio  # Optional

# Windows (PowerShell)
$env:ANDROID_HOME="C:\Users\YourUser\AppData\Local\Android\Sdk"
$env:ANDROID_STUDIO="C:\Program Files\Android\Android Studio"
```

3. **Install Magisk NDK:**
```bash
python3 build.py ndk
```

4. **Configure build (optional):**
```bash
# Copy config file
cp config.prop.sample config.prop

# Edit config.prop to customize:
# - version: Magisk version
# - outdir: output directory
# - abiList: architectures to build
```

5. **Build everything:**
```bash
# Full build (release)
python3 build.py -r all

# Or debug build (faster)
python3 build.py -v all

# Or just the APK (without native binaries)
python3 build.py app
```

6. **Find compiled files:**
```
out/
├── app-release.apk          # Release APK
├── app-debug.apk            # Debug APK
├── magisk-v1.1.0.zip        # Flashable ZIP
└── ...
```

### Estimated Time:
- **First build**: 15-30 minutes
- **Subsequent builds**: 5-10 minutes

---

## ⚙️ Option 3: Use GitHub Actions to Build (Automatic)

To automate builds in your own fork.

### How It Works:

The repository is already configured with GitHub Actions that **automatically compile** the APK when you:
- Push to the `master` branch
- Open a Pull Request
- Manually trigger the workflow

### Configured Workflow:

The `.github/workflows/build.yml` file already contains:
- ✅ Release and debug builds
- ✅ Artifact uploads
- ✅ RAFAELIA manifest generation
- ✅ Tests on multiple Android versions

### To Trigger Manually:

1. Go to [Actions](../../actions)
2. Click on "Magisk Build"
3. Click "Run workflow"
4. Select branch and click "Run workflow"
5. Wait for build to complete (15-20 minutes)
6. Download artifacts when finished

---

## 📦 What You Get

After building or downloading, you'll have:

### APKs:
- **app-release.apk**: Version for distribution (requires own keystore)
- **app-debug.apk**: Version for development (no signature verification)

### Native Binaries:
- `magisk` - Main Magisk binary
- `magiskboot` - Tool for manipulating boot images
- `magiskinit` - Init replacement
- `magiskpolicy` - SELinux policy manipulation

### Supported Architectures:
- ARM 64-bit (arm64-v8a)
- ARM 32-bit (armeabi-v7a)
- Intel/AMD 64-bit (x86_64)
- Intel/AMD 32-bit (x86)

### Extras:
- RAFAELIA manifest with SHA256 hash and metadata
- Debug symbols (if release build)

---

## 🔐 Signing the APK (Optional for Distribution)

If you want to distribute your own build:

1. **Generate a keystore:**
```bash
keytool -genkey -v -keystore my-release-key.jks \
  -keyalg RSA -keysize 2048 -validity 10000 \
  -alias my-alias
```

2. **Configure in config.prop:**
```properties
keyStore=my-release-key.jks
keyStorePass=your_password
keyAlias=my-alias
keyPass=key_password
```

3. **Build with release:**
```bash
python3 build.py -r all
```

⚠️ **IMPORTANT**: Official Magisk builds verify signatures. Use debug builds for development.

---

## 🐛 Common Issues

### "Could not GET dl.google.com"
- **Cause**: Network restrictions blocking Google Maven access
- **Solution**: Use Option 1 (GitHub Actions) or build in environment with full internet access

### "Unknown ABI: [string]"
- **Cause**: config.prop with placeholder values
- **Solution**: Leave `abiList=` empty or remove config.prop

### "NDK not found"
- **Cause**: Magisk NDK not installed
- **Solution**: Run `python3 build.py ndk`

### "Git submodules not initialized"
- **Cause**: Clone without `--recurse-submodules`
- **Solution**: Run `git submodule update --init --recursive`

### Build fails on Windows
- **Cause**: Symbolic links not enabled
- **Solution**: Enable developer mode on Windows and reinstall Git with symlink support

---

## 📚 Additional Documentation

- [Build.md](docs/build.md) - Complete build documentation
- [README.MD](README.MD) - Project overview
- [BUILD_SUCCESS.md](BUILD_SUCCESS.md) - Last successful build report
- [COMPILATION_SUMMARY.md](COMPILATION_SUMMARY.md) - Previous compilation summary

---

## ✅ APK Verification

After obtaining the APK, you can verify:

```bash
# View APK information
aapt dump badging app-release.apk | grep -E 'package|versionName'

# Verify signature
jarsigner -verify -verbose -certs app-release.apk

# Extract and inspect
unzip -l app-release.apk
```

---

## 🆘 Support

If you still have issues:

1. Check existing [Issues](../../issues)
2. Consult [official Magisk documentation](https://topjohnwu.github.io/Magisk/)
3. Open a new issue with:
   - Operating system
   - Python/Java version
   - Complete error log
   - Steps to reproduce

---

## 📝 Quick Summary

| Method | Difficulty | Time | Requirements | Best For |
|--------|------------|------|--------------|----------|
| **GitHub Actions Download** | ⭐ Easy | 5 min | Browser | End users |
| **Local Build** | ⭐⭐⭐ Medium | 15-30 min | Full dev env | Developers |
| **GitHub Actions Manual** | ⭐⭐ Easy | 20 min | GitHub account | Contributors |

---

**Version**: 1.1.0-rafaelia  
**Last Updated**: 2025-11-04  
**Signature**: RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩ
