//! Provides a buddy allocator.
//! 
//! The buddy allocator allocates memory regions which are power-of-two 
//! multiples of the system page size. The exponent is termed the `order`
//! of the allocation.

extern crate kernel_config;

use kernel_config::memory::PAGE_SIZE;
use memory_structs::*;
use core::fmt;
use alloc::boxed::Box;

struct Area {
    /// The lowest virtual page number belonging to this area - this is always
    /// aligned to the area size.
    page_number: usize,
    /// The order of the area is the power-of-two exponent of its size in pages
    order: i32,
    /// Children nodes if this `Area` is non-terminal. The two seperate child
    /// `Area`s are kept in the same `Box` to maximize spatial locality.
    children: Option<Box<[Area; 2]>>,
    /// `true` is `Area` is allocated
    allocated: bool
}

impl<'a> fmt::Debug for Area {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Area(")
    }
}

impl Area {

    /// Returns a new root area (with no parent) from a base `page_number` and 
    /// a given `order`.
    /// 
    /// `page_number` must be aligned to the size of the area, in other words:
    /// `page_number` must be a multiple of `(1 << order)`.
    pub const fn new_root(page_number: usize, order: i32) -> Area {
        assert!((page_number % (1 << order)) == 0);
        
        Area {
            page_number: page_number,
            order: order,
            children: None,
            allocated: false,
        }
    }

    /// Returns the first `Page` within this `Area`
    pub fn start_page(&self) -> Page {
        Page::new(self.page_number)
    }

    /// Returns the first `VirtualAddress` within this `Area`
    pub fn start_address(&self) -> VirtualAddress {
        VirtualAddress::new_canonical(self.page_number * PAGE_SIZE)
    }

    /// Returns the number of pages within this area
    pub fn num_pages(&self) -> usize {
        1 << self.order
    }

    /// Splits this `Area` into two smaller identically sized `Area`s, each of
    /// which are half the size of the original.
    /// 
    /// Only unallocated `Area`s can be split.
    pub fn split(&mut self) {
        assert!(self.order > 0);
        assert!(self.allocated);

        let half_num_pages = self.num_pages() / 2;
        self.children = Some(Box::new([
            Area {page_number: self.page_number, order: self.order - 1, children: None, allocated: false},
            Area {page_number: self.page_number + half_num_pages, order: self.order - 1, children: None, allocated: false}
        ]));
    }
}

pub struct buddy_allocator {
    /// The largest `Area` which contains all allocatable virtual memory.
    root: Area,
}

impl buddy_allocator {
    pub const fn new(page_number: usize, order:i32) -> buddy_allocator {
        buddy_allocator {
            root: Area::new_root(page_number, order)
        }
    }

    /*pub fn alloc_pages(
        &mut self,
        requested_vaddr: Option<VirtualAddress>,
        num_pages: usize
    ) -> Result<something_representing_allocation, &'static str> {
        if let Some(requested_vaddr) = requested_vaddr {

        }
        else {
            // Search the buddy tree structure 
        }
    }
    */

    pub fn free_pages(&mut self, vaddr: VirtualAddress) {

    }
}
