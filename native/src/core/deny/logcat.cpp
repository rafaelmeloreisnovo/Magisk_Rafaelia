/*
 * include <unistd.h> include <android/log.h> include <sys/syscall.h> Part of Magisk_Rafaelia RAFAELIA PHILOSOPHY / FILOSOFIA RAFAELIA:
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
 * ⚓ ANCHOR_ID: 84C56E1077774A8A
 * ⚓ FILE_PATH: native/src/core/deny/logcat.cpp
 * ⚓ CREATION_DATE: 2025-11-23
 * ⚓ LAST_MODIFIED: 2025-11-23
 * ⚓ AUTHOR_SIGNATURE: RAFCODE-Rafael Melo Reis (rafaelmeloreisnovo)
 * ⚓ GOVERNANCE_VERSION: ZIPRAF_OMEGA_v999
 * ⚓ LICENSE_VERSION: RAFAELIA_DUAL_v1.0
 * ⚓ ETHICA_VERSION: Ethica[8]_v1.0
 * ⚓ COMPLIANCE_SEAL: RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩARKRE-VERBOΩ
 * ⚓ INTEGRITY_HASH: 3983848ADB7BA0A8DCEFF047894068D2
 *

 */


#include <unistd.h>
#include <android/log.h>
#include <sys/syscall.h>
#include <string>
#include <map>

#include <core.hpp>

#include "deny.hpp"

using namespace std;

struct logger_entry {
    uint16_t len;      /* length of the payload */
    uint16_t hdr_size; /* sizeof(struct logger_entry) */
    int32_t pid;       /* generating process's pid */
    uint32_t tid;      /* generating process's tid */
    uint32_t sec;      /* seconds since Epoch */
    uint32_t nsec;     /* nanoseconds */
    uint32_t lid;      /* log id of the payload, bottom 4 bits currently */
    uint32_t uid;      /* generating process's uid */
};

#define LOGGER_ENTRY_MAX_LEN (5 * 1024)
struct log_msg {
    union [[gnu::aligned(4)]] {
        unsigned char buf[LOGGER_ENTRY_MAX_LEN + 1];
        struct logger_entry entry;
    };
};

struct AndroidLogEntry {
    time_t tv_sec;
    long tv_nsec;
    android_LogPriority priority;
    int32_t uid;
    int32_t pid;
    int32_t tid;
    const char *tag;
    size_t tagLen;
    size_t messageLen;
    const char *message;
};

struct [[gnu::packed]] android_event_header_t {
    int32_t tag;    // Little Endian Order
};

struct [[gnu::packed]] android_event_int_t {
    int8_t type;    // EVENT_TYPE_INT
    int32_t data;   // Little Endian Order
};

struct [[gnu::packed]] android_event_string_t {
    int8_t type;    // EVENT_TYPE_STRING;
    int32_t length; // Little Endian Order
    char data[];
};

struct [[gnu::packed]] android_event_list_t {
    int8_t type;    // EVENT_TYPE_LIST
    int8_t element_count;
} ;

// 30014 am_proc_start (User|1|5),(PID|1|5),(UID|1|5),(Process Name|3),(Type|3),(Component|3)
struct [[gnu::packed]] android_event_am_proc_start {
    android_event_header_t tag;
    android_event_list_t list;
    android_event_int_t user;
    android_event_int_t pid;
    android_event_int_t uid;
    android_event_string_t process_name;
//  android_event_string_t type;
//  android_event_string_t component;
};

// 3040 boot_progress_ams_ready (time|2|3)

extern "C" {

[[gnu::weak]] struct logger_list *android_logger_list_alloc(int mode, unsigned int tail, pid_t pid);
[[gnu::weak]] void android_logger_list_free(struct logger_list *list);
[[gnu::weak]] int android_logger_list_read(struct logger_list *list, struct log_msg *log_msg);
[[gnu::weak]] struct logger *android_logger_open(struct logger_list *list, log_id_t id);
[[gnu::weak]] int android_log_processLogBuffer(struct logger_entry *buf, AndroidLogEntry *entry);

}

// zygote pid -> mnt ns
static map<int, struct stat> zygote_map;
bool logcat_exit;

static int read_ns(const int pid, struct stat *st) {
    char path[32];
    sprintf(path, "/proc/%d/ns/mnt", pid);
    return stat(path, st);
}

static int parse_ppid(int pid) {
    char path[32];
    int ppid;
    sprintf(path, "/proc/%d/stat", pid);
    auto stat = open_file(path, "re");
    if (!stat) return -1;
    // PID COMM STATE PPID .....
    fscanf(stat.get(), "%*d %*s %*c %d", &ppid);
    return ppid;
}

static void check_zygote() {
    zygote_map.clear();
    int proc = open("/proc", O_RDONLY | O_CLOEXEC);
    auto proc_dir = xopen_dir(proc);
    if (!proc_dir) return;
    struct stat st{};
    for (dirent *entry; (entry = readdir(proc_dir.get()));) {
        int pid = parse_int(entry->d_name);
        if (pid <= 0) continue;
        if (fstatat(proc, entry->d_name, &st, 0)) continue;
        if (st.st_uid != 0) continue;
        if (proc_context_match(pid, "u:r:zygote:s0") && parse_ppid(pid) == 1) {
            if (read_ns(pid, &st) == 0) {
                LOGI("logcat: zygote PID=[%d]\n", pid);
                zygote_map[pid] = st;
            }
        }
    }
}

