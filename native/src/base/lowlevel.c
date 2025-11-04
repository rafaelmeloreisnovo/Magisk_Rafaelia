/*
 * Low-level C implementations for Android root injection
 * 
 * This file contains pure C implementations using direct syscalls and memory
 * manipulation. These functions are designed for critical injection points
 * where we need minimal dependencies and direct hardware access.
 */

#include <sys/syscall.h>
#include <unistd.h>
#include <stdint.h>
#include <sys/mman.h>
#include <sys/uio.h>
#include <fcntl.h>
#include <errno.h>
#include <string.h>

// Define AT_FDCWD if not already defined
#ifndef AT_FDCWD
#define AT_FDCWD -100
#endif

// Direct syscall wrapper for open
long lowlevel_open(const char* pathname, int flags, int mode) {
    return syscall(__NR_openat, AT_FDCWD, pathname, flags, mode);
}

// Direct syscall wrapper for read
long lowlevel_read(int fd, void* buf, size_t count) {
    return syscall(__NR_read, fd, buf, count);
}

// Direct syscall wrapper for write
long lowlevel_write(int fd, const void* buf, size_t count) {
    return syscall(__NR_write, fd, buf, count);
}

// Direct syscall wrapper for close
long lowlevel_close(int fd) {
    return syscall(__NR_close, fd);
}

// Direct syscall wrapper for mmap with error checking
void* lowlevel_mmap(void* addr, size_t length, int prot, int flags, int fd, off_t offset) {
    long result = syscall(__NR_mmap, addr, length, prot, flags, fd, offset);
    // Check if result is an error (small negative number, typically -1 to -4095)
    if ((unsigned long)result >= (unsigned long)-4095UL) {
        errno = -(int)result;
        return MAP_FAILED;
    }
    return (void*)result;
}

// Direct syscall wrapper for munmap
int lowlevel_munmap(void* addr, size_t length) {
    long result = syscall(__NR_munmap, addr, length);
    if (result < 0) {
        errno = -result;
        return -1;
    }
    return 0;
}

// Direct syscall wrapper for mprotect
int lowlevel_mprotect(void* addr, size_t len, int prot) {
    long result = syscall(__NR_mprotect, addr, len, prot);
    if (result < 0) {
        errno = -result;
        return -1;
    }
    return 0;
}

// Get page size using sysconf syscall
size_t lowlevel_getpagesize(void) {
    return (size_t)sysconf(_SC_PAGESIZE);
}

// Memory comparison at low level
int lowlevel_memcmp(const void* s1, const void* s2, size_t n) {
    const unsigned char* p1 = (const unsigned char*)s1;
    const unsigned char* p2 = (const unsigned char*)s2;
    
    for (size_t i = 0; i < n; i++) {
        if (p1[i] != p2[i]) {
            return p1[i] - p2[i];
        }
    }
    return 0;
}

// Memory copy at low level with volatile to prevent optimization
void lowlevel_memcpy(void* dest, const void* src, size_t n) {
    volatile unsigned char* d = (volatile unsigned char*)dest;
    const volatile unsigned char* s = (const volatile unsigned char*)src;
    
    for (size_t i = 0; i < n; i++) {
        d[i] = s[i];
    }
}

// Memory set at low level
void lowlevel_memset(void* s, int c, size_t n) {
    volatile unsigned char* p = (volatile unsigned char*)s;
    
    for (size_t i = 0; i < n; i++) {
        p[i] = (unsigned char)c;
    }
}

// String length at low level
size_t lowlevel_strlen(const char* s) {
    const char* p = s;
    while (*p) {
        p++;
    }
    return (size_t)(p - s);
}

