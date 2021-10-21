[![crates.io](https://img.shields.io/crates/v/gmod.svg)](https://crates.io/crates/gmod)

[![docs.rs](https://docs.rs/gmod/badge.svg)](https://docs.rs/gmod)

# ⚙ gmod-rs

A swiss army knife for creating binary modules for Garry's Mod in Rust.

# Example

### rust-toolchain.toml

Because we're using the [`C-unwind`](https://rust-lang.github.io/rfcs/2797-project-ffi-unwind.html) ABI, this crate must be used on a [Nightly Rust](https://doc.rust-lang.org/book/appendix-07-nightly-rust.html) compiler.

```toml
[toolchain]
channel = "nightly"
```

### Cargo.toml

```toml
[lib]
crate-type = ["cdylib"]

[dependencies]
gmod = "*"
```

### lib.rs

```rust
#[macro_use]
extern crate gmod;

#[gmod13_open]
fn gmod13_open(lua: gmod::lua::State) -> i32 {
    println!("Hello from binary module!");
    0
}

#[gmod13_close]
fn gmod13_close(lua: gmod::lua::State) -> i32 {
    println!("Goodbye from binary module!");
    0
}
```