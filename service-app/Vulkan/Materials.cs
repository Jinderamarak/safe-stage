using System;
using System.Collections.Generic;
using Avalonia.Media;
using Color = System.Drawing.Color;

namespace ServiceApp.Avalonia.Utility;

public static class Materials
{
    public static Color PathNode = Rgb(0xc026d3); // Fuchsia - 600
    public static Color PathEdge = Rgb(0xdb2777); // Pink - 600

    public static Color StaticStart = Rgb(0xAABBAA);
    public static Color StaticEnd = Rgb(0xAAFFAA);
    public static int StaticCount = 0;

    public static Color StageStart = Rgb(0xFF0000);
    public static Color StageEnd = Rgb(0x0000FF);
    public static int StageCount = 5;

    public static (Color, Color)[] RetractsRange = new[]
    {
        (Rgb(0xf97316), Rgb(0x7c2d12)), // Orange - 500, Orange - 900
        (Rgb(0xeab308), Rgb(0x713f12)), // Yellow - 500, Yellow - 900
        (Rgb(0x84cc16), Rgb(0x365314)), // Lime - 500, Lime - 900
        (Rgb(0x10b981), Rgb(0x064e3b)), // Emerald - 500, Emerald - 900
        (Rgb(0x06b6d4), Rgb(0x164e63)), // Cyan - 500, Cyan - 900
        (Rgb(0x0ea5e9), Rgb(0x0c4a6e)) // Sky - 500, Sky - 900
    };

    public static int RetractCount = 3;

    public static IEnumerable<Color> StaticRange(int count)
    {
        return RgbRange(StaticStart, StaticEnd, count, StaticCount);
    }

    public static IEnumerable<Color> StageRange(int count)
    {
        return RgbRange(StageStart, StageEnd, count, StageCount);
    }

    public static IEnumerable<Color> RetractRange(int index, int count)
    {
        var (from, to) = RetractsRange[index % RetractsRange.Length];
        return RgbRange(from, to, count, RetractCount);
    }

    private static Color Rgb(uint color)
    {
        var r = (byte)((color >> 16) & 0xFF);
        var g = (byte)((color >> 8) & 0xFF);
        var b = (byte)(color & 0xFF);
        return Color.FromArgb(r, g, b);
    }

    private static Color Interpolate(Color first, Color second, double t)
    {
        var r = (byte)(first.R + (second.R - first.R) * t);
        var g = (byte)(first.G + (second.G - first.G) * t);
        var b = (byte)(first.B + (second.B - first.B) * t);
        return Color.FromArgb(r, g, b);
    }

    private static IEnumerable<Color> RgbRange(Color from, Color to, int count, int expected)
    {
        var actual = Math.Max(count, expected);
        for (var i = 0; i < actual; i++)
        {
            var t = (double)i / (actual - 1);
            yield return Interpolate(from, to, t);
        }
    }
}