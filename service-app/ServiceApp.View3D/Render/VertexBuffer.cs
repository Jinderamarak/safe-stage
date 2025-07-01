using ServiceApp.View3D.Render.Shaders;
using Silk.NET.Vulkan;
using SkiaSharp;
using VulkanBuffer = Silk.NET.Vulkan.Buffer;

namespace ServiceApp.View3D.Render;

internal class VertexBuffer : IDisposable
{
    public int Count { get; private init; }
    
    private readonly object _lock = new();
    private bool _isUsed = true;
    private bool _isRendered = false;
    
    private readonly VulkanContext _context;
    
    private VulkanBuffer? _vertexBuffer;
    private DeviceMemory? _vertexMemory;

    public VertexBuffer(VulkanContext context, int vertexCount, Span<VertexInput> vertices)
    {
        _context = context;
        Count = vertexCount;
        
        VulkanBufferHelper.AllocateBuffer(
            _context,
            BufferUsageFlags.VertexBufferBit,
            out var vertexBuffer,
            out var vertexMemory,
            vertices
        );
        _vertexBuffer = vertexBuffer;
        _vertexMemory = vertexMemory;
    }

    public void UpdateVertices(Span<VertexInput> vertices)
    {
        if (_vertexMemory.HasValue)
            VulkanBufferHelper.UpdateBufferMemory(_context, _vertexMemory.Value, vertices);
    }
    
    public void Dispose()
    {
        unsafe
        {
            if (_vertexBuffer.HasValue)
            {
                _context.Api.DestroyBuffer(_context.Device, _vertexBuffer.Value, null);
                _vertexBuffer = null;
            }

            if (_vertexMemory.HasValue)
            {
                _context.Api.FreeMemory(_context.Device, _vertexMemory.Value, null);
                _vertexMemory = null;
            }
        }
    }

    public void MarkUnused()
    {
        lock (_lock)
        {
            _isUsed = false;
            if (!_isRendered)
            {
                Dispose();
            }
        }
    }

    public void MarkRenderBegin()
    {
        lock (_lock)
        {
            _isRendered = true;
        }
    }

    public void MarkRenderEnd()
    {
        lock (_lock)
        {
            _isRendered = false;
            if (!_isUsed)
            {
                Dispose();
            }
        }
    }
}
