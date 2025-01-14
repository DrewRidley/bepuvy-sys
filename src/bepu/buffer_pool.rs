use crate::types::handles::BufferPoolHandle;

pub struct BufferPool {
    handle: BufferPoolHandle,
}

impl Drop for BufferPool {
    fn drop(&mut self) {
        // SAFETY:
        //
        // The only place to obtain a buffer pool handle is the constructor of this type.
        // That constructor ties the handle's lifetime to 'self' so when 'self' is dropped there are no references to handle.
        unsafe {
            crate::ffi::buffer_pool::destroy_buffer_pool(self.handle);
        }
    }
}

impl Default for BufferPool {
    fn default() -> Self {
        Self::new(131072, 16)
    }
}

impl BufferPool {
    pub fn new(minimum_block_allocation_size: i32, expected_used_slot_count_per_pool: i32) -> Self {
        BufferPool {
            // SAFETY:
            //
            // Creating a handle is always safe as long as you use it responsibly.
            // `crate::ffi::buffer_pool::destroy_buffer_pool` needs to be called for every handle created.
            // This is handled in the 'Drop' impl for `BufferPool`
            handle: unsafe {
                crate::ffi::buffer_pool::create_buffer_pool(
                    minimum_block_allocation_size,
                    expected_used_slot_count_per_pool,
                )
            },
        }
    }

    pub(crate) fn handle(&self) -> BufferPoolHandle {
        self.handle
    }
}
