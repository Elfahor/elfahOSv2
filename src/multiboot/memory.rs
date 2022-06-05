use core::marker::PhantomData;
use multiboot::tags::TagType;

#[repr(C)]
#[derive(Debug)]
/// This tag provides memory map.
///`entry_size` contains the size of one entry so that in future new fields may be added to it. It’s guaranteed to be a multiple of 8. `entry_version` is currently set at 0.
/// Future versions will increment this field. Future version are guranteed to be backward compatible with older format.
/// Each entry has the structure of `MemoryArea`
pub struct MemoryMap {
	typ: TagType,
	size: u32,
	entry_size: u32,
	entry_version: u32,
	entries: MemoryArea
}

#[repr(C)]
#[derive(Debug)]
pub struct MemoryArea {
	pub base_addr: u64,
	pub length: u64,
	typ: MemoryAreaType,
	reserved: u32
}

#[repr(u32)]
#[derive(PartialEq)]
#[derive(Debug)]
pub enum MemoryAreaType {
	AvailableMemory = 1,
	ReservedHibernate = 2,
	AcpiInfo = 3,
	Reserved = 4,
	Defective = 5
}

impl MemoryMap {
	/// Get the whole list of memory areas
	pub fn get_mem_areas(&self) -> impl Iterator<Item = &MemoryArea> {
		MemoryAreaIter {
			current: (&self.entries) as *const MemoryArea as u64,
			last: self as *const MemoryMap as u64 + ((self.size - self.entry_size) as u64),
			size: self.entry_size,
			phantom: PhantomData
		}
	}
	/// Get only the available memory areas
	pub fn get_available_mem_areas(&self) -> impl Iterator<Item = &MemoryArea> {
		self.get_mem_areas().filter(|m|
			m.typ == MemoryAreaType::AvailableMemory)
	}
}

#[derive(Clone)]
/// An iterator i the MemoryAreas
pub struct MemoryAreaIter<'a> {
	current: u64,
	last: u64,
	size: u32,
	phantom: PhantomData<&'a MemoryArea>
}

impl<'a>  Iterator for MemoryAreaIter<'a> {
	type Item= &'a MemoryArea;
	
	fn next(&mut self) -> Option<Self::Item> {
		if self.current > self.last {
			None
		} else {
			let current = unsafe {&*(self.current as *const MemoryArea)};
			self.current += self.size as u64;
			Some(current)
		}
	}
}