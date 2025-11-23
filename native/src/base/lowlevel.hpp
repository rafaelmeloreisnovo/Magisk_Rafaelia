/*
 * pragma once Low-level memory and syscall utilities for Android injection This provides direct memory access and syscall interfaces for critical operations Part of Magisk_Rafaelia RAFAELIA PHILOSOPHY / FILOSOFIA RAFAELIA:
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
 * ⚓ ANCHOR_ID: CE81E0D5EE0D2D75
 * ⚓ FILE_PATH: native/src/base/lowlevel.hpp
 * ⚓ CREATION_DATE: 2025-11-23
 * ⚓ LAST_MODIFIED: 2025-11-23
 * ⚓ AUTHOR_SIGNATURE: RAFCODE-Rafael Melo Reis (rafaelmeloreisnovo)
 * ⚓ GOVERNANCE_VERSION: ZIPRAF_OMEGA_v999
 * ⚓ LICENSE_VERSION: RAFAELIA_DUAL_v1.0
 * ⚓ ETHICA_VERSION: Ethica[8]_v1.0
 * ⚓ COMPLIANCE_SEAL: RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩARKRE-VERBOΩ
 * ⚓ INTEGRITY_HASH: BC03229760CC2EE61767123D5642D969
 *

 */


#define SYSCALL_ERROR_THRESHOLD 4095UL

