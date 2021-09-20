use std::{mem::MaybeUninit, borrow::Cow, ffi::c_void};

use crate::lua::*;

use crate::userdata::UserData;

#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
pub struct LuaState(pub *mut std::ffi::c_void);
unsafe impl Send for LuaState {}
impl LuaState {
	/// Returns the Lua string as a slice of bytes.
	///
	/// **WARNING:** This will CHANGE the type of the value at the given index to a string.
	///
	/// Returns None if the value at the given index is not convertible to a string.
	pub unsafe fn get_binary_string(&self, index: i32) -> Option<&[u8]> {
		let mut len: usize = 0;
		let ptr = (LUA_SHARED.lua_tolstring)(*self, index, &mut len);

		if ptr.is_null() {
			return None;
		}

		Some(std::slice::from_raw_parts(ptr as *const u8, len))
	}

	/// Returns the Lua string as a Rust UTF-8 String.
	///
	/// **WARNING:** This will CHANGE the type of the value at the given index to a string.
	///
	/// Returns None if the value at the given index is not convertible to a string.
	///
	/// This is a lossy operation, and will replace any invalid UTF-8 sequences with the Unicode replacement character. See the documentation for `String::from_utf8_lossy` for more information.
	///
	/// If you need raw data, use `get_binary_string`.
	pub unsafe fn get_string(&self, index: i32) -> Option<std::borrow::Cow<'_, str>> {
		let mut len: usize = 0;
		let ptr = (LUA_SHARED.lua_tolstring)(*self, index, &mut len);

		if ptr.is_null() {
			return None;
		}

		let bytes = std::slice::from_raw_parts(ptr as *const u8, len);

