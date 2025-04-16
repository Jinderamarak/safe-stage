using System.Numerics;
using System.Runtime.InteropServices;

namespace ServiceApp.Vulkan.Render.Shaders;

[StructLayout(LayoutKind.Sequential, Pack = 4)]
internal struct VertexPushConstant
{
    public Vector3 ObjectColor;
}