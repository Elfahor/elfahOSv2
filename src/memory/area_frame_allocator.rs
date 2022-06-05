use memory::{Frame, FrameAllocator};
use multiboot::{MemoryArea, MemoryAreaIter};

pub struct AreaFrameAllocator<'a> {
	next_free_frame: Frame,
	current_area: Option<&'a MemoryArea>,
	areas: MemoryAreaIter<'a>,
	kernel_start: Frame,
	kernel_end: Frame,
	mbi_start: Frame,
	mbi_end: Frame
}

impl FrameAllocator for AreaFrameAllocator<'_> {
	fn alloc(&mut self) -> Option<Frame> {
		if let Some(area) = self.current_area {
			let frame = self.next_free_frame.clone();
			let addr = area.base_addr + area.length - 1;
			let current_area_last_frame = Frame::at_addr(addr as usize);
			
			if frame > current_area_last_frame { // switch to next area
				self.choose_next_area();
			} else if frame >= self.kernel_start && frame <= self.kernel_end { // we are in the kernel
				self.next_free_frame = Frame {
					number: self.kernel_end.number + 1
				}
			} else if frame >= self.mbi_start && frame <= self.mbi_end { // we are in the mbi
				self.next_free_frame = Frame {
					number: self.mbi_end.number + 1
				}
			} else { // frame is unused, yay!
				self.next_free_frame.number += 1;
				return Some(frame);
			}
			
			self.alloc()
			
		} else { // no more free frames
			None
		}
	}
	fn dealloc(&mut self, frame: Frame) {
		unimplemented!()
	}
}

impl<'a> AreaFrameAllocator<'a> {
	fn choose_next_area(&mut self) {
		self.current_area = self.areas.clone().filter(|a| {
			let addr = a.base_addr + a.length - 1;
			Frame::at_addr(addr as usize) >= self.next_free_frame
		}).min_by_key(|a| a.base_addr);
		if let Some(area) = self.current_area {
			let start_frame = Frame::at_addr(area.base_addr as usize);
			if self.next_free_frame < start_frame {
				self.next_free_frame = start_frame;
			}
		}
	}
	
	pub fn new(kernel_start: usize, kernel_end: usize, mbi_start: usize, mbi_end: usize, areas: MemoryAreaIter) -> AreaFrameAllocator {
		let mut allocator = AreaFrameAllocator {
			next_free_frame: Frame::at_addr(0),
			current_area: None,
			areas,
			kernel_start: Frame::at_addr(kernel_start),
			kernel_end: Frame::at_addr(kernel_end),
			mbi_start: Frame::at_addr(mbi_start),
			mbi_end: Frame::at_addr(mbi_end),
		};
		allocator.choose_next_area();
		allocator
	}
}