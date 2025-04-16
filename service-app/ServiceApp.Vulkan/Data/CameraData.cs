using System.Numerics;

namespace ServiceApp.Vulkan.Data;

public class CameraData
{
    public Vector3 Position { get; set; } = Vector3.One;
    public Vector3 Target { get; set; } = Vector3.Zero;
    public Vector3 Up { get; set; } = Vector3.UnitZ;

    public float Fov { get; set; } = MathF.PI / 4;
    public float Near { get; set; } = 0.1f;
    public float Far { get; set; } = 100f;

    internal Matrix4x4 ProjectionView(int width, int height)
    {
        return Matrix4x4.CreateLookAt(Position, Target, Up)
               * Matrix4x4.CreatePerspectiveFieldOfView(Fov, (float)width / height, Near, Far);
    }
}