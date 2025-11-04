# Low-Level Programming Guide for Magisk_Rafaelia

## Overview

This document describes the low-level programming interfaces added to Magisk_Rafaelia for direct memory access, syscall wrappers, and architecture-specific operations. These are designed for critical Android injection code where we need minimal dependencies and maximum control.

## Purpose

The low-level APIs provide:

1. **Direct syscall access** - Bypass libc wrappers for critical operations
2. **Memory manipulation** - Direct memory read/write with address control
3. **Inline assembly** - Architecture-specific CPU operations
4. **Cache control** - Instruction cache flushing for code injection
5. **Atomic operations** - Thread-safe memory operations for hooking

## Files

- `native/src/base/lowlevel.hpp` - C++ low-level utilities header
- `native/src/base/lowlevel.c` - C low-level implementations
- `native/src/base/asm_utils.h` - Inline assembly utilities

## Usage Examples

### 1. Direct Memory Access

```cpp
#include "lowlevel.hpp"

// Read 32-bit value from specific memory address
uintptr_t addr = 0x12345678;
uint32_t value = lowlevel::mem_read32(addr);

// Write to memory
lowlevel::mem_write32(addr, 0xDEADBEEF);

// Memory barrier to ensure write ordering
lowlevel::memory_barrier();
```

### 2. Direct Syscalls

```c
#include "lowlevel.h"

// Allocate memory with direct syscall
void* mem = lowlevel_mmap(NULL, 4096, 
                          PROT_READ | PROT_WRITE | PROT_EXEC,
                          MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);

// Write to another process
lowlevel_write_process_memory(target_pid, local_buf, remote_addr, size);

// Read from another process
lowlevel_read_process_memory(target_pid, local_buf, remote_addr, size);
```

### 3. Function Hooking

```c
void* original_func = NULL;

// Hook a function by modifying GOT/PLT entry
int result = lowlevel_hook_function(
    &target_function_ptr,  // Address to hook
    my_hook_function,      // Hook function
    &original_func         // Store original
);

// In hook, can call original
if (original_func) {
    ((void(*)())original_func)();
}
```

### 4. Instruction Cache Management

```cpp
#include "lowlevel.hpp"

// After modifying code in memory
void* code_addr = get_function_address();
size_t code_size = get_function_size();

// Flush instruction cache so CPU sees new code
lowlevel::icache_flush(code_addr, code_size);
```

### 5. Architecture-Specific Operations

```c
#include "asm_utils.h"

// Get current stack pointer (works on ARM64, x86_64, etc.)
uint64_t sp = asm_get_sp();

// Memory barriers for synchronization
asm_dmb();  // Data Memory Barrier (ARM)
asm_dsb();  // Data Synchronization Barrier (ARM)
asm_isb();  // Instruction Synchronization Barrier (ARM)

// On x86_64
asm_mfence();  // Memory Fence
```

### 6. Atomic Operations for Hooking

```c
#include "asm_utils.h"

void* expected = original_ptr;
void* desired = hook_ptr;

// Atomic compare-and-swap
if (asm_atomic_cas_ptr(&function_ptr, expected, desired)) {
    // Hook installed successfully
}
```

### 7. Process Memory Operations

```c
// Read memory from target process
pid_t target_pid = 12345;
char buffer[256];
ssize_t bytes_read = lowlevel_read_process_memory(
    target_pid, buffer, (void*)0x12340000, sizeof(buffer)
);

// Write to target process
const char* payload = "injected code";
ssize_t bytes_written = lowlevel_write_process_memory(
    target_pid, (void*)payload, (void*)0x12340000, strlen(payload)
);
```

## Architecture Support

The low-level APIs support multiple architectures:

### ARM64 (aarch64)
- Data/instruction cache operations (`DC CVAU`, `IC IVAU`)
- Memory barriers (`DMB`, `DSB`, `ISB`)
- System register access
- Full atomic operation support

### ARM32 (arm)
- Cache operations
- Memory barriers
- Atomic operations

### x86_64
- Memory fences (`MFENCE`, `SFENCE`, `LFENCE`)
- CPU identification (`CPUID`)
- Timestamp counter (`RDTSC`)
- Atomic operations

### x86 (i386)
- Memory fences
- Atomic operations

### RISC-V
- Fence instructions
- Instruction cache fence (`FENCE.I`)
- Atomic operations

## Use Cases in Magisk_Rafaelia

### 1. Zygisk Injection
- PLT/GOT hooking for function interception
- Code injection into Zygote process
- Memory protection manipulation

```cpp
// In zygisk/hook.cpp
#include "lowlevel.hpp"

void hook_plt_entry(void* plt_entry, void* hook_func) {
    // Make PLT writable
    void* page = lowlevel::page_align_down(plt_entry);
    lowlevel::sys_mprotect(page, 4096, PROT_READ | PROT_WRITE);
    
    // Install hook atomically
    lowlevel::mem_write64((uintptr_t)plt_entry, (uint64_t)hook_func);
    lowlevel::memory_barrier();
    
    // Restore protection
    lowlevel::sys_mprotect(page, 4096, PROT_READ);
}
```

