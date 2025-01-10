
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using BepuUtilities.Memory;

namespace Bepuvy;

public static partial class Entrypoints
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
    [return: TypeName("BufferPoolHandle")]
    public static InstanceHandle CreateBufferPool(int minimumBlockAllocationSize = 131072, int expectedUsedSlotCountPerPool = 16)
    {
        return bufferPools.Add(new BufferPool(minimumBlockAllocationSize, expectedUsedSlotCountPerPool));
    }

    /// <summary>
    /// Releases all allocations held by the buffer pool. The buffer pool remains in a usable state.
    /// </summary>
    /// <param name="handle">Buffer pool to clear.</param>
    [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "BufferPool.Clear")]
    public static void ClearBufferPool([TypeName("BufferPoolHandle")] InstanceHandle handle)
    {
        bufferPools[handle].Clear();
    }

    /// <summary>
    /// Releases all allocations held by the buffer pool and releases the buffer pool reference. The handle is invalidated.
    /// </summary>
    /// <param name="handle">Buffer pool to destroy.</param>
    [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "BufferPool.Destroy")]
    public static void DestroyBufferPool([TypeName("BufferPoolHandle")] InstanceHandle handle)
    {
        bufferPools[handle].Clear();
        bufferPools.Remove(handle);
    }

    /// <summary>
    /// Allocates a buffer from the buffer pool of the given size.
    /// </summary>
    /// <param name="bufferPoolHandle">Buffer pool to allocate from.</param>
    /// <param name="sizeInBytes">Size of the buffer to allocate in bytes.</param>
    /// <returns>Allocated buffer.</returns>
    [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "BufferPool.Allocate")]
    [return: TypeName("ByteBuffer")]
    public static Buffer<byte> Allocate([TypeName("BufferPoolHandle")] InstanceHandle bufferPoolHandle, int sizeInBytes)
    {
        bufferPools[bufferPoolHandle].Take<byte>(sizeInBytes, out var buffer);
        return buffer;
    }

    /// <summary>
    /// Allocates a buffer from the buffer pool with at least the given size.
    /// </summary>
    /// <param name="bufferPoolHandle">Buffer pool to allocate from.</param>
    /// <param name="sizeInBytes">Size of the buffer to allocate in bytes.</param>
    /// <returns>Allocated buffer.</returns>
    [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "BufferPool.AllocateAtLeast")]
    [return: TypeName("ByteBuffer")]
    public static Buffer<byte> AllocateAtLeast([TypeName("BufferPoolHandle")] InstanceHandle bufferPoolHandle, int sizeInBytes)
    {
        bufferPools[bufferPoolHandle].TakeAtLeast<byte>(sizeInBytes, out var buffer);
        return buffer;
    }

    /// <summary>
    /// Resizes a buffer from the buffer pool to the given size, reallocating if necessary.
    /// </summary>
    /// <param name="bufferPoolHandle">Buffer pool to allocate from.</param>
    /// <param name="buffer">Buffer to resize.</param>
    /// <param name="newSizeInBytes">Target size of the buffer to allocate in bytes.</param>
    /// <param name="copyCount">Number of bytes to copy from the old buffer into the new buffer.</param>
    [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "BufferPool.Resize")]
    public static unsafe void Resize([TypeName("BufferPoolHandle")] InstanceHandle bufferPoolHandle, [TypeName("ByteBuffer*")] Buffer<byte>* buffer, int newSizeInBytes, int copyCount)
    {
        bufferPools[bufferPoolHandle].Resize(ref *buffer, newSizeInBytes, copyCount);
    }

    /// <summary>
    /// Resizes a buffer from the buffer pool to at least the given size, reallocating if necessary.
    /// </summary>
    /// <param name="bufferPoolHandle">Buffer pool to allocate from.</param>
    /// <param name="buffer">Buffer to resize.</param>
    /// <param name="targetSizeInBytes">Target size of the buffer to allocate in bytes.</param>
    /// <param name="copyCount">Number of bytes to copy from the old buffer into the new buffer.</param>
    [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "BufferPool.ResizeToAtLeast")]
    public static unsafe void ResizeToAtLeast([TypeName("BufferPoolHandle")] InstanceHandle bufferPoolHandle, [TypeName("ByteBuffer*")] Buffer<byte>* buffer, int targetSizeInBytes, int copyCount)
    {
        bufferPools[bufferPoolHandle].ResizeToAtLeast(ref *buffer, targetSizeInBytes, copyCount);
    }

    /// <summary>
    /// Returns a buffer to the buffer pool.
    /// </summary>
    /// <param name="bufferPoolHandle">Buffer pool to return the buffer to.</param>
    /// <param name="buffer">Buffer to return to the pool.</param>
    [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "BufferPool.Deallocate")]
    public unsafe static void Deallocate([TypeName("BufferPoolHandle")] InstanceHandle bufferPoolHandle, [TypeName("ByteBuffer*")] Buffer<byte>* buffer)
    {
        bufferPools[bufferPoolHandle].Return(ref *buffer);
    }

    /// <summary>
    /// Returns a buffer to the buffer pool by its id.
    /// </summary>
    /// <param name="bufferPoolHandle">Buffer pool to return the buffer to.</param>
    /// <param name="bufferId">Id of the buffer to return to the pool.</param>
    [UnmanagedCallersOnly(CallConvs = new[] { typeof(CallConvCdecl) }, EntryPoint = "BufferPool.DeallocateId")]
    public unsafe static void DeallocateById([TypeName("BufferPoolHandle")] InstanceHandle bufferPoolHandle, int bufferId)
    {
        bufferPools[bufferPoolHandle].ReturnUnsafely(bufferId);
    }

    /// <summary>
    /// Computes the total number of bytes allocated from native memory in this buffer pool.
    /// Includes allocated memory regardless of whether it currently has outstanding references.
    /// </summary>
    /// <param name="bufferPoolHandle">Buffer pool to check the allocation size of.</param>
    /// <returns>Total number of bytes allocated from native memory in this buffer pool.</returns>
    [UnmanagedCallersOnly(CallConvs = new[] { typeof(CallConvCdecl) }, EntryPoint = "BufferPool.GetAllocatedMemorySizeInPool")]
    public unsafe static ulong GetAllocatedMemorySizeInPool([TypeName("BufferPoolHandle")] InstanceHandle bufferPoolHandle)
    {
        return bufferPools[bufferPoolHandle].GetTotalAllocatedByteCount();
    }
}
