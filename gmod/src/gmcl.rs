use std::{sync::{Arc, Mutex, TryLockError, atomic::AtomicBool}, time::Duration, os::raw::c_char, thread::JoinHandle};

lazy_static::lazy_static! {
	static ref STDOUT_OVERRIDE_THREAD: Mutex<Option<JoinHandle<()>>> = Mutex::new(None);
}

static SHUTDOWN_FLAG: AtomicBool = AtomicBool::new(false);

/// This function will **permanently** redirect stdout to the client console.
///
/// This allows for `println!()` and friends to print to the client console.
///
/// # IMPORTANT
///
/// You must undo this action when your module is unloaded or the game will crash.
///
/// This will be done automatically for you if you use the `#[gmod13_close]` attribute macro, otherwise, please call `gmod::gmcl::restore_stdout()` in your custom `gmod13_close` function.
pub fn override_stdout() {
	let mut join_handle = STDOUT_OVERRIDE_THREAD.lock().unwrap();

	if join_handle.is_some() {
		// We don't need to override twice
		return;
	}

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
		join_handle.replace(std::thread::spawn(move || loop {
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
			if SHUTDOWN_FLAG.load(std::sync::atomic::Ordering::Relaxed) {
				break;
			}
			std::thread::sleep(Duration::from_millis(250));
		}));

		std::io::set_output_capture(Some(output_buf_ref));
	};
}

/// Undoes `gmod::gmcl::override_stdout`. You must call this function in a custom `gmod13_close` function (you are not using the crate's provided `#[gmod13_close]` attribute macro) if you override stdout.
pub fn restore_stdout() {
	SHUTDOWN_FLAG.store(true, std::sync::atomic::Ordering::Release);

	if let Some(join_handle) = STDOUT_OVERRIDE_THREAD.lock().unwrap().take() {
		let _ = join_handle.join();
	}

	std::io::set_output_capture(None); // TODO fix side effect
}