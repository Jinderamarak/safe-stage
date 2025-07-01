using System.Numerics;

namespace ServiceApp.View3D.Data;

internal class CameraData
{
    public Vector3 Position { get; init; }
    public Vector3 Target { get; init; }
    public Vector3 Up { get; init; }

    public float Fov { get; init; }
    public float Near { get; init; }
    public float Far { get; init; }

    internal Matrix4x4 ProjectionView(int width, int height)
        => Matrix4x4.CreateLookAt(Position, Target, Up)
           * Matrix4x4.CreatePerspectiveFieldOfView(Fov, (float)width / height, Near, Far);
}