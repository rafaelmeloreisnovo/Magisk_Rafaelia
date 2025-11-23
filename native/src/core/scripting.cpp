/*
 * include <string> include <vector> include <sys/wait.h> Part of Magisk_Rafaelia RAFAELIA PHILOSOPHY / FILOSOFIA RAFAELIA:
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
 * ⚓ ANCHOR_ID: 4E39A15DF456EF84
 * ⚓ FILE_PATH: native/src/core/scripting.cpp
 * ⚓ CREATION_DATE: 2025-11-23
 * ⚓ LAST_MODIFIED: 2025-11-23
 * ⚓ AUTHOR_SIGNATURE: RAFCODE-Rafael Melo Reis (rafaelmeloreisnovo)
 * ⚓ GOVERNANCE_VERSION: ZIPRAF_OMEGA_v999
 * ⚓ LICENSE_VERSION: RAFAELIA_DUAL_v1.0
 * ⚓ ETHICA_VERSION: Ethica[8]_v1.0
 * ⚓ COMPLIANCE_SEAL: RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩARKRE-VERBOΩ
 * ⚓ INTEGRITY_HASH: 2CA1F9FD8448AFA9817A703742A86175
 *

 */


#include <string>
#include <vector>
#include <sys/wait.h>

#include <consts.hpp>
#include <base.hpp>
#include <core.hpp>

using namespace std;

#define BBEXEC_CMD bbpath(), "sh"

static const char *bbpath() {
    static string path;
    path = get_magisk_tmp();
    path += "/" BBPATH "/busybox";
    if (access(path.data(), X_OK) != 0) {
        path = DATABIN "/busybox";
    }
    return path.data();
}

static void set_script_env() {
    setenv("ASH_STANDALONE", "1", 1);
    char new_path[4096];
    ssprintf(new_path, sizeof(new_path), "%s:%s", getenv("PATH"), get_magisk_tmp());
    setenv("PATH", new_path, 1);
    if (MagiskD::Get().zygisk_enabled())
        setenv("ZYGISK_ENABLED", "1", 1);
};

void exec_script(Utf8CStr script) {
    exec_t exec {
        .pre_exec = set_script_env,
        .fork = fork_no_orphan
    };
    exec_command_sync(exec, BBEXEC_CMD, script.c_str());
}

static timespec pfs_timeout;

#define PFS_SETUP() \
if (pfs) { \
    if (int pid = xfork()) { \
        if (pid < 0) \
            return; \
        /* In parent process, simply wait for child to finish */ \
        waitpid(pid, nullptr, 0); \
        return; \
    } \
    timer_pid = xfork(); \
    if (timer_pid == 0) { \
        /* In timer process, count down */ \
        clock_nanosleep(CLOCK_MONOTONIC, TIMER_ABSTIME, &pfs_timeout, nullptr); \
        exit(0); \
    } \
}

#define PFS_WAIT() \
if (pfs) { \
    /* If we ran out of time, don't block */ \
    if (timer_pid < 0) \
        continue; \
    if (int pid = waitpid(-1, nullptr, 0); pid == timer_pid) { \
        LOGW("* post-fs-data scripts blocking phase timeout\n"); \
        timer_pid = -1; \
    } \
}

#define PFS_DONE() \
if (pfs) { \
    if (timer_pid > 0) \
        kill(timer_pid, SIGKILL); \
    exit(0); \
}

void exec_common_scripts(Utf8CStr stage) {
    LOGI("* Running %s.d scripts\n", stage.c_str());
    char path[4096];
    char *name = path + sprintf(path, SECURE_DIR "/%s.d", stage.c_str());
    auto dir = xopen_dir(path);
    if (!dir) return;

    bool pfs = stage == "post-fs-data"sv;
    int timer_pid = -1;
    if (pfs) {
        // Setup timer
        clock_gettime(CLOCK_MONOTONIC, &pfs_timeout);
        pfs_timeout.tv_sec += POST_FS_DATA_SCRIPT_MAX_TIME;
    }
    PFS_SETUP()

    *(name++) = '/';
    int dfd = dirfd(dir.get());
    for (dirent *entry; (entry = xreaddir(dir.get()));) {
        if (entry->d_type == DT_REG) {
            if (faccessat(dfd, entry->d_name, X_OK, 0) != 0)
                continue;
            LOGI("%s.d: exec [%s]\n", stage.c_str(), entry->d_name);
            strcpy(name, entry->d_name);
            exec_t exec {
                .pre_exec = set_script_env,
                .fork = pfs ? xfork : fork_dont_care
            };
            exec_command(exec, BBEXEC_CMD, path);
            PFS_WAIT()
        }
    }

    PFS_DONE()
}

