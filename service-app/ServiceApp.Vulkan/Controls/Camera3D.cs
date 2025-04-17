using System.Numerics;
using Avalonia;
using Avalonia.Controls;
using ServiceApp.Vulkan.Data;

namespace ServiceApp.Vulkan.Controls;

public class Camera3D : Control
{
    private Vector3 _position = Vector3.One;

    public static readonly DirectProperty<Camera3D, Vector3> PositionProperty =
        AvaloniaProperty.RegisterDirect<Camera3D, Vector3>(nameof(Position), o => o.Position, (o, v) => o.Position = v);

    public Vector3 Position
    {
        get => _position;
        set => SetAndRaise(PositionProperty, ref _position, value);
    }

    private Vector3 _target = Vector3.Zero;

    public static readonly DirectProperty<Camera3D, Vector3> TargetProperty =
        AvaloniaProperty.RegisterDirect<Camera3D, Vector3>(nameof(Target), o => o.Target, (o, v) => o.Target = v);

    public Vector3 Target
    {
        get => _target;
        set => SetAndRaise(TargetProperty, ref _target, value);
    }

    private Vector3 _up = Vector3.UnitZ;

    public static readonly DirectProperty<Camera3D, Vector3> UpProperty =
        AvaloniaProperty.RegisterDirect<Camera3D, Vector3>(nameof(Up), o => o.Up, (o, v) => o.Up = v);

    public Vector3 Up
    {
        get => _up;
        set => SetAndRaise(UpProperty, ref _up, value);
    }

    private float _fov = MathF.PI / 4;

    public static readonly DirectProperty<Camera3D, float> FovProperty =
        AvaloniaProperty.RegisterDirect<Camera3D, float>(nameof(Fov), o => o.Fov, (o, v) => o.Fov = v);

    public float Fov
    {
        get => _fov;
        set => SetAndRaise(FovProperty, ref _fov, value);
    }

    private float _near = 0.1f;

    public static readonly DirectProperty<Camera3D, float> NearProperty =
        AvaloniaProperty.RegisterDirect<Camera3D, float>(nameof(Near), o => o.Near, (o, v) => o.Near = v);

    public float Near
    {
        get => _near;
        set => SetAndRaise(NearProperty, ref _near, value);
    }

    private float _far = 100f;

    public static readonly DirectProperty<Camera3D, float> FarProperty =
        AvaloniaProperty.RegisterDirect<Camera3D, float>(nameof(Far), o => o.Far, (o, v) => o.Far = v);

    public float Far
    {
        get => _far;
        set => SetAndRaise(FarProperty, ref _far, value);
    }

    internal CameraData GetCameraData()
    {
        return new CameraData
        {
            Position = Position,
            Target = Target,
            Up = Up,
            Fov = Fov,
            Near = Near,
            Far = Far
        };
    }
}