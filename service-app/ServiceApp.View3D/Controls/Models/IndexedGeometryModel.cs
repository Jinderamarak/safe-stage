using System.Drawing;
using System.Numerics;
using Avalonia;
using Avalonia.Controls;
using ServiceApp.View3D.Data.Models;
using ServiceApp.View3D.Render;
using ServiceApp.View3D.Render.Shaders;

namespace ServiceApp.View3D.Controls.Models;

public class IndexedGeometryModel : Control, IGeometryModel
{
    private BufferedIndexedModel? _cached;
    private object _lock = new();

    private Color _color;

    public static readonly DirectProperty<IndexedGeometryModel, Color> ColorProperty =
        AvaloniaProperty.RegisterDirect<IndexedGeometryModel, Color>(nameof(Color), o => o.Color, (o, v) => o.Color = v);

    public Color Color
    {
        get => _color;
        set
        {
            Color old;
            lock (_lock)
            {
                if (_cached != null) _cached.Color = Color;
                old = _color;
                _color = value;
            }

            RaisePropertyChanged(ColorProperty, old, value);
        }
    }
    
    private (IEnumerable<Vector3>, IEnumerable<ushort>) _verticesAndIndices = (new List<Vector3>(), new List<ushort>());
    
    public static readonly DirectProperty<IndexedGeometryModel, (IEnumerable<Vector3>, IEnumerable<ushort>)> VerticesAndIndicesProperty =
        AvaloniaProperty.RegisterDirect<IndexedGeometryModel, (IEnumerable<Vector3>, IEnumerable<ushort>)>(nameof(VerticesAndIndices), o => o.VerticesAndIndices,
            (o, v) => o.VerticesAndIndices = v);
    
    public (IEnumerable<Vector3>, IEnumerable<ushort>) VerticesAndIndices
    {
        get => _verticesAndIndices;
        set
        {
            (IEnumerable<Vector3>, IEnumerable<ushort>) old;
            lock (_lock)
            {
                _cached?.UpdateModel(
            VertexInput.VerticesToInputs(value.Item1).ToArray(),
            value.Item2.ToArray());
                old = _verticesAndIndices;
                _verticesAndIndices = value;
            }
            
            RaisePropertyChanged(VerticesAndIndicesProperty, old, value);
        }
    }
    
    IDrawableObject IGeometryModel.GetOrCreateDrawable(VulkanContext context)
    {
        lock (_lock)
        {
            return _cached ??= new BufferedIndexedModel(
                VertexInput.VerticesToInputs(_verticesAndIndices.Item1).ToArray(),
                _verticesAndIndices.Item2.ToArray(),
                context);
        }
    }
}