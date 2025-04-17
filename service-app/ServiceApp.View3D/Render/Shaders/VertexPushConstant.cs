using System.Numerics;
using System.Runtime.InteropServices;

namespace ServiceApp.View3D.Render.Shaders;

[StructLayout(LayoutKind.Sequential, Pack = 4)]
internal struct VertexPushConstant
{
    public Vector3 ObjectColor;
}