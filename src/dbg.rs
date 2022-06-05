use core::fmt::Write;
use cpu::Port;

/// Writer to the 0x3f8 serial port. This port is used by QEMU to communicate with the host machine.
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