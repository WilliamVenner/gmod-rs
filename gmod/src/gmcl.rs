use std::{sync::{Arc, Mutex, TryLockError}, time::Duration, os::raw::c_char};

/// This function will **permanently** redirect stdout to the client console.
///
/// This allows for `println!()` and friends to print to the client console.
pub fn override_stdout() {
	unsafe {
		let (_lib, _path) = crate::open_library!("tier0").expect("Failed to open tier0.dll");

		#[allow(non_snake_case)]
		let ConMsg: extern "C" fn(*const c_char, ...) = *{
			#[cfg(target_os = "windows")] {
				_lib.get({
					#[cfg(all(target_os = "windows", target_pointer_width = "64"))] {
						b"?ConMsg@@YAXPEBDZZ\0"
					}
					#[cfg(all(target_os = "windows", target_pointer_width = "32"))] {
						b"?ConMsg@@YAXPBDZZ\0"
					}
				})
			}
			#[cfg(target_os = "linux")] {
				_lib.get(b"ConMsg\0").or_else(|_| _lib.get(b"_Z6ConMsgPKcz\0"))
			}
		}.expect("Failed to find ConMsg");

		let output_buf = Arc::new(Mutex::new(Vec::new()));
		let output_buf_ref = output_buf.clone();

		// This is actually a really dumb implementation, but appears to be the only way, unfortunately.
		std::thread::spawn(move || loop {
			match output_buf.try_lock() {
				Ok(mut data) => if !data.is_empty() {
					data.push(0); // cheeky
					ConMsg(data.as_ptr() as *const i8);

					data.truncate(0);
				},
				Err(TryLockError::Poisoned(err)) => panic!("{}", err),
				Err(TryLockError::WouldBlock) => {
					std::hint::spin_loop();
					std::thread::yield_now();
					continue
				}
			}
			std::thread::sleep(Duration::from_millis(250));
		});

		std::io::set_output_capture(Some(output_buf_ref));
	};
}