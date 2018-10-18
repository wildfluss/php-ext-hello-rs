# hello (Rust)

Build 

```
cd php-ext-hello-rs
RUSTFLAGS='-C prefer-dynamic' cargo build
```

and try run 

```
DYLD_LIBRARY_PATH=$HOME/.rustup/toolchains/a-toolchain/lib/ ./target/debug/hello
```

e.g.

```
DYLD_LIBRARY_PATH=$HOME/.rustup/toolchains/stable-x86_64-apple-darwin/lib/ ./target/debug/hello
```

How to generate zend.rs

Get yakovzaytsev/rust-gen-struct@ecb03b8 and

```
DYLD_LIBRARY_PATH=$HOME/.local/share/llvmenv/7.0.0/lib /path/to/rust-gen-struct /home/src/php-7.2.10/Zend/zend_modules.h _zend_module_entry > src/zend/zend_gen.rs
```
