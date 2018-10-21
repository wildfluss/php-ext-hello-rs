# hello (Rust)

PHP ext_skel for Rust.

To build extension only set (see: https://doc.rust-lang.org/reference/linkage.html)

```
crate-type = ["cdylib"]
```

in Cargo.toml and

- on linux 

    ```
    cd php-ext-hello-rs
    RUSTFLAGS='-C prefer-dynamic' cargo build --lib
    ```

- or on OSX 

    ```
    RUSTFLAGS="-Cprefer-dynamic -Clink-arg=-undefined -Clink-arg=dynamic_lookup" cargo build --lib
    ```

test PHP extension:

```
$ php -d extension=./target/debug/libhello.so -f hello.php
Functions available in the test extension:
confirm_hello_compiled


```

or set in Cargo.toml (otherwise: can't find crate hello)

```
crate-type = ["dylib"]
```

and also try **build and** run hello (which uses lib crate)

```
RUSTFLAGS='-C prefer-dynamic' cargo build
DYLD_LIBRARY_PATH=$HOME/.rustup/toolchains/a-toolchain/lib/ ./target/debug/hello
```

e.g.

```
DYLD_LIBRARY_PATH=$HOME/.rustup/toolchains/stable-x86_64-apple-darwin/lib/ ./target/debug/hello
```

How to generate zend.rs

Get yakovzaytsev/rust-gen-struct@ecb03b8 and

```
[DY]LD_LIBRARY_PATH=$HOME/.local/share/llvmenv/7.0.0/lib /path/to/rust-gen-struct /home/src/php-7.2.10/Zend/zend_modules.h _zend_module_entry > src/zend/zend_gen.rs
```
