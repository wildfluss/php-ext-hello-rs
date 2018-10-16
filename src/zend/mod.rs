mod zend_gen;

pub use self::zend_gen::_zend_module_entry;

use std::os::raw::{c_uint, c_uchar, c_void};
use std::ffi::CString;

pub const ZEND_MODULE_API_NO: c_uint = 20170718; // zend_modules.h
pub const ZEND_DEBUG: c_uchar = 0;        // php -ini | grep 'Debug =>'
pub const USING_ZTS: c_uchar = 0;         // NTS

use std::mem;
use std::ptr;

const ZEND_MODULE_BUILD_ID: &str = "API320170718,NTS"; // php -ini | grep 'Zend Extension Build'

impl _zend_module_entry {
    pub fn new(name: &str, version: &str) -> _zend_module_entry {
        _zend_module_entry {
            // STANDARD_MODULE_HEADER
            size: mem::size_of::<_zend_module_entry>() as u16, // STANDARD_MODULE_HEADER_EX 
            zend_api: ZEND_MODULE_API_NO, // STANDARD_MODULE_HEADER_EX 
            zend_debug: ZEND_DEBUG, // STANDARD_MODULE_HEADER_EX 
            zts: USING_ZTS, // STANDARD_MODULE_HEADER_EX 
            ini_entry: ptr::null_mut(),
            deps: ptr::null_mut(),

            name: CString::new(name).unwrap().as_ptr(), // XXX
            functions: ptr::null_mut(),
            module_startup_func: ptr::null_mut(), // PHP_MINIT(hello)
            module_shutdown_func: ptr::null_mut(), // PHP_MSHUTDOWN(hello)
            request_startup_func: ptr::null_mut(), // PHP_RINIT(hello)
            request_shutdown_func: ptr::null_mut(), // PHP_RSHUTDOWN(hello)
            info_func: ptr::null_mut(), // PHP_MINFO(hello)
            version: CString::new(version).unwrap().as_ptr(),
            
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
            build_id: CString::new(ZEND_MODULE_BUILD_ID).unwrap().as_ptr(), // STANDARD_MODULE_PROPERTIES_EX
        }
    }
}

