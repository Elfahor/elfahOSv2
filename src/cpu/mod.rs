mod instructions;

/// An I/O port for port-mapped I/O. This struct allows for safe manipulation of ports:
/// once one is created with `Port::new` (in which case the port number must be verified by the caller)
/// it is safe to write to and read from the port
pub struct Port {
	port: u16,
}

impl Port {
	pub unsafe fn new(port: u16) -> Port {
		Port { port }
	}
	pub fn write(&self, val: u8) {
		unsafe {instructions::outb(self.port, val);}
	}
	pub fn read(&self) -> u8 {
		unsafe {instructions::inb(self.port)}
	}
}

/// Stop the processor __definitively__ ! Only interrupts will fire.
pub fn hlt() -> ! {
	loop {
		instructions::hlt()
	}
}
