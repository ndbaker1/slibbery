#![allow(unused_imports)]
#![allow(nonstandard_style)]
use libc::*;

pub mod types;
pub use types::*;

pub mod dl;
pub use dl::*;

// A function we want to run at load time of the library.
#[no_mangle]
pub extern "C" fn custom_init_function() {
    eprintln!("--- RUST INIT SUCCESSFUL ---");
}

// A static reference to the initialization function pointer is placed
// in the .init_array section using linker directives.
#[used]
// This attribute ensures the compiler doesn't optimize away the
// static item if it thinks it's unused.
#[link_section = ".init_array"]
pub static INITIALIZER: extern "C" fn() = custom_init_function;

// Generated function stubs
{{FUNCTION_STUBS}}

// Unknown function stubs (functions not found in header)
{{UNKNOWN_FUNCTION_STUBS}}
