using System.Drawing;
using System.Numerics;
using Avalonia;
using Avalonia.Controls;
using ServiceApp.View3D.Data;
using ServiceApp.View3D.Render;
using ServiceApp.View3D.Render.Shaders;

namespace ServiceApp.View3D.Controls;

public class GeometryModel : Control
{
    private BufferedObject? _cached;
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
                _cached?.UpdateVertices(VerticesToInputs(value).ToArray());
                old = _vertices;
                _vertices = value;
            }

            RaisePropertyChanged(VerticesProperty, old, value);
        }
    }

    internal BufferedObject GetOrCreateBuffered(VulkanContext context)
    {
        lock (_lock)
        {
            return _cached ??= new BufferedObject(VerticesToInputs(_vertices).ToArray(), context)
            {
                Color = _color
            };
        }
    }

    private static IEnumerable<VertexInput> VerticesToInputs(IEnumerable<Vector3> vertices)
    {
        using var enumerator = vertices.GetEnumerator();
        while (true)
        {
            var has = enumerator.MoveNext();
            if (!has) yield break;
            var first = enumerator.Current;

            has = enumerator.MoveNext();
            if (!has) yield break;
            var second = enumerator.Current;

            has = enumerator.MoveNext();
            if (!has) yield break;
            var third = enumerator.Current;

            var normal = Vector3.Normalize(Vector3.Cross(third - second, first - second));
            yield return new VertexInput
            {
                Position = first,
                Normal = normal
            };
            yield return new VertexInput
            {
                Position = second,
                Normal = normal
            };
            yield return new VertexInput
            {
                Position = third,
                Normal = normal
            };
        }
    }
}