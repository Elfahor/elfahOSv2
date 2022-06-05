use core::fmt::Write;
use cpu::Port;

pub struct DebugWriter;

impl Write for DebugWriter {
	fn write_str(&mut self, s: &str) -> core::fmt::Result {
		let port = unsafe { Port::new(0x3f8) };
		for c in s.bytes() {
			port.write(c);
		}
		Ok(())
	}
}
