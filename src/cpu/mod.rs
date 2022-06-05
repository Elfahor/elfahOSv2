mod instructions;

pub struct Port {
	port: u16,
}

impl Port {
	pub unsafe fn new(port: u16) -> Port {
		Port { port }
	}
	pub fn write(&self, val: u8) {
		instructions::outb(self.port, val);
	}
	pub fn read(&self) -> u8 {
		instructions::inb(self.port)
	}
}

pub fn hlt() -> ! {
	loop {
		instructions::hlt()
	}
}
