use multiboot::tags::TagType;

#[repr(C)]
#[derive(Debug)]
/// `size` has to be 8 and this tag must be the last tag for the MBI to be valid
pub struct EndTag {
	typ: TagType,
	size: u32 // must be 8
}