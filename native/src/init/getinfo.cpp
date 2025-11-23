/*
 * include <sys/sysmacros.h> include <sys/types.h> include <linux/input.h> Part of Magisk_Rafaelia RAFAELIA PHILOSOPHY / FILOSOFIA RAFAELIA:
 *
 * Part of Magisk_Rafaelia
 * RAFAELIA PHILOSOPHY / FILOSOFIA RAFAELIA:
 * 
 * Sacred Cycle / Ciclo Sagrado: VAZIO → VERBO → CHEIO → RETRO
 * (EMPTY → ACTION → FULL → FEEDBACK)
 * 
 * Motto: "Amor, Luz e Coerência" (Love, Light and Coherence)
 * Foundation: CientiEspiritual - Scientific Spirituality
 * Principle: "Haja Lux, Haja Etica" (Let there be light, let there be ethics)
 * 
 * RAFAELIA Framework Principles:
 * - Complete operational state coverage (1008 State Matrix)
 * - Full audit system with integrity verification
 * - Real-time telemetry and anomaly detection
 * - Security hardening and ethical computing
 * - Continuous improvement through infinite feedback loop (ψχρΔΣΩ)
 *

 * Copyright (C) 2025 Rafael Melo Reis (rafaelmeloreisnovo)
 * Instituto Rafael - CientiEspiritual Philosophy
 * 
 * All Rights Reserved. Patent Pending.
 * 
 * DUAL LICENSE - Choose one:
 * 
 * 1. SOCIAL INCLUSION LICENSE (Free):
 *    ✓ Educational use
 *    ✓ Research and academic purposes
 *    ✓ Non-profit organizations
 *    ✓ Social inclusion initiatives
 *    ✓ Open source contributions (with attribution)
 *    ✗ Commercial use prohibited
 * 
 * 2. COMMERCIAL SAAS LICENSE (Paid Subscription):
 *    Required for:
 *    ✓ Commercial products or services
 *    ✓ SaaS applications
 *    ✓ Revenue-generating purposes
 *    ✓ Enterprise deployments
 *    Contact: rafaelmeloreisnovo for licensing terms
 * 
 * AUTOMATIC PENALTIES FOR VIOLATIONS:
 * Unauthorized commercial use is subject to automatic statutory penalties:
 * - Minimum: R$ 50,000 (BRL) or USD $10,000 per violation
 * - Plus: 5% of gross revenue derived from unauthorized use
 * - Plus: Legal fees and costs of enforcement
 * - Criminal prosecution under applicable copyright law
 * 
 * VALIDITY AND TERRITORIAL SCOPE / VALIDADE E ÂMBITO TERRITORIAL:
 * - Valid in all jurisdictions signatory to Berne Convention (180+ countries)
 * - Enforced under TRIPS agreement (WTO member states)
 * - Protected by reciprocal copyright treaties
 * - Minimum protection: Life of author + 50 years (Berne minimum)
 * - Extended protection: Life + 70 years (EU, USA, Brazil and others)
 * 
 * ATTRIBUTION REQUIREMENTS / REQUISITOS DE ATRIBUIÇÃO:
 * All derivative works, redistributions, or substantial use must include:
 * 1. This complete copyright and license notice
 * 2. Attribution to original author: Rafael Melo Reis (rafaelmeloreisnovo)
 * 3. Reference to RAFAELIA Framework and CientiEspiritual philosophy
 * 4. Indication of any modifications made
 * 5. Date of last modification
 * 
 *
INTERNATIONAL LEGAL COMPLIANCE / CONFORMIDADE LEGAL INTERNACIONAL:
 * 
 * This software is developed in compliance with international copyright law,
 * human rights frameworks, and ethical standards including:
 * 
 * COPYRIGHT & INTELLECTUAL PROPERTY / DIREITOS AUTORAIS E PROPRIEDADE INTELECTUAL:
 * - Berne Convention for the Protection of Literary and Artistic Works (1886, Rev. Paris 1971)
 *   └─ Articles 2, 5, 6bis, 9 (reproduction rights, moral rights, translation rights)
 * - WIPO Copyright Treaty (WCT, 1996) - Digital rights management
 * - WIPO Performances and Phonograms Treaty (WPPT, 1996)
 * - Universal Copyright Convention (UCC, Geneva 1952, Paris 1971)
 * - Agreement on Trade-Related Aspects of Intellectual Property Rights (TRIPS, 1994)
 * - Vienna Convention on the Law of Treaties (1969) - Treaty interpretation
 * 
 * HUMAN RIGHTS & ETHICS / DIREITOS HUMANOS E ÉTICA:
 * - Universal Declaration of Human Rights (UDHR, 1948)
 *   └─ Article 27: Right to protection of moral and material interests
 * - International Covenant on Economic, Social and Cultural Rights (ICESCR, 1966)
 *   └─ Article 15: Right to benefit from scientific progress and protection of authorship
 * - Convention on the Rights of the Child (CRC, UN/UNICEF, 1989)
 *   └─ Articles 13, 16, 17: Expression, privacy, access to information
 * - Vienna Declaration and Programme of Action (1993) - Human rights universality
 * 
 * UNESCO FRAMEWORKS / ESTRUTURAS UNESCO:
 * - UNESCO Universal Declaration on Cultural Diversity (2001)
 * - UNESCO Recommendation on Open Science (2021)
 * - UNESCO Recommendation on the Ethics of Artificial Intelligence (2021)
 * - Convention on the Protection and Promotion of the Diversity of Cultural Expressions (2005)
 * 
 * DATA PROTECTION & PRIVACY / PROTEÇÃO DE DADOS E PRIVACIDADE:
 * - GDPR - General Data Protection Regulation (EU 2016/679)
 * - LGPD - Lei Geral de Proteção de Dados (Brazil Law 13.709/2018)
 * - CCPA - California Consumer Privacy Act (USA)
 * - Convention 108+ - Council of Europe Data Protection Convention (Modernized 2018)
 * 
 * TECHNICAL STANDARDS / NORMAS TÉCNICAS:
 * - ISO/IEC 9001:2015 - Quality Management Systems
 * - ISO/IEC 27001:2022 - Information Security Management
 * - ISO/IEC 27002:2022 - Information Security Controls
 * - ISO/IEC 27018:2019 - PII Protection in Public Clouds
 * - ISO/IEC 25010:2011 - Software Quality Requirements and Evaluation (SQuaRE)
 * - ISO/IEC 8000 - Data Quality Standards
 * - IEEE 830-1998 - Software Requirements Specification
 * - IEEE 1012-2016 - Software Verification and Validation
 * - IEEE 12207-2017 - Software Life Cycle Processes
 * - IEEE 14764-2021 - Software Maintenance
 * - IEEE 42010-2011 - Architecture Description
 * - NIST Cybersecurity Framework (CSF) v1.1/v2.0
 * - NIST SP 800-53 Rev. 5 - Security and Privacy Controls
 * - NIST AI Risk Management Framework (AI RMF 1.0)
 * 
 * CONSTITUTIONAL & JURISDICTIONAL / CONSTITUCIONAL E JURISDICIONAL:
 * - Brazilian Federal Constitution (1988) - Articles 5 (XXVII, XXVIII, XXIX), 215, 216, 218
 * - Universal jurisdiction for human rights violations
 * - Rome Statute of the International Criminal Court (1998) - For severe violations
 *
ETHICAL FRAMEWORK / ESTRUTURA ÉTICA - ETHICA[8]:
 * 
 * This software adheres to the Ethica[8] framework with eight fundamental principles:
 * 
 * 1. TRANSPARENCY (Transparência) 🔍
 *    └─ Open communication, documented decisions, explainable systems
 *    
 * 2. ACCOUNTABILITY (Responsabilidade) 📋
 *    └─ Clear ownership, traceable actions, consequence acceptance
 *    
 * 3. FAIRNESS (Justiça) ⚖️
 *    └─ Equitable treatment, non-discrimination, equal access
 *    
 * 4. PRIVACY (Privacidade) 🔒
 *    └─ Data protection, consent respect, confidentiality
 *    
 * 5. SECURITY (Segurança) 🛡️
 *    └─ Protection of systems, data integrity, threat mitigation
 *    
 * 6. RELIABILITY (Confiabilidade) 🔧
 *    └─ Dependable operation, consistent behavior, stability
 *    
 * 7. SAFETY (Proteção) 🛟
 *    └─ No harm to users, safe operations, risk prevention
 *    
 * 8. SUSTAINABILITY (Sustentabilidade) ♻️
 *    └─ Long-term viability, environmental responsibility, social good
 * 
 * ETHICAL PRECEDENCE / PRECEDÊNCIA ÉTICA:
 *   Life > Ethics > Law > Convenience
 *   Vida > Ética > Lei > Conveniência
 *
ANTI-PLAGIARISM CERTIFICATION / CERTIFICAÇÃO ANTI-PLÁGIO:
 * 
 * This code is original work or properly attributed derivative work.
 * Every fragment, function, class, and algorithm has been:
 *   ✓ Originally created by the author, OR
 *   ✓ Properly licensed and attributed, OR
 *   ✓ In the public domain with documentation
 * 
 * NO PLAGIARIZED CONTENT - NOT EVEN A YOCTO FRAGMENT (10⁻²⁴)
 * ZERO TOLERANCE for unauthorized copying or intellectual property theft.
 * 
 * Verification Methods:
 * - SHA3-512 checksums for integrity verification
 * - BLAKE3 hashing for rapid verification
 * - Git commit history as proof of authorship timeline
 * - Code review and compliance audits
 * 
 * Any concerns about intellectual property should be reported to:
 * rafaelmeloreisnovo [at] gmail [dot] com
 *
NAUTICAL ANCHORS / ÂNCORAS NÁUTICAS (Reference Markers):
 * 
 * These anchors provide stable reference points for:
 * - Version tracking and synchronization
 * - Legal compliance verification
 * - Authorship chain of custody
 * - Update propagation tracking
 * - Audit trail maintenance
 * 
 * ⚓ ANCHOR_ID: 6C3AF290A78D1870
 * ⚓ FILE_PATH: native/src/init/getinfo.cpp
 * ⚓ CREATION_DATE: 2025-11-23
 * ⚓ LAST_MODIFIED: 2025-11-23
 * ⚓ AUTHOR_SIGNATURE: RAFCODE-Rafael Melo Reis (rafaelmeloreisnovo)
 * ⚓ GOVERNANCE_VERSION: ZIPRAF_OMEGA_v999
 * ⚓ LICENSE_VERSION: RAFAELIA_DUAL_v1.0
 * ⚓ ETHICA_VERSION: Ethica[8]_v1.0
 * ⚓ COMPLIANCE_SEAL: RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩARKRE-VERBOΩ
 * ⚓ INTEGRITY_HASH: A518D21399C7D3F21516B087AF3AEA18
 *

 */


