use core::marker::PhantomData;

#[derive(PartialEq, Debug)]
/// A tag gives the kernel information about something.
/// They all share a common structure (that of `Tag`) but add custom fields: as such, every `Tag` instance should probably be casted to something else.
pub struct Tag {
	pub typ: TagType,
	pub size: u32
}

#[repr(u32)]
#[derive(PartialEq, Debug)]
#[allow(dead_code)]
/// All the kinds of tags
pub enum TagType {
	End = 0,
	BootCommandLine = 1,
	BootLoaderName = 2,
	Modules = 3,
	BasicMemoryInfo = 4,
	BiosBootDevice = 5,
	MemoryMap = 6,
	VbeInfo = 7,
	FramebufferInfo = 8,
	ElfSymbols = 9,
	ApmTable = 10,
	Efi32SystemTablePtr = 11,
	Efi64SystemTablePtr = 12,
	SmBiosTables = 13,
	AcpiOldRsdp = 14,
	AcpiNewRsdp = 15,
	NetworkingInfo = 16,
	EfiMemMap = 17,
	EfiBootServicesNotTerminated = 18,
	Efi32ImageHandlePtr = 19,
	Efi64ImageHandlePtr = 20,
	ImageLoadBasePhysAddr = 21
}

#[derive(Clone)]
/// An iterator among all tags
pub struct TagIter<'a> {
	current: *const Tag,
	phantom: PhantomData<&'a Tag>
}

impl<'a> TagIter<'a> {
	pub fn new(start: usize) -> TagIter<'a> {
		TagIter {
			current: start as *const Tag,
			phantom: PhantomData
		}
	}
}

impl<'a> Iterator for TagIter<'a> {
	type Item = &'a Tag;
	
	fn next(&mut self) -> Option<Self::Item> {
		match unsafe {&*self.current} {
			&Tag { // if this is an end tag
				typ: TagType::End,
				size: 8
			} => None,
			tag => {
				// add tag size with 8 byte alignment
				self.current = (self.current as usize + ((tag.size + 7) & !7) as usize) as *const _;
				Some(tag)
			}
		}
	}
}