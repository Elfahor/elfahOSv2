use multiboot::tags::TagType;
use core::{str, slice};

pub struct BootLoaderName {
	typ: TagType,
	size: u32,
	string: u8
}

impl BootLoaderName {
	pub fn name(&self) -> &str {
		let len = self.size as usize - 8;
		let slice: &[u8] = unsafe {slice::from_raw_parts(&self.string as *const u8, len)};
		unsafe {str::from_utf8_unchecked(slice)}
	}
}