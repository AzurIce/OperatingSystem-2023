mod heap_allocator;
mod address;
mod page_table;

pub fn init() {
    heap_allocator::init_heap();
    heap_allocator::heap_test();
}