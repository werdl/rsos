#![feature(panic_info_message)]	//< Panic handling
#![no_std]	//< Kernels can't use std

mod libr;

/// Macros, need to be loaded before everything else due to how rust parses
macro_rules! log{
	( $($arg:tt)* ) => ({
		// Import the Writer trait (required by write!)
		use core::fmt::Write;
		let _ = write!(&mut ::logging::Writer::get(module_path!()), $($arg)*);
	})
}

// Achitecture-specific modules
#[cfg(target_arch="x86_64")] #[path="arch/amd64/mod.rs"]
pub mod arch;

/// Exception handling (panic)
use libr::unwind;

/// Logging code
use libr::logging;

// Kernel entrypoint (called by start.S)
#[no_mangle]
pub fn kmain()
{
	log!("Hello world! 1={}", 1);
	loop {}
}

