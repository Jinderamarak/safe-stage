using System.Drawing;
using System.Numerics;
using System.Runtime.InteropServices;
using ServiceApp.Vulkan.Render;
using ServiceApp.Vulkan.Render.Shaders;
using Silk.NET.Vulkan;
using VulkanBuffer = Silk.NET.Vulkan.Buffer;

namespace ServiceApp.Vulkan.Data;

public class BufferedObject : IDisposable
{
    private readonly VulkanContext _context;

    private readonly object _lock = new();
    private VulkanBuffer _vertexBuffer;

    private int _vertexCount;
    private DeviceMemory _vertexMemory;

    internal BufferedObject(Span<VertexInput> vertices, VulkanContext context)
    {
        _context = context;
        UpdateVertices(vertices);
    }

    public Color Color { get; set; }

    public void Dispose()
    {
        unsafe
        {
            _context.Api.DestroyBuffer(_context.Device, _vertexBuffer, null);
            _context.Api.FreeMemory(_context.Device, _vertexMemory, null);
        }
    }

    public void UpdateVertices(Span<VertexInput> vertices)
    {
        lock (_lock)
        {
            if (vertices.Length != _vertexCount)
            {
                if (_vertexBuffer.Handle != 0)
                    unsafe
                    {
                        _context.Api.DestroyBuffer(_context.Device, _vertexBuffer, null);
                        _context.Api.FreeMemory(_context.Device, _vertexMemory, null);
                    }

                VulkanBufferHelper.AllocateBuffer<VertexInput>(
                    _context,
                    BufferUsageFlags.VertexBufferBit,
                    out _vertexBuffer,
                    out _vertexMemory,
                    vertices
                );
                _vertexCount = vertices.Length;
            }
            else
            {
                VulkanBufferHelper.UpdateBufferMemory<VertexInput>(_context, _vertexMemory, vertices);
            }
        }
    }

    internal unsafe void Draw(Vk api, CommandBuffer cmd, PipelineLayout pipeline)
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