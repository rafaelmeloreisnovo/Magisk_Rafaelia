#pragma once

// Low-level memory and syscall utilities for Android injection
// This provides direct memory access and syscall interfaces for critical operations
// Used by: zygisk injection, init preload, su privilege escalation

#include <sys/syscall.h>
#include <unistd.h>
#include <stdint.h>
#include <sys/mman.h>

// Direct syscall wrappers to bypass libc
// These are critical for injection where we need to avoid library dependencies

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
    return reinterpret_cast<void*>(syscall(__NR_mmap, addr, length, prot, flags, fd, offset));
}

// Direct syscall for munmap
static inline int sys_munmap(void* addr, size_t length) {
    return static_cast<int>(syscall(__NR_munmap, addr, length));
}

// Direct syscall for mprotect - critical for code injection
static inline int sys_mprotect(void* addr, size_t len, int prot) {
    return static_cast<int>(syscall(__NR_mprotect, addr, len, prot));
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
