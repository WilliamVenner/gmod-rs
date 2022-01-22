[![crates.io](https://img.shields.io/crates/v/gmod.svg)](https://crates.io/crates/gmod)

[![docs.rs](https://docs.rs/gmod/badge.svg)](https://docs.rs/gmod)

# ⚙ gmod-rs

A swiss army knife for creating binary modules for Garry's Mod in Rust.

# Examples

[Click here](https://github.com/WilliamVenner/gmod-rs/tree/master/examples/) to see examples.

# Nightly requirement

Currently, this crate requires the Rust Nightly compiler to be used.

This is because of the nature of Rust <-> C FFI (which is used extensively in this crate for interfacing with Lua) and the undefined behaviour that occurs when Lua performs long jumps out of functions during errors, or when Rust panics and unwinds out of a foreign stack frame. The [`C-unwind` ABI](https://rust-lang.github.io/rfcs/2945-c-unwind-abi.html) is used to prevent this undefined behaviour.