extern crate libc;

pub mod zend;

// XXX
use self::zend::*;

impl _zend_module_entry {
    fn new() -> _zend_module_entry {
        _zend_module_entry { }
    }
}

#[no_mangle]
pub extern fn get_module() -> _zend_module_entry {
    _zend_module_entry::new()
}



