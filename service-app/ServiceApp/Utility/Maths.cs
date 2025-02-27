using System.Windows.Media.Media3D;
using BindingsCs.Safe.Types;

namespace ServiceApp.Utility;

public class Maths
{
    public static RotateTransform3D RotateTransformFromEulerAngles(double pitch, double yaw, double roll)
    {
        var pitchDeg = pitch / Math.PI * 180.0;
        var yawDeg = yaw / Math.PI * 180.0;
        var rollDeg = roll / Math.PI * 180.0;

        var q = new Quaternion(
                    new Vector3D(1, 0, 0), pitchDeg) *
                new Quaternion(new Vector3D(0, 1, 0), yawDeg) *
                new Quaternion(new Vector3D(0, 0, 1), rollDeg);

        q.Normalize();

        var axisAngleRotation = new AxisAngleRotation3D(q.Axis, q.Angle);
        return new RotateTransform3D(axisAngleRotation);
    }

    public static SixAxis PathInterpolation(PathResult<SixAxis> path, double t)
    {
        var nodes = path.Nodes.ToList();
        if (nodes.Count == 0)
            return new SixAxis();

        if (nodes.Count == 1)
            return nodes[0];

        var clamped = Math.Clamp(t, 0, nodes.Count - 1);
        var index = (int)clamped;
        var fraction = clamped - index;

        if (index == nodes.Count - 1)
            return nodes[index];

        return NodeInterpolation(nodes[index], nodes[index + 1], fraction);
    }

    public static SixAxis NodeInterpolation(SixAxis start, SixAxis end, double t)
    {
        return new SixAxis(
            start.X + (end.X - start.X) * t,
            start.Y + (end.Y - start.Y) * t,
            start.Z + (end.Z - start.Z) * t,
            start.Rx + (end.Rx - start.Rx) * t,
            start.Ry + (end.Ry - start.Ry) * t,
            start.Rz + (end.Rz - start.Rz) * t
        );
    }

    public static LinearState PathInterpolation(PathResult<LinearState> path, double t)
    {
        var nodes = path.Nodes.ToList();
        if (nodes.Count == 0)
            return new LinearState();

        if (nodes.Count == 1)
            return nodes[0];

        var clamped = Math.Clamp(t, 0, nodes.Count - 1);
        var index = (int)clamped;
        var fraction = clamped - index;

        if (index == nodes.Count - 1)
            return nodes[index];

        return NodeInterpolation(nodes[index], nodes[index + 1], fraction);
    }

    public static LinearState NodeInterpolation(LinearState start, LinearState end, double t)
    {
        return new LinearState(
            start.T + (end.T - start.T) * t
        );
    }
}