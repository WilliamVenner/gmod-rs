#![allow(non_upper_case_globals)]

use std::os::raw::c_char;

#[inline(always)]
pub fn printf_escape<S: AsRef<str>>(str: S) -> String {
	str.as_ref().replace('\\', "\\\\").replace('%', "%%")
}

#[repr(C)]
pub struct Color {
	r: u8,
	g: u8,
	b: u8,
	a: u8
}
impl Color {
	#[inline(always)]
	pub const fn new(r: u8, g: u8, b: u8) -> Color {
		Color { r, g, b, a: 255 }
	}
}

lazy_static::lazy_static! {
	pub static ref ConColorMsg: libloading::Symbol<'static, unsafe extern "C" fn(&Color, *const c_char, ...)> = unsafe {
		#[cfg(all(target_os = "windows", target_pointer_width = "64"))]
		let lib = libloading::Library::new("bin/win64/tier0.dll").expect("Failed to open tier0.dll");

		#[cfg(all(target_os = "windows", target_pointer_width = "32"))]
		let lib = libloading::Library::new("bin/tier0.dll").or_else(|_| libloading::Library::new("bin/win32/tier0.dll")).expect("Failed to open tier0.dll");

		#[cfg(all(target_os = "linux", target_pointer_width = "64"))]
		let lib = libloading::Library::new("bin/linux64/libtier0.so").expect("Failed to open libtier0.so");

		#[cfg(all(target_os = "linux", target_pointer_width = "32"))]
		let lib = libloading::Library::new("bin/libtier0_srv.so").or_else(|_| libloading::Library::new("bin/linux32/libtier0.so")).expect("Failed to open libtier0.so");

		#[cfg(target_os = "macos")]
		let lib = libloading::Library::new("bin/libtier0.dylib").or_else(|_| libloading::Library::new("GarrysMod_Signed.app/Contents/MacOS/libtier0.dylib")).expect("Failed to open libtier0.dylib");

		let lib = Box::leak(Box::new(lib));
		{
			#[cfg(all(target_os = "windows", target_pointer_width = "64"))] {
				lib.get(b"?ConColorMsg@@YAXAEBVColor@@PEBDZZ\0")
			}
			#[cfg(all(target_os = "windows", target_pointer_width = "32"))] {
				match lib.get(b"?ConColorMsg@@YAXABVColor@@PBDZZ\0") {
					Ok(symbol) => Ok(symbol),
					Err(_) => lib.get(b"?ConColorMsg@@YAXABVColor@@PBDZZ\0")
				}
			}
			#[cfg(any(target_os = "linux", target_os = "macos"))] {
				lib.get(b"_Z11ConColorMsgRK5ColorPKcz\0")
			}
		}
		.expect("Failed to get ConColorMsg")
	};
}
#[macro_export]
macro_rules! colormsg {
	($($arg:tt),+) => {
		$($crate::colormsg!(@print $arg));+
	};

	(@print [$r:literal, $g:literal, $b:literal] $fmt:literal % ($($arg:tt),+)) => {
		$crate::msgc::ConColorMsg(
			&$crate::msgc::Color::new($r, $g, $b),
			$crate::msgc::printf_escape(format!(concat!($fmt, '\0'), $($arg),+)).as_ptr() as *const _,
		)
	};

	(@print [$r:literal, $g:literal, $b:literal] $str:literal) => {
		$crate::msgc::ConColorMsg(
			&$crate::msgc::Color::new($r, $g, $b),
			$crate::msgc::printf_escape(concat!($str, '\0')).as_ptr() as *const _,
		)
	};
}