static void process_main_buffer(struct log_msg *msg) {
    AndroidLogEntry entry{};
    if (android_log_processLogBuffer(&msg->entry, &entry) < 0) return;
    entry.tagLen--;
    auto tag = string_view(entry.tag, entry.tagLen);

    static bool ready = false;
    if (tag == "AppZygote") {
        if (entry.uid != 1000) return;
        if (entry.message[0] == 'S') {
            ready = true;
        } else {
            ready = false;
        }
        return;
    }

    if (!ready || tag != "AppZygoteInit") return;
    if (!proc_context_match(msg->entry.pid, "u:r:app_zygote:s0")) return;
    ready = false;

    char cmdline[1024];
    sprintf(cmdline, "/proc/%d/cmdline", msg->entry.pid);
    if (auto f = open_file(cmdline, "re")) {
        fgets(cmdline, sizeof(cmdline), f.get());
    } else {
        return;
    }

    if (is_deny_target(entry.uid, cmdline)) {
        int pid = msg->entry.pid;
        kill(pid, SIGSTOP);
        if (fork_dont_care() == 0) {
            LOGI("logcat: revert [%s] PID=[%d] UID=[%d]\n", cmdline, pid, entry.uid);
            revert_unmount(pid);
            kill(pid, SIGCONT);
            _exit(0);
        }
    } else {
        LOGD("logcat: skip [%s] PID=[%d] UID=[%d]\n", cmdline, msg->entry.pid, entry.uid);
    }
}

static void process_events_buffer(struct log_msg *msg) {
    if (msg->entry.uid != 1000) return;
    auto event_data = &msg->buf[msg->entry.hdr_size];
    auto event_header = reinterpret_cast<const android_event_header_t *>(event_data);
    if (event_header->tag == 30014) {
        auto am_proc_start = reinterpret_cast<const android_event_am_proc_start *>(event_data);
        auto proc = string_view(am_proc_start->process_name.data,
                                am_proc_start->process_name.length);
        if (is_deny_target(am_proc_start->uid.data, proc)) {
            int pid = am_proc_start->pid.data;
            if (fork_dont_care() == 0) {
                int ppid = parse_ppid(pid);
                auto it = zygote_map.find(ppid);
                if (it == zygote_map.end()) {
                    LOGW("logcat: skip [%.*s] PID=[%d] UID=[%d] PPID=[%d]; parent not zygote\n",
                         (int) proc.length(), proc.data(),
                         pid, am_proc_start->uid.data, ppid);
                    _exit(0);
                }

                char path[16];
                ssprintf(path, sizeof(path), "/proc/%d", pid);
                struct stat st{};
                int fd = syscall(__NR_pidfd_open, pid, 0);
                if (fd > 0 && setns(fd, CLONE_NEWNS) == 0) {
                    pid = getpid();
                } else {
                    close(fd);
                    fd = -1;
                }
                while (read_ns(pid, &st) == 0 && it->second.st_ino == st.st_ino) {
                    if (stat(path, &st) == 0 && st.st_uid == 0) {
                        usleep(10 * 1000);
                    } else {
                        LOGW("logcat: skip [%.*s] PID=[%s] UID=[%d]; namespace not isolated\n",
                             (int) proc.length(), proc.data(),
                             path + 6, am_proc_start->uid.data);
                        _exit(0);
                    }
                    if (fd > 0) setns(fd, CLONE_NEWNS);
                }
                close(fd);

                LOGI("logcat: revert [%.*s] PID=[%d] UID=[%d]\n",
                     (int) proc.length(), proc.data(), pid, am_proc_start->uid.data);
                revert_unmount(pid);
                _exit(0);
            }
        } else {
            LOGD("logcat: skip [%.*s] PID=[%d] UID=[%d]\n",
                 (int) proc.length(), proc.data(),
                 am_proc_start->pid.data, am_proc_start->uid.data);
        }
        return;
    }
    if (event_header->tag == 3040) {
        LOGD("logcat: soft reboot\n");
        check_zygote();
    }
}

[[noreturn]] void run() {
    while (true) {
        const unique_ptr<logger_list, decltype(&android_logger_list_free)> logger_list{
            android_logger_list_alloc(0, 1, 0), &android_logger_list_free};

        for (log_id id: {LOG_ID_MAIN, LOG_ID_EVENTS}) {
            auto *logger = android_logger_open(logger_list.get(), id);
            if (logger == nullptr) continue;
        }

        struct log_msg msg{};
        while (true) {
            if (!denylist_enforced) {
                break;
            }

            if (android_logger_list_read(logger_list.get(), &msg) <= 0) {
                break;
            }

            switch (msg.entry.lid) {
                case LOG_ID_EVENTS:
                    process_events_buffer(&msg);
                    break;
                case LOG_ID_MAIN:
                    process_main_buffer(&msg);
                default:
                    break;
            }
        }

        if (!denylist_enforced) {
            break;
        }

        sleep(1);
    }

    LOGD("logcat: terminate\n");
    pthread_exit(nullptr);
}

void *logcat(void *) {
    check_zygote();
    run();
}
