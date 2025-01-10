using System.Runtime.InteropServices;
using System.Runtime.CompilerServices;
using BepuUtilities;

namespace Bepuvy;

public static partial class Entrypoints
{
    /// <summary>
    /// Creates a new thread dispatcher.
    /// </summary>
    /// <param name="threadCount">Number of threads to use within the thread dispatcher.</param>
    /// <param name="threadPoolAllocationBlockSize">Minimum size in bytes of blocks allocated in per-thread buffer pools. Allocations requiring more space can result in larger block sizes, but no pools will allocate smaller blocks.</param>
    [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "ThreadDispatcher.Create")]
    [return: TypeName("ThreadDispatcherHandle")]
    public static InstanceHandle CreateThreadDispatcher(int threadCount, int threadPoolAllocationBlockSize = 16384)
    {
        return threadDispatchers.Add(new ThreadDispatcher(threadCount, threadPoolAllocationBlockSize));
    }

    /// <summary>
    /// Releases all resources held by a thread dispatcher and invalidates its handle.
    /// </summary>
    /// <param name="handle">Thread dispatcher to destroy.</param>
    [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "ThreadDisaptcher.Destroy")]
    public static void DestroyThreadDispatcher([TypeName("ThreadDispatcherHandle")] InstanceHandle handle)
    {
        threadDispatchers[handle].Dispose();
        threadDispatchers.Remove(handle);
    }

    /// <summary>
    /// Releases all resources held by a thread dispatcher and invalidates its handle.
    /// </summary>
    /// <param name="handle">Thread dispatcher to check the thread count of.</param>
    [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "ThreadDispatcher.GetThreadCount")]
    public static int GetThreadCount([TypeName("ThreadDispatcherHandle")] InstanceHandle handle)
    {
        return threadDispatchers[handle].ThreadCount;
    }


    /// <summary>
    /// Computes the total number of bytes allocated from native memory in a dispatcher's per-thread pools.
    /// Includes allocated memory regardless of whether it currently has outstanding references.
    /// </summary>
    /// <param name="threadDispatcherHandle">Thread dispatcher to check allocations for.</param>
    /// <returns>Total number of bytes allocated from native memory in this thread dispatcher's per-thread pool.</returns>
    [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "ThreadDispatcher.GetAllocatedMemorySize")]
    public static ulong GetAllocatedMemorySizeInThreadDispatcher([TypeName("ThreadDispatcherHandle")] InstanceHandle threadDispatcherHandle)
    {
        ulong sum = 0;
        var dispatcher = threadDispatchers[threadDispatcherHandle];
        for (var i = 0; i < dispatcher.ThreadCount; ++i)
        {
            sum += dispatcher.WorkerPools[i].GetTotalAllocatedByteCount();
        }
        return sum;
    }
}
