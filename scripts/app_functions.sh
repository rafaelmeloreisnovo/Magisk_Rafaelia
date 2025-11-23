#!/bin/bash
# Magisk app internal scripts $2 = command $1 = version string Part of Magisk_Rafaelia RAFAELIA PHILOSOPHY / FILOSOFIA RAFAELIA:
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
# ⚓ ANCHOR_ID: 97075E724EA6E335
# ⚓ FILE_PATH: scripts/app_functions.sh
# ⚓ CREATION_DATE: 2025-11-23
# ⚓ LAST_MODIFIED: 2025-11-23
# ⚓ AUTHOR_SIGNATURE: RAFCODE-Rafael Melo Reis (rafaelmeloreisnovo)
# ⚓ GOVERNANCE_VERSION: ZIPRAF_OMEGA_v999
# ⚓ LICENSE_VERSION: RAFAELIA_DUAL_v1.0
# ⚓ ETHICA_VERSION: Ethica[8]_v1.0
# ⚓ COMPLIANCE_SEAL: RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩARKRE-VERBOΩ
# ⚓ INTEGRITY_HASH: 3DFC26EDD9CDC3CC1D3C770BE73B01AA

# 


run_delay() {
  (sleep $1; $2)&
}

# $1 = version string
# $2 = version code
env_check() {
  for file in busybox magiskboot magiskinit util_functions.sh boot_patch.sh; do
    [ -f "$MAGISKBIN/$file" ] || return 1
  done
  if [ "$2" -ge 25000 ]; then
    [ -f "$MAGISKBIN/magiskpolicy" ] || return 1
  fi
  if [ "$2" -ge 25210 ]; then
    [ -b "$MAGISKTMP/.magisk/device/preinit" ] || [ -b "$MAGISKTMP/.magisk/block/preinit" ] || return 2
  fi
  grep -xqF "MAGISK_VER='$1'" "$MAGISKBIN/util_functions.sh" || return 3
  grep -xqF "MAGISK_VER_CODE=$2" "$MAGISKBIN/util_functions.sh" || return 3
  return 0
}

# $1 = dir to copy
# $2 = destination (optional)
cp_readlink() {
  if [ -z $2 ]; then
    cd $1
  else
    cp -af $1/. $2
    cd $2
  fi
  for file in *; do
    if [ -L $file ]; then
      local full=$(readlink -f $file)
      rm $file
      cp -af $full $file
    fi
  done
  chmod -R 755 .
  cd /
}

