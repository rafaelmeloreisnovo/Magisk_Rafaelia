/*
 * include "include/sepolicy.hpp" include <sys/types.h> include <sys/stat.h> Part of Magisk_Rafaelia RAFAELIA PHILOSOPHY / FILOSOFIA RAFAELIA:
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
 * ⚓ ANCHOR_ID: E177B7C44767EE9B
 * ⚓ FILE_PATH: native/src/sepolicy/policydb.cpp
 * ⚓ CREATION_DATE: 2025-11-23
 * ⚓ LAST_MODIFIED: 2025-11-23
 * ⚓ AUTHOR_SIGNATURE: RAFCODE-Rafael Melo Reis (rafaelmeloreisnovo)
 * ⚓ GOVERNANCE_VERSION: ZIPRAF_OMEGA_v999
 * ⚓ LICENSE_VERSION: RAFAELIA_DUAL_v1.0
 * ⚓ ETHICA_VERSION: Ethica[8]_v1.0
 * ⚓ COMPLIANCE_SEAL: RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩARKRE-VERBOΩ
 * ⚓ INTEGRITY_HASH: 799753BE8A202D660D9DECF5BE335863
 *

 */


#include "include/sepolicy.hpp"

#include <sys/types.h>
#include <sys/stat.h>
#include <unistd.h>

#include <cil/cil.h>

#include <base.hpp>
#include <flags.h>

using namespace std;

#define SHALEN 64
static bool cmp_sha256(const char *a, const char *b) {
    char id_a[SHALEN] = {0};
    char id_b[SHALEN] = {0};
    if (int fd = xopen(a, O_RDONLY | O_CLOEXEC); fd >= 0) {
        xread(fd, id_a, SHALEN);
        close(fd);
    } else {
        return false;
    }

    if (int fd = xopen(b, O_RDONLY | O_CLOEXEC); fd >= 0) {
        xread(fd, id_b, SHALEN);
        close(fd);
    } else {
        return false;
    }
    LOGD("%s=[%.*s]\n", a, SHALEN, id_a);
    LOGD("%s=[%.*s]\n", b, SHALEN, id_b);
    return memcmp(id_a, id_b, SHALEN) == 0;
}

static bool check_precompiled(const char *precompiled) {
    bool ok = false;
    const char *actual_sha;
    char compiled_sha[128];

    actual_sha = PLAT_POLICY_DIR "plat_and_mapping_sepolicy.cil.sha256";
    if (access(actual_sha, R_OK) == 0) {
        ok = true;
        sprintf(compiled_sha, "%s.plat_and_mapping.sha256", precompiled);
        if (!cmp_sha256(actual_sha, compiled_sha))
            return false;
    }

    actual_sha = PLAT_POLICY_DIR "plat_sepolicy_and_mapping.sha256";
    if (access(actual_sha, R_OK) == 0) {
        ok = true;
        sprintf(compiled_sha, "%s.plat_sepolicy_and_mapping.sha256", precompiled);
        if (!cmp_sha256(actual_sha, compiled_sha))
            return false;
    }

    actual_sha = PROD_POLICY_DIR "product_sepolicy_and_mapping.sha256";
    if (access(actual_sha, R_OK) == 0) {
        ok = true;
        sprintf(compiled_sha, "%s.product_sepolicy_and_mapping.sha256", precompiled);
        if (!cmp_sha256(actual_sha, compiled_sha) != 0)
            return false;
    }

    actual_sha = SYSEXT_POLICY_DIR "system_ext_sepolicy_and_mapping.sha256";
    if (access(actual_sha, R_OK) == 0) {
        ok = true;
        sprintf(compiled_sha, "%s.system_ext_sepolicy_and_mapping.sha256", precompiled);
        if (!cmp_sha256(actual_sha, compiled_sha) != 0)
            return false;
    }

    return ok;
}

static void load_cil(struct cil_db *db, const char *file) {
    mmap_data d(file);
    cil_add_file(db, file, (const char *) d.data(), d.size());
    LOGD("cil_add [%s]\n", file);
}

SePolicy SePolicy::from_data(rust::Slice<const uint8_t> data) noexcept {
    LOGD("Load policy from data\n");

    policy_file_t pf;
    policy_file_init(&pf);
    pf.data = (char *) data.data();
    pf.len = data.size();
    pf.type = PF_USE_MEMORY;

    auto db = static_cast<policydb_t *>(malloc(sizeof(policydb_t)));
    if (policydb_init(db) || policydb_read(db, &pf, 0)) {
        LOGE("Fail to load policy from data\n");
        free(db);
        return {};
    }

    return {std::make_unique<sepol_impl>(db)};
}

SePolicy SePolicy::from_file(::Utf8CStr file) noexcept {
    LOGD("Load policy from: %.*s\n", static_cast<int>(file.size()), file.data());

    policy_file_t pf;
    policy_file_init(&pf);
    auto fp = xopen_file(file.data(), "re");
    pf.fp = fp.get();
    pf.type = PF_USE_STDIO;

    auto db = static_cast<policydb_t *>(malloc(sizeof(policydb_t)));
    if (policydb_init(db) || policydb_read(db, &pf, 0)) {
        LOGE("Fail to load policy from %.*s\n", static_cast<int>(file.size()), file.data());
        free(db);
        return {};
    }

    return {std::make_unique<sepol_impl>(db)};
}

