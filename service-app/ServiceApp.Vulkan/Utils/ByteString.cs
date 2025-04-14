using System.Runtime.InteropServices;

namespace ServiceApp.Vulkan.Utils;

internal unsafe class ByteString : IDisposable
{
    public ByteString(string s)
    {
        Pointer = Marshal.StringToHGlobalAnsi(s);
    }

    public IntPtr Pointer { get; }

    public void Dispose()
    {
        Marshal.FreeHGlobal(Pointer);
    }

    public static implicit operator byte*(ByteString h)
    {
        return (byte*)h.Pointer;
    }
}

internal unsafe class ByteStringList : IDisposable
{
    private readonly byte** _ptr;

    public ByteStringList(IEnumerable<string> items)
    {
        var inner = items.Select(x => new ByteString(x)).ToList();
        Count = inner.Count;
        _ptr = (byte**)Marshal.AllocHGlobal(IntPtr.Size * Count + 1);
        for (var c = 0; c < Count; c++)
            _ptr[c] = (byte*)inner[c].Pointer;
    }

    public int Count { get; }

    public uint UCount => (uint)Count;

    public void Dispose()
    {
        Marshal.FreeHGlobal(new IntPtr(_ptr));
    }

    public static implicit operator byte**(ByteStringList h)
    {
        return h._ptr;
    }
}