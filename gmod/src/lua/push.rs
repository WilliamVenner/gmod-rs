use std::time::{Duration, SystemTime};

pub trait PushToLua: Sized {
	/// Pushes this value to the Lua stack.
	unsafe fn push_to_lua(self, lua: crate::lua::State);
}
pub trait TryPushToLua: Sized {
	/// Checked `push_to_lua` for types that may not fit in an `i32`
	unsafe fn try_push_to_lua(self, lua: crate::lua::State) -> Result<(), Self>;
}
pub trait ForcePushToLua: Sized {
	/// `push_to_lua` but may result in loss of data
	unsafe fn force_push_to_lua(self, lua: crate::lua::State);
}
pub trait PushCollectionToLua: Sized {
	/// Pushes this collection to a table at the top of the Lua stack.
	///
	/// **You must create the table yourself**
	unsafe fn push_to_lua_table(self, lua: crate::lua::State);
}

impl<P: PushToLua> TryPushToLua for P {
	#[inline]
	unsafe fn try_push_to_lua(self, lua: crate::lua::State) -> Result<(), Self> {
		self.push_to_lua(lua);
		Ok(())
	}
}
impl<P: PushToLua>ForcePushToLua for P {
	#[inline]
	unsafe fn force_push_to_lua(self, lua: crate::lua::State) {
		self.push_to_lua(lua);
	}
}

macro_rules! push_primitives {
	{$($ty:ty => $fn:ident),*} => {$(
		impl PushToLua for $ty {
			#[inline]
			unsafe fn push_to_lua(self, lua: crate::lua::State) {
				lua.$fn(self as _);
			}
		}
	)*};
}
macro_rules! try_push_primitives {
	{$($ty:ty => $fn:ident / $forcefn:ident),*} => {$(
		impl TryPushToLua for $ty {
			#[inline]
			unsafe fn try_push_to_lua(self, lua: crate::lua::State) -> Result<(), Self> {
				lua.$fn(match self.try_into() {
					Ok(v) => v,
					Err(e) => return Err(self)
				});
				Ok(())
			}
		}
		impl ForcePushToLua for $ty {
			#[inline]
			unsafe fn force_push_to_lua(self, lua: crate::lua::State) {
				lua.$forcefn(self as _);
			}
		}
	)*};
}

push_primitives! {
	&str => push_string,
	bool => push_boolean,
	f64 => push_number,
	f32 => push_number,
	u8 => push_integer,
	i8 => push_integer,
	u16 => push_integer,
	i16 => push_integer,
	i32 => push_integer
}
try_push_primitives! {
	u32 => push_integer / push_number,
	i64 => push_integer / push_number,
	u64 => push_integer / push_number,
	u128 => push_integer / push_number,
	i128 => push_integer / push_number
}

impl PushToLua for String {
	#[inline]
	unsafe fn push_to_lua(self, lua: crate::lua::State) {
		lua.push_string(&self);
	}
}
impl PushToLua for Vec<u8> {
	#[inline]
	unsafe fn push_to_lua(self, lua: crate::lua::State) {
		lua.push_binary_string(&self);
	}
}
impl PushToLua for &[u8] {
	#[inline]
	unsafe fn push_to_lua(self, lua: crate::lua::State) {
		lua.push_binary_string(&self);
	}
}
impl PushToLua for Duration {
	#[inline]
	unsafe fn push_to_lua(self, lua: crate::lua::State) {
		lua.push_number(self.as_secs_f64());
	}
}
impl<T: PushToLua> PushToLua for Option<T> {
	#[inline]
	unsafe fn push_to_lua(self, lua: crate::lua::State) {
		match self {
			Some(val) => val.push_to_lua(lua),
			None => lua.push_nil()
		}
	}
}
impl<K: PushToLua, V: PushToLua> PushCollectionToLua for std::collections::BTreeMap<K, V> {
	#[inline]
	unsafe fn push_to_lua_table(self, lua: crate::lua::State) {
		for (k, v) in self {
			k.push_to_lua(lua);
			v.push_to_lua(lua);
			lua.set_table(-3);
		}
	}
}
impl<T: PushToLua> PushCollectionToLua for Vec<T> {
	#[inline]
	unsafe fn push_to_lua_table(self, lua: crate::lua::State) {
		iterator(lua, &mut self.into_iter())
	}
}

impl TryPushToLua for SystemTime {
	#[inline]
	unsafe fn try_push_to_lua(self, lua: crate::lua::State) -> Result<(), Self> {
		lua.push_number(self.duration_since(SystemTime::UNIX_EPOCH).map_err(|_| self)?.as_secs_f64());
		Ok(())
	}
}

/// Pushes all elements in an iterator to a Lua table at the top of the stack.
///
/// **You must create the table yourself**
#[inline]
pub unsafe fn iterator<T: PushToLua, I: Iterator<Item = T>>(lua: crate::lua::State, iter: &mut I) {
	for (i, val) in iter.enumerate() {
		lua.push_integer((i + 1) as _);
		val.push_to_lua(lua);
		lua.set_table(-3);
	}
}