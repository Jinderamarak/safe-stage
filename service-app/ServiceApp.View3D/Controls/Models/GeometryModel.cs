using System.Drawing;
using System.Numerics;
using Avalonia;
using Avalonia.Controls;
using ServiceApp.View3D.Data.Models;
using ServiceApp.View3D.Render;
using ServiceApp.View3D.Render.Shaders;

namespace ServiceApp.View3D.Controls.Models;

public class GeometryModel : Control, IGeometryModel
{
    private BufferedModel? _cached;
    private object _lock = new();

    private Color _color;

    public static readonly DirectProperty<GeometryModel, Color> ColorProperty =
        AvaloniaProperty.RegisterDirect<GeometryModel, Color>(nameof(Color), o => o.Color, (o, v) => o.Color = v);

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

    private IEnumerable<Vector3> _vertices = new List<Vector3>();

    public static readonly DirectProperty<GeometryModel, IEnumerable<Vector3>> VerticesProperty =
        AvaloniaProperty.RegisterDirect<GeometryModel, IEnumerable<Vector3>>(nameof(Vertices), o => o.Vertices,
            (o, v) => o.Vertices = v);

    public IEnumerable<Vector3> Vertices
    {
        get => _vertices;
        set
        {
            IEnumerable<Vector3> old;
            lock (_lock)
            {
                _cached?.UpdateModel(VertexInput.VerticesToInputs(value).ToArray());
                old = _vertices;
                _vertices = value;
            }

            RaisePropertyChanged(VerticesProperty, old, value);
        }
    }

    IDrawableObject IGeometryModel.GetOrCreateDrawable(VulkanContext context)
    {
        lock (_lock)
        {
            return _cached ??= new BufferedModel(VertexInput.VerticesToInputs(_vertices).ToArray(), context)
            {
                Color = _color
            };
        }
    }
}