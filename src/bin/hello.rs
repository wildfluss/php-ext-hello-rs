extern crate hello; // FIXME cannot satisfy dependencies so `std` only shows up once

//use hello::zend::_zend_module_entry;

const PHP_HELLO_VERSION: &str = "0.1.0";

fn main() -> std::io::Result<()> {
    // _zend_module_entry::new(
    //     "hello",
    //     PHP_HELLO_VERSION
    // );

    Ok(())
}

