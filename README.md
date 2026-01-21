# Slibbery - Shared Library Stub Generator

**Slibbery** (a pun on "shared lib foolery") generates drop-in replacement
shared libraries for Linux with function interception and mocking capabilities.

## Features

- **Header-based Parsing**: Extracts function signatures from C header files for accurate type mapping
- **Type-Aware Stubs**: Generates Rust code with proper C FFI types (`c_int`, `*mut c_int`, etc.)
- **Memory-Efficient Loading**: Uses `memfd_create` to load libraries entirely in memory
- **Zero-Copy Forwarding**: Stubs forward calls to original functions without copying data
- **Easy Mocking**: Generated stubs include placeholders for custom mock logic

## Usage

Generate a stub library by providing the shared library and its header file:

```bash
cargo run --release -- <input.so> <output_dir> <header.h>
cd <output_dir>
cargo build --release
LD_PRELOAD=./target/release/libmock_lib.so ./your_program
```

### Example

```bash
# Generate stubs for NVIDIA NVML
cargo run --release -- cuda/lib64/libnvidia-ml.so nvml_stubs cuda/include/nvml.h
cd nvml_stubs
cargo build --release
```
