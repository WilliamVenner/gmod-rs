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
		::gmod::cstr::cstr!($str).as_ptr()
	};
}
