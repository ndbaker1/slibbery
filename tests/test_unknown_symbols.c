#include <dlfcn.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// Test library with multiple functions
int test_add(int a, int b) { return a + b; }
int test_subtract(int a, int b) { return a - b; }
int test_multiply(int a, int b) { return a * b; }
int test_divide(int a, int b) { return a / b; }

// Function signature for dyll_resolve_symbol
typedef void* (*symbol_resolver)(const char*);

// Helper macro to make symbol resolution transparent
#define RESOLVE_SYMBOL(handle, name) \
    ({ \
        static symbol_resolver resolver = NULL; \
        if (!resolver) { \
            resolver = (symbol_resolver)dlsym(handle, "dyll_resolve_symbol"); \
            if (!resolver) { \
                fprintf(stderr, "Failed to find dyll_resolve_symbol\n"); \
                exit(1); \
            } \
        } \
        resolver(name); \
    })

#define GET_FUNCTION(handle, name, type) \
    ((type)RESOLVE_SYMBOL(handle, name))

int main(int argc, char* argv[]) {
    if (argc < 2) {
        fprintf(stderr, "Usage: %s <stub_library.so>\n", argv[0]);
        return 1;
    }

    const char* stub_lib = argv[1];

    // Load the stub library
    printf("Loading stub library: %s\n", stub_lib);
    void* handle = dlopen(stub_lib, RTLD_LAZY | RTLD_GLOBAL);
    if (!handle) {
        fprintf(stderr, "Failed to dlopen %s: %s\n", stub_lib, dlerror());
        return 1;
    }
    printf("Stub library loaded successfully\n");

    // Test functions that should be known (have stubs generated)
    typedef int (*binary_op)(int, int);

    printf("Looking for test_add...\n");
    binary_op add_func = (binary_op)dlsym(handle, "test_add");
    if (!add_func) {
        fprintf(stderr, "Failed to find test_add: %s\n", dlerror());
        dlclose(handle);
        return 1;
    }
    printf("Found test_add\n");

    // Test unknown functions using standard dlsym (should work with linking approach)
    printf("Looking for test_subtract...\n");
    binary_op subtract_func = (binary_op)dlsym(handle, "test_subtract");
    if (!subtract_func) {
        fprintf(stderr, "Failed to find test_subtract: %s\n", dlerror());
        dlclose(handle);
        return 1;
    }
    printf("Found test_subtract\n");

    // Test that the functions work correctly
    printf("Testing known symbol (test_add): 5 + 3 = %d\n", add_func(5, 3));

    // Verify results
    if (add_func(5, 3) != 8) {
        fprintf(stderr, "test_add failed\n");
        dlclose(handle);
        return 1;
    }

    printf("Testing unknown symbol (test_subtract): 10 - 4 = %d\n", subtract_func(10, 4));

    if (subtract_func(10, 4) != 6) {
        fprintf(stderr, "test_subtract failed: got %d, expected 6\n", subtract_func(10, 4));
        dlclose(handle);
        return 1;
    }

    printf("All symbol tests passed!\n");

    dlclose(handle);
    return 0;
}