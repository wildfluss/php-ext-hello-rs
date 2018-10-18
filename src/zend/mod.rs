mod zend_gen;

pub use self::zend_gen::_zend_module_entry;

use std::ffi::CString;
use std::os::raw::{c_uchar, c_uint, c_void};

pub const ZEND_MODULE_API_NO: c_uint = 20170718; // zend_modules.h
pub const ZEND_DEBUG: c_uchar = 0; // php -ini | grep 'Debug =>'
pub const USING_ZTS: c_uchar = 0; // NTS

use std::mem;
use std::ptr;

const ZEND_MODULE_BUILD_ID: &str = "API320170718,NTS"; // php -ini | grep 'Zend Extension Build'

impl _zend_module_entry {
    fn init(&mut self, name: &str, version: &str) {
        // STANDARD_MODULE_HEADER
        self.size = mem::size_of::<_zend_module_entry>() as u16; // STANDARD_MODULE_HEADER_EX
        self.zend_api = ZEND_MODULE_API_NO; // STANDARD_MODULE_HEADER_EX
        self.zend_debug = ZEND_DEBUG; // STANDARD_MODULE_HEADER_EX
        self.zts = USING_ZTS; // STANDARD_MODULE_HEADER_EX
        self.ini_entry = ptr::null_mut();
        self.deps = ptr::null_mut();

        self.name = ptr::null_mut();//CString::new(name).unwrap().as_ptr(); // XXX
        self.functions = ptr::null_mut();
        self.module_startup_func = ptr::null_mut(); // PHP_MINIT(hello)
        self.module_shutdown_func = ptr::null_mut(); // PHP_MSHUTDOWN(hello)
        self.request_startup_func = ptr::null_mut(); // PHP_RINIT(hello)
        self.request_shutdown_func = ptr::null_mut(); // PHP_RSHUTDOWN(hello)
        self.info_func = ptr::null_mut(); // PHP_MINFO(hello)
        self.version = ptr::null_mut();//CString::new(version).unwrap().as_ptr();

        // STANDARD_MODULE_PROPERTIES
        self.globals_size = 0; // NO_MODULE_GLOBALS,
        self.globals_ptr = ptr::null_mut(); // NO_MODULE_GLOBALS
        self.globals_ctor = ptr::null_mut(); // NO_MODULE_GLOBALS
        self.globals_dtor = ptr::null_mut(); // NO_MODULE_GLOBALS
        self.post_deactivate_func = ptr::null_mut();
        self.module_started = 0; // STANDARD_MODULE_PROPERTIES_EX
        self.type_ = 0; // STANDARD_MODULE_PROPERTIES_EX
        self.handle = ptr::null_mut(); // STANDARD_MODULE_PROPERTIES_EX
        self.module_number = 0; // STANDARD_MODULE_PROPERTIES_EX
        self.build_id = ptr::null_mut();//CString::new(ZEND_MODULE_BUILD_ID).unwrap().as_ptr(); // STANDARD_MODULE_PROPERTIES_EX
    }

    pub fn new(name: &str, version: &str) -> _zend_module_entry {
        unsafe {
            let mut entry: _zend_module_entry = mem::uninitialized();
            entry.init(name, version);
            entry
        }
    }
}