// Hook function by modifying memory directly
// This is used for PLT/GOT hooking in injection code
int lowlevel_hook_function(void* target_addr, void* hook_addr, void** original_addr) {
    size_t page_size = lowlevel_getpagesize();
    void* page = (void*)((uintptr_t)target_addr & ~(page_size - 1));
    
    // Make memory writable
    if (lowlevel_mprotect(page, page_size, PROT_READ | PROT_WRITE | PROT_EXEC) != 0) {
        return -1;
    }
    
    // Save original value
    if (original_addr) {
        *original_addr = *(void**)target_addr;
    }
    
    // Write hook address using volatile pointer
    volatile void** target_ptr = (volatile void**)target_addr;
    *target_ptr = hook_addr;
    
    // Memory barrier
#if defined(__aarch64__) || defined(__arm__)
    __asm__ __volatile__("dmb sy" ::: "memory");
#elif defined(__x86_64__) || defined(__i386__)
    __asm__ __volatile__("mfence" ::: "memory");
#elif defined(__riscv)
    __asm__ __volatile__("fence rw,rw" ::: "memory");
#else
    __sync_synchronize();
#endif
    
    // Restore memory protection
    lowlevel_mprotect(page, page_size, PROT_READ | PROT_EXEC);
    
    return 0;
}

// Read from process memory using process_vm_readv syscall
ssize_t lowlevel_read_process_memory(pid_t pid, void* local_addr, void* remote_addr, size_t size) {
    struct iovec local;
    struct iovec remote;
    
    local.iov_base = local_addr;
    local.iov_len = size;
    remote.iov_base = remote_addr;
    remote.iov_len = size;
    
    return syscall(__NR_process_vm_readv, pid, &local, 1UL, &remote, 1UL, 0UL);
}

// Write to process memory using process_vm_writev syscall
ssize_t lowlevel_write_process_memory(pid_t pid, void* local_addr, void* remote_addr, size_t size) {
    struct iovec local;
    struct iovec remote;
    
    local.iov_base = local_addr;
    local.iov_len = size;
    remote.iov_base = remote_addr;
    remote.iov_len = size;
    
    return syscall(__NR_process_vm_writev, pid, &local, 1UL, &remote, 1UL, 0UL);
}

// Get current process ID using syscall
pid_t lowlevel_getpid(void) {
    return (pid_t)syscall(__NR_getpid);
}

// Get current thread ID using syscall
pid_t lowlevel_gettid(void) {
    return (pid_t)syscall(__NR_gettid);
}

// Flush instruction cache after code modification
void lowlevel_flush_icache(void* addr, size_t len) {
#if defined(__aarch64__) || defined(__arm__)
    uintptr_t start = (uintptr_t)addr;
    uintptr_t end = start + len;
    
    // Clean data cache
    for (uintptr_t p = start; p < end; p += 64) {
        __asm__ __volatile__("dc cvau, %0" : : "r"(p) : "memory");
    }
    
    __asm__ __volatile__("dsb ish" ::: "memory");
    
    // Invalidate instruction cache
    for (uintptr_t p = start; p < end; p += 64) {
        __asm__ __volatile__("ic ivau, %0" : : "r"(p) : "memory");
    }
    
    __asm__ __volatile__("dsb ish" ::: "memory");
    __asm__ __volatile__("isb" ::: "memory");
#elif defined(__x86_64__) || defined(__i386__)
    // x86 has coherent I-cache - no flush needed
    (void)addr;
    (void)len;
    __asm__ __volatile__("" ::: "memory");
#elif defined(__riscv)
    (void)addr;
    (void)len;
    __asm__ __volatile__("fence.i" ::: "memory");
#else
    // For other architectures, try cacheflush if available
    (void)addr;
    (void)len;
#ifdef __NR_cacheflush
    syscall(__NR_cacheflush, addr, len, 0);
#endif
#endif
}

// Allocate executable memory for shellcode injection
void* lowlevel_alloc_executable(size_t size) {
    void* mem = lowlevel_mmap(NULL, size, PROT_READ | PROT_WRITE | PROT_EXEC,
                               MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
    if (mem == MAP_FAILED) {
        return NULL;
    }
    return mem;
}

// Free executable memory
int lowlevel_free_executable(void* addr, size_t size) {
    return lowlevel_munmap(addr, size);
}
