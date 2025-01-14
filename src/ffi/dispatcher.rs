use crate::types::handles::ThreadDispatcherHandle;

extern "C" {

    /// Creates a new thread dispatcher.
    ///
    /// # Arguments
    ///
    /// * `thread_count`: Number of threads to use within the thread dispatcher.
    /// * `thread_pool_allocation_block_size`: Minimum size in bytes of blocks allocated in per-thread buffer pools. Allocations requiring more space can result in larger block sizes, but no pools will allocate smaller blocks.
    #[link_name = "ThreadDispatcher.Create"]
    pub fn create_thread_dispatcher(
        thread_count: i32,
        thread_pool_allocation_block_size: i32,
    ) -> ThreadDispatcherHandle;
    /// Releases all resources held by a thread dispatcher and invalidates its handle.
    ///
    /// # Arguments
    ///
    /// * `handle`: Thread dispatcher to destroy.
    #[link_name = "ThreadDispatcher.Destroy"]
    pub fn destroy_thread_dispatcher(handle: ThreadDispatcherHandle);
    /// Releases all resources held by a thread dispatcher and invalidates its handle.
    ///
    /// # Arguments
    ///
    /// * `handle`: Thread dispatcher to check the thread count of.
    #[link_name = "ThreadDispatcher.GetThreadCount"]
    pub fn get_thread_count(handle: ThreadDispatcherHandle) -> i32;

    /// Computes the total number of bytes allocated from native memory in a dispatcher's per-thread pools.
    /// Includes allocated memory regardless of whether it currently has outstanding references.
    ///
    /// # Arguments
    ///
    /// * `thread_dispatcher_handle`: Thread dispatcher to check allocations for.
    ///
    /// # Returns
    ///
    /// Total number of bytes allocated from native memory in this thread dispatcher's per-thread pool.
    #[link_name = "ThreadDispatcher.GetAllocatedMemorySize"]
    pub fn get_allocated_memory_size_in_thread_dispatcher(
        thread_dispatcher_handle: ThreadDispatcherHandle,
    ) -> u64;
}
