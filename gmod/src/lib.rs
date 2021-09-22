#![feature(c_unwind)]

pub use libloading;
pub use detour;
pub use skidscan as sigscan;
pub use cstr;
pub use ctor::{ctor as dllopen, dtor as dllclose};
pub use gmod_macros::*;

/// Lua interface
pub mod lua;

/// Colorful printing
pub mod msgc;

/// Advanced dark magic utilities
pub mod hax;

/// Userdata types
pub mod userdata;

/// Returns whether this client is running the x86-64 branch
///
/// Current implementation checks the contents of the bin/ directory, so this is a blocking operation and requires syscalls, use sparingly
pub fn is_x86_64() -> bool {
	use std::path::PathBuf;
	#[cfg(target_os = "linux")] {
		PathBuf::from("bin/linux64").is_dir()
	}
	#[cfg(target_os = "windows")] {
		PathBuf::from("bin/win64").is_dir()
	}
}

/// Opens & returns a shared library loaded by Garry's Mod using the raw path to the module.
///
/// # Example
/// ```no_run
/// // This would only work on Windows x86-64 branch in 64-bit mode
/// let (engine, engine_path): (gmod::libloading::Library, &'static str) = open_library_srv!("bin/win64/engine.dll").expect("Failed to open engine.dll!");
/// println!("Opened engine.dll from: {}", engine_path);
/// ```
#[macro_export]
macro_rules! open_library_raw {
	($($path:literal),+) => {
		$crate::libloading::Library::new(concat!($($path),+)).map(|lib| (lib, concat!($($path),+)))
	}
}

/// Opens & returns a shared library loaded by Garry's Mod, in "server mode" (will prioritize _srv.so on Linux main branch)
///
/// Respects 32-bit/64-bit main/x86-64 branches and finds the correct library.
///
/// # Example
/// ```no_run
/// let (engine, engine_path): (gmod::libloading::Library, &'static str) = open_library_srv!("engine").expect("Failed to open engine.dll!");
/// println!("Opened engine.dll from: {}", engine_path);
/// ```
#[macro_export]
macro_rules! open_library_srv {
	($name:literal) => {{
		#[cfg(not(all(any(target_os = "windows", target_os = "linux"), any(target_pointer_width = "32", target_pointer_width = "64"))))] {
			compile_error!("Unsupported platform");
		}

		#[cfg(all(target_os = "windows", target_pointer_width = "64"))] {
			$crate::open_library_raw!("bin/win64/", $name, ".dll")
		}
		#[cfg(all(target_os = "windows", target_pointer_width = "32"))] {
			$crate::open_library_raw!("bin/", $name, ".dll")
			.or_else(|_| $crate::open_library_raw!("garrysmod/bin/", $name, ".dll"))
		}

		#[cfg(all(target_os = "linux", target_pointer_width = "64"))] {
			$crate::open_library_raw!("bin/linux64/", $name, ".so")
			.or_else(|_| $crate::open_library_raw!("bin/linux64/lib", $name, ".so"))
		}
		#[cfg(all(target_os = "linux", target_pointer_width = "32"))] {
			$crate::open_library_raw!("bin/linux32/", $name, ".so")
			.or_else(|_| $crate::open_library_raw!("bin/linux32/lib", $name, ".so"))
			.or_else(|_| $crate::open_library_raw!("bin/", $name, "_srv.so"))
			.or_else(|_| $crate::open_library_raw!("bin/lib", $name, "_srv.so"))
			.or_else(|_| $crate::open_library_raw!("garrysmod/bin/", $name, "_srv.so"))
			.or_else(|_| $crate::open_library_raw!("garrysmod/bin/lib", $name, "_srv.so"))
			.or_else(|_| $crate::open_library_raw!("bin/", $name, ".so"))
			.or_else(|_| $crate::open_library_raw!("bin/lib", $name, ".so"))
			.or_else(|_| $crate::open_library_raw!("garrysmod/bin/", $name, ".so"))
			.or_else(|_| $crate::open_library_raw!("garrysmod/bin/lib", $name, ".so"))
		}
	}};
}

/// Opens & returns a shared library loaded by Garry's Mod. You are most likely looking for `open_library_srv!`, as this will prioritize non-_srv.so libraries on Linux main branch.
///
/// Respects 32-bit/64-bit main/x86-64 branches and finds the correct library.
///
/// # Example
/// ```no_run
/// let (engine, engine_path): (gmod::libloading::Library, &'static str) = open_library!("engine").expect("Failed to open engine.dll!");
/// println!("Opened engine.dll from: {}", engine_path);
/// ```
#[macro_export]
macro_rules! open_library {
	($name:literal) => {{
		#[cfg(not(all(any(target_os = "windows", target_os = "linux"), any(target_pointer_width = "32", target_pointer_width = "64"))))] {
			compile_error!("Unsupported platform");
		}

		#[cfg(all(target_os = "windows", target_pointer_width = "64"))] {
			$crate::open_library_raw!("bin/win64/", $name, ".dll")
		}
		#[cfg(all(target_os = "windows", target_pointer_width = "32"))] {
			$crate::open_library_raw!("bin/", $name, ".dll")
			.or_else(|_| $crate::open_library_raw!("garrysmod/bin/", $name, ".dll"))
		}

		#[cfg(all(target_os = "linux", target_pointer_width = "64"))] {
			$crate::open_library_raw!("bin/linux64/", $name, ".so")
			.or_else(|_| $crate::open_library_raw!("bin/linux64/lib", $name, ".so"))
		}
		#[cfg(all(target_os = "linux", target_pointer_width = "32"))] {
			$crate::open_library_raw!("bin/linux32/", $name, ".so")
			.or_else(|_| $crate::open_library_raw!("bin/linux32/lib", $name, ".so"))
			.or_else(|_| $crate::open_library_raw!("bin/", $name, ".so"))
			.or_else(|_| $crate::open_library_raw!("bin/lib", $name, ".so"))
			.or_else(|_| $crate::open_library_raw!("garrysmod/bin/", $name, ".so"))
			.or_else(|_| $crate::open_library_raw!("garrysmod/bin/lib", $name, ".so"))
			.or_else(|_| $crate::open_library_raw!("bin/", $name, "_srv.so"))
			.or_else(|_| $crate::open_library_raw!("bin/lib", $name, "_srv.so"))
			.or_else(|_| $crate::open_library_raw!("garrysmod/bin/", $name, "_srv.so"))
			.or_else(|_| $crate::open_library_raw!("garrysmod/bin/lib", $name, "_srv.so"))
		}
	}};
}