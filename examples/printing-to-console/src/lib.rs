#![feature(c_unwind)]

use gmod::gmcl::override_stdout;
use gmod::lua::State;

#[macro_use] extern crate gmod;

#[gmod13_open]
fn gmod13_open(lua: State) -> i32 {
    // Here, if this module is running on the client.
    if lua.is_client() {
        // We overwrite println! so it prints to the console.
        override_stdout()
    }

    if lua.is_server() {
        println!("Hello Server, this is a binary module!")
    } else {
        println!("Hello Client, this is a binary module!")
    }

    0
}

#[gmod13_close]
fn gmod13_close(lua: State) -> i32 {
    println!("Goodbye from binary module!");
    0
}
