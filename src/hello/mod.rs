extern crate libc;

// XXX
use ::zend::{_zend_module_entry};

const PHP_HELLO_VERSION: &str = "0.1.0";

#[no_mangle]
pub extern fn get_module() -> _zend_module_entry {
    _zend_module_entry::new(
        "hello",
        PHP_HELLO_VERSION
    )
}



