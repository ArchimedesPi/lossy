mod area_frame_allocator;
pub use self::area_frame_allocator::AreaFrameAllocator;

// derive Debug so that a frame can be print!ed
// Eq, PartialEq, Ord, and PartialOrd are so that frames are comparable and ordered
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame {
    number: usize,
}

pub const PAGE_SIZE: usize = 4096; // size of a single page frame in bytes

impl Frame {
    fn containing_address(address: usize) -> Frame {
        Frame { number: address / PAGE_SIZE }
    }
}

pub trait FrameAllocator {
    fn allocate_frame(&mut self) -> Option<Frame>;
    fn deallocate_frame(&mut self, frame: Frame);
}
