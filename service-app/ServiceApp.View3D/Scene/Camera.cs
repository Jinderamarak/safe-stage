using System.Numerics;
using ServiceApp.View3D.Data;

namespace ServiceApp.View3D.Scene;

public class Camera
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
    
    public Vector3 Target
    {
        get => _target;
        set
        {
            lock (_lock)
            {
                if (_target != value)
                {
                    _target = value;
                    IsDirty = true;
                }
            }
        }
    }
    
    public Vector3 Up
    {
        get => _up;
        set
        {
            lock (_lock)
            {
                if (_up != value)
                {
                    _up = value;
                    IsDirty = true;
                }
            }
        }
    }
    
    public float Fov
    {
        get => _fov;
        set
        {
            lock (_lock)
            {
                if (_fov != value)
                {
                    _fov = value;
                    IsDirty = true;
                }
            }
        }
    }
    
    public float Near
    {
        get => _near;
        set
        {
            lock (_lock)
            {
                if (_near != value)
                {
                    _near = value;
                    IsDirty = true;
                }
            }
        }
    }
    
    public float Far
    {
        get => _far;
        set
        {
            lock (_lock)
            {
                if (_far != value)
                {
                    _far = value;
                    IsDirty = true;
                }
            }
        }
    }
    
    internal bool IsDirty { get; private set; } = true;

    private readonly object _lock = new();

    private Vector3 _position = Vector3.One;
    private Vector3 _target = Vector3.Zero;
    private Vector3 _up = Vector3.UnitZ;
    private float _fov = MathF.PI / 4;
    private float _near = 0.1f;
    private float _far = 100f;
 
    internal CameraData GetDataAndResetDirty()
    {
        lock (_lock)
        {
            IsDirty = false;
            return new CameraData
            {
                Position = _position,
                Target = _target,
                Up = _up,
                Fov = _fov,
                Near = _near,
                Far = _far
            };
        }
    }
}