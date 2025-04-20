using System.Drawing;
using System.Numerics;
using System.Runtime.InteropServices;
using ServiceApp.View3D.Render;
using ServiceApp.View3D.Render.Shaders;
using Silk.NET.Vulkan;
using VulkanBuffer = Silk.NET.Vulkan.Buffer;

namespace ServiceApp.View3D.Data.Models;

internal class BufferedModel : IDisposable, IDrawableObject
{
    public Color Color { get; set; }

    private int _vertexCount;
    private VulkanBuffer _vertexBuffer;
    private DeviceMemory _vertexMemory;
    
    private readonly VulkanContext _context;

    private readonly object _lock = new();

    internal BufferedModel(Span<VertexInput> vertices, VulkanContext context)
    {
        _context = context;
        UpdateModel(vertices);
    }

    public unsafe void Dispose()
    {
        _context.Api.DestroyBuffer(_context.Device, _vertexBuffer, null);
        _context.Api.FreeMemory(_context.Device, _vertexMemory, null);
    }

    public void UpdateModel(Span<VertexInput> vertices)
    {
        lock (_lock)
        {
            VulkanBufferHelper.UpdateOrReallocateBuffer(
                _context,
                BufferUsageFlags.VertexBufferBit,
                ref _vertexBuffer,
                ref _vertexMemory,
                ref _vertexCount,
                vertices
            );
        }
    }

    public unsafe void Draw(Vk api, CommandBuffer cmd, PipelineLayout pipeline)
    {
        var constants = new VertexPushConstant
        {
            ObjectColor = new Vector3(Color.R / 255f, Color.G / 255f, Color.B / 255f)
        };

        api.CmdPushConstants(cmd, pipeline, ShaderStageFlags.FragmentBit, 0, (uint)Marshal.SizeOf<VertexPushConstant>(),
            &constants);

        lock (_lock)
        {
            api.CmdBindVertexBuffers(cmd, 0, 1, _vertexBuffer, 0);
            api.CmdDraw(cmd, (uint)_vertexCount, 1, 0, 0);
        }
    }
}