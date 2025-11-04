/*
 * Low-level C API header for Android root injection
 * 
 * This header provides C function declarations for low-level operations.
 * For C++ code, use lowlevel.hpp instead.
 */

#ifndef LOWLEVEL_H
#define LOWLEVEL_H

#include <sys/types.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

// Direct syscall wrapper for open
long lowlevel_open(const char* pathname, int flags, int mode);

// Direct syscall wrapper for read
long lowlevel_read(int fd, void* buf, size_t count);

// Direct syscall wrapper for write
long lowlevel_write(int fd, const void* buf, size_t count);

// Direct syscall wrapper for close
long lowlevel_close(int fd);

// Direct syscall wrapper for mmap with error checking
void* lowlevel_mmap(void* addr, size_t length, int prot, int flags, int fd, off_t offset);

// Direct syscall wrapper for munmap
int lowlevel_munmap(void* addr, size_t length);

// Direct syscall wrapper for mprotect
int lowlevel_mprotect(void* addr, size_t len, int prot);

// Get page size using sysconf syscall
size_t lowlevel_getpagesize(void);

// Memory comparison at low level
int lowlevel_memcmp(const void* s1, const void* s2, size_t n);

// Memory copy at low level with volatile to prevent optimization
void lowlevel_memcpy(void* dest, const void* src, size_t n);

// Memory set at low level
void lowlevel_memset(void* s, int c, size_t n);

// String length at low level
size_t lowlevel_strlen(const char* s);

// Hook function by modifying memory directly
int lowlevel_hook_function(void* target_addr, void* hook_addr, void** original_addr);

// Read from process memory using process_vm_readv syscall
ssize_t lowlevel_read_process_memory(pid_t pid, void* local_addr, void* remote_addr, size_t size);

// Write to process memory using process_vm_writev syscall
ssize_t lowlevel_write_process_memory(pid_t pid, void* local_addr, void* remote_addr, size_t size);

// Get current process ID using syscall
pid_t lowlevel_getpid(void);

// Get current thread ID using syscall
pid_t lowlevel_gettid(void);

// Flush instruction cache after code modification
void lowlevel_flush_icache(void* addr, size_t len);

// Allocate executable memory for shellcode injection
void* lowlevel_alloc_executable(size_t size);

// Free executable memory
int lowlevel_free_executable(void* addr, size_t size);

#ifdef __cplusplus
}
#endif

#endif // LOWLEVEL_H
