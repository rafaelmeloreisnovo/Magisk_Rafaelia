#!/usr/bin/env bash
set -euo pipefail

APK="${1:-}"
OUT="${2:-}"
mkdir -p "$(dirname "${OUT}")"

if [[ -z "${APK}" || -z "${OUT}" || ! -f "${APK}" ]]; then
  echo "Usage: $0 <apk_path> <out_manifest.json>" >&2
  exit 1
fi

timestamp="$(date -u +'%Y-%m-%dT%H:%M:%SZ')"
commit="${GITHUB_SHA:-$(git rev-parse --short=8 HEAD 2>/dev/null || echo '')}"
branch="${GITHUB_REF_NAME:-$(git rev-parse --abbrev-ref HEAD 2>/dev/null || echo '')}"

# sha256
if command -v sha256sum >/dev/null 2>&1; then
  sha256="$(sha256sum "${APK}" | awk '{print $1}')"
elif command -v shasum >/dev/null 2>&1; then
  sha256="$(shasum -a 256 "${APK}" | awk '{print $1}')"
else
  sha256="$(python3 -c 'import sys,hashlib;print(hashlib.sha256(open(sys.argv[1],"rb").read()).hexdigest())' "${APK}")"
fi

# sha3-256 via Python
sha3="$(python3 - <<'PY' "${APK}"
import sys,hashlib
with open(sys.argv[1],'rb') as f:
    print(hashlib.sha3_256(f.read()).hexdigest())
PY
)"

# blake3 (best effort)
blake3=""
if python3 -c 'import blake3' >/dev/null 2>&1; then
  blake3="$(python3 - <<'PY' "${APK}"
import sys, blake3
with open(sys.argv[1],'rb') as f:
    print(blake3.blake3(f.read()).hexdigest())
PY
)"
fi

# file size (linux/macos)
size_bytes="$(stat -c '%s' "${APK}" 2>/dev/null || stat -f '%z' "${APK}")"

cat > "${OUT}" <<JSON
{
  "signature": "RAFCODE-\\u03a6-\\u2206RafaelVerbo\\u03a9",
  "timestamp": "${timestamp}",
  "artifact": "$(basename "${APK}")",
  "size_bytes": ${size_bytes},
  "hashes": {
    "sha256": "${sha256}",
    "sha3_256": "${sha3}",
    "blake3": "${blake3}"
  },
  "build": {
    "commit": "${commit}",
    "branch": "${branch}",
    "toolchain": "Gradle+ONDK r29.2",
    "abi": "",
    "purpose": "APK integrity manifest"
  },
  "notes": "Generated under ψχρΔΣΩ_LOOP with Φ_ethica"
}
JSON

echo "Manifest generated at ${OUT}" at $OUT"