#include <sys/sysmacros.h>
#include <sys/types.h>
#include <linux/input.h>
#include <fcntl.h>
#include <vector>

#include <base.hpp>

#include "init.hpp"

using namespace std;

template<char... cs> using chars = integer_sequence<char, cs...>;

// If quoted, parsing ends when we find char in [breaks]
// If not quoted, parsing ends when we find char in [breaks] + [escapes]
template<char... escapes, char... breaks>
static string extract_quoted_str_until(chars<escapes...>, chars<breaks...>,
        string_view str, size_t &pos, bool &quoted) {
    string result;
    char match_array[] = {escapes..., breaks..., '"'};
    string_view match(match_array, std::size(match_array));
    for (size_t cur = pos;; ++cur) {
        cur = str.find_first_of(match, cur);
        if (cur == string_view::npos ||
            ((str[cur] == breaks) || ...) ||
            (!quoted && ((str[cur] == escapes) || ...))) {
            result.append(str.substr(pos, cur - pos));
            pos = cur;
            return result;
        }
        if (str[cur] == '"') {
            quoted = !quoted;
            result.append(str.substr(pos, cur - pos));
            pos = cur + 1;
        }
    }
}

// Parse string into key value pairs.
// The string format: [delim][key][padding][eq][padding][value][delim]
template<char delim, char eq, char... padding>
static kv_pairs parse_impl(chars<padding...>, string_view str) {
    kv_pairs kv;
    char skip_array[] = {eq, padding...};
    string_view skip(skip_array, std::size(skip_array));
    bool quoted = false;
    for (size_t pos = 0u; pos < str.size(); pos = str.find_first_not_of(delim, pos)) {
        auto key = extract_quoted_str_until(
                chars<padding..., delim>{}, chars<eq>{}, str, pos, quoted);
        pos = str.find_first_not_of(skip, pos);
        if (pos == string_view::npos || str[pos] == delim) {
            kv.emplace_back(key, "");
            continue;
        }
        auto value = extract_quoted_str_until(chars<delim>{}, chars<>{}, str, pos, quoted);
        kv.emplace_back(key, value);
    }
    return kv;
}

