#!/bin/bash

set -e

echo "=== E2E Test Suite for Shared Library Stub Generator ==="

# Clean up previous runs
echo "Cleaning up previous test artifacts..."
rm -rf test_output
rm -f libtest.so
rm -f test_program*

# Build the stub generator
echo "Building stub generator..."
cd ..
cargo build --release
cd tests

# Create and compile test library
echo "Creating and compiling test shared library..."
cat > test_lib.c << 'EOF'
#include <stdio.h>

int add(int a, int b) {
    return a + b;
}

void print_hello() {
    printf("Hello from original library!\n");
}

int add_and_double(int a, int b) {
    int sum = add(a, b);
    return sum * 2;
}
EOF
gcc -shared -fPIC -g -o libtest.so test_lib.c

# Generate stub library
echo "Generating stub library..."
../target/release/slibbery libtest.so test_output test_lib.h

# Build the stub library
echo "Building stub library..."
cd test_output
cargo build --release
cd ..

# Create test programs
echo "Creating test programs..."

# Test program using dlopen/dlsym
cat > test_program_dlopen.c << 'EOF'
#include <stdio.h>
#include <dlfcn.h>

typedef int (*add_func)(int, int);
typedef void (*print_hello_func)();
typedef int (*add_and_double_func)(int, int);

int main() {
    void *handle = dlopen("./test_output/target/release/libmock_lib.so", RTLD_LAZY);
    if (!handle) {
        fprintf(stderr, "Failed to load library: %s\n", dlerror());
        return 1;
    }

    add_func add = (add_func)dlsym(handle, "add");
    if (!add) {
        fprintf(stderr, "Failed to find add function: %s\n", dlerror());
        return 1;
    }

    print_hello_func print_hello = (print_hello_func)dlsym(handle, "print_hello");
    if (!print_hello) {
        fprintf(stderr, "Failed to find print_hello function: %s\n", dlerror());
        return 1;
    }

    add_and_double_func add_and_double = (add_and_double_func)dlsym(handle, "add_and_double");
    if (!add_and_double) {
        fprintf(stderr, "Failed to find add_and_double function: %s\n", dlerror());
        return 1;
    }

    int result_add = add(5, 3);
    printf("dlopen: Testing add(5, 3) = %d\n", result_add);
    if (result_add != 8) {
        fprintf(stderr, "add(5, 3) failed: expected 8, got %d\n", result_add);
        return 1;
    }

    printf("dlopen: Calling print_hello:\n");
    print_hello();

    int result_double = add_and_double(5, 3);
    printf("dlopen: Testing add_and_double(5, 3) = %d\n", result_double);
    if (result_double != 16) {
        fprintf(stderr, "add_and_double(5, 3) failed: expected 16, got %d\n", result_double);
        return 1;
    }

    dlclose(handle);
    return 0;
}
EOF

# Test program using LD_PRELOAD (direct calls)
cat > test_program_preload.c << 'EOF'
#include <stdio.h>

// Declare functions (will be resolved by LD_PRELOAD)
extern int add(int a, int b);
extern void print_hello();
extern int add_and_double(int a, int b);

int main() {
    int result_add = add(5, 3);
    printf("LD_PRELOAD: Testing add(5, 3) = %d\n", result_add);
    if (result_add != 8) {
        fprintf(stderr, "add(5, 3) failed: expected 8, got %d\n", result_add);
        return 1;
    }

    printf("LD_PRELOAD: Calling print_hello:\n");
    print_hello();

    int result_double = add_and_double(5, 3);
    printf("LD_PRELOAD: Testing add_and_double(5, 3) = %d\n", result_double);
    if (result_double != 16) {
        fprintf(stderr, "add_and_double(5, 3) failed: expected 16, got %d\n", result_double);
        return 1;
    }

    return 0;
}
EOF

# Compile and run test programs
echo "Compiling test programs..."
gcc -o test_program_dlopen test_program_dlopen.c -ldl
gcc -o test_program_preload test_program_preload.c -lc -Wl,--unresolved-symbols=ignore-all

echo "Running test program with dlopen..."
./test_program_dlopen

echo "Running test program with LD_PRELOAD..."
LD_PRELOAD=$(pwd)/test_output/target/release/libmock_lib.so ./test_program_preload

echo "=== Test completed successfully! ==="

# Clean up
echo "Cleaning up..."
rm -rf test_output
rm -f libtest.so
rm -f test_program*
rm -f *.c
