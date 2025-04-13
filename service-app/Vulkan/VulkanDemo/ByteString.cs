using System;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.InteropServices;

namespace GpuInterop.VulkanDemo;

unsafe class ByteString : IDisposable
{
    public IntPtr Pointer { get; }

    public ByteString(string s)
    {
        Pointer = Marshal.StringToHGlobalAnsi(s);
    }

    public void Dispose()
    {
        Marshal.FreeHGlobal(Pointer);
    }

    public static implicit operator byte*(ByteString h) => (byte*)h.Pointer;
}
    
unsafe class ByteStringList : IDisposable
{
    private int _size;
    private byte** _ptr;

    public ByteStringList(IEnumerable<string> items)
    {
        var inner = items.Select(x => new ByteString(x)).ToList();
        _size = inner.Count;
        _ptr = (byte**)Marshal.AllocHGlobal(IntPtr.Size * _size + 1);
        for (var c = 0; c < _size; c++)
            _ptr[c] = (byte*)inner[c].Pointer;
    }

    public int Count => _size;
    public uint UCount => (uint)_size;

    public void Dispose()
    {
        Marshal.FreeHGlobal(new IntPtr(_ptr));
    }

    public static implicit operator byte**(ByteStringList h) => h._ptr;
}
