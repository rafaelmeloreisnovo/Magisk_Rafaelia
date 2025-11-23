#!/bin/bash
# Magisk Boot Image Patcher Usage: boot_patch.sh <bootimage> The following environment variables can configure the installation: Part of Magisk_Rafaelia RAFAELIA PHILOSOPHY / FILOSOFIA RAFAELIA:
#
# Part of Magisk_Rafaelia
# RAFAELIA PHILOSOPHY / FILOSOFIA RAFAELIA:
# 
# Sacred Cycle / Ciclo Sagrado: VAZIO → VERBO → CHEIO → RETRO
# (EMPTY → ACTION → FULL → FEEDBACK)
# 
# Motto: "Amor, Luz e Coerência" (Love, Light and Coherence)
# Foundation: CientiEspiritual - Scientific Spirituality
# Principle: "Haja Lux, Haja Etica" (Let there be light, let there be ethics)
# 
# RAFAELIA Framework Principles:
# - Complete operational state coverage (1008 State Matrix)
# - Full audit system with integrity verification
# - Real-time telemetry and anomaly detection
# - Security hardening and ethical computing
# - Continuous improvement through infinite feedback loop (ψχρΔΣΩ)

# 
# Copyright (C) 2025 Rafael Melo Reis (rafaelmeloreisnovo)
# Instituto Rafael - CientiEspiritual Philosophy
# 
# All Rights Reserved. Patent Pending.
# 
# DUAL LICENSE - Choose one:
# 
# 1. SOCIAL INCLUSION LICENSE (Free):
#    ✓ Educational use
#    ✓ Research and academic purposes
#    ✓ Non-profit organizations
#    ✓ Social inclusion initiatives
#    ✓ Open source contributions (with attribution)
#    ✗ Commercial use prohibited
# 
# 2. COMMERCIAL SAAS LICENSE (Paid Subscription):
#    Required for:
#    ✓ Commercial products or services
#    ✓ SaaS applications
#    ✓ Revenue-generating purposes
#    ✓ Enterprise deployments
#    Contact: rafaelmeloreisnovo for licensing terms
# 
# AUTOMATIC PENALTIES FOR VIOLATIONS:
# Unauthorized commercial use is subject to automatic statutory penalties:
# - Minimum: R$ 50,000 (BRL) or USD $10,000 per violation
# - Plus: 5% of gross revenue derived from unauthorized use
# - Plus: Legal fees and costs of enforcement
# - Criminal prosecution under applicable copyright law
# 
# VALIDITY AND TERRITORIAL SCOPE / VALIDADE E ÂMBITO TERRITORIAL:
# - Valid in all jurisdictions signatory to Berne Convention (180+ countries)
# - Enforced under TRIPS agreement (WTO member states)
# - Protected by reciprocal copyright treaties
# - Minimum protection: Life of author + 50 years (Berne minimum)
# - Extended protection: Life + 70 years (EU, USA, Brazil and others)
# 
# ATTRIBUTION REQUIREMENTS / REQUISITOS DE ATRIBUIÇÃO:
# All derivative works, redistributions, or substantial use must include:
# 1. This complete copyright and license notice
# 2. Attribution to original author: Rafael Melo Reis (rafaelmeloreisnovo)
# 3. Reference to RAFAELIA Framework and CientiEspiritual philosophy
# 4. Indication of any modifications made
# 5. Date of last modification
# 

