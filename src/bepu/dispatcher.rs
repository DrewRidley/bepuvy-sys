use crate::types::handles::ThreadDispatcherHandle;

pub struct ThreadDispatcher {
    handle: ThreadDispatcherHandle,
}

impl Drop for ThreadDispatcher {
    fn drop(&mut self) {
        // SAFETY:
        //
        // The only place to obtain a buffer pool handle is the constructor of this type.
        // That constructor ties the handle's lifetime to 'self' so when 'self' is dropped there are no references to handle.
        unsafe {
            crate::ffi::dispatcher::destroy_thread_dispatcher(self.handle);
        }
    }
}

impl ThreadDispatcher {
    pub fn new(thread_count: i32, alloc_block_size: i32) -> Self {
        ThreadDispatcher {
            // SAFETY:
            //
            // Creating a handle is always safe as long as you use it responsibly.
            // `crate::ffi::buffer_pool::destroy_thread_dispatcher` needs to be called for every handle created.
            // This is handled in the 'Drop' impl for `ThreadDispatcher`
            handle: unsafe {
                crate::ffi::dispatcher::create_thread_dispatcher(thread_count, alloc_block_size)
            },
        }
    }

    pub(crate) fn handle(&self) -> ThreadDispatcherHandle {
        self.handle
    }
}
