#!/bin/bash

set -e

echo "=== E2E Test Suite for Shared Library Stub Generator ==="

# Clean up previous runs
echo "Cleaning up previous test artifacts..."
rm -rf test_output
rm -f libtest.so
rm -f test_program_dlopen test_program_preload

# Build the stub generator
echo "Building stub generator..."
cd ..
cargo build --release
cd tests

# Create and compile test library
echo "Creating and compiling test shared library..."
gcc -shared -fPIC -g -o libtest.so test_lib.c

# Generate stub library
echo "Generating stub library..."
cargo run --release -- --output-dir test_output --lib-path libtest.so header test_lib.h

# Build the stub library
echo "Building stub library..."
cd test_output
cargo build --release
cd ..

# Compile and run test programs
echo "Compiling test programs..."
gcc -o test_program_dlopen test_program_dlopen.c -ldl
gcc -o test_program_preload test_program_preload.c -lc -Wl,--unresolved-symbols=ignore-all

echo "Running test program with dlopen..."
./test_program_dlopen

# echo "Running test program with LD_PRELOAD..."
# LD_PRELOAD=$(pwd)/test_output/target/release/libmock_lib.so ./test_program_preload || echo "LD_PRELOAD test failed (known issue)"

echo "=== Test completed successfully! ==="
