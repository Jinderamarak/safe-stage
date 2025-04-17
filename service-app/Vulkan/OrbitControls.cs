using System.Numerics;
using Avalonia;
using Avalonia.Controls;
using Avalonia.Input;
using ServiceApp.View3D.Controls;

namespace ServiceApp.Utility;

public class OrbitControls
{
    public double ScrollSensitivity { get; init; } = 0.05;
    public double DragSensitivity { get; init; } = 0.01;

    private readonly Camera3D _camera;
    private readonly PointLight _light;

    private Point _lastPosition;
    private bool _isDragging;

    private double _radialDistance;
    private double _polarAngle;
    private double _azimuthalAngle;

    public OrbitControls(
        Camera3D camera,
        PointLight light,
        double radialDistance = 1,
        double polarAngle = 1,
        double azimuthalAngle = 1)
    {
        _camera = camera;
        _light = light;

        _lastPosition = new Point();
        _isDragging = false;

        _radialDistance = radialDistance;
        _polarAngle = polarAngle;
        _azimuthalAngle = azimuthalAngle;

        RecalculateSphere();
    }

    public void MouseScrolled(PointerWheelEventArgs args)
    {
        var d = args.Delta.Y * ScrollSensitivity;
        _radialDistance -= args.Delta.Y * ScrollSensitivity;
        _radialDistance = Math.Clamp(_radialDistance, double.Epsilon, double.MaxValue);
        Console.WriteLine($"R: {_radialDistance}, D: {d}");
        RecalculateSphere();
    }

    public void MouseMoved(object? sender, PointerEventArgs args)
    {
        var point = args.GetCurrentPoint(sender as Control);
        if (!point.Properties.IsLeftButtonPressed)
        {
            _isDragging = false;
            return;
        }

        var position = point.Position;
        if (!_isDragging)
        {
            _isDragging = true;
            _lastPosition = position;
            return;
        }

        var delta = new Point(
            position.X - _lastPosition.X,
            position.Y - _lastPosition.Y
        );

        _azimuthalAngle -= delta.X * DragSensitivity;
        _polarAngle -= delta.Y * DragSensitivity;

        const double angleLimit = 1e-20;
        _polarAngle = Math.Clamp(_polarAngle, angleLimit, Math.PI - angleLimit);

        RecalculateSphere();

        _lastPosition = position;
    }

    private void RecalculateSphere()
    {
        var x = _radialDistance * Math.Sin(_polarAngle) * Math.Cos(_azimuthalAngle);
        var y = _radialDistance * Math.Sin(_polarAngle) * Math.Sin(_azimuthalAngle);
        var z = _radialDistance * Math.Cos(_polarAngle);

        _camera.Position = new Vector3
        {
            X = (float)x,
            Y = (float)y,
            Z = (float)z
        };
    }
}