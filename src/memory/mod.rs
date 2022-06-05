pub mod area_frame_allocator;

pub const PAGE_SIZE: usize = 4096;

#[derive(Clone, PartialOrd, PartialEq)]
pub struct Frame {
	number: usize
}

impl Frame {
	pub fn at_addr(addr: usize) -> Frame {
		Frame {number: addr / 4096}
	}
}

pub trait FrameAllocator {
	fn alloc(&mut self) -> Option<Frame>;
	fn dealloc(&mut self, frame: Frame);
}