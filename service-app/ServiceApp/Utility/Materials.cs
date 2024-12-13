using System.Windows.Media;
using System.Windows.Media.Media3D;

namespace ServiceApp.Utility;

public static class Materials
{
    // Red - 500
    public static Material Stage = Diffuse(0xdc2626);

    // Red - 50
    public static Material StageBack = Diffuse(0xfef2f2);

    // Slate - 400
    public static Material Static = Diffuse(0x94a3b8);

    // Fuchsia - 600
    public static Material PathNode = Diffuse(0xc026d3);

    // Pink - 600
    public static Material PathEdge = Diffuse(0xdb2777);

    public static Material[] Retracts = new[]
    {
        // Orange - 500
        Diffuse(0xf97316),
        // Yellow - 500
        Diffuse(0xeab308),
        // Lime - 500
        Diffuse(0x84cc16),
        // Emerald - 500
        Diffuse(0x10b981),
        // Cyan - 500
        Diffuse(0x06b6d4),
        // Sky - 500
        Diffuse(0x0ea5e9)
    };

    private static Material Diffuse(uint color)
    {
        var r = (byte)((color >> 16) & 0xFF);
        var g = (byte)((color >> 8) & 0xFF);
        var b = (byte)(color & 0xFF);
        return new DiffuseMaterial(new SolidColorBrush(Color.FromRgb(r, g, b)));
    }
}