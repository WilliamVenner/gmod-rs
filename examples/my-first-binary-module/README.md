# Installing Rust

Installing Rust is as easy as downloading [rustup](https://rustup.rs/) and running it!

# Building the example

To build the example in debug mode, you'll need to specify the target architecture for your build.

| Platform | Command | Description |
|:---:|:---:|:---:|
| `win32` | `cargo build --target i686-pc-windows-msvc` | Windows 32-bit<br>Use this if your server is running Windows and is on the `main` branch of Garry's Mod (this is the default branch.) |
| `win64` | `cargo build --target x86_64-pc-windows-msvc` | Windows 64-bit<br>Use this if your server is running Windows and is on the `x86-64` branch of Garry's Mod. |
| `linux` | `cargo build --target i686-unknown-linux-gnu` | Linux 32-bit<br>Use this if your server is running Linux and is on the `main` branch of Garry's Mod (this is the default branch.) |
| `linux64` | `cargo build --target x86_64-unknown-linux-gnu` |Linux 64-bit<br>Use this if your server is running Linux and is on the `x86-64` branch of Garry's Mod. |

You can find the compiled binary in `target/<TARGET>/debug/my_first_binary_module.dll` on Windows or `target/<TARGET>/debug/libmy_first_binary_module.so` on Linux.

If Rust complains it can't find the target/toolchain, you'll need to install it. By default Rust only installs your system's native toolchain, which is most likely Windows 64-bit (`x86_64-pc-windows-msvc`)

I don't recommend cross-compiling Linux binaries on Windows. If you want to compile Linux binaries on Windows, do it in WSL.

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

`cargo build --target <TARGET> --release`

This enables performance optimization of the compiled binary and removes debug symbols which make the binary huge, whilst taking longer to compile.

On Linux, you'll want to run the `strip` command on the compiled binary to remove debug symbols.
