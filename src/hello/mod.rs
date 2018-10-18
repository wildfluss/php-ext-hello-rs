extern crate libc;

// XXX
use zend::_zend_module_entry;

const PHP_HELLO_VERSION: &str = "0.1.0";

// static hello_module_entry: _zend_module_entry =

#[no_mangle]
pub extern "C" fn get_module() -> _zend_module_entry {
    _zend_module_entry::new(
        "hello",
        PHP_HELLO_VERSION
    )
}
