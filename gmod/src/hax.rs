#[macro_export]
macro_rules! __vtable_offset {
	($name:ident = {
		win64: $win64:literal,
		win32: $win32:literal,

		linux64: $linux64:literal,
		linux32: $linux32:literal
	}) => {
		#[cfg(all(target_os = "windows", target_pointer_width = "64"))]
		pub const $name: usize = $win64;

		#[cfg(all(target_os = "windows", target_pointer_width = "32"))]
		pub const $name: usize = $win32;

		#[cfg(all(target_os = "linux", target_pointer_width = "64"))]
		pub const $name: usize = $linux64;

		#[cfg(all(target_os = "linux", target_pointer_width = "32"))]
		pub const $name: usize = $linux32;
	};
}

#[macro_export]
macro_rules! __vtable_func {
	($ty:ident = extern fn($($ident:ident: $arg:ty),*) $(-> $rtn:ty)?) => {
		#[cfg(target_pointer_width = "64")]
		pub type $ty = extern "fastcall" fn($($ident: $arg),*) $(-> $rtn)?;

		#[cfg(all(target_os = "windows", target_pointer_width = "32"))]
		pub type $ty = extern "thiscall" fn($($ident: $arg),*) $(-> $rtn)?;

		#[cfg(all(target_os = "linux", target_pointer_width = "32"))]
		pub type $ty = extern "C" fn($($ident: $arg),*) $(-> $rtn)?;
	}
}

#[macro_export]
macro_rules! __hook_func {
	($ty:ident = extern fn $fn:ident($($ident:ident: $arg:ty),*) $(-> $rtn:ty)? $code:block) => {
		#[cfg(target_pointer_width = "64")]
		type $ty = extern "fastcall" fn($($ident: $arg),*) $(-> $rtn)?;

		#[cfg(all(target_os = "windows", target_pointer_width = "32"))]
		type $ty = extern "thiscall" fn($($ident: $arg),*) $(-> $rtn)?;

		#[cfg(all(target_os = "linux", target_pointer_width = "32"))]
		type $ty = extern "C" fn($($ident: $arg),*) $(-> $rtn)?;

		#[cfg(target_pointer_width = "64")]
		extern "fastcall" fn $fn($($ident: $arg),*) $(-> $rtn)? $code

		#[cfg(all(target_os = "windows", target_pointer_width = "32"))]
		extern "thiscall" fn $fn($($ident: $arg),*) $(-> $rtn)? $code

		#[cfg(all(target_os = "linux", target_pointer_width = "32"))]
		extern "C" fn $fn($($ident: $arg),*) $(-> $rtn)? $code
	};
}