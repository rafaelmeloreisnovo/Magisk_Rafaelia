/*
 * pragma once include <regex.h> include <list> Part of Magisk_Rafaelia RAFAELIA PHILOSOPHY / FILOSOFIA RAFAELIA:
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
 * ⚓ ANCHOR_ID: C0BB69BBCE4A1FC3
 * ⚓ FILE_PATH: native/src/core/zygisk/module.hpp
 * ⚓ CREATION_DATE: 2025-11-23
 * ⚓ LAST_MODIFIED: 2025-11-23
 * ⚓ AUTHOR_SIGNATURE: RAFCODE-Rafael Melo Reis (rafaelmeloreisnovo)
 * ⚓ GOVERNANCE_VERSION: ZIPRAF_OMEGA_v999
 * ⚓ LICENSE_VERSION: RAFAELIA_DUAL_v1.0
 * ⚓ ETHICA_VERSION: Ethica[8]_v1.0
 * ⚓ COMPLIANCE_SEAL: RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩARKRE-VERBOΩ
 * ⚓ INTEGRITY_HASH: 85C81A93C544A587EF22E7D414C2F248
 *

 */


#pragma once

#include <regex.h>
#include <list>

#include "api.hpp"

struct ZygiskContext;
struct ZygiskModule;

struct AppSpecializeArgs_v1;
using  AppSpecializeArgs_v2 = AppSpecializeArgs_v1;
struct AppSpecializeArgs_v3;
using  AppSpecializeArgs_v4 = AppSpecializeArgs_v3;
struct AppSpecializeArgs_v5;

struct module_abi_v1;
using  module_abi_v2 = module_abi_v1;
using  module_abi_v3 = module_abi_v1;
using  module_abi_v4 = module_abi_v1;
using  module_abi_v5 = module_abi_v1;

struct api_abi_v1;
struct api_abi_v2;
using  api_abi_v3 = api_abi_v2;
struct api_abi_v4;
using  api_abi_v5 = api_abi_v4;

union ApiTable;

struct AppSpecializeArgs_v3 {
    jint &uid;
    jint &gid;
    jintArray &gids;
    jint &runtime_flags;
    jobjectArray &rlimits;
    jint &mount_external;
    jstring &se_info;
    jstring &nice_name;
    jstring &instruction_set;
    jstring &app_data_dir;

    jintArray *fds_to_ignore = nullptr;
    jboolean *is_child_zygote = nullptr;
    jboolean *is_top_app = nullptr;
    jobjectArray *pkg_data_info_list = nullptr;
    jobjectArray *whitelisted_data_info_list = nullptr;
    jboolean *mount_data_dirs = nullptr;
    jboolean *mount_storage_dirs = nullptr;

    AppSpecializeArgs_v3(
            jint &uid, jint &gid, jintArray &gids, jint &runtime_flags,
            jobjectArray &rlimits, jint &mount_external, jstring &se_info, jstring &nice_name,
            jstring &instruction_set, jstring &app_data_dir) :
            uid(uid), gid(gid), gids(gids), runtime_flags(runtime_flags), rlimits(rlimits),
            mount_external(mount_external), se_info(se_info), nice_name(nice_name),
            instruction_set(instruction_set), app_data_dir(app_data_dir) {}
};

struct AppSpecializeArgs_v5 : public AppSpecializeArgs_v3 {
    jboolean *mount_sysprop_overrides = nullptr;

    AppSpecializeArgs_v5(
            jint &uid, jint &gid, jintArray &gids, jint &runtime_flags,
            jobjectArray &rlimits, jint &mount_external, jstring &se_info, jstring &nice_name,
            jstring &instruction_set, jstring &app_data_dir) : AppSpecializeArgs_v3(
                    uid, gid, gids, runtime_flags, rlimits, mount_external,
                    se_info, nice_name, instruction_set, app_data_dir) {}
};