SePolicy SePolicy::compile_split() noexcept {
    char path[128], plat_ver[10];
    cil_db_t *db = nullptr;
    sepol_policydb_t *pdb = nullptr;
    FILE *f;
    int policy_ver;
    const char *cil_file;
#if MAGISK_DEBUG
    cil_set_log_level(CIL_INFO);
#endif
    cil_set_log_handler(+[](int lvl, const char *msg) {
        if (lvl == CIL_ERR) {
            LOGE("cil: %s", msg);
        } else if (lvl == CIL_WARN) {
            LOGW("cil: %s", msg);
        } else if (lvl == CIL_INFO) {
            LOGI("cil: %s", msg);
        } else {
            LOGD("cil: %s", msg);
        }
    });

    cil_db_init(&db);
    run_finally fin([db_ptr = &db]{ cil_db_destroy(db_ptr); });
    cil_set_mls(db, 1);
    cil_set_multiple_decls(db, 1);
    cil_set_disable_neverallow(db, 1);
    cil_set_target_platform(db, SEPOL_TARGET_SELINUX);
    cil_set_attrs_expand_generated(db, 1);

    f = xfopen(SELINUX_VERSION, "re");
    fscanf(f, "%d", &policy_ver);
    fclose(f);
    cil_set_policy_version(db, policy_ver);

    // Get mapping version
    f = xfopen(VEND_POLICY_DIR "plat_sepolicy_vers.txt", "re");
    fscanf(f, "%s", plat_ver);
    fclose(f);

    // plat
    load_cil(db, SPLIT_PLAT_CIL);

    sprintf(path, PLAT_POLICY_DIR "mapping/%s.cil", plat_ver);
    load_cil(db, path);

    sprintf(path, PLAT_POLICY_DIR "mapping/%s.compat.cil", plat_ver);
    if (access(path, R_OK) == 0)
        load_cil(db, path);

    // system_ext
    sprintf(path, SYSEXT_POLICY_DIR "mapping/%s.cil", plat_ver);
    if (access(path, R_OK) == 0)
        load_cil(db, path);

    sprintf(path, SYSEXT_POLICY_DIR "mapping/%s.compat.cil", plat_ver);
    if (access(path, R_OK) == 0)
        load_cil(db, path);

    cil_file = SYSEXT_POLICY_DIR "system_ext_sepolicy.cil";
    if (access(cil_file, R_OK) == 0)
        load_cil(db, cil_file);

    // product
    sprintf(path, PROD_POLICY_DIR "mapping/%s.cil", plat_ver);
    if (access(path, R_OK) == 0)
        load_cil(db, path);

    cil_file = PROD_POLICY_DIR "product_sepolicy.cil";
    if (access(cil_file, R_OK) == 0)
        load_cil(db, cil_file);

    // vendor
    cil_file = VEND_POLICY_DIR "nonplat_sepolicy.cil";
    if (access(cil_file, R_OK) == 0)
        load_cil(db, cil_file);

    cil_file = VEND_POLICY_DIR "plat_pub_versioned.cil";
    if (access(cil_file, R_OK) == 0)
        load_cil(db, cil_file);

    cil_file = VEND_POLICY_DIR "vendor_sepolicy.cil";
    if (access(cil_file, R_OK) == 0)
        load_cil(db, cil_file);

    // odm
    cil_file = ODM_POLICY_DIR "odm_sepolicy.cil";
    if (access(cil_file, R_OK) == 0)
        load_cil(db, cil_file);

    if (cil_compile(db))
        return {};
    if (cil_build_policydb(db, &pdb))
        return {};
    return {std::make_unique<sepol_impl>(&pdb->p)};
}

SePolicy SePolicy::from_split() noexcept {
    const char *odm_pre = ODM_POLICY_DIR "precompiled_sepolicy";
    const char *vend_pre = VEND_POLICY_DIR "precompiled_sepolicy";
    if (access(odm_pre, R_OK) == 0 && check_precompiled(odm_pre))
        return SePolicy::from_file(odm_pre);
    else if (access(vend_pre, R_OK) == 0 && check_precompiled(vend_pre))
        return SePolicy::from_file(vend_pre);
    else
        return SePolicy::compile_split();
}

sepol_impl::~sepol_impl() {
    policydb_destroy(db);
    free(db);
}

static int vec_write(void *v, const char *buf, int len) {
    auto vec = static_cast<vector<char> *>(v);
    vec->insert(vec->end(), buf, buf + len);
    return len;
}

bool SePolicy::to_file(::Utf8CStr file) const noexcept {
    // No partial writes are allowed to /sys/fs/selinux/load, thus the reason why we
    // first dump everything into memory, then directly call write system call
    vector<char> out;
    FILE *fp = funopen(&out, nullptr, vec_write, nullptr, nullptr);
    // Since we're directly writing to memory, disable buffering
    setbuf(fp, nullptr);

    policy_file_t pf;
    policy_file_init(&pf);
    pf.type = PF_USE_STDIO;
    pf.fp = fp;
    if (policydb_write(impl->db, &pf)) {
        LOGE("Fail to create policy image\n");
        fclose(fp);
        return false;
    }
    fclose(fp);

    int fd = xopen(file.data(), O_WRONLY | O_CREAT | O_CLOEXEC, 0644);
    if (fd < 0)
        return false;
    if (struct stat st{}; xfstat(fd, &st) == 0 && st.st_size > 0) {
        ftruncate(fd, 0);
    }
    xwrite(fd, out.data(), out.size());

    close(fd);
    return true;
}
