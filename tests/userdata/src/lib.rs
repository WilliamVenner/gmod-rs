#[macro_use]
extern crate gmod;

static mut DROP_OK: bool = false;

#[derive(PartialEq, Eq, Debug)]
pub struct DropMe {
	pub x: i32,
	pub y: i32,
	pub z: i32,
	pub hello: String
}
impl Drop for DropMe {
	fn drop(&mut self) {
		unsafe {
			if DROP_OK {
				DROP_OK = false;
				println!("USERDATA DROP TEST PASSED");
				println!("USERDATA DROP TEST PASSED");
				println!("USERDATA DROP TEST PASSED");
				println!("USERDATA DROP TEST PASSED");
				println!("USERDATA DROP TEST PASSED");
				println!("USERDATA DROP TEST PASSED");
				println!("USERDATA DROP TEST PASSED");
				println!("USERDATA DROP TEST PASSED");
				println!("USERDATA DROP TEST PASSED");
				println!("USERDATA DROP TEST PASSED");
				println!("USERDATA DROP TEST PASSED");
				println!("USERDATA DROP TEST PASSED");
				println!("USERDATA DROP TEST PASSED");
			} else {
				panic!("Dropped too early or too late");
			}
		}
	}
}

macro_rules! drop_me {
	() => {
		DropMe {
			x: 69,
			y: 420,
			z: 123,
			hello: "Hello".to_string()
		}
	};
}

#[gmod13_open]
unsafe fn gmod13_open(lua: gmod::lua::State) -> i32 {
	let ud = lua.new_userdata(drop_me!(), None);
	assert_eq!(&*ud, Box::leak(Box::new(drop_me!())));

	lua.set_global(lua_string!("GMOD_RUST_DROP_TEST"));

	lua.push_nil();
	lua.set_global(lua_string!("GMOD_RUST_DROP_TEST"));
	DROP_OK = true;

	lua.get_global(lua_string!("collectgarbage"));
	lua.push_value(-1);
	lua.call(0, 0);
	lua.call(0, 0);

	let ud = lua.new_userdata(420_i32, None);
	assert_eq!(*ud, 420_i32);

	lua.get_global(lua_string!("collectgarbage"));
	lua.push_value(-1);
	lua.call(0, 0);
	lua.call(0, 0);

	0
}