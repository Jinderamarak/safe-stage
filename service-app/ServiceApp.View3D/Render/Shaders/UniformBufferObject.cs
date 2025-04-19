using System.Numerics;
using System.Runtime.InteropServices;
using Silk.NET.Vulkan;

namespace ServiceApp.View3D.Render.Shaders;

[StructLayout(LayoutKind.Sequential, Pack = 4)]
internal struct UniformBufferObject
{
    public Matrix4x4 Projection;
    public Vector3 LightPosition;
    public Vector3 LightColor;
    public float LightStrength;
    
    internal static DescriptorSetLayoutBinding DescriptorSetLayoutBinding =>
        new()
        {
            Binding = 0,
            DescriptorType = DescriptorType.UniformBuffer,
            DescriptorCount = 1,
            StageFlags = ShaderStageFlags.VertexBit | ShaderStageFlags.FragmentBit
        };
}