static bool operator>(const timespec &a, const timespec &b) {
    if (a.tv_sec != b.tv_sec)
        return a.tv_sec > b.tv_sec;
    return a.tv_nsec > b.tv_nsec;
}

void exec_module_scripts(Utf8CStr stage, const rust::Vec<ModuleInfo> &module_list) {
    LOGI("* Running module %s scripts\n", stage.c_str());
    if (module_list.empty())
        return;

    bool pfs = stage == "post-fs-data";
    if (pfs) {
        timespec now{};
        clock_gettime(CLOCK_MONOTONIC, &now);
        // If we had already timed out, treat it as service mode
        if (now > pfs_timeout)
            pfs = false;
    }
    int timer_pid = -1;
    PFS_SETUP()

    char path[4096];
    for (auto &m : module_list) {
        sprintf(path, MODULEROOT "/%.*s/%s.sh", (int) m.name.size(), m.name.data(), stage.c_str());
        if (access(path, F_OK) == -1)
            continue;
        LOGI("%.*s: exec [%s.sh]\n", (int) m.name.size(), m.name.data(), stage.c_str());
        exec_t exec {
            .pre_exec = set_script_env,
            .fork = pfs ? xfork : fork_dont_care
        };
        exec_command(exec, BBEXEC_CMD, path);
        PFS_WAIT()
    }

    PFS_DONE()
}

constexpr char install_script[] = R"EOF(
APK=%s
log -t Magisk "pm_install: $APK"
log -t Magisk "pm_install: $(pm install -g -r $APK 2>&1)"
appops set %s REQUEST_INSTALL_PACKAGES allow
rm -f $APK
)EOF";

void install_apk(Utf8CStr apk) {
    setfilecon(apk.c_str(), MAGISK_FILE_CON);
    char cmds[sizeof(install_script) + 4096];
    ssprintf(cmds, sizeof(cmds), install_script, apk.c_str(), JAVA_PACKAGE_NAME);
    exec_command_async("/system/bin/sh", "-c", cmds);
}

constexpr char uninstall_script[] = R"EOF(
PKG=%s
log -t Magisk "pm_uninstall: $PKG"
log -t Magisk "pm_uninstall: $(pm uninstall $PKG 2>&1)"
)EOF";

void uninstall_pkg(Utf8CStr pkg) {
    char cmds[sizeof(uninstall_script) + 256];
    ssprintf(cmds, sizeof(cmds), uninstall_script, pkg.c_str());
    exec_command_async("/system/bin/sh", "-c", cmds);
}

constexpr char clear_script[] = R"EOF(
PKG=%s
USER=%d
log -t Magisk "pm_clear: $PKG (user=$USER)"
log -t Magisk "pm_clear: $(pm clear --user $USER $PKG 2>&1)"
)EOF";

void clear_pkg(const char *pkg, int user_id) {
    char cmds[sizeof(clear_script) + 288];
    ssprintf(cmds, sizeof(cmds), clear_script, pkg, user_id);
    exec_command_async("/system/bin/sh", "-c", cmds);
}

[[noreturn]] __printflike(2, 3)
static void abort(FILE *fp, const char *fmt, ...) {
    va_list valist;
    va_start(valist, fmt);
    vfprintf(fp, fmt, valist);
    fprintf(fp, "\n\n");
    va_end(valist);
    exit(1);
}

constexpr char install_module_script[] = R"EOF(
. /data/adb/magisk/util_functions.sh
install_module
exit 0
)EOF";

void install_module(Utf8CStr file) {
    if (getuid() != 0)
        abort(stderr, "Run this command with root");
    if (access(DATABIN, F_OK) ||
        access(bbpath(), X_OK) ||
        access(DATABIN "/util_functions.sh", F_OK))
        abort(stderr, "Incomplete Magisk install");
    if (access(file.c_str(), F_OK))
        abort(stderr, "'%s' does not exist", file.c_str());

    char *zip = realpath(file.c_str(), nullptr);
    setenv("OUTFD", "1", 1);
    setenv("ZIPFILE", zip, 1);
    setenv("ASH_STANDALONE", "1", 1);
    setenv("MAGISKTMP", get_magisk_tmp(), 0);
    free(zip);

    int fd = xopen("/dev/null", O_RDONLY);
    xdup2(fd, STDERR_FILENO);
    close(fd);

    const char *argv[] = { BBEXEC_CMD, "-c", install_module_script, nullptr };
    execve(argv[0], (char **) argv, environ);
    abort(stdout, "Failed to execute BusyBox shell");
}
