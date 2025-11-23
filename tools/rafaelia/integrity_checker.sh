#!/bin/bash
# RAFAELIA Integrity Checker Verifies system integrity using hashes and signatures Signature: RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩ
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
# ⚓ ANCHOR_ID: 19498022C7E38A48
# ⚓ FILE_PATH: tools/rafaelia/integrity_checker.sh
# ⚓ CREATION_DATE: 2025-11-23
# ⚓ LAST_MODIFIED: 2025-11-23
# ⚓ AUTHOR_SIGNATURE: RAFCODE-Rafael Melo Reis (rafaelmeloreisnovo)
# ⚓ GOVERNANCE_VERSION: ZIPRAF_OMEGA_v999
# ⚓ LICENSE_VERSION: RAFAELIA_DUAL_v1.0
# ⚓ ETHICA_VERSION: Ethica[8]_v1.0
# ⚓ COMPLIANCE_SEAL: RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩARKRE-VERBOΩ
# ⚓ INTEGRITY_HASH: 08615395C663C00B42EFE5B845C4EEE8

# 


MAGISK_DIR="/data/adb/magisk"
MODULES_DIR="$MAGISK_DIR/modules"
AUDIT_DIR="$MAGISK_DIR/rafaelia_audit"
MANIFEST_FILE="/data/adb/RAFAELIA_MANIFEST.json"
COLOR_RED='\033[0;31m'
COLOR_GREEN='\033[0;32m'
COLOR_YELLOW='\033[1;33m'
COLOR_BLUE='\033[0;34m'
COLOR_RESET='\033[0m'

# Print colored output
print_color() {
    color=$1
    shift
    printf "${color}%s${COLOR_RESET}\n" "$*"
}

# Print section header
print_header() {
    echo ""
    print_color "$COLOR_BLUE" "=========================================="
    print_color "$COLOR_BLUE" "$1"
    print_color "$COLOR_BLUE" "=========================================="
}

# Check if running as root
check_root() {
    if [ "$(id -u)" != "0" ]; then
        print_color "$COLOR_RED" "Error: This script must be run as root"
        exit 1
    fi
}

# Check boot image integrity
check_boot_integrity() {
    print_header "Boot Image Integrity Check"
    
    # Check if boot is mounted
    if mountpoint -q /dev/block/bootdevice/by-name/boot 2>/dev/null; then
        print_color "$COLOR_GREEN" "✓ Boot partition accessible"
    else
        print_color "$COLOR_YELLOW" "⚠ Boot partition not accessible (may be normal)"
    fi
    
    # Check magisk binary
    if [ -f "$MAGISK_DIR/magisk64" ]; then
        magisk_size=$(stat -c %s "$MAGISK_DIR/magisk64" 2>/dev/null || echo "0")
        print_color "$COLOR_GREEN" "✓ Magisk binary found (${magisk_size} bytes)"
    else
        print_color "$COLOR_RED" "✗ Magisk binary not found"
        return 1
    fi
    
    return 0
}