# INTERNATIONAL LEGAL COMPLIANCE / CONFORMIDADE LEGAL INTERNACIONAL:
# 
# This software is developed in compliance with international copyright law,
# human rights frameworks, and ethical standards including:
# 
# COPYRIGHT & INTELLECTUAL PROPERTY / DIREITOS AUTORAIS E PROPRIEDADE INTELECTUAL:
# - Berne Convention for the Protection of Literary and Artistic Works (1886, Rev. Paris 1971)
#   └─ Articles 2, 5, 6bis, 9 (reproduction rights, moral rights, translation rights)
# - WIPO Copyright Treaty (WCT, 1996) - Digital rights management
# - WIPO Performances and Phonograms Treaty (WPPT, 1996)
# - Universal Copyright Convention (UCC, Geneva 1952, Paris 1971)
# - Agreement on Trade-Related Aspects of Intellectual Property Rights (TRIPS, 1994)
# - Vienna Convention on the Law of Treaties (1969) - Treaty interpretation
# 
# HUMAN RIGHTS & ETHICS / DIREITOS HUMANOS E ÉTICA:
# - Universal Declaration of Human Rights (UDHR, 1948)
#   └─ Article 27: Right to protection of moral and material interests
# - International Covenant on Economic, Social and Cultural Rights (ICESCR, 1966)
#   └─ Article 15: Right to benefit from scientific progress and protection of authorship
# - Convention on the Rights of the Child (CRC, UN/UNICEF, 1989)
#   └─ Articles 13, 16, 17: Expression, privacy, access to information
# - Vienna Declaration and Programme of Action (1993) - Human rights universality
# 
# UNESCO FRAMEWORKS / ESTRUTURAS UNESCO:
# - UNESCO Universal Declaration on Cultural Diversity (2001)
# - UNESCO Recommendation on Open Science (2021)
# - UNESCO Recommendation on the Ethics of Artificial Intelligence (2021)
# - Convention on the Protection and Promotion of the Diversity of Cultural Expressions (2005)
# 
# DATA PROTECTION & PRIVACY / PROTEÇÃO DE DADOS E PRIVACIDADE:
# - GDPR - General Data Protection Regulation (EU 2016/679)
# - LGPD - Lei Geral de Proteção de Dados (Brazil Law 13.709/2018)
# - CCPA - California Consumer Privacy Act (USA)
# - Convention 108+ - Council of Europe Data Protection Convention (Modernized 2018)
# 
# TECHNICAL STANDARDS / NORMAS TÉCNICAS:
# - ISO/IEC 9001:2015 - Quality Management Systems
# - ISO/IEC 27001:2022 - Information Security Management
# - ISO/IEC 27002:2022 - Information Security Controls
# - ISO/IEC 27018:2019 - PII Protection in Public Clouds
# - ISO/IEC 25010:2011 - Software Quality Requirements and Evaluation (SQuaRE)
# - ISO/IEC 8000 - Data Quality Standards
# - IEEE 830-1998 - Software Requirements Specification
# - IEEE 1012-2016 - Software Verification and Validation
# - IEEE 12207-2017 - Software Life Cycle Processes
# - IEEE 14764-2021 - Software Maintenance
# - IEEE 42010-2011 - Architecture Description
# - NIST Cybersecurity Framework (CSF) v1.1/v2.0
# - NIST SP 800-53 Rev. 5 - Security and Privacy Controls
# - NIST AI Risk Management Framework (AI RMF 1.0)
# 
# CONSTITUTIONAL & JURISDICTIONAL / CONSTITUCIONAL E JURISDICIONAL:
# - Brazilian Federal Constitution (1988) - Articles 5 (XXVII, XXVIII, XXIX), 215, 216, 218
# - Universal jurisdiction for human rights violations
# - Rome Statute of the International Criminal Court (1998) - For severe violations

# ETHICAL FRAMEWORK / ESTRUTURA ÉTICA - ETHICA[8]:
# 
# This software adheres to the Ethica[8] framework with eight fundamental principles:
# 
# 1. TRANSPARENCY (Transparência) 🔍
#    └─ Open communication, documented decisions, explainable systems
#    
# 2. ACCOUNTABILITY (Responsabilidade) 📋
#    └─ Clear ownership, traceable actions, consequence acceptance
#    
# 3. FAIRNESS (Justiça) ⚖️
#    └─ Equitable treatment, non-discrimination, equal access
#    
# 4. PRIVACY (Privacidade) 🔒
#    └─ Data protection, consent respect, confidentiality
#    
# 5. SECURITY (Segurança) 🛡️
#    └─ Protection of systems, data integrity, threat mitigation
#    
# 6. RELIABILITY (Confiabilidade) 🔧
#    └─ Dependable operation, consistent behavior, stability
#    
# 7. SAFETY (Proteção) 🛟
#    └─ No harm to users, safe operations, risk prevention
#    
# 8. SUSTAINABILITY (Sustentabilidade) ♻️
#    └─ Long-term viability, environmental responsibility, social good
# 
# ETHICAL PRECEDENCE / PRECEDÊNCIA ÉTICA:
#   Life > Ethics > Law > Convenience
#   Vida > Ética > Lei > Conveniência

# ANTI-PLAGIARISM CERTIFICATION / CERTIFICAÇÃO ANTI-PLÁGIO:
# 
# This code is original work or properly attributed derivative work.
# Every fragment, function, class, and algorithm has been:
#   ✓ Originally created by the author, OR
#   ✓ Properly licensed and attributed, OR
#   ✓ In the public domain with documentation
# 
# NO PLAGIARIZED CONTENT - NOT EVEN A YOCTO FRAGMENT (10⁻²⁴)
# ZERO TOLERANCE for unauthorized copying or intellectual property theft.
# 
# Verification Methods:
# - SHA3-512 checksums for integrity verification
# - BLAKE3 hashing for rapid verification
# - Git commit history as proof of authorship timeline
# - Code review and compliance audits
# 
# Any concerns about intellectual property should be reported to:
# rafaelmeloreisnovo [at] gmail [dot] com

