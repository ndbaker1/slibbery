use std::sync::Once;

use std::os::raw::{c_void, c_int, c_uint, c_long, c_ulong, c_short, c_ushort, c_char, c_uchar, c_float, c_double};

static mut LIB_HANDLE: Option<*mut c_void> = None;
static INIT: Once = Once::new();

// Embed original library
LIB_INCLUDE_PLACEHOLDER

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

// FUNCTION_STUBS_PLACEHOLDER
// This placeholder will be replaced with generated function stubs
