# hello (Rust)

How to generate zend.rs

Get yakovzaytsev/rust-gen-struct#7af71c8 and

```
DYLD_LIBRARY_PATH=$HOME/.local/share/llvmenv/7.0.0/lib /path/to/rust-gen-struct /home/src/php-7.2.10/Zend/zend_modules.h _zend_module_entry > src/hello/zend.rs
```
