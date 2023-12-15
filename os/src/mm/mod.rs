mod heap_allocator;
mod address;

pub fn init() {
    heap_allocator::init_heap();
    heap_allocator::heap_test();
}