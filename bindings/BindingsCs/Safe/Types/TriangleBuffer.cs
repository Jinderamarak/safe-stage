namespace BindingsCs.Safe.Types;

public class TriangleBuffer
{
    public Vector3[] Buffer { get; }
    
    public TriangleBuffer(Vector3[] buffer)
    {
        Buffer = buffer;
    }

    internal TriangleBuffer(Unsafe.TriangleBuffer nativeBuffer)
    {
        Buffer = new Vector3[nativeBuffer.len];
        unsafe
        {
            for (uint i = 0; i < nativeBuffer.len; i++)
            {
                Buffer[i] = new Vector3(nativeBuffer.data[i]);
            }
        }
        
        Unsafe.NativeMethods.trianglebuffer_drop(nativeBuffer);
    }
}