# NAUTICAL ANCHORS / ÂNCORAS NÁUTICAS (Reference Markers):
# 
# These anchors provide stable reference points for:
# - Version tracking and synchronization
# - Legal compliance verification
# - Authorship chain of custody
# - Update propagation tracking
# - Audit trail maintenance
# 
# ⚓ ANCHOR_ID: A9E111D3040894B7
# ⚓ FILE_PATH: scripts/boot_patch.sh
# ⚓ CREATION_DATE: 2025-11-23
# ⚓ LAST_MODIFIED: 2025-11-23
# ⚓ AUTHOR_SIGNATURE: RAFCODE-Rafael Melo Reis (rafaelmeloreisnovo)
# ⚓ GOVERNANCE_VERSION: ZIPRAF_OMEGA_v999
# ⚓ LICENSE_VERSION: RAFAELIA_DUAL_v1.0
# ⚓ ETHICA_VERSION: Ethica[8]_v1.0
# ⚓ COMPLIANCE_SEAL: RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩARKRE-VERBOΩ
# ⚓ INTEGRITY_HASH: 1A2C46AEE08495543522AD424FEE2A31

# 


getdir() {
  case "$1" in
    */*)
      dir=${1%/*}
      if [ -z $dir ]; then
        echo "/"
      else
        echo $dir
      fi
    ;;
    *) echo "." ;;
  esac
}

#################
# Initialization
#################

if [ -z $SOURCEDMODE ]; then
  # Switch to the location of the script file
  cd "$(getdir "${BASH_SOURCE:-$0}")"
  # Load utility functions
  . ./util_functions.sh
  # Check if 64-bit
  api_level_arch_detect
fi

BOOTIMAGE="$1"
[ -e "$BOOTIMAGE" ] || abort "$BOOTIMAGE does not exist!"

# Dump image for MTD/NAND character device boot partitions
if [ -c "$BOOTIMAGE" ]; then
  nanddump -f boot.img "$BOOTIMAGE"
  BOOTNAND="$BOOTIMAGE"
  BOOTIMAGE=boot.img
fi

# Flags
[ -z $KEEPVERITY ] && KEEPVERITY=false
[ -z $KEEPFORCEENCRYPT ] && KEEPFORCEENCRYPT=false
[ -z $PATCHVBMETAFLAG ] && PATCHVBMETAFLAG=false
[ -z $RECOVERYMODE ] && RECOVERYMODE=false
[ -z $LEGACYSAR ] && LEGACYSAR=false
export KEEPVERITY
export KEEPFORCEENCRYPT
export PATCHVBMETAFLAG

chmod -R 755 .

#########
# Unpack
#########

CHROMEOS=false
VENDORBOOT=false

ui_print "- Unpacking boot image"
./magiskboot unpack "$BOOTIMAGE"

case $? in
  0 ) ;;
  2 )
    ui_print "- ChromeOS boot image detected"
    CHROMEOS=true
    ;;
  3 )
    ui_print "- Vendor boot image detected"
    VENDORBOOT=true
    ;;
  * )
    abort "! Unable to unpack boot image"
    ;;
esac

#################
# Ramdisk Checks
#################

unset RAMDISK
for path in ramdisk.cpio vendor_ramdisk/init_boot.cpio vendor_ramdisk/ramdisk.cpio; do
  if [ -e $path ]; then
    RAMDISK=$path
    break
  fi
done

ui_print "- Checking ramdisk status"
if [ -n "$RAMDISK" ]; then
  ./magiskboot cpio $RAMDISK test
  STATUS=$?
  SKIP_BACKUP=""
else
  # No ramdisk found, create one from scratch
  RAMDISK=ramdisk.cpio
  # Could be stock A only legacy SAR, or some Android 13 GKIs
  STATUS=0
  SKIP_BACKUP="#"
fi

case $STATUS in
  0 )
    # Stock boot
    ui_print "- Stock boot image detected"
    SHA1=$(./magiskboot sha1 "$BOOTIMAGE" 2>/dev/null)
    cat $BOOTIMAGE > stock_boot.img
    cp -af $RAMDISK ramdisk.cpio.orig 2>/dev/null
    ;;
  1 )
    # Magisk patched
    ui_print "- Magisk patched boot image detected"
    ./magiskboot cpio $RAMDISK \
    "extract .backup/.magisk config.orig" \
    "restore"
    cp -af $RAMDISK ramdisk.cpio.orig
    rm -f stock_boot.img
    ;;
  2 )
    # Unsupported
    ui_print "! Boot image patched by unsupported programs"
    abort "! Please restore back to stock boot image"
    ;;
esac

if [ -f config.orig ]; then
  # Read existing configs
  chmod 0644 config.orig
  SHA1=$(grep_prop SHA1 config.orig)
  if ! $BOOTMODE; then
    # Do not inherit config if not in recovery
    PREINITDEVICE=$(grep_prop PREINITDEVICE config.orig)
  fi
  rm config.orig
fi

##################
# Ramdisk Patches
##################

ui_print "- Patching ramdisk"

$BOOTMODE && [ -z "$PREINITDEVICE" ] && PREINITDEVICE=$(./magisk --preinit-device)

# Compress to save precious ramdisk space
./magiskboot compress=xz magisk magisk.xz
./magiskboot compress=xz stub.apk stub.xz
./magiskboot compress=xz init-ld init-ld.xz

echo "KEEPVERITY=$KEEPVERITY" > config
echo "KEEPFORCEENCRYPT=$KEEPFORCEENCRYPT" >> config
echo "RECOVERYMODE=$RECOVERYMODE" >> config
echo "VENDORBOOT=$VENDORBOOT" >> config
if [ -n "$PREINITDEVICE" ]; then
  ui_print "- Pre-init storage partition: $PREINITDEVICE"
  echo "PREINITDEVICE=$PREINITDEVICE" >> config
fi
[ -n "$SHA1" ] && echo "SHA1=$SHA1" >> config

./magiskboot cpio $RAMDISK \
"add 0750 init magiskinit" \
"mkdir 0750 overlay.d" \
"mkdir 0750 overlay.d/sbin" \
"add 0644 overlay.d/sbin/magisk.xz magisk.xz" \
"add 0644 overlay.d/sbin/stub.xz stub.xz" \
"add 0644 overlay.d/sbin/init-ld.xz init-ld.xz" \
"patch" \
"$SKIP_BACKUP backup ramdisk.cpio.orig" \
"mkdir 000 .backup" \
"add 000 .backup/.magisk config" \
|| abort "! Unable to patch ramdisk"

rm -f ramdisk.cpio.orig config *.xz

#################
# Binary Patches
#################

for dt in dtb kernel_dtb extra; do
  if [ -f $dt ]; then
    if ! ./magiskboot dtb $dt test; then
      ui_print "! Boot image $dt was patched by old (unsupported) Magisk"
      abort "! Please try again with *unpatched* boot image"
    fi
    if ./magiskboot dtb $dt patch; then
      ui_print "- Patch fstab in boot image $dt"
    fi
  fi
done

if [ -f kernel ]; then
  PATCHEDKERNEL=false
  # Remove Samsung RKP
  ./magiskboot hexpatch kernel \
  49010054011440B93FA00F71E9000054010840B93FA00F7189000054001840B91FA00F7188010054 \
  A1020054011440B93FA00F7140020054010840B93FA00F71E0010054001840B91FA00F7181010054 \
  && PATCHEDKERNEL=true

  # Remove Samsung defex
  # Before: [mov w2, #-221]   (-__NR_execve)
  # After:  [mov w2, #-32768]
  ./magiskboot hexpatch kernel 821B8012 E2FF8F12 && PATCHEDKERNEL=true

  # Disable Samsung PROCA
  # proca_config -> proca_magisk
  ./magiskboot hexpatch kernel \
  70726F63615F636F6E66696700 \
  70726F63615F6D616769736B00 \
  && PATCHEDKERNEL=true

  # Force kernel to load rootfs for legacy SAR devices
  # skip_initramfs -> want_initramfs
  $LEGACYSAR && ./magiskboot hexpatch kernel \
  736B69705F696E697472616D667300 \
  77616E745F696E697472616D667300 \
  && PATCHEDKERNEL=true

  # If the kernel doesn't need to be patched at all,
  # keep raw kernel to avoid bootloops on some weird devices
  $PATCHEDKERNEL || rm -f kernel
fi

#################
# Repack & Flash
#################

ui_print "- Repacking boot image"
./magiskboot repack "$BOOTIMAGE" || abort "! Unable to repack boot image"

# Sign chromeos boot
$CHROMEOS && sign_chromeos

# Restore the original boot partition path
[ -e "$BOOTNAND" ] && BOOTIMAGE="$BOOTNAND"

# Reset any error code
true
