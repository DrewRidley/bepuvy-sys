using System.Diagnostics;

namespace Bepuvy;

/// <summary>
/// Points to an instance in an instance directory.
/// </summary>
public struct InstanceHandle
{
    public int RawValue;

    public int Index => RawValue & 0x00FF_FFFF;
    public int Version => (RawValue >> 24) & 0xF;
    public int TypeIndex => (RawValue >> 28) & 0x7;

    public bool Null => RawValue == 0;

    public InstanceHandle(int index, int version, int typeIndex)
    {
        Debug.Assert(index < (1 << 24), "This handle assumes there are less than 2^24 instances. There really should be less than a few dozen. Something is probably wrong.");
        Debug.Assert(typeIndex < 8, "This handle assumes there are less than 8 types being registered into instance directories. Bepuphysics2 doesn't need many; if there's more, something may be wrong or this may need to be changed.");
        RawValue = (1 << 31) | index | (version << 24) | (typeIndex << 28);
    }
}