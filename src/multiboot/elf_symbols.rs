use multiboot::tags::TagType;

#[repr(C)]
pub struct ElfSymbols {
	typ: TagType,
	size: u32,
	num: u16,
	entsize: u16,
	shndx: u16,
	reserved: u16,
	
}