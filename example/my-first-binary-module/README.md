# Installing Rust

Installing Rust is as easy as downloading [rustup](https://rustup.rs/) and running it!

# Building the example

To build the example in debug mode, simply type in a terminal:

`cargo build`

You can find the compiled binary in `target/debug/my_first_binary_module.dll` (or `.so` if you're building on Linux)

# Using the example in Garry's Mod

First, rename the compiled binary to `gmsv_my_first_binary_module_PLATFORM.dll` where `PLATFORM` is one of the following:

| Platform | Description |
|:---:|:---:|
| `win32` | Windows 32-bit<br>Use this if your server is running Windows and is on the `main` branch of Garry's Mod (this is the default branch.) |
| `win64` | Windows 64-bit<br>Use this if your server is running Windows and is on the `x86-64` branch of Garry's Mod. |
| `linux` | Linux 32-bit<br>Use this if your server is running Linux and is on the `main` branch of Garry's Mod (this is the default branch.) |
| `linux64` | Linux 64-bit<br>Use this if your server is running Linux and is on the `x86-64` branch of Garry's Mod. |

Then, move the compiled binary to `garrysmod/lua/bin/` on your server. If the `bin` folder doesn't exist, create it.

Finally, you can load the module from Lua!

```lua
require("my_first_binary_module")
```

# Preparing your module for release

If you've written a useful module and want to release it to the world, or just on your server, build with the `--release` flag:

`cargo build --release`

This enables performance optimization of the compiled binary and removes debug symbols which make the binary huge, whilst taking longer to compile.

On Linux, you'll want to run the `strip` command on the compiled binary to remove debug symbols.
