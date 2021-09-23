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