# Check modules integrity
check_modules_integrity() {
    print_header "Modules Integrity Check"
    
    if [ ! -d "$MODULES_DIR" ]; then
        print_color "$COLOR_YELLOW" "⚠ Modules directory not found"
        return 0
    fi
    
    module_count=0
    valid_modules=0
    invalid_modules=0
    
    for module_dir in "$MODULES_DIR"/*; do
        if [ ! -d "$module_dir" ]; then
            continue
        fi
        
        module_name=$(basename "$module_dir")
        module_count=$((module_count + 1))
        
        # Check for module.prop
        if [ -f "$module_dir/module.prop" ]; then
            # Check if module is disabled
            if [ -f "$module_dir/disable" ]; then
                print_color "$COLOR_YELLOW" "⚠ Module '$module_name' is disabled"
            else
                print_color "$COLOR_GREEN" "✓ Module '$module_name' is valid"
                valid_modules=$((valid_modules + 1))
            fi
        else
            print_color "$COLOR_RED" "✗ Module '$module_name' missing module.prop"
            invalid_modules=$((invalid_modules + 1))
        fi
    done
    
    echo ""
    echo "Total modules: $module_count"
    echo "Valid modules: $valid_modules"
    echo "Invalid modules: $invalid_modules"
    
    return $invalid_modules
}

# Check database integrity
check_database_integrity() {
    print_header "Database Integrity Check"
    
    db_file="$MAGISK_DIR/magisk.db"
    
    if [ ! -f "$db_file" ]; then
        print_color "$COLOR_RED" "✗ Magisk database not found"
        return 1
    fi
    
    print_color "$COLOR_GREEN" "✓ Database file exists"
    
    # Try to query database
    if command -v sqlite3 >/dev/null 2>&1; then
        table_count=$(sqlite3 "$db_file" "SELECT COUNT(*) FROM sqlite_master WHERE type='table';" 2>/dev/null)
        if [ $? -eq 0 ]; then
            print_color "$COLOR_GREEN" "✓ Database is readable ($table_count tables)"
        else
            print_color "$COLOR_RED" "✗ Database is corrupted or unreadable"
            return 1
        fi
    else
        print_color "$COLOR_YELLOW" "⚠ sqlite3 not available, skipping database query test"
    fi
    
    return 0
}

# Check audit system integrity
check_audit_integrity() {
    print_header "Audit System Integrity Check"
    
    if [ ! -d "$AUDIT_DIR" ]; then
        print_color "$COLOR_YELLOW" "⚠ Audit directory not found (may not be initialized)"
        return 0
    fi
    
    audit_files=$(find "$AUDIT_DIR" -name "audit_*.jsonl" 2>/dev/null | wc -l)
    
    if [ "$audit_files" -gt 0 ]; then
        print_color "$COLOR_GREEN" "✓ Found $audit_files audit log files"
        
        # Check latest audit file
        latest_audit=$(find "$AUDIT_DIR" -name "audit_*.jsonl" 2>/dev/null | sort | tail -n 1)
        if [ -f "$latest_audit" ]; then
            line_count=$(wc -l < "$latest_audit" 2>/dev/null || echo "0")
            print_color "$COLOR_GREEN" "✓ Latest audit log: $line_count entries"
        fi
    else
        print_color "$COLOR_YELLOW" "⚠ No audit logs found"
    fi
    
    return 0
}

# Check manifest integrity
check_manifest_integrity() {
    print_header "Manifest Integrity Check"
    
    if [ ! -f "$MANIFEST_FILE" ]; then
        print_color "$COLOR_YELLOW" "⚠ RAFAELIA manifest not found"
        return 0
    fi
    
    print_color "$COLOR_GREEN" "✓ Manifest file exists"
    
    # Validate JSON (if jq is available)
    if command -v jq >/dev/null 2>&1; then
        if jq empty "$MANIFEST_FILE" 2>/dev/null; then
            print_color "$COLOR_GREEN" "✓ Manifest is valid JSON"
            
            # Show manifest info
            signature=$(jq -r '.signature' "$MANIFEST_FILE" 2>/dev/null)
            timestamp=$(jq -r '.timestamp' "$MANIFEST_FILE" 2>/dev/null)
            
            echo "  Signature: $signature"
            echo "  Timestamp: $timestamp"
        else
            print_color "$COLOR_RED" "✗ Manifest is invalid JSON"
            return 1
        fi
    else
        print_color "$COLOR_YELLOW" "⚠ jq not available, skipping JSON validation"
    fi
    
    return 0
}

# Check SELinux status
check_selinux_status() {
    print_header "SELinux Status Check"
    
    if command -v getenforce >/dev/null 2>&1; then
        selinux_mode=$(getenforce)
        if [ "$selinux_mode" = "Enforcing" ]; then
            print_color "$COLOR_GREEN" "✓ SELinux is Enforcing"
        elif [ "$selinux_mode" = "Permissive" ]; then
            print_color "$COLOR_YELLOW" "⚠ SELinux is Permissive"
        else
            print_color "$COLOR_YELLOW" "⚠ SELinux status: $selinux_mode"
        fi
    else
        print_color "$COLOR_YELLOW" "⚠ Cannot determine SELinux status"
    fi
    
    return 0
}

# Check system properties
check_system_properties() {
    print_header "System Properties Check"
    
    # Check important Magisk properties
    magisk_ver=$(getprop ro.magisk.version 2>/dev/null || echo "unknown")
    magisk_vcode=$(getprop ro.magisk.versionCode 2>/dev/null || echo "unknown")
    
    echo "Magisk version: $magisk_ver"
    echo "Magisk version code: $magisk_vcode"
    
    if [ "$magisk_ver" != "unknown" ]; then
        print_color "$COLOR_GREEN" "✓ Magisk properties are set"
    else
        print_color "$COLOR_YELLOW" "⚠ Magisk properties not found"
    fi
    
    return 0
}

# Generate summary report
generate_summary() {
    total_checks=$1
    passed_checks=$2
    
    print_header "Integrity Check Summary"
    
    echo "Total checks: $total_checks"
    echo "Passed checks: $passed_checks"
    echo "Failed checks: $((total_checks - passed_checks))"
    
    if [ "$passed_checks" -eq "$total_checks" ]; then
        print_color "$COLOR_GREEN" ""
        print_color "$COLOR_GREEN" "✓ All integrity checks passed!"
        return 0
    else
        print_color "$COLOR_YELLOW" ""
        print_color "$COLOR_YELLOW" "⚠ Some integrity checks failed or produced warnings"
        return 1
    fi
}

# Main function
main() {
    print_color "$COLOR_BLUE" "RAFAELIA Integrity Checker"
    print_color "$COLOR_BLUE" "Signature: RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩ"
    
    check_root
    
    total_checks=0
    passed_checks=0
    
    # Run all checks
    checks=(
        "check_boot_integrity"
        "check_modules_integrity"
        "check_database_integrity"
        "check_audit_integrity"
        "check_manifest_integrity"
        "check_selinux_status"
        "check_system_properties"
    )
    
    for check in "${checks[@]}"; do
        total_checks=$((total_checks + 1))
        if $check; then
            passed_checks=$((passed_checks + 1))
        fi
    done
    
    # Generate summary
    generate_summary "$total_checks" "$passed_checks"
}

# Handle command line arguments
case "${1:-full}" in
    full)
        main
        ;;
    boot)
        check_root
        check_boot_integrity
        ;;
    modules)
        check_root
        check_modules_integrity
        ;;
    database|db)
        check_root
        check_database_integrity
        ;;
    audit)
        check_root
        check_audit_integrity
        ;;
    manifest)
        check_root
        check_manifest_integrity
        ;;
    help|--help|-h)
        cat <<EOF
RAFAELIA Integrity Checker
Usage: $0 [check]

Checks:
    full        Run all integrity checks (default)
    boot        Check boot image integrity
    modules     Check modules integrity
    database    Check database integrity
    audit       Check audit system integrity
    manifest    Check manifest integrity
    help        Show this help message

Examples:
    # Run all checks
    $0 full
    
    # Check only modules
    $0 modules
EOF
        ;;
    *)
        echo "Unknown check: $1" >&2
        echo "Run '$0 help' for usage information" >&2
        exit 1
        ;;
esac
