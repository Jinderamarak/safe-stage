using System.Drawing;
using System.Numerics;
using Avalonia;
using Avalonia.Controls;
using ServiceApp.View3D.Data;

namespace ServiceApp.View3D.Controls;

public class PointLight : Control
{
    private Vector3 _position = Vector3.One;

    public static readonly DirectProperty<PointLight, Vector3> PositionProperty =
        AvaloniaProperty.RegisterDirect<PointLight, Vector3>(nameof(Position), o => o.Position,
            (o, v) => o.Position = v);

    public Vector3 Position
    {
        get => _position;
        set => SetAndRaise(PositionProperty, ref _position, value);
    }

    private Color _color = Color.White;

    public static readonly DirectProperty<PointLight, Color> ColorProperty =
        AvaloniaProperty.RegisterDirect<PointLight, Color>(nameof(Color), o => o.Color, (o, v) => o.Color = v);

    public Color Color
    {
        get => _color;
        set => SetAndRaise(ColorProperty, ref _color, value);
    }

    private float _strength = 1f;

    public static readonly DirectProperty<PointLight, float> StrengthProperty =
        AvaloniaProperty.RegisterDirect<PointLight, float>(nameof(Strength), o => o.Strength, (o, v) => o.Strength = v);

    public float Strength
    {
        get => _strength;
        set => SetAndRaise(StrengthProperty, ref _strength, value);
    }

    internal LightData GetLightData()
    {
        return new LightData
        {
            Position = Position,
            Color = Color,
            Strength = Strength
        };
    }
}