		Some(String::from_utf8_lossy(bytes))
	}

	/// Returns the name of the type of the value at the given index.
	pub unsafe fn get_type(&self, index: i32) -> &str {
		let lua_type = (LUA_SHARED.lua_type)(*self, index);
		let lua_type_str_ptr = (LUA_SHARED.lua_typename)(*self, lua_type);
		let lua_type_str = std::ffi::CStr::from_ptr(lua_type_str_ptr);
		unsafe { std::str::from_utf8_unchecked(lua_type_str.to_bytes()) }
	}

	#[inline]
	pub unsafe fn get_top(&self) -> i32 {
		(LUA_SHARED.lua_gettop)(*self)
	}

	#[inline]
	/// Pops the stack, inserts the value into the registry table, and returns the registry index of the value.
	///
	/// Use `from_reference` with the reference index to push the value back onto the stack.
	///
	/// Use `dereference` to free the reference from the registry table.
	pub unsafe fn reference(&self) -> LuaReference {
		(LUA_SHARED.lual_ref)(*self, LUA_REGISTRYINDEX)
	}

	#[inline]
	pub unsafe fn dereference(&self, r#ref: LuaReference) {
		(LUA_SHARED.lual_unref)(*self, LUA_REGISTRYINDEX, r#ref)
	}

	#[inline]
	pub unsafe fn from_reference(&self, r#ref: LuaReference) {
		self.raw_geti(LUA_REGISTRYINDEX, r#ref)
	}

	#[inline]
	pub unsafe fn is_nil(&self, index: i32) -> bool {
		(LUA_SHARED.lua_type)(*self, index) == LUA_TNIL
	}

	#[inline]
	pub unsafe fn is_function(&self, index: i32) -> bool {
		(LUA_SHARED.lua_type)(*self, index) == LUA_TFUNCTION
	}

	#[inline]
	pub unsafe fn is_table(&self, index: i32) -> bool {
		(LUA_SHARED.lua_type)(*self, index) == LUA_TTABLE
	}

	#[inline]
	pub unsafe fn is_boolean(&self, index: i32) -> bool {
		(LUA_SHARED.lua_type)(*self, index) == LUA_TBOOLEAN
	}

	#[inline]
	pub unsafe fn remove(&self, index: i32) {
		(LUA_SHARED.lua_remove)(*self, index)
	}

	#[inline]
	pub unsafe fn push_value(&self, index: i32) {
		(LUA_SHARED.lua_pushvalue)(*self, index)
	}

	#[inline]
	pub unsafe fn get_field(&self, index: i32, k: LuaString) {
		(LUA_SHARED.lua_getfield)(*self, index, k)
	}

	#[inline]
	pub unsafe fn push_boolean(&self, boolean: bool) {
		(LUA_SHARED.lua_pushboolean)(*self, if boolean { 1 } else { 0 })
	}

	#[inline]
	pub unsafe fn push_integer(&self, int: LuaInt) {
		(LUA_SHARED.lua_pushinteger)(*self, int)
	}

	#[inline]
	pub unsafe fn push_number(&self, num: LuaNumber) {
		(LUA_SHARED.lua_pushnumber)(*self, num)
	}

	#[inline]
	pub unsafe fn push_nil(&self) {
		(LUA_SHARED.lua_pushnil)(*self)
	}

	#[inline]
	pub unsafe fn pcall(&self, nargs: i32, nresults: i32, errfunc: i32) -> i32 {
		(LUA_SHARED.lua_pcall)(*self, nargs, nresults, errfunc)
	}

	pub unsafe fn load_string(&self, src: LuaString) -> Result<(), LuaError> {
		let lua_error_code = (LUA_SHARED.lual_loadstring)(*self, src);
		if lua_error_code == 0 {
			Ok(())
		} else {
			Err(LuaError::from_lua_state(*self, lua_error_code))
		}
	}

	pub unsafe fn load_buffer(&self, buff: &[u8], name: LuaString) -> Result<(), LuaError> {
		let lua_error_code = (LUA_SHARED.lual_loadbuffer)(*self, buff.as_ptr() as LuaString, buff.len(), name);
		if lua_error_code == 0 {
			Ok(())
		} else {
			Err(LuaError::from_lua_state(*self, lua_error_code))
		}
	}

	pub unsafe fn load_file(&self, path: LuaString) -> Result<(), LuaError> {
		let lua_error_code = (LUA_SHARED.lual_loadfile)(*self, path);
		if lua_error_code == 0 {
			Ok(())
		} else {
			Err(LuaError::from_lua_state(*self, lua_error_code))
		}
	}

	#[inline]
	pub unsafe fn pop(&self) {
		self.pop_n(1);
	}

	#[inline]
	pub unsafe fn pop_n(&self, count: i32) {
		self.set_top(-count - 1);
	}

	#[inline]
	pub unsafe fn set_top(&self, index: i32) {
		(LUA_SHARED.lua_settop)(*self, index)
	}

	#[inline]
	pub unsafe fn lua_type(&self, index: i32) -> i32 {
		(LUA_SHARED.lua_type)(*self, index)
	}

	pub unsafe fn lua_type_name(&self, lua_type_id: i32) -> Cow<'_, str> {
		let type_str_ptr = (LUA_SHARED.lua_typename)(*self, lua_type_id);
		let type_str = std::ffi::CStr::from_ptr(type_str_ptr);
		type_str.to_string_lossy()
	}

	#[inline]
	pub unsafe fn replace(&self, index: i32) {
		(LUA_SHARED.lua_replace)(*self, index)
	}

	#[inline]
	pub unsafe fn push_globals(&self) {
		(LUA_SHARED.lua_pushvalue)(*self, LUA_GLOBALSINDEX)
	}

	#[inline]
	pub unsafe fn push_string(&self, data: &str) {
		(LUA_SHARED.lua_pushlstring)(*self, data.as_ptr() as LuaString, data.len())
	}

	#[inline]
	pub unsafe fn push_binary_string(&self, data: &[u8]) {
		(LUA_SHARED.lua_pushlstring)(*self, data.as_ptr() as LuaString, data.len())
	}

	#[inline]
	pub unsafe fn push_function(&self, func: LuaFunction) {
		(LUA_SHARED.lua_pushcclosure)(*self, func, 0)
	}

	#[inline]
	pub unsafe fn set_table(&self, index: i32) {
		(LUA_SHARED.lua_settable)(*self, index)
	}

	#[inline]
	pub unsafe fn set_field(&self, index: i32, k: LuaString) {
		(LUA_SHARED.lua_setfield)(*self, index, k)
	}

	#[inline]
	pub unsafe fn get_global(&self, name: LuaString) {
		(LUA_SHARED.lua_getfield)(*self, LUA_GLOBALSINDEX, name)
	}

	#[inline]
	pub unsafe fn set_global(&self, name: LuaString) {
		(LUA_SHARED.lua_setfield)(*self, LUA_GLOBALSINDEX, name)
	}

	#[inline]
	pub unsafe fn call(&self, nargs: i32, nresults: i32) {
		(LUA_SHARED.lua_call)(*self, nargs, nresults)
	}

	#[inline]
	pub unsafe fn insert(&self, index: i32) {
		(LUA_SHARED.lua_insert)(*self, index)
	}

	/// Creates a new table and pushes it to the stack.
	/// seq_n is a hint as to how many sequential elements the table may have.
	/// hash_n is a hint as to how many non-sequential/hashed elements the table may have.
	/// Lua may use these hints to preallocate memory.
	#[inline]
	pub unsafe fn create_table(&self, seq_n: i32, hash_n: i32) {
		(LUA_SHARED.lua_createtable)(*self, seq_n, hash_n)
	}

	/// Creates a new table and pushes it to the stack without memory preallocation hints.
	/// Equivalent to `create_table(0, 0)`
	#[inline]
	pub unsafe fn new_table(&self) {
		(LUA_SHARED.lua_createtable)(*self, 0, 0)
	}

	#[inline]
	pub unsafe fn get_table(&self, index: i32) {
		(LUA_SHARED.lua_gettable)(*self, index)
	}

	pub unsafe fn check_binary_string(&self, arg: i32) -> &[u8] {
		let mut len: usize = 0;
		let ptr = (LUA_SHARED.lual_checklstring)(*self, arg, &mut len);
		std::slice::from_raw_parts(ptr as *const u8, len)
	}

	pub unsafe fn check_string(&self, arg: i32) -> Cow<'_, str> {
		let mut len: usize = 0;
		let ptr = (LUA_SHARED.lual_checklstring)(*self, arg, &mut len);
		String::from_utf8_lossy(std::slice::from_raw_parts(ptr as *const u8, len))
	}

	#[inline]
	pub unsafe fn check_userdata(&self, arg: i32, name: LuaString) -> *mut UserData {
		(LUA_SHARED.lual_checkudata)(*self, arg, name) as *mut _
	}

	pub unsafe fn test_userdata(&self, index: i32, name: LuaString) -> bool {
		if !(LUA_SHARED.lua_touserdata)(*self, index).is_null() {
			if self.get_metatable(index) != 0 {
				self.get_field(LUA_REGISTRYINDEX, name);
				let result = self.raw_equal(-1, -2);
				self.pop_n(2);
				if result {
					return true;
				}
			}
		}
		false
	}

	#[inline]
	pub unsafe fn raw_equal(&self, a: i32, b: i32) -> bool {
		(LUA_SHARED.lua_rawequal)(*self, a, b) == 1
	}

	#[inline]
	pub unsafe fn get_metatable(&self, index: i32) -> i32 {
		(LUA_SHARED.lua_getmetatable)(*self, index)
	}

	#[inline]
	pub unsafe fn check_integer(&self, arg: i32) -> LuaInt {
		(LUA_SHARED.lual_checkinteger)(*self, arg)
	}

	#[inline]
	pub unsafe fn check_number(&self, arg: i32) -> f64 {
		(LUA_SHARED.lual_checknumber)(*self, arg)
	}

	#[inline]
	pub unsafe fn to_integer(&self, index: i32) -> LuaInt {
		(LUA_SHARED.lua_tointeger)(*self, index)
	}

	#[inline]
	pub unsafe fn to_number(&self, index: i32) -> f64 {
		(LUA_SHARED.lua_tonumber)(*self, index)
	}

	#[inline]
	pub unsafe fn get_boolean(&self, index: i32) -> bool {
		(LUA_SHARED.lua_toboolean)(*self, index) == 1
	}

	#[inline]
	pub unsafe fn set_metatable(&self, index: i32) -> i32 {
		(LUA_SHARED.lua_setmetatable)(*self, index)
	}

	#[inline]
	pub unsafe fn len(&self, index: i32) -> i32 {
		(LUA_SHARED.lua_objlen)(*self, index)
	}

	#[inline]
	pub unsafe fn raw_geti(&self, t: i32, index: i32) {
		(LUA_SHARED.lua_rawgeti)(*self, t, index)
	}

	#[inline]
	pub unsafe fn raw_seti(&self, t: i32, index: i32) {
		(LUA_SHARED.lua_rawseti)(*self, t, index)
	}

	#[inline]
	pub unsafe fn next(&self, index: i32) -> i32 {
		(LUA_SHARED.lua_next)(*self, index)
	}

	pub unsafe fn error<S: AsRef<str>>(&self, msg: S) -> ! {
		self.push_string(msg.as_ref());
		(LUA_SHARED.lua_error)(*self);
		unreachable!()
	}

	pub unsafe fn debug_get_info(&self, what: LuaString) -> Option<LuaDebug> {
		let mut ar = MaybeUninit::uninit();
		if (LUA_SHARED.lua_getinfo)(*self, what, ar.as_mut_ptr()) != 0 {
			Some(ar.assume_init())
		} else {
			None
		}
	}

	pub unsafe fn debug_get_invocation_info(&self, level: i32, what: LuaString) -> Option<LuaDebug> {
		let mut ar = MaybeUninit::uninit();
		if (LUA_SHARED.lua_getstack)(*self, level, ar.as_mut_ptr()) != 0 {
			if (LUA_SHARED.lua_getinfo)(*self, what, ar.as_mut_ptr()) != 0 {
				return Some(ar.assume_init());
			}
		}
		None
	}

	#[cfg(debug_assertions)]
	pub unsafe fn dump_stack(&self) {
		let top = self.get_top();
		println!("\n=== STACK DUMP ===");
		println!("Stack size: {}", top);
		for i in 1..=top {
			let lua_type = self.lua_type(i);
			let lua_type_name = self.lua_type_name(lua_type);
			match lua_type_name.as_ref() {
				"string" => println!("{}. {}: {:?}", i, lua_type_name, {
					self.push_value(i);
					let str = self.get_string(-1);
					self.pop();
					str
				}),
				_ => println!("{}. {}", i, lua_type_name),
			}
		}
		println!();
	}

	#[cfg(debug_assertions)]
	pub unsafe fn dump_val(&self, index: i32) -> String {
		let lua_type_name = self.lua_type_name(self.lua_type(index));
		match lua_type_name.as_ref() {
			"string" => {
				self.push_value(index);
				let str = self.get_string(-1);
				self.pop();
				format!("{:?}", str.unwrap().into_owned())
			},
			"boolean" => {
				self.push_value(index);
				let boolean = self.get_boolean(index);
				self.pop();
				format!("{}", boolean)
			},
			"number" => {
				self.push_value(index);
				let n = self.to_number(index);
				self.pop();
				format!("{}", n)
			},
			_ => lua_type_name.into_owned(),
		}
	}
}
impl std::ops::Deref for LuaState {
	type Target = *mut std::ffi::c_void;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct LuaDebug {
	pub event: i32,
	pub name: LuaString,
	pub namewhat: LuaString,
	pub what: LuaString,
	pub source: LuaString,
	pub currentline: i32,
	pub nups: i32,
	pub linedefined: i32,
	pub lastlinedefined: i32,
	pub short_src: [std::os::raw::c_char; LUA_IDSIZE],
	i_ci: i32
}
impl std::fmt::Debug for LuaDebug {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		unsafe {
			f.debug_struct("LuaDebug")
			.field("event", &self.event)
			.field("name", &std::ffi::CStr::from_ptr(self.name))
			.field("namewhat", &std::ffi::CStr::from_ptr(self.namewhat))
			.field("what", &std::ffi::CStr::from_ptr(self.what))
			.field("source", &std::ffi::CStr::from_ptr(self.source))
			.field("currentline", &self.currentline)
			.field("nups", &self.nups)
			.field("linedefined", &self.linedefined)
			.field("lastlinedefined", &self.lastlinedefined)
			.field("short_src", &std::ffi::CStr::from_ptr(self.short_src.as_ptr()))
			.field("i_ci", &self.i_ci)
			.finish()
		}
	}
}
