/*
 * Low-level injection utilities for Zygisk
 * 
 * This file demonstrates using the low-level APIs for Android process injection.
 * It provides direct memory manipulation and hooking capabilities.
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
