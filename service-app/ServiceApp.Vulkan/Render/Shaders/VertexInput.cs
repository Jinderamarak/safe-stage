using System.Numerics;
using System.Runtime.InteropServices;
using Silk.NET.Vulkan;

namespace ServiceApp.Vulkan.Render.Shaders;

[StructLayout(LayoutKind.Sequential, Pack = 4)]
public struct VertexInput
{
    public Vector3 Position;
    public Vector3 Normal;

    internal static VertexInputBindingDescription VertexInputBindingDescription =>
        new()
        {
            Binding = 0,
            Stride = (uint)Marshal.SizeOf<VertexInput>(),
            InputRate = VertexInputRate.Vertex
        };

    internal static VertexInputAttributeDescription[] VertexInputAttributeDescription =>
    [
        new()
        {
            Binding = 0,
            Location = 0,
            Format = Format.R32G32B32Sfloat,
            Offset = (uint)Marshal.OffsetOf<VertexInput>(nameof(Position))
        },
        new()
        {
            Binding = 0,
            Location = 1,
            Format = Format.R32G32B32Sfloat,
            Offset = (uint)Marshal.OffsetOf<VertexInput>(nameof(Normal))
        }
    ];
}