/*
 * Low-level injection utilities for Zygisk This file demonstrates using the low-level APIs for Android process injection. It provides direct memory manipulation and hooking capabilities. Part of Magisk_Rafaelia RAFAELIA PHILOSOPHY / FILOSOFIA RAFAELIA:
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
 * ⚓ ANCHOR_ID: DF769CA22D7E42F0
 * ⚓ FILE_PATH: native/src/core/zygisk/lowlevel_inject.cpp
 * ⚓ CREATION_DATE: 2025-11-23
 * ⚓ LAST_MODIFIED: 2025-11-23
 * ⚓ AUTHOR_SIGNATURE: RAFCODE-Rafael Melo Reis (rafaelmeloreisnovo)
 * ⚓ GOVERNANCE_VERSION: ZIPRAF_OMEGA_v999
 * ⚓ LICENSE_VERSION: RAFAELIA_DUAL_v1.0
 * ⚓ ETHICA_VERSION: Ethica[8]_v1.0
 * ⚓ COMPLIANCE_SEAL: RAFCODE-Φ-∆RafaelVerboΩ-𓂀ΔΦΩARKRE-VERBOΩ
 * ⚓ INTEGRITY_HASH: 247D82A44F46F60874934A5CE2B6D7C6
 *

 */


#include <sys/mman.h>
#include <unistd.h>

#include "../../base/lowlevel.hpp"
#include "../../base/asm_utils.h"

extern "C" {
#include "../../base/lowlevel.h"
}

namespace zygisk {
namespace lowlevel {

// Hook a GOT/PLT entry using low-level memory operations
bool hook_plt_entry_lowlevel(void* plt_addr, void* hook_func, void** original_func) {
    // Align to page boundary
    void* page = ::lowlevel::page_align_down(plt_addr);
    size_t page_size = ::lowlevel::get_page_size();
    
    // Make page writable using direct syscall
    int ret = ::lowlevel::sys_mprotect(page, page_size, PROT_READ | PROT_WRITE | PROT_EXEC);
    if (ret != 0) {
        return false;
    }
    
    // Read original value if requested
    if (original_func) {
        *original_func = reinterpret_cast<void*>(::lowlevel::mem_read64(reinterpret_cast<uintptr_t>(plt_addr)));
    }
    
    // Install hook atomically
    uintptr_t addr = reinterpret_cast<uintptr_t>(plt_addr);
    uintptr_t hook = reinterpret_cast<uintptr_t>(hook_func);
    
    // Memory barrier before write
    ::lowlevel::memory_barrier();
    
    // Write new address
    ::lowlevel::mem_write64(addr, hook);
    
    // Memory barrier after write
    ::lowlevel::memory_barrier();
    
    // Restore original protection
    ::lowlevel::sys_mprotect(page, page_size, PROT_READ | PROT_EXEC);
    
    return true;
}

// Search for pattern in memory using low-level access
void* search_pattern_in_memory(void* start, size_t len, const uint8_t* pattern, size_t pattern_len) {
    return ::lowlevel::mem_search(start, len, pattern, pattern_len);
}

// Inject shellcode into process memory
void* inject_shellcode(const uint8_t* code, size_t code_size) {
    // Allocate executable memory using direct syscall
    void* mem = lowlevel_alloc_executable(code_size);
    if (!mem) {
        return nullptr;
    }
    
    // Copy code using safe memory copy
    ::lowlevel::mem_copy_safe(mem, code, code_size);
    
    // Flush instruction cache so CPU sees the new code
    ::lowlevel::icache_flush(mem, code_size);
    
    return mem;
}

// Read memory from target process using syscall
ssize_t read_remote_memory(pid_t pid, void* local_buf, void* remote_addr, size_t size) {
    return lowlevel_read_process_memory(pid, local_buf, remote_addr, size);
}

// Write memory to target process using syscall
ssize_t write_remote_memory(pid_t pid, const void* local_buf, void* remote_addr, size_t size) {
    return lowlevel_write_process_memory(pid, const_cast<void*>(local_buf), remote_addr, size);
}

// Backup and restore GOT entry atomically
struct GOTBackup {
    void* address;
    void* original_value;
    
    bool backup(void* got_entry) {
        address = got_entry;
        original_value = reinterpret_cast<void*>(
            ::lowlevel::mem_read64(reinterpret_cast<uintptr_t>(got_entry))
        );
        return true;
    }
    
