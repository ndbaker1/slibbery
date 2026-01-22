# Dyll - Shared Library Stub Generator

**Dyll** 🥒 generates drop-in replacement shared libraries for Linux with
function interception and mocking capabilities.

## Features

- **Header-based Parsing**: Extracts function signatures from C header files for accurate type mapping
- **Type-Aware Stubs**: Generates Rust code with proper C FFI types (`c_int`, `*mut c_int`, etc.)
- **Memory-Efficient Loading**: Uses `memfd_create` to load libraries entirely in memory
- **Zero-Copy Forwarding**: Stubs forward calls to original functions without copying data
- **Easy Mocking**: Generated stubs include placeholders for custom mock logic

## Usage

Generate a stub library using the `header` subcommand:

```bash
cargo run --release -- --output-dir <output_dir> --lib-path <input.so> header <header.h>
cd <output_dir>
cargo build --release
LD_PRELOAD=./target/release/libmock_lib.so ./your_program
```

### CLI Options

- `--output-dir, -o`: Output directory for the generated stub library
- `--lib-path, -l`: Path to the input .so library
- `header`: Subcommand to generate stubs from a C header file

#### Getting Help

```bash
# Show global help
cargo run --release -- --help

# Show help for the header subcommand
cargo run --release -- header --help
```

### Example

```bash
# Generate stubs for NVIDIA NVML
cargo run --release -- --output-dir nvml_stubs --lib-path cuda/lib64/libnvidia-ml.so header cuda/include/nvml.h
cd nvml_stubs
cargo build --release
```
