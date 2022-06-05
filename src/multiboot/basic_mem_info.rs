use multiboot::tags::TagType;

#[repr(C)]
#[derive(Debug)]
/// Very basic information about memory
/// `mem_lower` and `mem_upper` indicate the amount of lower and upper memory, respectively, in kilobytes. Lower memory starts at address 0, and upper memory starts at address 1 megabyte.
pub struct BasicMemoryInformation {
	typ: TagType,
	size: u32,
	mem_lower: u32,
	mem_upper: u32
}