# Release, CI, ABI and failsafe contract

This repository treats APK delivery as a single chain: native binaries, Gradle packaging, signing, artifact upload, and runtime validation must agree on the same contracts.

## ABI source of truth

The supported Android release ABI floor is:

- `armeabi-v7a` for ARM32 compatibility.
- `arm64-v8a` for ARM64 devices.

The CI validation config in `.github/ci.prop` must keep both ABIs enabled. Local developers can still override `abiList` in a private `config.prop`, but release CI must not silently drop ARM32 or ARM64.

## APK outputs

The primary CI workflow builds and uploads:

- `out/app-release.apk`: release APK signed through the configured release signing path, or the debug fallback only when no release keystore is configured.
- `out/app-debug.apk`: debug APK for development validation.
- `out/app-unsigned.apk`: unsigned release-derived APK for explicit self-signing or internal validation.

The unsigned path is separate from the official release path. It must not replace the signed release APK.

## CI checks and artifacts

The build workflow validates that each APK exists, is a readable ZIP/APK, and contains native libraries for both `armeabi-v7a` and `arm64-v8a`. It uploads the full `out/` directory plus an individual APK bundle artifact, both with `if-no-files-found: error` to prevent silent green builds with missing outputs.

## Rollback, failover and watchdog posture

- Build rollback: every artifact is isolated under `out/` for the current commit and uploaded with commit-scoped names.
- Fail-safe packaging: unsigned packaging may emit an APK without applying signing material, but signed release packaging remains independent and mandatory.
- Failover diagnostics: AVD and Cuttlefish jobs upload logs on failure for audit and recovery.
- Watchdog validation: runtime watchdog behavior is tested through the existing emulator/device validation jobs; CI must expose failures rather than masking them.

## Local verification commands

```bash
python3 -m py_compile build.py scripts/pre_ci_validate.py
./app/gradlew -p app :buildSrc:compileKotlin
./build.py -vr all
./build.py -v all
./app/gradlew -p app :apk:assembleUnsigned -PconfigPath=$(pwd)/config.prop
```