namespace lowlevel {

// Direct memory read/write using volatile pointers to prevent optimization
static inline uint32_t mem_read32(uintptr_t addr) {
    return *reinterpret_cast<volatile uint32_t*>(addr);
}

static inline uint64_t mem_read64(uintptr_t addr) {
    return *reinterpret_cast<volatile uint64_t*>(addr);
}

static inline void mem_write32(uintptr_t addr, uint32_t value) {
    *reinterpret_cast<volatile uint32_t*>(addr) = value;
}

static inline void mem_write64(uintptr_t addr, uint64_t value) {
    *reinterpret_cast<volatile uint64_t*>(addr) = value;
}

// Direct syscall for mmap to allocate memory at specific addresses
static inline void* sys_mmap(void* addr, size_t length, int prot, int flags, int fd, off_t offset) {
#ifdef __NR_mmap2
    // 32-bit ARM and other architectures use mmap2 which takes offset in page units
    long result = syscall(__NR_mmap2, addr, length, prot, flags, fd, static_cast<off_t>(offset >> 12));
#else
    long result = syscall(__NR_mmap, addr, length, prot, flags, fd, offset);
#endif
    // Check if result is an error
    if (static_cast<unsigned long>(result) >= static_cast<unsigned long>(-SYSCALL_ERROR_THRESHOLD)) {
        errno = -static_cast<int>(result);
        return MAP_FAILED;
    }
    return reinterpret_cast<void*>(result);
}

// Direct syscall for munmap
static inline int sys_munmap(void* addr, size_t length) {
    long result = syscall(__NR_munmap, addr, length);
    if (result < 0) {
        errno = -static_cast<int>(result);
        return -1;
    }
    return 0;
}

// Direct syscall for mprotect - critical for code injection
static inline int sys_mprotect(void* addr, size_t len, int prot) {
    long result = syscall(__NR_mprotect, addr, len, prot);
    if (result < 0) {
        errno = -static_cast<int>(result);
        return -1;
    }
    return 0;
}

// Direct syscall for reading memory from another process
static inline ssize_t sys_process_vm_readv(pid_t pid, const struct iovec* local_iov,
                                            unsigned long liovcnt, const struct iovec* remote_iov,
                                            unsigned long riovcnt, unsigned long flags) {
    return syscall(__NR_process_vm_readv, pid, local_iov, liovcnt, remote_iov, riovcnt, flags);
}

// Direct syscall for writing memory to another process
static inline ssize_t sys_process_vm_writev(pid_t pid, const struct iovec* local_iov,
                                             unsigned long liovcnt, const struct iovec* remote_iov,
                                             unsigned long riovcnt, unsigned long flags) {
    return syscall(__NR_process_vm_writev, pid, local_iov, liovcnt, remote_iov, riovcnt, flags);
}

// Direct syscall for ptrace - used in process injection
static inline long sys_ptrace(int request, pid_t pid, void* addr, void* data) {
    return syscall(__NR_ptrace, request, pid, addr, data);
}

// Memory barrier for ensuring write ordering in injection code
static inline void memory_barrier() {
#if defined(__aarch64__) || defined(__arm__)
    __asm__ __volatile__("dmb sy" ::: "memory");
#elif defined(__x86_64__) || defined(__i386__)
    __asm__ __volatile__("mfence" ::: "memory");
#elif defined(__riscv)
    __asm__ __volatile__("fence rw,rw" ::: "memory");
#else
    __sync_synchronize();
#endif
}

// Instruction cache flush - critical after code injection
static inline void icache_flush(void* addr, size_t len) {
#if defined(__aarch64__) || defined(__arm__)
    uintptr_t start = reinterpret_cast<uintptr_t>(addr);
    uintptr_t end = start + len;
    
    // DC CVAU - Data Cache Clean by VA to Point of Unification
    for (uintptr_t p = start; p < end; p += 64) {
        __asm__ __volatile__("dc cvau, %0" : : "r"(p) : "memory");
    }
    
    // DSB - Data Synchronization Barrier
    __asm__ __volatile__("dsb ish" ::: "memory");
    
    // IC IVAU - Instruction Cache Invalidate by VA to Point of Unification
    for (uintptr_t p = start; p < end; p += 64) {
        __asm__ __volatile__("ic ivau, %0" : : "r"(p) : "memory");
    }
    
    // DSB + ISB
    __asm__ __volatile__("dsb ish" ::: "memory");
    __asm__ __volatile__("isb" ::: "memory");
#elif defined(__x86_64__) || defined(__i386__)
    // x86 has coherent instruction cache
    (void)addr;
    (void)len;
    __asm__ __volatile__("" ::: "memory");
#elif defined(__riscv)
    // RISC-V fence.i instruction
    (void)addr;
    (void)len;
    __asm__ __volatile__("fence.i" ::: "memory");
#else
    // Fallback - use system call
    syscall(__NR_cacheflush, addr, len, 0);
#endif
}

// Direct syscall for prctl - used for process manipulation
static inline int sys_prctl(int option, unsigned long arg2, unsigned long arg3,
                            unsigned long arg4, unsigned long arg5) {
    return static_cast<int>(syscall(__NR_prctl, option, arg2, arg3, arg4, arg5));
}

// Direct syscall for getpid
static inline pid_t sys_getpid() {
    return static_cast<pid_t>(syscall(__NR_getpid));
}

// Direct syscall for gettid
static inline pid_t sys_gettid() {
    return static_cast<pid_t>(syscall(__NR_gettid));
}

// Memory search for pattern - used in PLT hooking
static inline void* mem_search(void* start, size_t len, const void* pattern, size_t pattern_len) {
    if (pattern_len == 0 || pattern_len > len) {
        return nullptr;
    }
    
    const uint8_t* haystack = static_cast<const uint8_t*>(start);
    const uint8_t* needle = static_cast<const uint8_t*>(pattern);
    const uint8_t* end = haystack + len - pattern_len + 1;
    
    for (const uint8_t* p = haystack; p < end; ++p) {
        size_t i;
        for (i = 0; i < pattern_len; ++i) {
            if (p[i] != needle[i]) {
                break;
            }
        }
        if (i == pattern_len) {
            return const_cast<void*>(static_cast<const void*>(p));
        }
    }
    return nullptr;
}

// Atomic compare and swap for hooking
static inline bool atomic_cas_ptr(void** ptr, void* expected, void* desired) {
    return __sync_bool_compare_and_swap(ptr, expected, desired);
}

// Get page size at compile time or runtime
static inline size_t get_page_size() {
    static size_t page_size = 0;
    if (page_size == 0) {
        page_size = static_cast<size_t>(sysconf(_SC_PAGESIZE));
    }
    return page_size;
}

// Align address to page boundary
static inline void* page_align_down(void* addr) {
    return reinterpret_cast<void*>(
        reinterpret_cast<uintptr_t>(addr) & ~(get_page_size() - 1)
    );
}

static inline void* page_align_up(void* addr) {
    uintptr_t a = reinterpret_cast<uintptr_t>(addr);
    size_t page_size = get_page_size();
    return reinterpret_cast<void*>((a + page_size - 1) & ~(page_size - 1));
}

// Copy memory with memory barriers - safe for code modification
static inline void mem_copy_safe(void* dest, const void* src, size_t n) {
    memory_barrier();
    __builtin_memcpy(dest, src, n);
    memory_barrier();
}

} // namespace lowlevel
