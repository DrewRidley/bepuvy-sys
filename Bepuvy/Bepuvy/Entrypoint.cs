using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using BepuPhysics;
using BepuUtilities;
using BepuUtilities.Memory;

namespace Bepuvy;

public static partial class Entrypoint
{
    private static InstanceDirectory<BufferPool> pools = new InstanceDirectory<BufferPool>(0);
    private static InstanceDirectory<Simulation> simulations = new InstanceDirectory<Simulation>(1);
    private static InstanceDirectory<ThreadDispatcher> dispatchers = new InstanceDirectory<ThreadDispatcher>(2);

    /// <summary>
    /// Creates a new thread dispatcher.
    /// </summary>
    /// <param name="threadCount">Number of threads to use within the thread dispatcher.</param>
    /// <param name="threadPoolAllocationBlockSize">Minimum size in bytes of blocks allocated in per-thread buffer pools. Allocations requiring more space can result in larger block sizes, but no pools will allocate smaller blocks.</param>
    [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "ThreadDispatcher.Create")]
    public static InstanceHandle CreateThreadDispatcher(int threadCount, int threadPoolAllocationBlockSize = 16384)
    {
        return dispatchers.Add(new ThreadDispatcher(threadCount, threadPoolAllocationBlockSize));
    }

    /// <summary>
    /// Releases all resources held by a thread dispatcher and invalidates its handle.
    /// </summary>
    /// <param name="handle">Thread dispatcher to destroy.</param>
    [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "ThreadDispatcher.Destroy")]
    public static void DestroyThreadDispatcher(InstanceHandle handle)
    {
        dispatchers[handle].Dispose();
        dispatchers.Remove(handle);
    }
}
