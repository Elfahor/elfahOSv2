use core::arch::asm;

/// halt the processor. It will be put out of sleep on the next interrupt.
pub fn hlt() {
	unsafe { asm!("hlt") }
}

/// unsafely read from a port. Use `cpu::Port` instead.
pub unsafe fn inb(port: u16) -> u8 {
	let mut data;
	unsafe {
		asm!(
		"in al, dx",
		in("dx") port,
		lateout("al") data,
		)
	}
	data
}

/// unsafely write to a port. Use `cpu::Port` instead.
pub unsafe fn outb(port: u16, data: u8) {
	unsafe {
		asm!(
		"out dx, al",
		in("dx") port,
		in("al") data,
		)
	}
}