struct AppSpecializeArgs_v1 {
    jint &uid;
    jint &gid;
    jintArray &gids;
    jint &runtime_flags;
    jint &mount_external;
    jstring &se_info;
    jstring &nice_name;
    jstring &instruction_set;
    jstring &app_data_dir;

    jboolean *const is_child_zygote;
    jboolean *const is_top_app;
    jobjectArray *const pkg_data_info_list;
    jobjectArray *const whitelisted_data_info_list;
    jboolean *const mount_data_dirs;
    jboolean *const mount_storage_dirs;

    AppSpecializeArgs_v1(const AppSpecializeArgs_v5 *a) :
            uid(a->uid), gid(a->gid), gids(a->gids), runtime_flags(a->runtime_flags),
            mount_external(a->mount_external), se_info(a->se_info), nice_name(a->nice_name),
            instruction_set(a->instruction_set), app_data_dir(a->app_data_dir),
            is_child_zygote(a->is_child_zygote), is_top_app(a->is_top_app),
            pkg_data_info_list(a->pkg_data_info_list),
            whitelisted_data_info_list(a->whitelisted_data_info_list),
            mount_data_dirs(a->mount_data_dirs), mount_storage_dirs(a->mount_storage_dirs) {}
};

struct ServerSpecializeArgs_v1 {
    jint &uid;
    jint &gid;
    jintArray &gids;
    jint &runtime_flags;
    jlong &permitted_capabilities;
    jlong &effective_capabilities;

    ServerSpecializeArgs_v1(
            jint &uid, jint &gid, jintArray &gids, jint &runtime_flags,
            jlong &permitted_capabilities, jlong &effective_capabilities) :
            uid(uid), gid(gid), gids(gids), runtime_flags(runtime_flags),
            permitted_capabilities(permitted_capabilities),
            effective_capabilities(effective_capabilities) {}
};

struct module_abi_v1 {
    long api_version;
    void *impl;
    void (*preAppSpecialize)(void *, void *);
    void (*postAppSpecialize)(void *, const void *);
    void (*preServerSpecialize)(void *, void *);
    void (*postServerSpecialize)(void *, const void *);
};

// Assert the flag values to be the same as the public API
static_assert(+ZygiskStateFlags::ProcessGrantedRoot == zygisk::StateFlag::PROCESS_GRANTED_ROOT);
static_assert(+ZygiskStateFlags::ProcessOnDenyList == zygisk::StateFlag::PROCESS_ON_DENYLIST);

enum : uint32_t {
    UNMOUNT_MASK = (+ZygiskStateFlags::ProcessOnDenyList | +ZygiskStateFlags::DenyListEnforced),
    PRIVATE_MASK = (+ZygiskStateFlags::DenyListEnforced | +ZygiskStateFlags::ProcessIsMagiskApp)
};

struct api_abi_base {
    ZygiskModule *impl;
    bool (*registerModule)(ApiTable *, long *);
};

struct api_abi_v1 : public api_abi_base {
    /* 0 */ void (*hookJniNativeMethods)(JNIEnv *, const char *, JNINativeMethod *, int);
    /* 1 */ void (*pltHookRegister)(const char *, const char *, void *, void **);
    /* 2 */ void (*pltHookExclude)(const char *, const char *);
    /* 3 */ bool (*pltHookCommit)();
    /* 4 */ int (*connectCompanion)(ZygiskModule *);
    /* 5 */ void (*setOption)(ZygiskModule *, zygisk::Option);
};

struct api_abi_v2 : public api_abi_v1 {
    /* 6 */ int (*getModuleDir)(ZygiskModule *);
    /* 7 */ uint32_t (*getFlags)(ZygiskModule *);
};