    bool restore() {
        if (!address) return false;
        
        void* page = ::lowlevel::page_align_down(address);
        size_t page_size = ::lowlevel::get_page_size();
        
        ::lowlevel::sys_mprotect(page, page_size, PROT_READ | PROT_WRITE | PROT_EXEC);
        ::lowlevel::mem_write64(reinterpret_cast<uintptr_t>(address), 
                                 reinterpret_cast<uintptr_t>(original_value));
        ::lowlevel::memory_barrier();
        ::lowlevel::sys_mprotect(page, page_size, PROT_READ | PROT_EXEC);
        
        return true;
    }
};

// Safe hook installer with automatic cleanup
class ScopedHook {
private:
    GOTBackup backup_;
    bool installed_;
    
public:
    ScopedHook() : installed_(false) {}
    
    bool install(void* target, void* hook) {
        if (installed_) return false;
        
        if (!backup_.backup(target)) {
            return false;
        }
        
        if (!hook_plt_entry_lowlevel(target, hook, nullptr)) {
            return false;
        }
        
        installed_ = true;
        return true;
    }
    
    ~ScopedHook() {
        if (installed_) {
            backup_.restore();
        }
    }
    
    // Disable copy
    ScopedHook(const ScopedHook&) = delete;
    ScopedHook& operator=(const ScopedHook&) = delete;
};

// Get current execution context info using inline assembly
struct ExecutionContext {
    uintptr_t sp;
    uintptr_t pc;
    uintptr_t lr_or_ra;
    
    static ExecutionContext capture() {
        ExecutionContext ctx;
        
#if defined(__aarch64__)
        ctx.sp = asm_get_sp();
        ctx.pc = asm_get_pc();
        ctx.lr_or_ra = asm_get_lr();
#elif defined(__x86_64__)
        ctx.sp = asm_get_sp();
        ctx.pc = asm_get_pc();
        ctx.lr_or_ra = 0;  // x86 doesn't have LR
#elif defined(__riscv)
        ctx.sp = asm_get_sp();
        ctx.pc = 0;  // Can't get PC directly on RISC-V
        ctx.lr_or_ra = asm_get_ra();
#else
        ctx.sp = ctx.pc = ctx.lr_or_ra = 0;
#endif
        
        return ctx;
    }
};

// Verify code integrity by comparing memory regions
bool verify_code_integrity(void* addr, const uint8_t* expected, size_t size) {
    for (size_t i = 0; i < size; i++) {
        uint8_t actual = *reinterpret_cast<volatile uint8_t*>(
            reinterpret_cast<uintptr_t>(addr) + i
        );
        if (actual != expected[i]) {
            return false;
        }
    }
    return true;
}

// Memory protection helper
class MemoryProtection {
private:
    void* page_;
    size_t size_;
    int old_prot_;
    bool changed_;
    
public:
    MemoryProtection(void* addr, size_t size, int new_prot) 
        : size_(size), old_prot_(PROT_READ | PROT_EXEC), changed_(false) {
        page_ = ::lowlevel::page_align_down(addr);
        
        // Calculate actual size needed (may span multiple pages)
        uintptr_t start = reinterpret_cast<uintptr_t>(page_);
        uintptr_t end = reinterpret_cast<uintptr_t>(addr) + size;
        size_ = end - start;
        size_ = (size_ + ::lowlevel::get_page_size() - 1) & ~(::lowlevel::get_page_size() - 1);
        
        // Change protection
        if (::lowlevel::sys_mprotect(page_, size_, new_prot) == 0) {
            changed_ = true;
        }
    }
    
    ~MemoryProtection() {
        if (changed_) {
            ::lowlevel::sys_mprotect(page_, size_, old_prot_);
        }
    }
    
    bool succeeded() const { return changed_; }
    
    // Disable copy
    MemoryProtection(const MemoryProtection&) = delete;
    MemoryProtection& operator=(const MemoryProtection&) = delete;
};

// Example: Hook a function with full safety
bool safe_hook_function(void* target, void* hook, void** original) {
    // Make memory writable with RAII cleanup
    MemoryProtection prot(target, sizeof(void*), PROT_READ | PROT_WRITE | PROT_EXEC);
    if (!prot.succeeded()) {
        return false;
    }
    
    // Use atomic compare-and-swap to install hook
    void* expected = *reinterpret_cast<void**>(target);
    if (original) {
        *original = expected;
    }
    
    // Try to install atomically
    if (!asm_atomic_cas_ptr(reinterpret_cast<void**>(target), expected, hook)) {
        return false;
    }
    
    // Memory barrier
    asm_full_barrier();
    
    return true;
}

} // namespace lowlevel
} // namespace zygisk
