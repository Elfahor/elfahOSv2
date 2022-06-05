use multiboot::tags::TagType;
use core::{str, slice};

#[repr(C)]
#[derive(Debug)]
///`string` contains the name of a boot loader booting the kernel. The name is a normal C-style UTF-8 zero-terminated string. You can retrieve it with `BootLoaderName::name`
pub struct BootLoaderName {
	typ: TagType,
	size: u32,
	string: u8
}

impl BootLoaderName {
	/// Use this to retrieve the name
	pub fn name(&self) -> &str {
		let len = self.size as usize - 8;
		let slice: &[u8] = unsafe {slice::from_raw_parts(&self.string as *const u8, len)};
		unsafe {str::from_utf8_unchecked(slice)}
	}
}