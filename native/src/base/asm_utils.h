/*
 * Architecture-specific inline assembly utilities
 * 
 * This provides direct CPU-level operations for critical Android injection code.
 * Includes: cache operations, barriers, atomic operations, register access.
 */

#ifndef ASM_UTILS_H
#define ASM_UTILS_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// ARM/ARM64 Architecture
// ============================================================================

#if defined(__aarch64__)

// Get current stack pointer
static inline uint64_t asm_get_sp(void) {
    uint64_t sp;
    __asm__ __volatile__("mov %0, sp" : "=r"(sp));
    return sp;
}

// Get current program counter (approximate)
static inline uint64_t asm_get_pc(void) {
    uint64_t pc;
    __asm__ __volatile__("adr %0, ." : "=r"(pc));
    return pc;
}

// Get current link register
static inline uint64_t asm_get_lr(void) {
    uint64_t lr;
    __asm__ __volatile__("mov %0, x30" : "=r"(lr));
    return lr;
}

// Data memory barrier
static inline void asm_dmb(void) {
    __asm__ __volatile__("dmb sy" ::: "memory");
}

// Data synchronization barrier
static inline void asm_dsb(void) {
    __asm__ __volatile__("dsb sy" ::: "memory");
}

// Instruction synchronization barrier
static inline void asm_isb(void) {
    __asm__ __volatile__("isb" ::: "memory");
}

// Clean data cache line by address
static inline void asm_dc_cvau(uint64_t addr) {
    __asm__ __volatile__("dc cvau, %0" : : "r"(addr) : "memory");
}

// Invalidate instruction cache line by address
static inline void asm_ic_ivau(uint64_t addr) {
    __asm__ __volatile__("ic ivau, %0" : : "r"(addr) : "memory");
}

// Read system register (example: MIDR_EL1 - CPU ID)
// NOTE: Reading system registers may require elevated privileges (EL1/EL2).
// On user-space applications, this may cause an illegal instruction exception.
// Use only in kernel modules or privileged contexts.
static inline uint64_t asm_read_midr(void) {
    uint64_t value;
    __asm__ __volatile__("mrs %0, midr_el1" : "=r"(value));
    return value;
}

#elif defined(__arm__)

// Get current stack pointer (ARM32)
static inline uint32_t asm_get_sp(void) {
    uint32_t sp;
    __asm__ __volatile__("mov %0, sp" : "=r"(sp));
    return sp;
}

// Get current program counter (ARM32)
static inline uint32_t asm_get_pc(void) {
    uint32_t pc;
    __asm__ __volatile__("mov %0, pc" : "=r"(pc));
    return pc;
}

// Get current link register (ARM32)
static inline uint32_t asm_get_lr(void) {
    uint32_t lr;
    __asm__ __volatile__("mov %0, lr" : "=r"(lr));
    return lr;
}

// Data memory barrier (ARM32)
static inline void asm_dmb(void) {
    __asm__ __volatile__("dmb" ::: "memory");
}

// Data synchronization barrier (ARM32)
static inline void asm_dsb(void) {
    __asm__ __volatile__("dsb" ::: "memory");
}

// Instruction synchronization barrier (ARM32)
static inline void asm_isb(void) {
    __asm__ __volatile__("isb" ::: "memory");
}

#endif // ARM/ARM64

// ============================================================================
// x86/x86_64 Architecture
// ============================================================================

#if defined(__x86_64__)

// Get current stack pointer (x86_64)
static inline uint64_t asm_get_sp(void) {
    uint64_t sp;
    __asm__ __volatile__("mov %%rsp, %0" : "=r"(sp));
    return sp;
}

// Get instruction pointer (x86_64) - using call trick
static inline uint64_t asm_get_pc(void) {
    uint64_t pc;
    __asm__ __volatile__(
        "call 1f\n"
        "1: pop %0\n"
        : "=r"(pc)
    );
    return pc;
}

// Memory fence
static inline void asm_mfence(void) {
    __asm__ __volatile__("mfence" ::: "memory");
}

// Store fence
static inline void asm_sfence(void) {
    __asm__ __volatile__("sfence" ::: "memory");
}

// Load fence
static inline void asm_lfence(void) {
    __asm__ __volatile__("lfence" ::: "memory");
}

// CPU ID instruction
static inline void asm_cpuid(uint32_t leaf, uint32_t* eax, uint32_t* ebx, 
                             uint32_t* ecx, uint32_t* edx) {
    __asm__ __volatile__(
        "cpuid"
        : "=a"(*eax), "=b"(*ebx), "=c"(*ecx), "=d"(*edx)
        : "a"(leaf)
    );
}

