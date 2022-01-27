#[cfg(debug_assertions)]
use std::sync::atomic::AtomicI64;

use std::{cell::UnsafeCell, ffi::c_void};

use libloading::{Library, Symbol};

use super::{LuaError, State as LuaState, LuaDebug, returns::ValuesReturned};

pub type LuaInt = isize;
pub type LuaSize = usize;
pub type LuaString = *const std::os::raw::c_char;
pub type LuaFunction = unsafe extern "C-unwind" fn(state: LuaState) -> i32;
pub type LuaNumber = f64;
pub type LuaReference = i32;

pub const LUA_REGISTRYINDEX: i32 = -10000;
pub const LUA_ENVIRONINDEX: i32 = -10001;
pub const LUA_GLOBALSINDEX: i32 = -10002;

pub const LUA_MULTRET: i32 = -1;
pub const LUA_NOREF: LuaReference = -2;
pub const LUA_REFNIL: LuaReference = -1;

pub const LUA_TNONE: i32 = -1;
pub const LUA_TNIL: i32 = 0;
pub const LUA_TBOOLEAN: i32 = 1;
pub const LUA_TLIGHTUSERDATA: i32 = 2;
pub const LUA_TNUMBER: i32 = 3;
pub const LUA_TSTRING: i32 = 4;
pub const LUA_TTABLE: i32 = 5;
pub const LUA_TFUNCTION: i32 = 6;
pub const LUA_TUSERDATA: i32 = 7;
pub const LUA_TTHREAD: i32 = 8;

pub const LUA_OK: i32 = 0;
pub const LUA_YIELD: i32 = 1;
pub const LUA_ERRRUN: i32 = 2;
pub const LUA_ERRSYNTAX: i32 = 3;
pub const LUA_ERRMEM: i32 = 4;
pub const LUA_ERRERR: i32 = 5;
pub const LUA_ERRFILE: i32 = LUA_ERRERR + 1;

pub const LUA_IDSIZE: usize = 60;

impl LuaError {
	fn get_error_message(lua_state: LuaState) -> Option<String> {
		unsafe { lua_state.get_string(-1).map(|str| str.into_owned()) }
	}

	pub(crate) fn from_lua_state(lua_state: LuaState, lua_int_error_code: i32) -> Self {
		use super::LuaError::*;
		match lua_int_error_code {
			LUA_ERRMEM => MemoryAllocationError,
			LUA_ERRERR => ErrorHandlerError,
			LUA_ERRSYNTAX | LUA_ERRRUN | LUA_ERRFILE => {
				let msg = LuaError::get_error_message(lua_state);
				match lua_int_error_code {
					LUA_ERRSYNTAX => SyntaxError(msg),
					LUA_ERRRUN => RuntimeError(msg),
					LUA_ERRFILE => FileError(msg),
					_ => unreachable!(),
				}
			}
			_ => Unknown(lua_int_error_code),
		}
	}
}

