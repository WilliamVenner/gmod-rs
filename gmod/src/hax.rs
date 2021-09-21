#[macro_export]
/// Common pattern for detouring.
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
/// Common pattern for detouring.
/// # Example
/// ```norun
/// __callingconv_func! {
///     #[win32 = "stdcall"]
///     #[win64 = "stdcall"]
///     #[linux32 = "C"]
///     #[linux64 = "C"]
/// 	CVoiceGameMgrHelper_CanPlayerHearPlayer = extern fn(listener: *mut c_void, talker: *mut c_void) -> bool {
/// 	    println!("listener {:?} talker {:?}", listener, talker);
/// 	    false
///     }
/// }
/// ```
macro_rules! __callingconv_func {
	{ @callingconv $ty:ident = $vis:vis extern $callingconv:tt fn $fn_ident:ident($($ident:ident: $arg:ty),*) $(-> $rtn:ty)? $code:block } => {
		$vis type $ty = extern $callingconv fn($($ident: $arg),*) $(-> $rtn)?;
		$vis extern $callingconv fn $fn_ident($($ident: $arg),*) $(-> $rtn)? $code
	};

	{ @callingconv $ty:ident = $vis:vis extern $callingconv:tt fn($($ident:ident: $arg:ty),*) $(-> $rtn:ty)? } => {
		$vis type $ty = extern $callingconv fn($($ident: $arg),*) $(-> $rtn)?;
	};

	{ #[win32 = $win32:tt] #[win64 = $win64:tt] #[linux32 = $linux32:tt] #[linux64 = $linux64:tt] $ty:ident = $vis:vis extern fn $($fn_ident:ident)?($($ident:ident: $arg:ty),*) $(-> $rtn:ty)? $code:block } => {
		#[cfg(all(target_os = "windows", target_pointer_width = "32"))]
		$crate::__callingconv_func! { @callingconv $ty = $vis extern $win32 fn $($fn_ident)?($($ident: $arg),*) $(-> $rtn)? $code }

		#[cfg(all(target_os = "windows", target_pointer_width = "64"))]
		$crate::__callingconv_func! { @callingconv $ty = $vis extern $win64 fn $($fn_ident)?($($ident: $arg),*) $(-> $rtn)? $code }

		#[cfg(all(target_os = "linux", target_pointer_width = "32"))]
		$crate::__callingconv_func! { @callingconv $ty = $vis extern $linux32 fn $($fn_ident)?($($ident: $arg),*) $(-> $rtn)? $code }

		#[cfg(all(target_os = "linux", target_pointer_width = "64"))]
		$crate::__callingconv_func! { @callingconv $ty = $vis extern $linux64 fn $($fn_ident)?($($ident: $arg),*) $(-> $rtn)? $code }
	};

	{ #[win32 = $win32:tt] #[win64 = $win64:tt] #[linux32 = $linux32:tt] #[linux64 = $linux64:tt] $ty:ident = $vis:vis extern fn $($fn_ident:ident)?($($ident:ident: $arg:ty),*) $(-> $rtn:ty)? } => {
		#[cfg(all(target_os = "windows", target_pointer_width = "32"))]
		$crate::__callingconv_func! { @callingconv $ty = $vis extern $win32 fn $($fn_ident)?($($ident: $arg),*) $(-> $rtn)? }

		#[cfg(all(target_os = "windows", target_pointer_width = "64"))]
		$crate::__callingconv_func! { @callingconv $ty = $vis extern $win64 fn $($fn_ident)?($($ident: $arg),*) $(-> $rtn)? }

		#[cfg(all(target_os = "linux", target_pointer_width = "32"))]
		$crate::__callingconv_func! { @callingconv $ty = $vis extern $linux32 fn $($fn_ident)?($($ident: $arg),*) $(-> $rtn)? }

		#[cfg(all(target_os = "linux", target_pointer_width = "64"))]
		$crate::__callingconv_func! { @callingconv $ty = $vis extern $linux64 fn $($fn_ident)?($($ident: $arg),*) $(-> $rtn)? }
	};
}

#[macro_export]
/// Common pattern for detouring.
macro_rules! __gmod_func {
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
/// Common pattern for detouring.
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

#[macro_export]
/// Common pattern for detouring.
macro_rules! find_gmod_signature {
	(($library:ident, $library_path:ident), @EXPORT = $export:literal) => {
		$library.get(concat!($export, '\0').as_bytes()).ok().map(|func: ::gmod::libloading::Symbol<'_, _>| *func)
	};

	(($library:ident, $library_path:ident), @SIG = $sig:literal) => {
		$crate::sigscan::signature!($sig).scan_module($library_path).ok().map(|x| std::mem::transmute(x))
	};

	(($library:ident, $library_path:ident) -> {
		win64_x86_64: [$($win64_x86_64:tt)+],
		win32_x86_64: [$($win32_x86_64:tt)+],

		linux64_x86_64: [$($linux64_x86_64:tt)+],
		linux32_x86_64: [$($linux32_x86_64:tt)+],

		win32: [$($win32:tt)+],
		linux32: [$($linux32:tt)+],
	}) => {{
		let x86_64 = $crate::is_x86_64();
		if x86_64 {
			#[cfg(all(target_os = "windows", target_pointer_width = "64"))] {
				$crate::find_gmod_signature!(($library, $library_path), $($win64_x86_64)+)
			}
			#[cfg(all(target_os = "windows", target_pointer_width = "32"))] {
				$crate::find_gmod_signature!(($library, $library_path), $($win32_x86_64)+)
			}
			#[cfg(all(target_os = "linux", target_pointer_width = "64"))] {
				$crate::find_gmod_signature!(($library, $library_path), $($linux64_x86_64)+)
			}
			#[cfg(all(target_os = "linux", target_pointer_width = "32"))] {
				$crate::find_gmod_signature!(($library, $library_path), $($linux32_x86_64)+)
			}
		} else {
			#[cfg(target_os = "windows")] {
				$crate::find_gmod_signature!(($library, $library_path), $($win32)+)
			}
			#[cfg(target_os = "linux")] {
				$crate::find_gmod_signature!(($library, $library_path), $($linux32)+)
			}
		}
	}}
}