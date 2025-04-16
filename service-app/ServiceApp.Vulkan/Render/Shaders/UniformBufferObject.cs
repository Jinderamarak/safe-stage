using System.Numerics;
using System.Runtime.InteropServices;

namespace ServiceApp.Vulkan.Render.Shaders;

[StructLayout(LayoutKind.Sequential, Pack = 4)]
internal struct UniformBufferObject
{
    public Matrix4x4 Projection;
    public Vector3 LightPosition;
    public Vector3 LightColor;
    public float LightStrength;
}