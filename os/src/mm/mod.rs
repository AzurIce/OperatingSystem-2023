mod heap_allocator;
mod address;
mod page_table;
mod frame_allocator;
mod memory_set;

pub fn init() {
    heap_allocator::init_heap();
    heap_allocator::heap_test();
    frame_allocator::init_frame_allocator();
    frame_allocator::frame_allocator_test();
}