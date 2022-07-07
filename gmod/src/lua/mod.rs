#![allow(unused)]

mod import;
use std::cell::Cell;

pub use import::*;

mod lua_state;
pub use lua_state::LuaState as State;

mod push;
pub use push::*;

mod returns;
pub use returns::ValuesReturned;

mod raw_bind;

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
///     lua.get_global(lua_string!("hook"));
///     lua.get_field(-1, lua_string!("Add"));
///     lua.push_string("PlayerInitialSpawn");
///     lua.push_string("RustHook");
///     lua.push_function(player_initial_spawn);
///     lua.call(3, 0);
///     // lua.pop();
/// });
/// // PANIC: stack is dirty! We forgot to pop the hook library off the stack.
/// ```
#[macro_export]
macro_rules! lua_stack_guard {
	( $lua:ident => $code:block ) => {{
		#[cfg(debug_assertions)] {
			let top = $lua.get_top();
			let ret = $code;
			if top != $lua.get_top() {
				$lua.dump_stack();
				panic!("Stack is dirty! Expected the stack to have {} elements, but it has {}!", top, $lua.get_top());
			}
			ret
		}

		#[cfg(not(debug_assertions))]
		$code
	}};

	( $lua:ident => $elem:literal => $code:block ) => {{
		#[cfg(debug_assertions)] {
			let ret = (|| $code)();
			if $lua.get_top() != $elem {
				$lua.dump_stack();
				panic!("Stack is dirty! Expected the stack to have ", $elem, " (fixed size) elements, but it has {}!", $lua.get_top());
			}
			ret
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
	pub i_ci: i32
}

#[inline(always)]
/// Loads lua_shared and imports all functions. This is already done for you if you add `#[gmod::gmod13_open]` to your `gmod13_open` function.
pub unsafe fn load() {
	import::LUA_SHARED.load()
}

thread_local! {
	#[cfg(debug_assertions)]
	static LUA: Cell<Option<State>> = Cell::new(None);

	#[cfg(not(debug_assertions))]
	static LUA: Cell<State> = Cell::new(State(std::ptr::null_mut()));
}
/// Acquires a pointer to the Lua state for the current thread.
///
/// This will panic if called from anywhere but the main thread. This will panic if you are not using the `#[gmod13_open]` macro to open the Lua state.
///
/// This will NOT panic in release mode under these conditions and will instead cause undefined behaviour.
pub unsafe fn state() -> State {
	LUA.with(|cell| {
		#[cfg(debug_assertions)] {
			cell.get().expect("The Lua state cannot be found in this thread. Perhaps you are calling this function from a thread other than the main thread? Perhaps you forgot to use the `#[gmod13_open]` macro?")
		}
		#[cfg(not(debug_assertions))] {
			cell.get()
		}
	})
}

#[doc(hidden)]
#[allow(non_snake_case)]
pub fn __set_state__internal(state: State) {
	LUA.with(|cell| {
		#[cfg(debug_assertions)]
		cell.set(Some(state));

		#[cfg(not(debug_assertions))]
		cell.set(state);
	})
}
