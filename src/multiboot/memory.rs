use core::marker::PhantomData;
use multiboot::tags::TagType;

#[repr(C)]
#[derive(Debug)]
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
	pub(crate) base_addr: u64,
	pub(crate) length: u64,
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
	pub fn get_mem_areas(&self) -> impl Iterator<Item = &MemoryArea> {
		MemoryAreaIter {
			current: (&self.entries) as *const MemoryArea as u64,
			last: self as *const MemoryMap as u64 + ((self.size - self.entry_size) as u64),
			size: self.entry_size,
			phantom: PhantomData
		}
	}
	pub fn get_available_mem_areas(&self) -> impl Iterator<Item = &MemoryArea> {
		self.get_mem_areas().filter(|m|
			m.typ == MemoryAreaType::AvailableMemory)
	}
}

#[derive(Clone)]
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