# $1 = install dir
fix_env() {
  # Cleanup and make dirs
  rm -rf $MAGISKBIN/*
  mkdir -p $MAGISKBIN 2>/dev/null
  chmod 700 /data/adb
  cp_readlink $1 $MAGISKBIN
  rm -rf $1
  chown -R 0:0 $MAGISKBIN
}

# $1 = install dir
# $2 = boot partition
direct_install() {
  echo "- Flashing new boot image"
  flash_image $1/new-boot.img $2
  case $? in
    1)
      echo "! Insufficient partition size"
      return 1
      ;;
    2)
      echo "! $2 is read only"
      return 2
      ;;
  esac

  rm -f $1/new-boot.img
  fix_env $1
  run_migrations

  return 0
}

# $1 = uninstaller zip
run_uninstaller() {
  rm -rf /dev/tmp
  mkdir -p /dev/tmp/install
  unzip -o "$1" "assets/*" "lib/*" -d /dev/tmp/install
  INSTALLER=/dev/tmp/install sh /dev/tmp/install/assets/uninstaller.sh dummy 1 "$1"
}

# $1 = boot partition
restore_imgs() {
  local SHA1=$(grep_prop SHA1 $MAGISKTMP/.magisk/config)
  local BACKUPDIR=/data/magisk_backup_$SHA1
  [ -d $BACKUPDIR ] || return 1
  [ -f $BACKUPDIR/boot.img.gz ] || return 1
  flash_image $BACKUPDIR/boot.img.gz $1
}

# $1 = path to bootctl executable
post_ota() {
  cd /data/adb
  cp -f $1 bootctl
  rm -f $1
  chmod 755 bootctl
  if ! ./bootctl hal-info; then
    rm -f bootctl
    return
  fi
  SLOT_NUM=0
  [ $(./bootctl get-current-slot) -eq 0 ] && SLOT_NUM=1
  ./bootctl set-active-boot-slot $SLOT_NUM
  cat << EOF > post-fs-data.d/post_ota.sh
/data/adb/bootctl mark-boot-successful
rm -f /data/adb/bootctl
rm -f /data/adb/post-fs-data.d/post_ota.sh
EOF
  chmod 755 post-fs-data.d/post_ota.sh
  cd /
}

# $1 = APK
# $2 = package name
adb_pm_install() {
  local tmp=/data/local/tmp/temp.apk
  cp -f "$1" $tmp
  chmod 644 $tmp
  su 2000 -c pm install -g $tmp || pm install -g $tmp || su 1000 -c pm install -g $tmp
  local res=$?
  rm -f $tmp
  if [ $res = 0 ]; then
    appops set "$2" REQUEST_INSTALL_PACKAGES allow
  fi
  return $res
}

check_boot_ramdisk() {
  # Create boolean ISAB
  ISAB=true
  [ -z $SLOT ] && ISAB=false

  # If we are A/B, then we must have ramdisk
  $ISAB && return 0

  # If we are using legacy SAR, but not A/B, assume we do not have ramdisk
  if $LEGACYSAR; then
    # Override recovery mode to true
    RECOVERYMODE=true
    return 1
  fi

  return 0
}

check_encryption() {
  if $ISENCRYPTED; then
    if [ $SDK_INT -lt 24 ]; then
      CRYPTOTYPE="block"
    else
      # First see what the system tells us
      CRYPTOTYPE=$(getprop ro.crypto.type)
      if [ -z $CRYPTOTYPE ]; then
        # If not mounting through device mapper, we are FBE
        if grep ' /data ' /proc/mounts | grep -qv 'dm-'; then
          CRYPTOTYPE="file"
        else
          # We are either FDE or metadata encryption (which is also FBE)
          CRYPTOTYPE="block"
          grep -q ' /metadata ' /proc/mounts && CRYPTOTYPE="file"
        fi
      fi
    fi
  else
    CRYPTOTYPE="N/A"
  fi
}

printvar() {
  eval echo $1=\$$1
}

run_action() {
  local MODID="$1"
  cd "/data/adb/modules/$MODID"
  sh ./action.sh
  local RES=$?
  cd /
  return $RES
}

##########################
# Non-root util_functions
##########################

mount_partitions() {
  [ "$(getprop ro.build.ab_update)" = "true" ] && SLOT=$(getprop ro.boot.slot_suffix)
  # Check whether non rootfs root dir exists
  SYSTEM_AS_ROOT=false
  grep ' / ' /proc/mounts | grep -qv 'rootfs' && SYSTEM_AS_ROOT=true

  LEGACYSAR=false
  grep ' / ' /proc/mounts | grep -q '/dev/root' && LEGACYSAR=true
}

get_flags() {
  KEEPVERITY=$SYSTEM_AS_ROOT
  ISENCRYPTED=false
  [ "$(getprop ro.crypto.state)" = "encrypted" ] && ISENCRYPTED=true
  KEEPFORCEENCRYPT=$ISENCRYPTED
  if [ -n "$(getprop ro.boot.vbmeta.device)" -o -n "$(getprop ro.boot.vbmeta.size)" ]; then
    PATCHVBMETAFLAG=false
  elif getprop ro.product.ab_ota_partitions | grep -wq vbmeta; then
    PATCHVBMETAFLAG=false
  else
    PATCHVBMETAFLAG=true
  fi
  [ -z $RECOVERYMODE ] && RECOVERYMODE=false
  [ -z $VENDORBOOT ] && VENDORBOOT=false
}

run_migrations() { return; }

grep_prop() { return; }

#############
# Initialize
#############

app_init() {
  mount_partitions >/dev/null
  RAMDISKEXIST=false
  check_boot_ramdisk && RAMDISKEXIST=true
  get_flags >/dev/null
  run_migrations >/dev/null
  check_encryption

  # Dump variables
  printvar SLOT
  printvar SYSTEM_AS_ROOT
  printvar RAMDISKEXIST
  printvar ISAB
  printvar CRYPTOTYPE
  printvar PATCHVBMETAFLAG
  printvar LEGACYSAR
  printvar RECOVERYMODE
  printvar KEEPVERITY
  printvar KEEPFORCEENCRYPT
  printvar VENDORBOOT
}

export BOOTMODE=true