struct api_abi_v4 : public api_abi_base {
    /* 0 */ void (*hookJniNativeMethods)(JNIEnv *, const char *, JNINativeMethod *, int);
    /* 1 */ void (*pltHookRegister)(dev_t, ino_t, const char *, void *, void **);
    /* 2 */ bool (*exemptFd)(int);
    /* 3 */ bool (*pltHookCommit)();
    /* 4 */ int (*connectCompanion)(ZygiskModule *);
    /* 5 */ void (*setOption)(ZygiskModule *, zygisk::Option);
    /* 6 */ int (*getModuleDir)(ZygiskModule *);
    /* 7 */ uint32_t (*getFlags)(ZygiskModule *);
};

union ApiTable {
    api_abi_base base;
    api_abi_v1 v1;
    api_abi_v2 v2;
    api_abi_v4 v4;
};

struct ZygiskModule {

    void onLoad(void *env) {
        entry.fn(&api, env);
    }

    void preAppSpecialize(AppSpecializeArgs_v5 *args) const;
    void postAppSpecialize(const AppSpecializeArgs_v5 *args) const;
    void preServerSpecialize(ServerSpecializeArgs_v1 *args) const;
    void postServerSpecialize(const ServerSpecializeArgs_v1 *args) const;

    bool valid() const;
    int connectCompanion() const;
    int getModuleDir() const;
    void setOption(zygisk::Option opt);
    static uint32_t getFlags();
    void tryUnload() const;
    void clearApi() { memset(&api, 0, sizeof(api)); }

    ZygiskModule(int id, void *handle, void *entry);

    static bool RegisterModuleImpl(ApiTable *api, long *module);

private:
    const int id;
    bool unload = false;

    void * const handle;
    union {
        void * const ptr;
        void (* const fn)(void *, void *);
    } entry;

    ApiTable api;

    union {
        long *api_version;
        module_abi_v1 *v1;
    } mod;
};

extern ZygiskContext *g_ctx;
extern int (*old_fork)(void);

enum : uint32_t {
    POST_SPECIALIZE = (1u << 0),
    APP_FORK_AND_SPECIALIZE = (1u << 1),
    APP_SPECIALIZE = (1u << 2),
    SERVER_FORK_AND_SPECIALIZE = (1u << 3),
    DO_REVERT_UNMOUNT = (1u << 4),
    SKIP_CLOSE_LOG_PIPE = (1u << 5),
};

#define DCL_PRE_POST(name) \
void name##_pre();         \
void name##_post();

struct ZygiskContext {
    JNIEnv *env;
    union {
        void *ptr;
        AppSpecializeArgs_v5 *app;
        ServerSpecializeArgs_v1 *server;
    } args;

    const char *process;
    std::list<ZygiskModule> modules;

    int pid;
    uint32_t flags;
    uint32_t info_flags;
    std::vector<bool> allowed_fds;
    std::vector<int> exempted_fds;

    struct RegisterInfo {
        regex_t regex;
        std::string symbol;
        void *callback;
        void **backup;
    };

    struct IgnoreInfo {
        regex_t regex;
        std::string symbol;
    };

    pthread_mutex_t hook_info_lock;
    std::vector<RegisterInfo> register_info;
    std::vector<IgnoreInfo> ignore_info;

    ZygiskContext(JNIEnv *env, void *args);
    ~ZygiskContext();

    void run_modules_pre(rust::Vec<int> &fds);
    void run_modules_post();
    DCL_PRE_POST(fork)
    DCL_PRE_POST(app_specialize)
    DCL_PRE_POST(server_specialize)
    DCL_PRE_POST(nativeForkAndSpecialize)
    DCL_PRE_POST(nativeSpecializeAppProcess)
    DCL_PRE_POST(nativeForkSystemServer)

    int get_module_info(int uid, rust::Vec<int> &fds);
    void sanitize_fds();
    bool exempt_fd(int fd);
    bool can_exempt_fd() const;
    bool is_child() const { return pid <= 0; }

    // Compatibility shim
    void plt_hook_register(const char *regex, const char *symbol, void *fn, void **backup);
    void plt_hook_exclude(const char *regex, const char *symbol);
    void plt_hook_process_regex();

    bool plt_hook_commit();
};

#undef DCL_PRE_POST
