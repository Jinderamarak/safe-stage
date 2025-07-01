using System.Drawing;
using System.Numerics;
using Avalonia.Controls;
using ServiceApp.View3D.Data;

namespace ServiceApp.View3D.Scene;

public class PointLight : Control
{
    public Vector3 Position
    {
        get => _position;
        set
        {
            lock (_lock)
            {
                if (_position != value)
                {
                    _position = value;
                    IsDirty = true;
                }
            }
        }
    }
    
    public Color Color
    {
        get => _color;
        set
        {
            lock (_lock)
            {
                if (_color != value)
                {
                    _color = value;
                    IsDirty = true;
                }
            }
        }
    }
    
    public float Strength
    {
        get => _strength;
        set
        {
            lock (_lock)
            {
                if (_strength != value)
                {
                    _strength = value;
                    IsDirty = true;
                }
            }
        }
    }

    internal bool IsDirty { get; private set; } = true;

    private readonly object _lock = new();

    private Vector3 _position = Vector3.One;
    private Color _color = Color.White;
    private float _strength = 1.0f;
 
    internal LightData GetDataAndResetDirty()
    {
        lock (_lock)
        {
            IsDirty = false;
            return new LightData
            {
                Position = _position,
                Color = _color,
                Strength = _strength
            };
        }
    }
}