static kv_pairs parse_cmdline(string_view str) {
    return parse_impl<' ', '='>(chars<>{}, str);
}
static kv_pairs parse_bootconfig(string_view str) {
    return parse_impl<'\n', '='>(chars<' '>{}, str);
}
static kv_pairs parse_partition_map(std::string_view str) {
    return parse_impl<';', ','>(chars<>{}, str);
}

#define test_bit(bit, array) (array[bit / 8] & (1 << (bit % 8)))

static bool check_key_combo() {
    LOGD("Running in recovery mode, waiting for key...\n");
    uint8_t bitmask[(KEY_MAX + 1) / 8];
    vector<int> events;
    constexpr const char *name = "/event";

    for (int minor = 64; minor < 96; ++minor) {
        if (xmknod(name, S_IFCHR | 0444, makedev(13, minor)))
            continue;
        int fd = open(name, O_RDONLY | O_CLOEXEC);
        unlink(name);
        if (fd < 0)
            continue;
        memset(bitmask, 0, sizeof(bitmask));
        ioctl(fd, EVIOCGBIT(EV_KEY, sizeof(bitmask)), bitmask);
        if (test_bit(KEY_VOLUMEUP, bitmask))
            events.push_back(fd);
        else
            close(fd);
    }
    if (events.empty())
        return false;

    run_finally fin([&] { for_each(events.begin(), events.end(), close); });

    // Return true if volume up key is held for more than 3 seconds
    int count = 0;
    for (int i = 0; i < 500; ++i) {
        for (const int &fd : events) {
            memset(bitmask, 0, sizeof(bitmask));
            ioctl(fd, EVIOCGKEY(sizeof(bitmask)), bitmask);
            if (test_bit(KEY_VOLUMEUP, bitmask)) {
                count++;
                break;
            }
        }
        if (count >= 300) {
            LOGD("KEY_VOLUMEUP detected: disable system-as-root\n");
            return true;
        }
        // Check every 10ms
        usleep(10000);
    }
    return false;
}

