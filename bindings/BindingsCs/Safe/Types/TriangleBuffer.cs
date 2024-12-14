namespace BindingsCs.Safe.Types;

public class TriangleBuffer
{
    public Vector3[] Buffer { get; }

    public TriangleBuffer(Vector3[] buffer)
    {
        Buffer = buffer;
    }

    internal static List<TriangleBuffer> FromNativeVec(Unsafe.TriangleBufferVec vec)
    {
        var list = new List<TriangleBuffer>();
        unsafe
        {
            for (uint i = 0; i < vec.len; i++) list.Add(new TriangleBuffer(vec.data[i]));
        }

        return list;
    }

    internal TriangleBuffer(Unsafe.TriangleBuffer nativeBuffer)
    {
        Buffer = new Vector3[nativeBuffer.len];
        unsafe
        {
            for (uint i = 0; i < nativeBuffer.len; i++) Buffer[i] = new Vector3(nativeBuffer.data[i]);
        }

        Unsafe.NativeMethods.trianglebuffer_drop(nativeBuffer);
    }
}