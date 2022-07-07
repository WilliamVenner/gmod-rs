use crate::lua::*;

pub trait CLuaFunction: Copy {}

macro_rules! impl_c_lua_function {
	($($($arg:ident) *;)*) => {
		$(
			impl<$($arg, )* R> CLuaFunction for extern "C-unwind" fn($($arg),*) -> R {}
			impl<$($arg, )* R> CLuaFunction for unsafe extern "C-unwind" fn($($arg),*) -> R {}
			impl<$($arg, )* R> CLuaFunction for extern "C" fn($($arg),*) -> R {}
			impl<$($arg, )* R> CLuaFunction for unsafe extern "C" fn($($arg),*) -> R {}
		)*
	};
}
impl_c_lua_function!(
	;
	T1;
	T1 T2;
	T1 T2 T3;
	T1 T2 T3 T4;
	T1 T2 T3 T4 T5;
	T1 T2 T3 T4 T5 T6;
	T1 T2 T3 T4 T5 T6 T7;
	T1 T2 T3 T4 T5 T6 T7 T8;
	T1 T2 T3 T4 T5 T6 T7 T8 T9;
	T1 T2 T3 T4 T5 T6 T7 T8 T9 T10;
	T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11;
	T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12;
	T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13;
	T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14;
	T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15;
	T1 T2 T3 T4 T5 T6 T7 T8 T9 T10 T11 T12 T13 T14 T15 T16;
);

impl State {
	#[inline(always)]
	/// Binds to a raw Lua C function.
	///
	/// If anything is missing from this library, you can use this function to bind it yourself.
	///
	/// Note, this may be a somewhat expensive operation, so storing its result in some way is recommended.
	pub unsafe fn raw_bind<F: CLuaFunction>(&self, symbol: &[u8]) -> Result<F, libloading::Error> {
		LUA_SHARED.library.get::<F>(symbol).map(|f| *f)
	}
}