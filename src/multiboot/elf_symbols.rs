use multiboot::tags::TagType;

#[repr(C)]
/// This tag contains section header table from an ELF kernel, the size of each entry, number of entries, and the string table used as the index of names.
/// They correspond to the `shdr_*` entries (`shdr_num`, etc.) in the Executable and Linkable Format (ELF) specification in the program header.
/// All sections are loaded, and the physical address fields of the ELF section header then refer to where the sections are in memory
pub struct ElfSymbols {
	typ: TagType,
	size: u32,
	num: u16,
	entsize: u16,
	shndx: u16,
	reserved: u16,
	
}