# C/C++ in Rust compiled with wasm-pack issue

I am trying to include C++ code in my rust code. This repository shows how I'm currently trying to do so. If you have a better solution please tell me!

## Error
When compiling with wasm-pack I get the following message:

```
user@ubuntu:~/c-in-rust-with-wasm-pack$ wasm-pack build
[INFO]: Checking for the Wasm target...
[INFO]: Compiling to Wasm...
   Compiling c-in-rust-with-wasm-pack v0.1.0 (/home/user/c-in-rust-with-wasm-pack)
error[E0425]: cannot find function `add_one` in this scope
  --> src/lib.rs:25:52
   |
25 |     alert(format!("Evaluating {}!", unsafe { add_one(1) }).as_str());
   |                                              ^^^^^^^ not found in this scope

For more information about this error, try `rustc --explain E0425`.
error: could not compile `c-in-rust-with-wasm-pack` due to previous error
Error: Compiling your crate to WebAssembly failed
Caused by: failed to execute `cargo build`: exited with exit status: 101
  full command: "cargo" "build" "--lib" "--release" "--target" "wasm32-unknown-unknown"
```

## Prerequisites

This example is using [cc-rs](https://github.com/rust-lang/cc-rs) which has some requirements seen [here](https://github.com/rust-lang/cc-rs#compile-time-requirements). For Linux clang is required and can be installed using:

```
sudo apt-get install clang
```

wasm-pack is also needed and can be installed with:

```
cargo install wasm-pack
```

## Run

```
wasm-pack build
```
