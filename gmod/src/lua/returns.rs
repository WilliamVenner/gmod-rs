use std::num::NonZeroI32;

#[repr(transparent)]
pub struct ValuesReturned(pub i32);

impl Into<i32> for ValuesReturned {
	#[inline(always)]
	fn into(self) -> i32 {
		self.0
	}
}

impl From<i32> for ValuesReturned {
	#[inline(always)]
	fn from(n: i32) -> Self {
		ValuesReturned(n)
	}
}

impl From<NonZeroI32> for ValuesReturned {
	#[inline(always)]
	fn from(n: NonZeroI32) -> ValuesReturned {
		ValuesReturned(i32::from(n))
	}
}

impl From<()> for ValuesReturned {
	#[inline(always)]
	fn from(_: ()) -> ValuesReturned {
		ValuesReturned(0)
	}
}

impl From<Option<NonZeroI32>> for ValuesReturned {
	#[inline(always)]
	fn from(opt: Option<NonZeroI32>) -> ValuesReturned {
		ValuesReturned(match opt {
			Some(vals) => i32::from(vals),
			None => {
				unsafe { super::state().push_nil() };
				1
			},
		})
	}
}

impl<E: std::fmt::Debug> From<Result<i32, E>> for ValuesReturned {
	#[inline(always)]
	fn from(res: Result<i32, E>) -> ValuesReturned {
		match res {
			Ok(vals) => ValuesReturned(vals),
			Err(err) => unsafe { super::state().error(&format!("{:?}", err)) }
		}
	}
}