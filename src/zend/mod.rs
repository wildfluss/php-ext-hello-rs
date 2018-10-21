extern crate libc;

mod zend_gen;

pub use self::zend_gen::_zend_module_entry;

use std::ffi::CString;
use std::ffi::CStr;
use std::os::raw::{c_uchar, c_char, c_int, c_uint, c_void, c_long, c_double};

pub const ZEND_MODULE_API_NO: c_uint = 20170718; // zend_modules.h
pub const ZEND_DEBUG: c_uchar = 0; // php -ini | grep 'Debug =>'
pub const USING_ZTS: c_uchar = 0; // NTS

use std::mem;
use std::ptr;

pub const ZEND_MODULE_BUILD_ID: &str = "API20170718,NTS"; // php -ini | grep 'PHP Extension Build'

impl _zend_module_entry {
    pub fn init(&mut self, name: &str, version: &str) {
        // STANDARD_MODULE_HEADER
        self.size = mem::size_of::<_zend_module_entry>() as u16; // STANDARD_MODULE_HEADER_EX
        // println!("size {}", self.size);
        self.zend_api = ZEND_MODULE_API_NO; // STANDARD_MODULE_HEADER_EX
        self.zend_debug = ZEND_DEBUG; // STANDARD_MODULE_HEADER_EX
        self.zts = USING_ZTS; // STANDARD_MODULE_HEADER_EX
        self.ini_entry = ptr::null_mut();
        self.deps = ptr::null_mut();

        self.name = CString::new(name).unwrap().into_raw();
        self.functions = ptr::null_mut();
        self.module_startup_func = ptr::null_mut(); // PHP_MINIT(hello)
        self.module_shutdown_func = ptr::null_mut(); // PHP_MSHUTDOWN(hello)
        self.request_startup_func = ptr::null_mut(); // PHP_RINIT(hello)
        self.request_shutdown_func = ptr::null_mut(); // PHP_RSHUTDOWN(hello)
        self.info_func = ptr::null_mut(); // PHP_MINFO(hello)
        self.version = CString::new(version).unwrap().into_raw();

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
        let c_string = CString::new(ZEND_MODULE_BUILD_ID).unwrap();
        let ptr = c_string.into_raw();
        // unsafe {
        //     println!("{:?}", *ptr as u8);
        //     println!("{:?}", *ptr.offset(1) as u8);
        //     println!("{:?}", *ptr.offset(2) as u8);
        //     println!("{:?}", *ptr.offset(3) as u8);
        // }
        self.build_id = ptr; // STANDARD_MODULE_PROPERTIES_EX
        //self.build_id = 42 as *const c_char;
    }

    pub fn new(name: &str, version: &str) -> _zend_module_entry {
        unsafe {
            let mut entry: _zend_module_entry = mem::uninitialized();
            entry.init(name, version);
            entry
        }
    }
}

// #[repr(C)]
// pub struct _zend_internal_arg_info {
//     pub name: *const c_char,
//     // zend_type uintptr_t
//     pub type_: *mut c_void,
//     // zend_uchar unsigned char
//     pub pass_by_reference: c_uchar,
//     // zend_bool unsigned char
//     pub is_variadic: c_uchar,
// }

#[repr(C)]
pub struct _zend_function_entry {
    pub fname: *const c_char,
    pub handler: *mut c_void, // void (*zif_handler)(INTERNAL_FUNCTION_PARAMETERS);
    pub arg_info: *mut c_void, // XXX const struct _zend_internal_arg_info *
    pub num_args: c_uint, // uint32_t
    pub flags: c_uint, // uint32_t
}

#[repr(C)]
#[derive(Clone, Copy)] // otherwise unions with non-`Copy` fields are unstable
pub struct v {
    // ZEND_ENDIAN_LOHI_3
    pub type_: c_uchar,
    pub flags: c_uchar,
    pub gc_info: libc::uint16_t,
}

#[repr(C)]
pub union u {
    pub v: v,
    pub type_info: libc::uint32_t,
}

#[repr(C)]
pub struct zend_refcounted_h {
    pub refcount: libc::uint32_t,
    pub u: u,
}

#[repr(C)]
pub struct _zend_string {
    pub gc: zend_refcounted_h,
    pub h: libc::int64_t,
    pub len: libc::size_t,
    pub val: c_char,
}

#[repr(C)]
pub union _zend_value {
    pub lval: c_long, // long value zend_long int64_t
    pub dval: c_double,
    pub str: *mut _zend_string,
    // TODO: 
}

#[repr(C)]
pub union u1 {
    // TODO: 
    pub type_info: libc::uint32_t,
}

#[repr(C)]
pub union u2 {
    pub next: libc::uint32_t,
    // TODO: 
}

#[repr(C)]
pub struct _zval_struct { // zval
    pub value: _zend_value,
    pub u1: u1,
    pub u2: u2,
}

pub const IS_STRING_EX: libc::uint32_t = 5126;

extern "C" {
    fn zend_strpprintf(max_len: libc::size_t, format: *const c_char, ...) -> *mut _zend_string;
}

// XXX
pub fn strpprintf(max_len: libc::size_t, format: &str) -> *mut _zend_string {
    let c_format = CString::new(format).unwrap();
    unsafe {
        let strg = zend_strpprintf(max_len, c_format.as_ptr());
        strg // ???
    }
}

#[macro_export]
macro_rules! RETURN_STR {
    ( $return_value:expr, $strg:expr ) => {
        {
            unsafe {
                (*$return_value).value.str = $strg; // RETURN_STR
                (*$return_value).u1.type_info = IS_STRING_EX;
            }
        }
    };
}

#[macro_export]
macro_rules! PHP_FE_END {
    // () that macro takes no args
    () => {
        _zend_function_entry {
            fname: ptr::null_mut(),
            handler: ptr::null_mut(),
            arg_info: ptr::null_mut(),
            num_args: 0,
            flags: 0,
        }
    }
}
