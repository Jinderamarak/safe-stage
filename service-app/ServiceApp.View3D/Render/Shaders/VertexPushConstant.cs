using System.Numerics;
using System.Runtime.InteropServices;
using Silk.NET.Vulkan;

namespace ServiceApp.View3D.Render.Shaders;

[StructLayout(LayoutKind.Sequential, Pack = 4)]
internal struct VertexPushConstant
{
    public Vector3 ObjectColor;
    
    internal static PushConstantRange PushConstantRange =>
        new()
        {
            Offset = 0,
            StageFlags = ShaderStageFlags.FragmentBit,
            Size = (uint)Marshal.SizeOf<VertexPushConstant>()
        };
}