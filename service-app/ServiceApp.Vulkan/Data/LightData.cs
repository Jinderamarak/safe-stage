using System.Drawing;
using System.Numerics;

namespace ServiceApp.Vulkan.Data;

internal class LightData
{
    public Vector3 Position { get; set; }
    public Color Color { get; set; }
    public float Strength { get; set; }

    internal Vector3 ColorVector => new(Color.R / 255f, Color.G / 255f, Color.B / 255f);
}