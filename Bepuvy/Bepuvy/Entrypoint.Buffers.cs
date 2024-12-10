using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using BepuUtilities.Memory;

namespace Bepuvy;



public static partial class Entrypoint
{
       
    /// <summary>
    /// Creates a new buffer pool.
    /// </summary>
    /// <param name="minimumBlockAllocationSize">Minimum size of individual block allocations. Must be a power of 2.
    /// Pools with single allocations larger than the minimum will use the minimum value necessary to hold one element.
    /// Buffers will be suballocated from blocks.</param>
    /// <param name="expectedUsedSlotCountPerPool">Number of suballocations to preallocate reference space for.
    /// This does not preallocate actual blocks, just the space to hold references that are waiting in the pool.</param>
    [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "BufferPool.Create")]
    public static InstanceHandle CreateBufferPool(int minimumBlockAllocationSize = 131072, int expectedUsedSlotCountPerPool = 16)
    {
        return pools.Add(new BufferPool(minimumBlockAllocationSize, expectedUsedSlotCountPerPool));
    }

    /// <summary>
    /// Releases all allocations held by the buffer pool. The buffer pool remains in a usable state.
    /// </summary>
    /// <param name="handle">Buffer pool to clear.</param>
    [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "BufferPool.Clear")]
    public static void ClearBufferPool(InstanceHandle handle)
    {
        pools[handle].Clear();
    }

    /// <summary>
    /// Releases all allocations held by the buffer pool and releases the buffer pool reference. The handle is invalidated.
    /// </summary>
    /// <param name="handle">Buffer pool to destroy.</param>
    [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "BufferPool.Destroy")]
    public static void DestroyBufferPool(InstanceHandle handle)
    {
        pools[handle].Clear();
        pools.Remove(handle);
    }
    
    /// <summary>
    /// Allocates a buffer from the buffer pool of the given size.
    /// </summary>
    /// <param name="handle">Buffer pool to allocate from.</param>
    /// <param name="sizeInBytes">Size of the buffer to allocate in bytes.</param>
    /// <returns>Allocated buffer.</returns>
    [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "BufferPool.Allocate")]
    public static Buffer<byte> Allocate(InstanceHandle handle, int sizeInBytes)
    {
        pools[handle].Take<byte>(sizeInBytes, out var buffer);
        return buffer;
    }
    /// <summary>
    /// Allocates a buffer from the buffer pool with at least the given size.
    /// </summary>
    /// <param name="handle">Buffer pool to allocate from.</param>
    /// <param name="sizeInBytes">Size of the buffer to allocate in bytes.</param>
    /// <returns>Allocated buffer.</returns>
    [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "BufferPool.AllocateAtLeast")]
    public static Buffer<byte> AllocateAtLeast(InstanceHandle handle, int sizeInBytes)
    {
        pools[handle].TakeAtLeast<byte>(sizeInBytes, out var buffer);
        return buffer;
    }
    
    /// <summary>
    /// Resizes a buffer from the buffer pool to the given size, reallocating if necessary.
    /// </summary>
    /// <param name="handle">Buffer pool to allocate from.</param>
    /// <param name="buffer">Buffer to resize.</param>
    /// <param name="newSizeInBytes">Target size of the buffer to allocate in bytes.</param>
    /// <param name="copyCount">Number of bytes to copy from the old buffer into the new buffer.</param>    
    [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "BufferPool.Resize")]
    public static unsafe void Resize(InstanceHandle handle,  Buffer<byte>* buffer, int newSizeInBytes, int copyCount)
    {
        pools[handle].Resize(ref *buffer, newSizeInBytes, copyCount);
    }
    
    /// <summary>
    /// Resizes a buffer from the buffer pool to at least the given size, reallocating if necessary.
    /// </summary>
    /// <param name="handle">Buffer pool to allocate from.</param>
    /// <param name="buffer">Buffer to resize.</param>
    /// <param name="targetSizeInBytes">Target size of the buffer to allocate in bytes.</param>
    /// <param name="copyCount">Number of bytes to copy from the old buffer into the new buffer.</param>
    [UnmanagedCallersOnly(CallConvs = new[] { typeof(CallConvCdecl) }, EntryPoint = "BufferPool.ResizeToAtLeast")]
    public static unsafe void ResizeToAtLeast(InstanceHandle handle, Buffer<byte>* buffer, int targetSizeInBytes, int copyCount)
    {
        pools[handle].ResizeToAtLeast(ref *buffer, targetSizeInBytes, copyCount);
    }

    /// <summary>
    /// Returns a buffer to the buffer pool.
    /// </summary>
    /// <param name="handle">Buffer pool to return the buffer to.</param>
    /// <param name="buffer">Buffer to return to the pool.</param>
    [UnmanagedCallersOnly(CallConvs = new[] { typeof(CallConvCdecl) }, EntryPoint = "BufferPool.Deallocate")]
    public unsafe static void Deallocate(InstanceHandle handle, Buffer<byte>* buffer)
    {
        pools[handle].Return(ref *buffer);
    }

    /// <summary>
    /// Returns a buffer to the buffer pool by its id.
    /// </summary>
    /// <param name="handle">Buffer pool to return the buffer to.</param>
    /// <param name="bufferId">Id of the buffer to return to the pool.</param>
    [UnmanagedCallersOnly(CallConvs = new[] { typeof(CallConvCdecl) }, EntryPoint = "BufferPool.DeallocateById")]
    public static void DeallocateById(InstanceHandle handle, int bufferId)
    {
        pools[handle].ReturnUnsafely(bufferId);
    }
}