#[cfg_attr(not(debug_assertions), repr(transparent))]
pub struct LuaSharedInterface(pub(crate) UnsafeCell<*mut LuaShared>, #[cfg(debug_assertions)] AtomicI64);
impl LuaSharedInterface {
	#[cfg(debug_assertions)]
	pub(crate) fn debug_assertions(&self) {
		assert!(!unsafe { *self.0.get() }.is_null(), "The Lua state has not been initialized yet. Add `#[gmod::gmod13_open]` to your module's gmod13_open function to fix this. You can also manually load the Lua state with `gmod::load_lua_state()` or `gmod::set_lua_state(*mut c_void)`");

		let thread_id = u64::from(std::thread::current().id().as_u64()) as i64;
		match self.1.compare_exchange(-1, thread_id, std::sync::atomic::Ordering::SeqCst, std::sync::atomic::Ordering::SeqCst) {
			Ok(-1) => {}, // This is the first thread to use this Lua state.
			Ok(_) => unreachable!(),
			Err(remembered_thread_id) => assert_eq!(thread_id, remembered_thread_id, "Tried to access the Lua state from another thread! The Lua state is NOT thread-safe, and should only be accessed from the main thread.")
		}
	}

	pub(super) unsafe fn load(&self) {
		*self.0.get() = Box::leak(Box::new(LuaShared::import()));
	}

	pub(super) unsafe fn set(&self, ptr: *mut c_void) {
		*self.0.get() = ptr as *mut LuaShared;
	}
}
impl std::ops::Deref for LuaSharedInterface {
	type Target = LuaShared;

	#[inline]
	fn deref(&self) -> &Self::Target {
		#[cfg(debug_assertions)]
		self.debug_assertions();

		unsafe { &**self.0.get() }
	}
}
impl std::ops::DerefMut for LuaSharedInterface {
	#[inline]
	fn deref_mut(&mut self) -> &mut Self::Target {
		#[cfg(debug_assertions)]
		self.debug_assertions();

		unsafe { &mut **self.0.get_mut() }
	}
}

pub static mut LUA_SHARED: LuaSharedInterface = LuaSharedInterface(UnsafeCell::new(std::ptr::null_mut()), #[cfg(debug_assertions)] AtomicI64::new(-1));

pub struct LuaShared {
	pub lual_newstate: Symbol<'static, unsafe extern "C-unwind" fn() -> LuaState>,
	pub lual_openlibs: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState)>,
	pub lual_loadfile: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, path: LuaString) -> i32>,
	pub lual_loadstring: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, path: LuaString) -> i32>,
	pub lual_loadbuffer: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, buff: LuaString, sz: LuaSize, name: LuaString) -> i32>,
	pub lua_getfield: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, index: i32, k: LuaString)>,
	pub lua_pushvalue: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, index: i32)>,
	pub lua_pushlightuserdata: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, data: *mut c_void)>,
	pub lua_pushboolean: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, bool: i32)>,
	pub lua_tolstring: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, index: i32, out_size: *mut LuaSize) -> LuaString>,
	pub lua_pcall: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, nargs: i32, nresults: i32, errfunc: i32) -> i32>,
	pub lua_remove: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, index: i32)>,
	pub lua_gettop: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState) -> i32>,
	pub lua_type: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, index: i32) -> i32>,
	pub lua_typename: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, lua_type_id: i32) -> LuaString>,
	pub lua_setfield: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, index: i32, k: LuaString)>,
	pub lua_call: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, nargs: i32, nresults: i32)>,
	pub lua_createtable: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, narr: i32, nrec: i32)>,
	pub lua_settop: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, count: i32)>,
	pub lua_replace: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, index: i32)>,
	pub lua_pushlstring: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, data: LuaString, length: LuaSize)>,
	pub lua_pushcclosure: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, func: LuaFunction, upvalues: i32)>,
	pub lua_settable: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, index: i32)>,
	pub lua_gettable: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, index: i32)>,
	pub lua_error: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState) -> i32>,
	pub lua_insert: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, index: i32)>,
	pub lual_checkinteger: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, arg: i32) -> LuaInt>,
	pub lual_checklstring: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, arg: i32, out_size: *mut LuaSize) -> LuaString>,
	pub lua_toboolean: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, index: i32) -> i32>,
	pub lual_checktype: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, index: i32, r#type: i32)>,
	pub lua_setmetatable: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, index: i32) -> i32>,
	pub lua_pushinteger: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, int: LuaInt)>,
	pub lua_pushnumber: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, int: LuaNumber)>,
	pub lua_pushnil: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState)>,
	pub lual_checknumber: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, arg: i32) -> LuaNumber>,
	pub lua_tointeger: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, index: i32) -> LuaInt>,
	pub lua_tonumber: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, index: i32) -> LuaNumber>,
	pub lual_checkudata: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, arg: i32, name: LuaString) -> *mut std::ffi::c_void>,
	pub lual_ref: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, index: i32) -> i32>,
	pub lual_unref: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, index: i32, r#ref: i32)>,
	pub lua_objlen: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, index: i32) -> i32>,
	pub lua_rawgeti: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, t: i32, index: i32)>,
	pub lua_rawseti: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, t: i32, index: i32)>,
	pub lua_getmetatable: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, index: i32) -> i32>,
	pub lua_rawequal: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, a: i32, b: i32) -> i32>,
	pub lua_touserdata: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, index: i32) -> *mut std::ffi::c_void>,
	pub lua_getinfo: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, what: LuaString, ar: *mut LuaDebug) -> i32>,
	pub lua_getstack: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, level: i32, ar: *mut LuaDebug) -> i32>,
	pub lua_next: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, index: i32) -> i32>,
	pub lua_topointer: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, index: i32) -> *const c_void>,
	pub lua_newuserdata: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, size: usize) -> *mut c_void>,
	pub lual_newmetatable: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, name: LuaString) -> i32>,
	pub lua_resume: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, narg: i32) -> i32>,
	pub lua_newthread: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState) -> LuaState>,
	pub lua_yield: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, nresults: i32) -> i32>,
	pub lua_pushthread: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState) -> i32>,
	pub lua_tothread: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState, index: i32) -> LuaState>,
	pub lua_status: Symbol<'static, unsafe extern "C-unwind" fn(state: LuaState) -> i32>,
	pub lua_xmove: Symbol<'static, unsafe extern "C-unwind" fn(thread1: LuaState, thread2: LuaState, n: i32)>,
}
unsafe impl Sync for LuaShared {}
impl LuaShared {
	fn import() -> Self {
		unsafe {
			let (library, path) = Self::find_lua_shared();
			let library = Box::leak(Box::new(library)); // Keep this library referenced forever

			macro_rules! find_symbol {
				( $symbol:literal ) => {
					Self::find_symbol(library, concat!($symbol, "\0").as_bytes())
				};
			}

			Self {
				lual_newstate: find_symbol!("luaL_newstate"),
				lual_openlibs: find_symbol!("luaL_openlibs"),
				lua_pushlightuserdata: find_symbol!("lua_pushlightuserdata"),
				lual_checktype: find_symbol!("luaL_checktype"),
				lual_loadfile: find_symbol!("luaL_loadfile"),
				lual_loadstring: find_symbol!("luaL_loadstring"),
				lual_loadbuffer: find_symbol!("luaL_loadbuffer"),
				lua_getfield: find_symbol!("lua_getfield"),
				lua_pushvalue: find_symbol!("lua_pushvalue"),
				lua_pushboolean: find_symbol!("lua_pushboolean"),
				lua_tolstring: find_symbol!("lua_tolstring"),
				lua_pcall: find_symbol!("lua_pcall"),
				lua_remove: find_symbol!("lua_remove"),
				lua_gettop: find_symbol!("lua_gettop"),
				lua_type: find_symbol!("lua_type"),
				lua_typename: find_symbol!("lua_typename"),
				lua_setfield: find_symbol!("lua_setfield"),
				lua_call: find_symbol!("lua_call"),
				lua_createtable: find_symbol!("lua_createtable"),
				lua_settop: find_symbol!("lua_settop"),
				lua_replace: find_symbol!("lua_replace"),
				lua_pushlstring: find_symbol!("lua_pushlstring"),
				lua_pushcclosure: find_symbol!("lua_pushcclosure"),
				lua_settable: find_symbol!("lua_settable"),
				lua_gettable: find_symbol!("lua_gettable"),
				lua_error: find_symbol!("lua_error"),
				lua_insert: find_symbol!("lua_insert"),
				lual_checkinteger: find_symbol!("luaL_checkinteger"),
				lual_checklstring: find_symbol!("luaL_checklstring"),
				lua_toboolean: find_symbol!("lua_toboolean"),
				lua_pushnumber: find_symbol!("lua_pushnumber"),
				lua_pushinteger: find_symbol!("lua_pushinteger"),
				lua_pushnil: find_symbol!("lua_pushnil"),
				lual_checknumber: find_symbol!("luaL_checknumber"),
				lua_tointeger: find_symbol!("lua_tointeger"),
				lua_tonumber: find_symbol!("lua_tonumber"),
				lual_checkudata: find_symbol!("luaL_checkudata"),
				lual_ref: find_symbol!("luaL_ref"),
				lual_unref: find_symbol!("luaL_unref"),
				lua_setmetatable: find_symbol!("lua_setmetatable"),
				lua_objlen: find_symbol!("lua_objlen"),
				lua_rawgeti: find_symbol!("lua_rawgeti"),
				lua_rawseti: find_symbol!("lua_rawseti"),
				lua_getmetatable: find_symbol!("lua_getmetatable"),
				lua_rawequal: find_symbol!("lua_rawequal"),
				lua_touserdata: find_symbol!("lua_touserdata"),
				lua_getinfo: find_symbol!("lua_getinfo"),
				lua_getstack: find_symbol!("lua_getstack"),
				lua_next: find_symbol!("lua_next"),
				lua_topointer: find_symbol!("lua_topointer"),
				lua_newuserdata: find_symbol!("lua_newuserdata"),
				lual_newmetatable: find_symbol!("luaL_newmetatable"),
				lua_resume: find_symbol!("lua_resume_real"),
				lua_newthread: find_symbol!("lua_newthread"),
				lua_yield: find_symbol!("lua_yield"),
				lua_pushthread: find_symbol!("lua_pushthread"),
				lua_tothread: find_symbol!("lua_tothread"),
				lua_status: find_symbol!("lua_status"),
				lua_xmove: find_symbol!("lua_xmove"),
			}
		}
	}

