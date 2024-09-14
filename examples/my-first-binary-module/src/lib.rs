#[macro_use] extern crate gmod;

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