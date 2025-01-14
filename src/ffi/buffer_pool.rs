use crate::types::{handles::BufferPoolHandle, utilities::ByteBuffer};

extern "C" {
    /// Creates a new buffer pool.
    ///
    /// # Arguments
    ///
    /// * `minimum_block_allocation_size`: Minimum size of individual block allocations. Must be a power of 2.
    /// Pools with single allocations larger than the minimum will use the minimum value necessary to hold one element.
    /// Buffers will be suballocated from blocks.
    /// * `expected_used_slot_count_per_pool`: Number of suballocations to preallocate reference space for.
    /// This does not preallocate actual blocks, just the space to hold references that are waiting in the pool.
    #[link_name = "BufferPool.Create"]
    pub fn create_buffer_pool(
        minimum_block_allocation_size: i32,
        expected_used_slot_count_per_pool: i32,
    ) -> BufferPoolHandle;
    /// Releases all allocations held by the buffer pool. The buffer pool remains in a usable state.
    ///
    /// # Arguments
    ///
    /// * `handle`: Buffer pool to clear.
    #[link_name = "BufferPool.Clear"]
    pub fn clear_buffer_pool(handle: BufferPoolHandle);
    /// Releases all allocations held by the buffer pool and releases the buffer pool reference. The handle is invalidated.
    ///
    /// # Arguments
    ///
    /// * `handle`: Buffer pool to destroy.
    #[link_name = "BufferPool.Destroy"]
    pub fn destroy_buffer_pool(handle: BufferPoolHandle);
    /// Allocates a buffer from the buffer pool of the given size.
    ///
    /// # Arguments
    ///
    /// * `buffer_pool_handle`: Buffer pool to allocate from.
    /// * `size_in_bytes`: Size of the buffer to allocate in bytes.
    ///
    /// # Returns
    ///
    /// Allocated buffer.
    #[link_name = "BufferPool.Allocate"]
    pub fn allocate(buffer_pool_handle: BufferPoolHandle, size_in_bytes: i32) -> ByteBuffer;
    /// Allocates a buffer from the buffer pool with at least the given size.
    ///
    /// # Arguments
    ///
    /// * `buffer_pool_handle`: Buffer pool to allocate from.
    /// * `size_in_bytes`: Size of the buffer to allocate in bytes.
    ///
    /// # Returns
    ///
    /// Allocated buffer.
    #[link_name = "BufferPool.AllocateAtLeast"]
    pub fn allocate_at_least(
        buffer_pool_handle: BufferPoolHandle,
        size_in_bytes: i32,
    ) -> ByteBuffer;
    /// Resizes a buffer from the buffer pool to the given size, reallocating if necessary.
    ///
    /// # Arguments
    ///
    /// * `buffer_pool_handle`: Buffer pool to allocate from.
    /// * `buffer`: Buffer to resize.
    /// * `new_size_in_bytes`: Target size of the buffer to allocate in bytes.
    /// * `copy_count`: Number of bytes to copy from the old buffer into the new buffer.
    #[link_name = "BufferPool.Resize"]
    pub fn resize(
        buffer_pool_handle: BufferPoolHandle,
        buffer: *mut ByteBuffer,
        new_size_in_bytes: i32,
        copy_count: i32,
    );
    /// Resizes a buffer from the buffer pool to at least the given size, reallocating if necessary.
    ///
    /// # Arguments
    ///
    /// * `buffer_pool_handle`: Buffer pool to allocate from.
    /// * `buffer`: Buffer to resize.
    /// * `target_size_in_bytes`: Target size of the buffer to allocate in bytes.
    /// * `copy_count`: Number of bytes to copy from the old buffer into the new buffer.
    #[link_name = "BufferPool.ResizeToAtLeast"]
    pub fn resize_to_at_least(
        buffer_pool_handle: BufferPoolHandle,
        buffer: *mut ByteBuffer,
        target_size_in_bytes: i32,
        copy_count: i32,
    );
    /// Returns a buffer to the buffer pool.
    ///
    /// # Arguments
    ///
    /// * `buffer_pool_handle`: Buffer pool to return the buffer to.
    /// * `buffer`: Buffer to return to the pool.
    #[link_name = "BufferPool.Deallocate"]
    pub fn deallocate(buffer_pool_handle: BufferPoolHandle, buffer: *mut ByteBuffer);
    /// Returns a buffer to the buffer pool by its id.
    ///
    /// # Arguments
    ///
    /// * `buffer_pool_handle`: Buffer pool to return the buffer to.
    /// * `buffer_id`: Id of the buffer to return to the pool.
    #[link_name = "BufferPool.DeallocateId"]
    pub fn deallocate_by_id(buffer_pool_handle: BufferPoolHandle, buffer_id: i32);

    /// Computes the total number of bytes allocated from native memory in this buffer pool.
    /// Includes allocated memory regardless of whether it currently has outstanding references.
    ///
    /// # Arguments
    ///
    /// * `buffer_pool_handle`: Buffer pool to check the allocation size of.
    ///
    /// # Returns
    ///
    /// Total number of bytes allocated from native memory in this buffer pool.
    #[link_name = "BufferPool.GetAllocatedMemorySizeInPool"]
    pub fn get_allocated_memory_size_in_pool(buffer_pool_handle: BufferPoolHandle) -> u64;
}
