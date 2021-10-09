#![allow(unused)]

mod import;
pub use import::*;

mod lua_state;
pub use lua_state::LuaState as State;

#[derive(Debug, Clone)]
pub enum LuaError {
	/// Out of memory
	///
	/// `LUA_ERRMEM`
	MemoryAllocationError,

	/// A syntax error occurred in the passed Lua source code.
	///
	/// `LUA_ERRSYNTAX`
	SyntaxError(Option<String>),

	/// Lua failed to load the given file.
	///
	/// `LUA_ERRFILE`
	FileError(Option<String>),

	/// A runtime error occurred.
	///
	/// `LUA_ERRRUN`
	RuntimeError(Option<String>),

	/// An error occurred while running the error handler function.
	///
	/// `LUA_ERRERR`
	ErrorHandlerError,

	/// Unknown Lua error code
	Unknown(i32),
}

/// Converts a string literal to a Lua-compatible NUL terminated string at compile time.
#[macro_export]
macro_rules! lua_string {
	( $str:literal ) => {
		$crate::cstr::cstr!($str).as_ptr()
	};
}

/// Enforces a debug assertion that the Lua stack is unchanged after this block of code is executed.
///
/// Useful for ensuring stack hygiene.
///
/// `lua` is the Lua state to check.
///
/// # Example
///
/// ```rust,norun
/// lua_stack_guard!(lua => {
/// 	lua.get_global(lua_string!("hook"));
/// 	lua.get_field(-1, lua_string!("Add"));
/// 	lua.push_string("PlayerInitialSpawn");
/// 	lua.push_string("RustHook");
/// 	lua.push_function(player_initial_spawn);
/// 	lua.call(3, 0);
/// 	// lua.pop();
/// });
/// // PANIC: stack is dirty! We forgot to pop the hook library off the stack.
/// ```
#[macro_export]
macro_rules! lua_stack_guard {
	( $lua:ident => $code:block ) => {{
		#[cfg(debug_assertions)] {
			let top = $lua.get_top();
			$code
			assert_eq!(top, $lua.get_top(), "Stack is dirty!");
		}

		#[cfg(not(debug_assertions))]
		$code
	}};
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
