using BindingsCs.Safe.Types;

namespace ServiceApp.Utility.Extensions;

public static class SixAxisExtensions
{
    public static string ToFormattedString(this SixAxis state)
    {
        var xMm = state.X * 1e3;
        var yMm = state.Y * 1e3;
        var zMm = state.Z * 1e3;
        var rxDeg = state.Rx * 180 / Math.PI;
        var ryDeg = state.Ry * 180 / Math.PI;
        var rzDeg = state.Rz * 180 / Math.PI;
        return $"X: {xMm:F2} mm, Y: {yMm:F2} mm, Z: {zMm:F2} mm, Rx: {rxDeg:F2}°, Ry: {ryDeg:F2}°, Rz: {rzDeg:F2}°";
    }
}