### 2. Init/Preload Operations
- SELinux policy injection
- Early boot modifications
- File descriptor manipulation

```c
// In init/preload.c
#include "lowlevel.h"

void inject_policy_lowlevel(void* policy_data, size_t len) {
    // Use direct syscalls to avoid libc dependencies
    int fd = lowlevel_open("/sys/fs/selinux/load", O_WRONLY, 0);
    lowlevel_write(fd, policy_data, len);
    lowlevel_close(fd);
}
```

### 3. SU Privilege Escalation
- Process credential manipulation
- Capability setting
- Namespace operations

```cpp
// In core/su/su.cpp
#include "lowlevel.hpp"

void set_process_credentials() {
    // Use direct prctl syscall
    lowlevel::sys_prctl(PR_SET_KEEPCAPS, 1, 0, 0, 0);
    
    // Other privilege operations...
}
```

## Performance Considerations

### When to Use Low-Level APIs

✅ **Use for:**
- Critical injection code paths
- PLT/GOT hooking
- Process memory manipulation
- Code modification at runtime
- Early boot operations
- Minimal dependency contexts

❌ **Don't use for:**
- Regular application logic
- High-level operations
- Code where maintainability > performance
- Non-critical paths

### Optimization Tips

1. **Cache Management**: Only flush I-cache when absolutely necessary
2. **Atomic Operations**: Use appropriate memory ordering (relaxed, acquire, release)
3. **Page Alignment**: Always align memory operations to page boundaries
4. **Syscall Batching**: Batch multiple syscalls when possible

## Safety and Security

### Memory Safety
- Always validate addresses before access
- Check return values from syscalls
- Use page alignment functions
- Handle MAP_FAILED and error codes

### Thread Safety
- Use atomic operations for shared memory
- Use proper memory barriers
- Consider race conditions in hooks

### SELinux Considerations
- Some syscalls may be restricted by SELinux
- Test on enforcing mode
- May need policy adjustments

## Testing

### Unit Tests
```cpp
// Test memory operations
void test_mem_operations() {
    uint32_t test_var = 0x12345678;
    uintptr_t addr = (uintptr_t)&test_var;
    
    assert(lowlevel::mem_read32(addr) == 0x12345678);
    
    lowlevel::mem_write32(addr, 0xDEADBEEF);
    assert(test_var == 0xDEADBEEF);
}
```

### Integration Tests
- Test on actual Android devices
- Verify with different Android versions
- Test across all supported architectures

## Build Integration

These low-level files are automatically included in the build:

```cmake
# Already integrated in native/CMakeLists.txt
add_library(base
    src/base/lowlevel.c
    src/base/base.cpp
    # ... other files
)
```

## References

### ARM Documentation
- ARM Architecture Reference Manual
- ARM Cortex-A Series Programmer's Guide

### x86 Documentation
- Intel 64 and IA-32 Architectures Software Developer Manuals
- AMD64 Architecture Programmer's Manual

### Linux Kernel
- Linux syscall interface
- `/usr/include/asm/unistd.h` - Syscall numbers
- `/usr/include/sys/syscall.h` - Syscall wrappers

### Android Specific
- Android NDK documentation
- Bionic libc source code
- Android SELinux policy

## Migration Guide

### Converting High-Level to Low-Level

**Before:**
```cpp
void* mem = mmap(NULL, size, prot, flags, -1, 0);
```

**After:**
```cpp
void* mem = lowlevel::sys_mmap(NULL, size, prot, flags, -1, 0);
// or
void* mem = lowlevel_mmap(NULL, size, prot, flags, -1, 0);
```

**Before:**
```cpp
memcpy(dest, src, n);
```

**After (for critical paths):**
```cpp
lowlevel::mem_copy_safe(dest, src, n);
```

## Troubleshooting

### Common Issues

1. **Segmentation Fault**
   - Check page alignment
   - Verify memory protection flags
   - Ensure address is valid

2. **Hook Not Working**
   - Flush instruction cache
   - Check if memory is writable
   - Verify atomic operations

3. **Syscall Returns -1**
   - Check errno
   - Verify syscall availability on Android version
   - Check SELinux denials

### Debug Tools

```c
// Enable debug output
#ifdef DEBUG_LOWLEVEL
#define LL_DEBUG(fmt, ...) \
    lowlevel_write(2, fmt "\n", sizeof(fmt))
#else
#define LL_DEBUG(fmt, ...)
#endif
```

## Future Enhancements

Planned additions:
- [ ] eBPF integration for kernel-level hooks
- [ ] Hardware breakpoint support
- [ ] Performance counter access
- [ ] More architecture-specific optimizations
- [ ] Seccomp filter manipulation

## License

These low-level utilities are part of Magisk_Rafaelia and are licensed under GPL v3.

---

**Note**: Low-level programming requires careful consideration of architecture differences, memory safety, and security implications. Always test thoroughly on target platforms.