	unsafe fn find_symbol<T>(library: &'static Library, name: &[u8]) -> Symbol<'static, T> {
		match library.get(name) {
			Ok(symbol) => symbol,
			Err(err) => panic!("Failed to find symbol \"{}\"\n{:#?}", String::from_utf8_lossy(name), err),
		}
	}

	#[cfg(all(target_os = "windows", target_pointer_width = "64"))]
	pub unsafe fn find_lua_shared() -> (Library, &'static str) {
		crate::open_library_raw!("bin/win64/lua_shared.dll")
		.expect("Failed to load lua_shared.dll")
	}

	#[cfg(all(target_os = "windows", target_pointer_width = "32"))]
	pub unsafe fn find_lua_shared() -> (Library, &'static str) {
		crate::__private__gmod_rs__try_chained_open! {
			crate::open_library_raw!("garrysmod/bin/lua_shared.dll"),
			crate::open_library_raw!("bin/lua_shared.dll")
		}
		.expect("Failed to load lua_shared.dll")
	}

	#[cfg(all(target_os = "linux", target_pointer_width = "32"))]
	pub unsafe fn find_lua_shared() -> (Library, &'static str) {
		crate::__private__gmod_rs__try_chained_open! {
			crate::open_library_raw!("garrysmod/bin/lua_shared_srv.so"),
			crate::open_library_raw!("bin/linux32/lua_shared.so"),
			crate::open_library_raw!("garrysmod/bin/lua_shared.so")
		}
		.expect("Failed to find lua_shared.so or lua_shared_srv.so")
	}

	#[cfg(all(target_os = "linux", target_pointer_width = "64"))]
	pub unsafe fn find_lua_shared() -> (Library, &'static str) {
		crate::open_library_raw!("bin/linux64/lua_shared.so")
		.expect("Failed to find lua_shared.so")
	}
}
