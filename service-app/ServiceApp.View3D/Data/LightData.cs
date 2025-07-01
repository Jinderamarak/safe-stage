using System.Drawing;
using System.Numerics;

namespace ServiceApp.View3D.Data;

internal class LightData
{
    public Vector3 Position { get; init; }
    public Color Color { get; init; }
    public float Strength { get; init; }

    internal Vector3 ColorVector()
        => new(Color.R / 255f, Color.G / 255f, Color.B / 255f);
}