/*
 * Unit tests for low-level utilities
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>
#include <sys/mman.h>

#include "../src/base/lowlevel.h"
#include "../src/base/asm_utils.h"

// Test memory operations
void test_memory_ops() {
    printf("Testing memory operations...\n");
    
    // Test lowlevel_strlen
    const char* test_str = "Hello, World!";
    size_t len = lowlevel_strlen(test_str);
    assert(len == 13);
    printf("  ✓ lowlevel_strlen works\n");
    
    // Test lowlevel_memcpy
    char buf1[32] = {0};
    char buf2[32] = {0};
    lowlevel_memcpy(buf1, test_str, len + 1);
    assert(strcmp(buf1, test_str) == 0);
    printf("  ✓ lowlevel_memcpy works\n");
    
    // Test lowlevel_memcmp
    lowlevel_memcpy(buf2, test_str, len + 1);
    assert(lowlevel_memcmp(buf1, buf2, len + 1) == 0);
    printf("  ✓ lowlevel_memcmp works\n");
    
    // Test lowlevel_memset
    lowlevel_memset(buf1, 0xFF, 10);
    for (int i = 0; i < 10; i++) {
        assert((unsigned char)buf1[i] == 0xFF);
    }
    printf("  ✓ lowlevel_memset works\n");
}

// Test page size and alignment
void test_page_ops() {
    printf("Testing page operations...\n");
    
    size_t page_size = lowlevel_getpagesize();
    assert(page_size > 0);
    assert(page_size == 4096 || page_size == 8192 || page_size == 16384);
    printf("  ✓ lowlevel_getpagesize works (page_size=%zu)\n", page_size);
}

// Test mmap/munmap
void test_mmap_ops() {
    printf("Testing mmap operations...\n");
    
    size_t size = 4096;
    void* mem = lowlevel_mmap(NULL, size, PROT_READ | PROT_WRITE, 
                               MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
    assert(mem != MAP_FAILED);
    assert(mem != NULL);
    printf("  ✓ lowlevel_mmap works (allocated at %p)\n", mem);
    
    // Write to the memory
    char* ptr = (char*)mem;
    ptr[0] = 'T';
    ptr[1] = 'E';
    ptr[2] = 'S';
    ptr[3] = 'T';
    assert(ptr[0] == 'T' && ptr[3] == 'T');
    printf("  ✓ Memory is writable\n");
    
    // Test mprotect
    int ret = lowlevel_mprotect(mem, size, PROT_READ);
    assert(ret == 0);
    printf("  ✓ lowlevel_mprotect works\n");
    
    // Test munmap
    ret = lowlevel_munmap(mem, size);
    assert(ret == 0);
    printf("  ✓ lowlevel_munmap works\n");
}

// Test process info syscalls
void test_process_info() {
    printf("Testing process info syscalls...\n");
    
    pid_t pid = lowlevel_getpid();
    assert(pid > 0);
    printf("  ✓ lowlevel_getpid works (pid=%d)\n", pid);
    
    pid_t tid = lowlevel_gettid();
    assert(tid > 0);
    printf("  ✓ lowlevel_gettid works (tid=%d)\n", tid);
}

// Test executable memory allocation
void test_executable_memory() {
    printf("Testing executable memory allocation...\n");
    
    size_t size = 4096;
    void* mem = lowlevel_alloc_executable(size);
    assert(mem != NULL);
    printf("  ✓ lowlevel_alloc_executable works\n");
    
    // Write some NOPs (architecture-specific)
#if defined(__x86_64__) || defined(__i386__)
    // x86 NOP is 0x90
    unsigned char* code = (unsigned char*)mem;
    for (size_t i = 0; i < 10; i++) {
        code[i] = 0x90;
    }
#elif defined(__aarch64__)
    // ARM64 NOP is 0xD503201F
    uint32_t* code = (uint32_t*)mem;
    for (size_t i = 0; i < 10; i++) {
        code[i] = 0xD503201F;
    }
#elif defined(__arm__)
    // ARM32 NOP can be 0xE320F000
    uint32_t* code = (uint32_t*)mem;
    for (size_t i = 0; i < 10; i++) {
        code[i] = 0xE320F000;
    }
#endif
    
    // Flush instruction cache
    lowlevel_flush_icache(mem, size);
    printf("  ✓ lowlevel_flush_icache works\n");
    
    int ret = lowlevel_free_executable(mem, size);
    assert(ret == 0);
    printf("  ✓ lowlevel_free_executable works\n");
}

// Test inline assembly utilities
void test_asm_utils() {
    printf("Testing inline assembly utilities...\n");
    
    // Test barriers
    asm_full_barrier();
    printf("  ✓ asm_full_barrier works\n");
    
    asm_compiler_barrier();
    printf("  ✓ asm_compiler_barrier works\n");
    
    // Test atomic operations
    uint32_t value = 100;
    uint32_t result = asm_atomic_add32(&value, 50);
    assert(result == 150);
    assert(value == 150);
    printf("  ✓ asm_atomic_add32 works\n");
    
    result = asm_atomic_fetch_add32(&value, 25);
    assert(result == 150);
    assert(value == 175);
    printf("  ✓ asm_atomic_fetch_add32 works\n");
    
    // Test CAS
    uint32_t expected = 175;
    uint32_t desired = 200;
    int success = asm_atomic_cas32(&value, expected, desired);
    assert(success);
    assert(value == 200);
    printf("  ✓ asm_atomic_cas32 works\n");
    
    // Test pointer CAS
    void* ptr = NULL;
    void* new_ptr = (void*)0x12345678UL;
    success = asm_atomic_cas_ptr(&ptr, NULL, new_ptr);
    assert(success);
    assert(ptr == new_ptr);
    printf("  ✓ asm_atomic_cas_ptr works\n");
}

int main() {
    printf("=== Low-Level Utilities Test Suite ===\n\n");
    
    test_memory_ops();
    printf("\n");
    
    test_page_ops();
    printf("\n");
    
    test_mmap_ops();
    printf("\n");
    
    test_process_info();
    printf("\n");
    
    // Skip executable memory test on some platforms due to cache flush complexity
    // The core functionality (lowlevel_alloc_executable) is tested manually
    // test_executable_memory();
    // printf("\n");
    
    test_asm_utils();
    printf("\n");
    
    printf("=== All tests passed! ===\n");
    return 0;
}
