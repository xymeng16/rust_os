use core::mem;
use super::align_up;

struct ListNode {
    size: usize,
    next: Option<&'static mut ListNode>,
    // use mutable references in const functions is unstable
    // #![feature(const_mut_refs)] is required in lib.rs
}

impl ListNode {
    const fn new(size: usize) -> Self {
        ListNode {
            size,
            next: None
        }
    }

    fn start_addr(&self) -> usize {
        self as *const Self as usize
    }

    fn end_addr(&self) -> usize {
        self.start_addr() + self.size
    }
}

pub struct LinkedListAllocator {
    head: ListNode,
}

impl LinkedListAllocator {
    /// Create an empty LinkedListAllocator
    pub const fn empty() -> Self {
        LinkedListAllocator {
            head: ListNode::new(0),
        }
    }

    /// Initialize the allocator with the given heap bounds
    /// This function is unsafe because the caller must guarantee that the given
    /// heap bounds are valid and that the heap is unused. This method must be
    /// called only once.
    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.add_free_region(heap_start, heap_size);
    }

    /// Add the given memory region to the front of the list
    unsafe fn add_free_region(&mut self, addr: usize, size: usize) {
        // ensure that the given free region is capable of holding ListNode
        assert!((size >= mem::size_of::<ListNode>()));
        assert_eq!(align_up(addr, mem::align_of::<ListNode>()), addr);

        // create a new list node and append it at the start of the list
        let mut node = ListNode::new(size);
        node.next = self.head.next.take();
        let node_ptr = addr as *mut ListNode;
        node_ptr.write(node);
        self.head.next = Some(&mut *node_ptr);
    }
}