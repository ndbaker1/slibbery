# Slibbery - Shared Library Stub Generator

**Slibbery** (a pun on "shared lib foolery") is a tool that generates drop-in
replacement shared libraries (.so files) for Linux that can intercept and mock
function calls while forwarding others to the original library.

## Features

- **Header-based Parsing**: Extracts function signatures from C header files for accurate type mapping
- **Type-Aware Stubs**: Generates Rust code with proper C FFI types (`c_int`, `*mut c_int`, etc.)
- **Memory-Efficient Loading**: Uses `memfd_create` to load libraries entirely in memory
- **Zero-Copy Forwarding**: Stubs forward calls to original functions without copying data
- **Easy Mocking**: Generated stubs include placeholders for custom mock logic

## Usage

Generate a stub library by providing the shared library and its header file:

```bash
./target/release/slibbery /path/to/libexample.so ./output_dir /path/to/libexample.h
cd output_dir
cargo build --release
LD_PRELOAD=./target/release/libmock_lib.so ./your_program
```

### Options

- `<input.so>`: Path to the shared library to stub
- `<output_dir>`: Directory where the Rust project will be generated
- `<header.h>`: C header file containing function declarations
- `--lib-path <path>`: Override the path used for embedding the original library (optional)

### Example

```bash
# Generate stubs for NVIDIA NVML
./target/release/slibbery cuda/lib64/libnvidia-ml.so nvml_stubs cuda/include/nvml.h
cd nvml_stubs
cargo build --release
```

The generated Rust code will include type-safe function stubs like:

```rust
#[no_mangle]
pub extern "C" fn nvmlInit_v2() -> c_int {
    unsafe {
        // Add your mock logic here
        
        // Forward to original
        let orig: extern "C" fn() -> c_int = std::mem::transmute(get_symbol(b"nvmlInit_v2\0"));
        orig()
    }
}
```

## Why Header Files?

Slibbery uses C header files instead of DWARF debug symbols because:

- **Reliability**: Header files provide canonical function signatures
- **Type Safety**: Proper C-to-Rust type mapping (enums, pointers, structs)
- **Portability**: Works with any compiled shared library
- **Clarity**: Easy to understand and modify generated code

## Testing

Run the end-to-end test suite:

```bash
cd tests
bash test.sh
```

This tests both dlopen-based loading and LD_PRELOAD interception.
