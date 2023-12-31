use core::sync::atomic;
use core::fmt;
pub struct Writer(bool);
static LOGGING_LOCK: atomic::AtomicBool = atomic::AtomicBool::new(false);

impl Writer {
	pub fn get(module: &str) -> Writer {
		// This "acquires" the lock (actually just disables output if paralel writes are attempted
		let mut ret = Writer( ! LOGGING_LOCK.swap(true, atomic::Ordering::Acquire) );
		
		// Print the module name before returning (prefixes all messages)
		{
			use core::fmt::Write;
			let _ = write!(&mut ret, "[{}] ", module);
		}
		
		ret
	}
}

impl ::core::ops::Drop for Writer {
	fn drop(&mut self) {
		{
			use core::fmt::Write;
			let _ = write!(self, "\n");
		}
		// On drop, "release" the lock
		if self.0 {
			LOGGING_LOCK.store(false, atomic::Ordering::Release);
		}
	}
}

impl fmt::Write for Writer {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		// If the lock is owned by this instance, then we can safely write to the output
		if self.0 {
			unsafe {
				::arch::debug::puts(s);
			}
		}
		Ok( () )
	}
}

