use crate::{lua::{self, LuaFunction}, lua_string};

#[inline(always)]
pub unsafe fn add_network_strings<S: AsRef<str>>(lua: lua::State, network_strings: &[S]) {
	match network_strings.len() {
		0 => {},
		1 => {
			lua.get_global(lua_string!("util"));
			lua.get_field(-1, lua_string!("AddNetworkString"));
			lua.push_string(network_strings[0].as_ref());
			lua.call(1, 0);
			lua.pop();
		},
		_ => {
			lua.get_global(lua_string!("util"));
			lua.get_field(-1, lua_string!("AddNetworkString"));
			for network_string in network_strings {
				lua.push_value(-1);
				lua.push_string(network_string.as_ref());
				lua.call(1, 0);
			}
			lua.pop_n(2);
		}
	}
}

#[inline(always)]
pub unsafe fn receive<S: AsRef<str>>(lua: lua::State, network_string: S, func: LuaFunction) {
	lua.get_global(lua_string!("net"));
	lua.get_field(-1, lua_string!("Receive"));
	lua.push_string(network_string.as_ref());
	lua.push_function(func);
	lua.call(2, 0);
	lua.pop();
}