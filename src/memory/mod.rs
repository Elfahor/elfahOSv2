pub mod area_frame_allocator;

/// size of a normal page. There is currently no support for huge pages.
pub const PAGE_SIZE: usize = 4096;

/// A physical frame, should be mapped to a virtual page
#[derive(Clone, PartialOrd, PartialEq)]
pub struct Frame {
	number: usize
}

impl Frame {
	/// Returns the frame containing `addr`
	pub fn at_addr(addr: usize) -> Frame {
		Frame {number: addr / 4096}
	}
}

/// An allocator for physical frames
pub trait FrameAllocator {
	/// Allocate a new physical frame for later use
	fn alloc(&mut self) -> Option<Frame>;
	/// Deallocate a physical frame
	fn dealloc(&mut self, frame: Frame);
}