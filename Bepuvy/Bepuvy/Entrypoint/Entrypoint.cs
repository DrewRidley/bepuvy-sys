using BepuPhysics;
using BepuPhysics.Collidables;
using BepuPhysics.CollisionDetection;
using BepuUtilities;
using BepuUtilities.Collections;
using BepuUtilities.Memory;
using System;
using System.CodeDom;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;
using System.Numerics;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using System.Runtime.Intrinsics;
using System.Security.Cryptography.X509Certificates;
using System.Text;
using System.Threading.Tasks;
using Bepuvy.Callbacks;

namespace Bepuvy;


public class TypeNameAttribute : Attribute
{
    public string TypeName;
    public TypeNameAttribute(string typeName)
    {
        TypeName = typeName;
    }
}



public static partial class Entrypoints
{
    static InstanceDirectory<BufferPool> bufferPools = new InstanceDirectory<BufferPool>(0);
    static InstanceDirectory<Simulation> simulations = new InstanceDirectory<Simulation>(1);
    static InstanceDirectory<ThreadDispatcher> threadDispatchers = new InstanceDirectory<ThreadDispatcher>(2);
    static InstanceDirectory<RustCallback> callbacks = new InstanceDirectory<RustCallback>(3);

    /// <summary>
    /// Gets the number of threads exposed by the operating system on this platform. Cores with SMT can show as having multiple threads.
    /// </summary>
    /// <returns>Number of threads exposed by the operating system on this platform.</returns>
    [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "Utils.GetPlatformThreadCount")]
    public static int GetPlatformThreadCount()
    {
        return Environment.ProcessorCount;
    }
    
    /// <summary>
    /// Gets the number of threads exposed by the operating system on this platform. Cores with SMT can show as having multiple threads.
    /// </summary>
    /// <returns>Number of threads exposed by the operating system on this platform.</returns>
    [UnmanagedCallersOnly(CallConvs = [typeof(CallConvCdecl)], EntryPoint = "Utils.GetSIMDWidth")]
    public static int GetSimdWidth()
    {
        return Vector<int>.Count;
    }

    /// <summary>
    /// Estimates the number of bytes managed by the garbage collector.
    /// </summary>
    /// <returns>Estimated number of bytes allocated from managed memory.</returns>
    [UnmanagedCallersOnly(CallConvs = new[] { typeof(CallConvCdecl) }, EntryPoint = "Utils.GetGCMemorySize")]
    public unsafe static ulong GetGCAllocatedMemorySize()
    {
        return (ulong)GC.GetTotalMemory(false);
    }
}