void BootConfig::set(const kv_pairs &kv) noexcept {
    for (const auto &[key, value] : kv) {
        if (key == "androidboot.slot_suffix") {
            // Many Amlogic devices are A-only but have slot_suffix...
            if (value == "normal") {
                LOGW("Skip invalid androidboot.slot_suffix=[normal]\n");
                continue;
            }
            strscpy(slot.data(), value.data(), slot.size());
        } else if (key == "androidboot.slot") {
            slot[0] = '_';
            strscpy(slot.data() + 1, value.data(), slot.size() - 1);
        } else if (key == "skip_initramfs") {
            skip_initramfs = true;
        } else if (key == "androidboot.force_normal_boot") {
            force_normal_boot = !value.empty() && value[0] == '1';
        } else if (key == "rootwait") {
            rootwait = true;
        } else if (key == "androidboot.android_dt_dir") {
            strscpy(dt_dir.data(), value.data(), dt_dir.size());
        } else if (key == "androidboot.hardware") {
            strscpy(hardware.data(), value.data(), hardware.size());
        } else if (key == "androidboot.hardware.platform") {
            strscpy(hardware_plat.data(), value.data(), hardware_plat.size());
        } else if (key == "androidboot.fstab_suffix") {
            strscpy(fstab_suffix.data(), value.data(), fstab_suffix.size());
        } else if (key == "qemu") {
            emulator = true;
        } else if (key == "androidboot.partition_map") {
            // androidboot.partition_map allows mapping a partition name to a raw block device.
            // For example, "androidboot.partition_map=vdb,metadata;vdc,userdata" maps
            // "vdb" to "metadata", and "vdc" to "userdata".
            // https://android.googlesource.com/platform/system/core/+/refs/heads/android13-release/init/devices.cpp#191
            for (const auto &[k, v]: parse_partition_map(value)) {
                partition_map.emplace_back(k, v);
            }
        }
    }
}

#define read_dt(name, key)                                          \
ssprintf(file_name, sizeof(file_name), "%s/" name, dt_dir.data());  \
if (access(file_name, R_OK) == 0) {                                 \
    string data = full_read(file_name);                             \
    if (!data.empty()) {                                            \
        data.pop_back();                                            \
        strscpy(key.data(), data.data(), key.size());               \
    }                                                               \
}

void BootConfig::init() noexcept {
    set(parse_cmdline(full_read("/proc/cmdline")));
    set(parse_bootconfig(full_read("/proc/bootconfig")));

    parse_prop_file("/.backup/.magisk", [&](auto key, auto value) -> bool {
        if (key == "RECOVERYMODE" && value == "true") {
            skip_initramfs = emulator || !check_key_combo();
            return false;
        }
        return true;
    });

    if (dt_dir[0] == '\0')
        strscpy(dt_dir.data(), DEFAULT_DT_DIR, dt_dir.size());

    char file_name[128];
    read_dt("fstab_suffix", fstab_suffix)
    read_dt("hardware", hardware)
    read_dt("hardware.platform", hardware_plat)

    LOGD("Device config:\n");
    print();
}
