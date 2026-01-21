use std::sync::Once;

#[allow(unused_imports)]
use std::os::raw::{
    c_char, c_double, c_float, c_int, c_long, c_short, c_uchar, c_uint, c_ulong, c_ushort, c_void,
};

static mut LIB_HANDLE: Option<*mut c_void> = None;
static INIT: Once = Once::new();

// Embed original library
{{LIB_INCLUDE}}

/// Loads the embedded shared library using memfd_create instead of a tempfile.
/// 
/// memfd_create creates an anonymous file in memory that can be passed to dlopen.
/// Benefits over using a temporary file:
/// - No disk I/O: The library stays in memory, avoiding slow disk writes/reads
/// - Security: No temporary files left on disk that could be exploited or inspected
/// - Atomicity: The file is created and used entirely in memory, no filesystem state
/// - Cleanup: Automatically cleaned up when the process exits, no manual deletion needed
/// - Performance: Faster loading since no filesystem operations are involved
/// 
/// The memfd is created with a name for debugging purposes, then written with the
/// embedded library bytes. A path like /proc/self/fd/<fd> is constructed for dlopen.
#[cfg(target_os = "linux")]
fn load_embedded_lib() -> *mut c_void {
    unsafe {
        // Create anonymous file in memory
        let fd = libc::syscall(libc::SYS_memfd_create, c"embedded_lib".as_ptr(), 0);
        if fd < 0 {
            panic!("memfd_create failed");
        }

        // Write library to memfd
        let written = libc::write(fd as i32, ORIGINAL_SO.as_ptr() as *const c_void, ORIGINAL_SO.len());
        if written != ORIGINAL_SO.len() as isize {
            panic!("Failed to write to memfd");
        }

        // Create path to memfd
        let path = format!("/proc/self/fd/{}\0", fd);

        // Load library from memfd
        let handle = libc::dlopen(path.as_ptr() as *const i8, libc::RTLD_LAZY);
        if handle.is_null() {
            panic!("Failed to dlopen embedded library");
        }

        handle
    }
}



fn get_lib() -> *mut c_void {
    unsafe {
        INIT.call_once(|| {
            LIB_HANDLE = Some(load_embedded_lib());
        });
        LIB_HANDLE.unwrap()
    }
}

unsafe fn get_symbol(name: &[u8]) -> *const c_void {
    let handle = get_lib();
    let sym = libc::dlsym(handle, name.as_ptr() as *const i8);
    if sym.is_null() {
        panic!("Symbol not found: {}", std::str::from_utf8(name).unwrap());
    }
    sym
}

// {{FUNCTION_STUBS}}
// This placeholder will be replaced with generated function stubs
