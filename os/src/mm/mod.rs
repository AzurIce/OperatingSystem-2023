mod heap_allocator;
mod address;
mod page_table;
mod frame_allocator;

pub fn init() {
    heap_allocator::init_heap();
    heap_allocator::heap_test();
}