// Read timestamp counter
static inline uint64_t asm_rdtsc(void) {
    uint32_t low, high;
    __asm__ __volatile__("rdtsc" : "=a"(low), "=d"(high));
    return ((uint64_t)high << 32) | low;
}

#elif defined(__i386__)

// Get current stack pointer (x86)
static inline uint32_t asm_get_sp(void) {
    uint32_t sp;
    __asm__ __volatile__("mov %%esp, %0" : "=r"(sp));
    return sp;
}

// Get instruction pointer (x86)
static inline uint32_t asm_get_pc(void) {
    uint32_t pc;
    __asm__ __volatile__(
        "call 1f\n"
        "1: pop %0\n"
        : "=r"(pc)
    );
    return pc;
}

// Memory fence (x86)
static inline void asm_mfence(void) {
    __asm__ __volatile__("mfence" ::: "memory");
}

#endif // x86/x86_64

// ============================================================================
// RISC-V Architecture
// ============================================================================

#if defined(__riscv)

// Get current stack pointer (RISC-V)
static inline uint64_t asm_get_sp(void) {
    uint64_t sp;
    __asm__ __volatile__("mv %0, sp" : "=r"(sp));
    return sp;
}

// Get return address (RISC-V)
static inline uint64_t asm_get_ra(void) {
    uint64_t ra;
    __asm__ __volatile__("mv %0, ra" : "=r"(ra));
    return ra;
}

// Fence instruction (RISC-V)
static inline void asm_fence(void) {
    __asm__ __volatile__("fence rw, rw" ::: "memory");
}

// Fence instruction for I-cache (RISC-V)
static inline void asm_fence_i(void) {
    __asm__ __volatile__("fence.i" ::: "memory");
}

#endif // RISC-V

// ============================================================================
// Cross-platform atomic operations using GCC builtins
// ============================================================================

// Atomic compare and swap (pointer)
static inline int asm_atomic_cas_ptr(void** ptr, void* expected, void* desired) {
    return __sync_bool_compare_and_swap(ptr, expected, desired);
}

// Atomic compare and swap (32-bit)
static inline int asm_atomic_cas32(uint32_t* ptr, uint32_t expected, uint32_t desired) {
    return __sync_bool_compare_and_swap(ptr, expected, desired);
}

// Atomic compare and swap (64-bit)
static inline int asm_atomic_cas64(uint64_t* ptr, uint64_t expected, uint64_t desired) {
    return __sync_bool_compare_and_swap(ptr, expected, desired);
}

// Atomic add and fetch
static inline uint32_t asm_atomic_add32(uint32_t* ptr, uint32_t value) {
    return __sync_add_and_fetch(ptr, value);
}

// Atomic fetch and add
static inline uint32_t asm_atomic_fetch_add32(uint32_t* ptr, uint32_t value) {
    return __sync_fetch_and_add(ptr, value);
}

// Atomic exchange
static inline void* asm_atomic_exchange_ptr(void** ptr, void* value) {
    return __sync_lock_test_and_set(ptr, value);
}

// ============================================================================
// Breakpoint and debugging
// ============================================================================

// Software breakpoint (architecture-specific)
static inline void asm_breakpoint(void) {
#if defined(__aarch64__)
    __asm__ __volatile__("brk #0");
#elif defined(__arm__)
    __asm__ __volatile__("bkpt #0");
#elif defined(__x86_64__) || defined(__i386__)
    __asm__ __volatile__("int3");
#elif defined(__riscv)
    __asm__ __volatile__("ebreak");
#endif
}

// ============================================================================
// Cross-platform compiler barriers
// ============================================================================

// Compiler barrier (prevent reordering but no CPU fence)
static inline void asm_compiler_barrier(void) {
    __asm__ __volatile__("" ::: "memory");
}

// Full memory barrier (CPU + compiler)
static inline void asm_full_barrier(void) {
#if defined(__aarch64__) || defined(__arm__)
    __asm__ __volatile__("dmb sy" ::: "memory");
#elif defined(__x86_64__) || defined(__i386__)
    __asm__ __volatile__("mfence" ::: "memory");
#elif defined(__riscv)
    __asm__ __volatile__("fence rw, rw" ::: "memory");
#else
    __sync_synchronize();
#endif
}

#ifdef __cplusplus
}
#endif

#endif // ASM_UTILS_H
