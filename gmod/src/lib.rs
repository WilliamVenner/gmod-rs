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