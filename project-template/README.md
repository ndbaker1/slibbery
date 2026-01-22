# Dyll Project Template

This directory contains the template files used by Dyll to generate stub shared libraries.

## Files

- `Cargo.toml`: Rust project configuration with CDylib output for shared library generation
- `src/lib.rs`: Main library template with:
  - Memory-efficient embedded library loading using `memfd_create`
  - Function symbol resolution via `dlsym`
  - Placeholder for generated function stubs (`{{FUNCTION_STUBS}}`)
  - Placeholder for library include path (`{{LIB_INCLUDE}}`)

## Purpose

These templates provide the foundation for generated Rust projects that:

1. **Embed the original library** in the binary using `include_bytes!`
2. **Load libraries in memory** using Linux's `memfd_create` for zero-disk-I/O loading
3. **Provide FFI-safe stubs** that forward calls to the original implementations
4. **Allow runtime interception** for testing and mocking

## Template Variables

- `{{FUNCTION_STUBS}}`: Replaced with generated Rust function implementations
- `{{LIB_INCLUDE}}`: Replaced with the `include_bytes!` macro path to the embedded library

## Why Templates?

- **Consistency**: Ensures all generated projects have the same structure and dependencies
- **Maintainability**: Template changes apply to all generated stubs
- **Performance**: Optimized loading mechanism shared across all stubs
- **Safety**: Proper FFI boundaries and memory management