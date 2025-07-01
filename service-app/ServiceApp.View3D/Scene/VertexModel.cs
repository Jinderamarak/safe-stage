using System.Drawing;
using ServiceApp.View3D.Data;

namespace ServiceApp.View3D.Scene;

public class VertexModel
{
    public Color Color
    {
        get => _object.Color;
        set
        {
            lock (_lock)
            {
                if (_object.Color != value)
                {
                    _object.Color = value;
                    IsDirty = true;
                }
            }
        }
    }
    
    internal bool IsDirty { get; private set; } = true;

    private readonly object _lock = new();

    private readonly BufferedObject _object;
}