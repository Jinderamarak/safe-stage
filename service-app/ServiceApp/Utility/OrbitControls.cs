using System.Windows;
using System.Windows.Input;
using System.Windows.Media;
using System.Windows.Media.Media3D;

namespace ServiceApp.Utility;

public class OrbitControls
{
    public double ScrollSensitivity { get; init; } = 1.0;
    public double DragSensitivity { get; init; } = 1.0;

    private readonly PerspectiveCamera _camera;
    private readonly DirectionalLight _light;

    private Point _lastPosition;
    private bool _isDragging;

    private double _radialDistance;
    private double _polarAngle;
    private double _azimuthalAngle;

    private Point3D _position;
    private Vector3D _direction;

    public OrbitControls(
        PerspectiveCamera camera,
        DirectionalLight light,
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

        CompositionTarget.Rendering += BeforeRender;
    }

    ~OrbitControls()
    {
        CompositionTarget.Rendering -= BeforeRender;
    }

    private void BeforeRender(object? sender, EventArgs args)
    {
        _camera.Position = _position;
        _camera.LookDirection = _direction;
        _light.Direction = _direction;
    }

    public void MouseScrolled(MouseWheelEventArgs args)
    {
        _radialDistance -= args.Delta * ScrollSensitivity;
        _radialDistance = Math.Clamp(_radialDistance, double.Epsilon, double.MaxValue);
        RecalculateSphere();
    }

    public void MouseMoved(Point position, MouseEventArgs args)
    {
        if (args.LeftButton == MouseButtonState.Released)
        {
            _isDragging = false;
            return;
        }

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

        _position = new Point3D(x, y, z);
        _direction = new Vector3D(-x, -y, -z);
    }
}