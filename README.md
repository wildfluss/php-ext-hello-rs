# hello (Rust)

How to generate zend.rs

Get yakovzaytsev/rust-gen-struct@ecb03b8 and

```
DYLD_LIBRARY_PATH=$HOME/.local/share/llvmenv/7.0.0/lib /path/to/rust-gen-struct /home/src/php-7.2.10/Zend/zend_modules.h _zend_module_entry > src/zend/zend_gen.rs
```
