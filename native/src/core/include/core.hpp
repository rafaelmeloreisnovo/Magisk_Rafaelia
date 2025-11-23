/*
 * pragma once include <sys/socket.h> include <string> Part of Magisk_Rafaelia RAFAELIA PHILOSOPHY / FILOSOFIA RAFAELIA:
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
 * ⚓ ANCHOR_ID: D1BA2B6A12D7D93A
 * ⚓ FILE_PATH: native/src/core/include/core.hpp
 * ⚓ CREATION_DATE: 2025-11-23
 * ⚓ LAST_MODIFIED: 2025-11-23
 * ⚓ AUTHOR_SIGNATURE: RAFCODE-Rafael Melo Reis (rafaelmeloreisnovo)
 * ⚓ GOVERNANCE_VERSION: ZIPRAF_OMEGA_v999
 * ⚓ LICENSE_VERSION: RAFAELIA_DUAL_v1.0
 * ⚓ ETHICA_VERSION: Ethica[8]_v1.0
 * ⚓ COMPLIANCE_SEAL: RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩARKRE-VERBOΩ
 * ⚓ INTEGRITY_HASH: 0B5B0C2462E7D777D2CDB0ACE99E9152
 *

 */


#pragma once

#include <sys/socket.h>
#include <string>
#include <atomic>
#include <functional>

#include <base.hpp>

#include "../core-rs.hpp"

#define AID_ROOT   0
#define AID_SHELL  2000
#define AID_USER_OFFSET 100000

#define to_app_id(uid)  (uid % AID_USER_OFFSET)
#define to_user_id(uid) (uid / AID_USER_OFFSET)

#define SDK_INT      (MagiskD::Get().sdk_int())
#define APP_DATA_DIR (SDK_INT >= 24 ? "/data/user_de" : "/data/user")

inline int connect_daemon(RequestCode req) {
    return connect_daemon(req, false);
}

// Multi-call entrypoints
int su_client_main(int argc, char *argv[]);
int zygisk_main(int argc, char *argv[]);

struct ModuleInfo;

// Utils
const char *get_magisk_tmp();
void unlock_blocks();
bool check_key_combo();
template<typename T> requires(std::is_trivially_copyable_v<T>)
T read_any(int fd) {
    T val;
    if (xxread(fd, &val, sizeof(val)) != sizeof(val))
        return -1;
    return val;
}
template<typename T> requires(std::is_trivially_copyable_v<T>)
void write_any(int fd, T val) {
    if (fd < 0) return;
    xwrite(fd, &val, sizeof(val));
}
inline int read_int(int fd) { return read_any<int>(fd); }
inline void write_int(int fd, int val) { write_any(fd, val); }
std::string read_string(int fd);
bool read_string(int fd, std::string &str);
void write_string(int fd, std::string_view str);
template<typename T> requires(std::is_trivially_copyable_v<T>)
void write_vector(int fd, const std::vector<T> &vec) {
    write_int(fd, vec.size());
    xwrite(fd, vec.data(), vec.size() * sizeof(T));
}
template<typename T> requires(std::is_trivially_copyable_v<T>)
bool read_vector(int fd, std::vector<T> &vec) {
    int size = read_int(fd);
    vec.resize(size);
    return xread(fd, vec.data(), size * sizeof(T)) == size * sizeof(T);
}

// Scripting
void install_apk(Utf8CStr apk);
void uninstall_pkg(Utf8CStr pkg);
void exec_common_scripts(Utf8CStr stage);
void exec_module_scripts(Utf8CStr stage, const rust::Vec<ModuleInfo> &module_list);
void exec_script(Utf8CStr script);
void clear_pkg(const char *pkg, int user_id);
[[noreturn]] void install_module(Utf8CStr file);

// Denylist
extern std::atomic<bool> denylist_enforced;
int denylist_cli(rust::Vec<rust::String> &args);
void denylist_handler(int client);
void initialize_denylist();
void scan_deny_apps();
bool is_deny_target(int uid, std::string_view process);
void revert_unmount(int pid = -1) noexcept;
void update_deny_flags(int uid, rust::Str process, uint32_t &flags);

// MagiskSU
void exec_root_shell(int client, int pid, SuRequest &req, MntNsMode mode);

// Rust bindings
inline Utf8CStr get_magisk_tmp_rs() { return get_magisk_tmp(); }
inline rust::String resolve_preinit_dir_rs(Utf8CStr base_dir) {
    return resolve_preinit_dir(base_dir.c_str());
}
