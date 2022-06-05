#![no_std]
#![feature(lang_items)]

mod cpu;
mod dbg;
mod vga_text;
mod multiboot;
mod memory;

use core::{fmt::Write, panic::PanicInfo};
use vga_text::{Color, VgaWriter};

/// main entry point of the kernel
#[no_mangle]
pub extern "C" fn kmain(mbi_addr: usize) -> ! {
	let mut w = VgaWriter::default();
	
	writeln!(w, "MBI address: 0x{:x}", mbi_addr).unwrap();
	let mbi = unsafe { multiboot::load(mbi_addr).unwrap() };
	writeln!(w, "{:?}", mbi).unwrap();
	for (i, t) in mbi.get_tags().enumerate() {
		writeln!(w, "{:?}", t).unwrap();
	}
	let bl_name = mbi.get_bootloader_name().unwrap().name();
	writeln!(w, "Booted by {}", bl_name).unwrap();
	let mem_map = mbi.get_memory_map().unwrap();
	for m in mem_map.get_available_mem_areas() {
		writeln!(w, "{:x?}", m).unwrap();
	}
	
	cpu::hlt();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	writeln!(VgaWriter::new(Color::Red, Color::Black), "{}", _info);
	cpu::hlt();
}
