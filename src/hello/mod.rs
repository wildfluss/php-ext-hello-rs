extern crate libc;

// XXX
use zend::*;
use std::mem;
use std::ptr;
use std::os::raw::{c_char};

const PHP_HELLO_VERSION: &str = "0.1.0";

static mut hello_module_entry: _zend_module_entry = 
        _zend_module_entry {
            // STANDARD_MODULE_HEADER
            size: mem::size_of::<_zend_module_entry>() as u16, // STANDARD_MODULE_HEADER_EX 
            zend_api: ZEND_MODULE_API_NO, // STANDARD_MODULE_HEADER_EX 
            zend_debug: ZEND_DEBUG, // STANDARD_MODULE_HEADER_EX 
            zts: USING_ZTS, // STANDARD_MODULE_HEADER_EX 
            ini_entry: ptr::null_mut(),
            deps: ptr::null_mut(),

            name: 0 as *const c_char, // XXX
            functions: ptr::null_mut(),
            module_startup_func: ptr::null_mut(), // PHP_MINIT(hello)
            module_shutdown_func: ptr::null_mut(), // PHP_MSHUTDOWN(hello)
            request_startup_func: ptr::null_mut(), // PHP_RINIT(hello)
            request_shutdown_func: ptr::null_mut(), // PHP_RSHUTDOWN(hello)
            info_func: ptr::null_mut(), // PHP_MINFO(hello)
            version: 0 as *const c_char,
            
            // STANDARD_MODULE_PROPERTIES
            globals_size: 0, // NO_MODULE_GLOBALS,
            globals_ptr: ptr::null_mut(), // NO_MODULE_GLOBALS
            globals_ctor: ptr::null_mut(), // NO_MODULE_GLOBALS
            globals_dtor: ptr::null_mut(), // NO_MODULE_GLOBALS
            post_deactivate_func: ptr::null_mut(),
            module_started: 0, // STANDARD_MODULE_PROPERTIES_EX
            type_: 0, // STANDARD_MODULE_PROPERTIES_EX
            handle: ptr::null_mut(), // STANDARD_MODULE_PROPERTIES_EX
            module_number: 0, // STANDARD_MODULE_PROPERTIES_EX
            build_id: 0 as *const c_char, // STANDARD_MODULE_PROPERTIES_EX
        };

#[no_mangle]
pub extern "C" fn get_module() -> *const _zend_module_entry {
    unsafe {
        hello_module_entry.init(
            "hello",
            PHP_HELLO_VERSION
        );
        &hello_module_entry
    }
}
