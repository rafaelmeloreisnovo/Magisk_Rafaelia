# CI Gradle Wrapper Hotfix Status

## Status

`ci_hotfix_status / operational_safety_note / claim_boundary`

## Repository

`exacordex-crypto/PCR_Rafaelia_Code_seed`

## Purpose

This document records the current status of a conservative CI hotfix for Gradle wrapper invocation.

The goal is operational reliability only:

- make CI use the actual project Gradle wrapper under `app/`;
- avoid runner permission failures;
- avoid touching boot/root/native/Zygisk logic;
- keep the change auditable and reversible.

## Claim boundary

This note does not validate Android runtime behavior.

This note does not validate Magisk rooting behavior.

This note does not validate boot image patching.

This note does not prove device compatibility.

This note does not change scientific, philosophical, legal or commercial claims.

## Applied change

A hotfix was applied to:

```text
.github/workflows/build-unsigned-apk.yml
```

The workflow now prepares the Gradle wrapper before invoking it:

```bash
chmod +x ./app/gradlew
```

This is an operational hardening change only. It does not alter build logic, source code, signing policy, Android source, native code, MagiskBoot, Zygisk, boot scripts or root scripts.

## Commit

```text
248c254b5acff441eeeea03b6d00f8ce90df76b0
ci: harden unsigned APK Gradle wrapper invocation
```

## Remaining recommended patch

The instrumented-test workflow still contains a Gradle invocation that should be aligned with the project layout.

Recommended change in:

```text
.github/workflows/ci.yml
```

Add a Gradle wrapper preparation step after checkout in the `android-instrumented` job:

```yaml
- name: Prepare Gradle wrapper
  run: chmod +x ./app/gradlew
```

Then replace the root-level Gradle wrapper call with the app wrapper:

```diff
- ./gradlew connectedAndroidTest --no-daemon --stacktrace
+ ./app/gradlew -p app connectedAndroidTest --no-daemon --stacktrace
```

## Why this remains pending

The direct automated update of `.github/workflows/ci.yml` was blocked by tool safety filtering while attempting to rewrite the complete workflow file. The pending change is therefore recorded here as an explicit, narrow manual patch rather than being silently claimed as applied.

## Operational safety

Do not combine this CI wrapper patch with changes to:

- `native/`;
- `tools/rafaelia/activate_rafaelia.sh`;
- boot patching scripts;
- uninstall scripts;
- Zygisk logic;
- device-specific root/boot flows.

Keep the CI wrapper correction as a separate commit so any regression can be isolated.

## Validation checklist

After applying the remaining `ci.yml` patch, run or inspect:

```bash
git diff --check
python3 -m py_compile build.py scripts/pre_ci_validate.py
python3 scripts/pre_ci_validate.py --skip-slow
```

Then observe GitHub Actions for:

- unsigned APK workflow startup;
- Android build path resolution;
- instrumented-test Gradle wrapper resolution;
- artifact upload behavior.

## Final note

This is a small operational hardening step. It is valuable because it reduces CI friction without changing the runtime or root-sensitive parts of the repository.
