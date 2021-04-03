use x86_64::{
    addr::VirtAddr,
    structures::paging::{
        mapper::MapToError, FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB,
    },
    registers::control::Cr3
};
//use core::ptr::null_mut;
//Will be removed in favor of a custom allocator in the future
pub mod linked_list;
use core;
use crate::memory;
use crate::syscall;
/// The start adress of the heap.
pub const HEAP_START: usize = 0x4444_0000;
/// The size of the heap. It is for now pretty small.
pub const HEAP_SIZE: usize = 100 * 1024;

#[alloc_error_handler]
fn alloc_error_handler(layout: core::alloc::Layout) -> ! {
    panic!("Allocation error : {:?}", layout)
}

use linked_list::LinkedListAllocator;

#[global_allocator]
static ALLOCATOR: Locked<LinkedListAllocator> = Locked::new(LinkedListAllocator::new());

pub fn init_allocator() {
    // Memory allocation Initialization
    let phys_mem_offset = VirtAddr::new(12_u64);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    unsafe {
        if let Some(frame_allocator) = &mut memory::FRAME_ALLOCATOR {
            let (level_4_frame, _) = Cr3::read();
            frame_allocator
                .deallocate_level_4_page(level_4_frame.start_address(), PageTableFlags::BIT_9)
                .expect("Didn't manage to clean bootloader data");
            init(&mut mapper, frame_allocator).expect("Heap init failed :((");
        } else {
            panic!("Frame allocator wasn't initialized");
        }
    };
}

/// Inits the Allocator, responsible for the...
///
/// TODO : continue working on this
pub fn init(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), MapToError<Size4KiB>> {
    let page_range = {
        let heap_start = VirtAddr::new(HEAP_START as u64);
        let heap_end = heap_start + HEAP_SIZE - 1u64;
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);
        Page::range_inclusive(heap_start_page, heap_end_page)
    };
    for page in page_range {
        let frame = frame_allocator
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;
        let flags =
            PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::USER_ACCESSIBLE;
        unsafe {
            mapper.map_to(page, frame, flags, frame_allocator)?.flush();
        };
    }

    unsafe {
        ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
    }

    Ok(())
}

pub struct Locked<A> {
    inner: spin::Mutex<A>,
}

impl<A> Locked<A> {
    pub const fn new(inner: A) -> Self {
        Locked {
            inner: spin::Mutex::new(inner),
        }
    }

    pub fn lock(&self) -> spin::MutexGuard<A> {
        self.inner.lock()
    }
}

fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}
