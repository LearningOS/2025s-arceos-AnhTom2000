#![no_std]

use allocator::{BaseAllocator, ByteAllocator, PageAllocator,AllocResult,AllocError};
use core::{ptr::NonNull,alloc::Layout};
/// Early memory allocator
/// Use it before formal bytes-allocator and pages-allocator can work!
/// This is a double-end memory range:
/// - Alloc bytes forward
/// - Alloc pages backward
///
/// [ bytes-used | avail-area | pages-used ]
/// |            | -->    <-- |            |
/// start       b_pos        p_pos       end
///
/// For bytes area, 'count' records number of allocations.
/// When it goes down to ZERO, free bytes-used area.
/// For pages area, it will never be freed!
///
pub struct EarlyAllocator<const PAGE_SIZE : usize>{
    start: usize,               
    end: usize,                
    byte_pos: usize,          
    page_pos: usize,          
    count: usize, 
}

impl<const PAGE_SIZE:usize> EarlyAllocator<PAGE_SIZE> {
 pub const fn new() -> Self {
        Self {
            start: 0,
            end: 0,
            byte_pos: 0,
            page_pos: 0,
            count: 0,
        }
    }
  pub fn init(&mut self , start : usize , size : usize){
        self.start = start;
        self.end = start+size;
        self.byte_pos = start;  
        self.page_pos = self.end; 
        self.count = 0;
  }   
}

impl<const PAGE_SIZE:usize> BaseAllocator for EarlyAllocator<PAGE_SIZE> {
    fn init(&mut self, start: usize, size: usize) {
        self.init(start, start+size);
    }
    fn add_memory(&mut self, _start: usize, _size: usize) -> AllocResult {
        unimplemented!()
    }
}

impl<const PAGE_SIZE:usize> ByteAllocator for EarlyAllocator<PAGE_SIZE> {
    fn alloc(&mut self, layout: Layout) -> AllocResult<NonNull<u8>> {
        let align = layout.align();
        let size = layout.size();
        let aligned_pos = align_up(self.byte_pos, align);

        if aligned_pos + size > self.page_pos {
            return Err(AllocError::NoMemory);
        }
        self.byte_pos = aligned_pos + size;
        self.count += 1;
        NonNull::new(aligned_pos as *mut u8).ok_or(AllocError::NoMemory)
    }
    
    fn available_bytes(&self) -> usize {
        self.page_pos - self.byte_pos
    }

    fn dealloc(&mut self, _pos: NonNull<u8>, _layout: Layout) {
        self.count -= 1;
        if self.count == 0 {
           self.byte_pos = self.start;
        }
}
    fn total_bytes(&self) -> usize {
        self.end - self.start
    }

    fn used_bytes(&self) -> usize {
      self.byte_pos - self.start + (self.end - self.page_pos)
    }
}

impl<const PAGE_SIZE:usize> PageAllocator for EarlyAllocator<PAGE_SIZE> {
    const PAGE_SIZE: usize = PAGE_SIZE;

    fn alloc_pages(&mut self, num_pages: usize, align_pow2: usize) -> AllocResult<usize> {
        let size = num_pages * Self::PAGE_SIZE;
        let align = 1 << align_pow2;
        
        let aligned_pos = align_down(self.page_pos - size, align);
        
        if aligned_pos >= self.byte_pos {
            self.page_pos = aligned_pos;
            Ok(aligned_pos)
        } else {
            Err(AllocError::NoMemory)
        }
    }

    fn available_pages(&self) -> usize {
        (self.page_pos - self.byte_pos) / Self::PAGE_SIZE
    }

    fn dealloc_pages(&mut self, _pos: usize, _num_pages: usize) {
        unimplemented!()
    }

    fn total_pages(&self) -> usize {
       (self.end - self.start) / Self::PAGE_SIZE
    }

    fn used_pages(&self) -> usize {
       (self.end - self.page_pos) / Self::PAGE_SIZE
    }
}

#[inline]
const fn align_up(pos: usize, align: usize) -> usize {
    (pos + align - 1) & !(align - 1)
}

#[inline]
const fn align_down(pos: usize, align: usize) -> usize {
    pos & !(align - 1)
}