use multiboot::{
	bootloader_name::BootLoaderName,
	tags::{Tag, TagIter, TagType},
	elf_symbols::ElfSymbols
};
use multiboot::basic_mem_info::BasicMemoryInformation;
use multiboot::end_tag::EndTag;
pub use multiboot::memory::{MemoryMap, MemoryArea, MemoryAreaIter};

mod tags;
mod memory;
mod bootloader_name;
mod elf_symbols;
mod basic_mem_info;
mod end_tag;
mod apm_table;

/// errors that can occur when loading an MBI
#[derive(Debug)]
pub enum MbiErr {
	AddressError,
	SizeError,
	EndTagError
}

/// a Multiboot information structure, see https://www.gnu.org/software/grub/manual/multiboot2/multiboot.html#Boot-informations
#[repr(C)]
#[derive(Debug)]
pub struct MultibootInfo {
	total_size: u32,
	/// should be 0
	reserved: u32
}

impl MultibootInfo {
	/// A multiboot information structure must end with an End tag: "Tags are terminated by a tag of type ‘0’ and size ‘8’. "
	/// This method returns whether or not `self` has an end tag, and is used to check whether or not `self` is actually an MBI.
	pub fn is_end_tag_valid(&self) -> bool {
		// we cannot use get_tag(TagType::End) because if the MBI is wrong we would get nothing
		// address of this + size - 8 (size of the tag)
		let end_tag_addr  = self as *const _ as usize + (self.total_size - 8) as usize;
		// assert if the content of this address is an end tag
		unsafe {*(end_tag_addr as *const Tag) == Tag { typ: TagType::End, size: 8 } }
	}
	
	/// Get the whole list of tags
	pub fn get_tags(&self) -> impl Iterator<Item = &'_ Tag> {
		TagIter::new(self.get_start() + 8)
	}
	
	/// Get the Bootloader name tag
	pub fn get_bootloader_name(&self) -> Option<&BootLoaderName> {
		let tag = self.get_tag(TagType::BootLoaderName);
		match tag {
			None => None,
			Some(tag) =>
				Some(unsafe {&*(tag as *const Tag as *const BootLoaderName)})
		}
	}
	/// Get the memory map tag
	pub fn get_memory_map(&self) -> Option<&MemoryMap> {
		let tag = self.get_tag(TagType::MemoryMap);
		match tag {
			None => None,
			Some(tag) =>
				Some(unsafe {&*(tag as *const Tag as *const MemoryMap)})
		}
	}
	/// Get the ELF sections of the booted kernel
	pub fn get_elf_sections(&self) -> Option<&ElfSymbols> {
		let tag = self.get_tag(TagType::ElfSymbols);
		match tag {
			None => None,
			Some(tag) =>
				Some(unsafe {&*(tag as *const Tag as *const ElfSymbols)})
		}
	}
	/// Get the basic memory information tag
	pub fn get_basic_mem_info(&self) -> Option<&BasicMemoryInformation> {
		let tag = self.get_tag(TagType::BasicMemoryInfo);
		match tag {
			None => None,
			Some(tag) =>
				Some(unsafe {&*(tag as *const Tag as *const BasicMemoryInformation)})
		}
	}
	/// Get the end tag. This is probably useless once the MBI has been verified, and the implementation cannot be used to verify the MBI...
	pub fn get_end_tag(&self) -> Option<&EndTag> {
		let tag = self.get_tag(TagType::BasicMemoryInfo);
		match tag {
			None => None,
			Some(tag) =>
				Some(unsafe { &*(tag as *const Tag as *const EndTag) })
		}
	}
	
	/// Get a specific tag. Should be then casted to its type. Wrappers for the latter operation are provided
	pub fn get_tag(&self, tag: TagType) -> Option<&Tag>{
		self.get_tags().find(|t| t.typ == tag)
	}
	
	pub fn get_start(&self) -> usize {
		self as *const MultibootInfo as usize
	}
}

/// Load an MBI from an address. Verification of the address if left to the caller (it should be passed by the bootloader in `ebx`)
pub unsafe fn load(addr: usize) -> Result<&'static MultibootInfo, MbiErr> {
	// address is null or not 8-byte aligned
	if addr == 0 || addr & 0b111 != 0 {
		return Err(MbiErr::AddressError);
	}
	
	let mbi = &*(addr as *const MultibootInfo);
	// total_size must a multiple of 8
	if mbi.total_size & 0b111 != 0 {
		return Err(MbiErr::SizeError);
	}
	if !mbi.is_end_tag_valid() {
		return Err(MbiErr::EndTagError);
	}
	
	Ok(mbi)
}