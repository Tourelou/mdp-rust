use std::io::{self, Read, Write};
use std::os::unix::io::AsRawFd;

// --- Configuration Système ---

#[cfg(target_os = "linux")]
#[repr(C)]
#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct termios {
	pub c_iflag: u32,
	pub c_oflag: u32,
	pub c_cflag: u32,
	pub c_lflag: u32,
	pub c_line: u8,
	pub c_cc: [u8; 32],
	pub c_ispeed: u32,
	pub c_ospeed: u32,
}
#[cfg(target_os = "linux")] const ECHO: u32 = 0o000010;
#[cfg(target_os = "linux")] const ICANON: u32 = 0o000002;

#[cfg(target_os = "macos")]
#[repr(C)]
#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
pub struct termios {
	pub c_iflag: usize,
	pub c_oflag: usize,
	pub c_cflag: usize,
	pub c_lflag: usize,
	pub c_cc: [u8; 20],
	pub c_ispeed: usize,
	pub c_ospeed: usize,
}
#[cfg(target_os = "macos")] const ECHO: usize = 0x00000008;
#[cfg(target_os = "macos")] const ICANON: usize = 0x00000100;

const TCSADRAIN: i32 = 1;

unsafe extern "C" {
	fn tcgetattr(fd: i32, termios_p: *mut termios) -> i32;
	fn tcsetattr(fd: i32, optional_actions: i32, termios_p: *const termios) -> i32;
}

// --- Gestion du terminal (RAII) ---

struct RawModeGuard {
	fd: i32,
	original: termios,
}

impl Drop for RawModeGuard {
	fn drop(&mut self) {
		unsafe { tcsetattr(self.fd, TCSADRAIN, &self.original) };
	}
}

// --- La Macro ---

#[macro_export]
macro_rules! get_pw {
	($msg:expr) => {
		// On indique le chemin complet : crate -> module -> fonction
		$crate::get_pw::get_pw_impl($msg, '*')
	};
	($msg:expr, $mask:expr) => {
		$crate::get_pw::get_pw_impl($msg, $mask)
	};
}

// --- L'implémentation ---

pub fn get_pw_impl(message: &str, mask: char) -> String {
	let stdin = io::stdin();
	let fd = stdin.as_raw_fd();
	let mut password = String::new();

	print!("{}", message);
	let _ = io::stdout().flush();

	let mut original = unsafe { std::mem::zeroed() };
	if unsafe { tcgetattr(fd, &mut original) } != 0 {
		return String::new();
	}

	let _guard = RawModeGuard { fd, original };

	let mut hidden = original;
	hidden.c_lflag &= !(ECHO | ICANON);
	unsafe { tcsetattr(fd, TCSADRAIN, &hidden) };

	let mut buffer = [0u8; 1];
	let mut handle = stdin.lock();

	loop {
		if handle.read_exact(&mut buffer).is_ok() {
			match buffer[0] {
				b'\n' | 0x0D => break,
				0x7F | 0x08 => {
					if !password.is_empty() {
						password.pop();
						print!("\x08 \x08");
						let _ = io::stdout().flush();
					}
				}
				0x1B => {
					let mut seq = [0u8; 2];
					let _ = handle.read(&mut seq);
				}
				c if c >= 32 && c <= 126 => {
					password.push(c as char);
					print!("{}", mask);
					let _ = io::stdout().flush();
				}
				_ => {}
			}
		}
	}
	println!();
	password
}
