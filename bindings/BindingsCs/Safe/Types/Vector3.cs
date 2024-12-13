namespace BindingsCs.Safe.Types;

public struct Vector3
{
    internal Unsafe.CVector3 Inner;

    public double X
    {
        get => Inner.x;
        set => Inner.x = value;
    }

    public double Y
    {
        get => Inner.y;
        set => Inner.y = value;
    }

    public double Z
    {
        get => Inner.z;
        set => Inner.z = value;
    }

    internal Vector3(Unsafe.CVector3 inner)
    {
        Inner = inner;
    }

    public Vector3(double x = 0, double y = 0, double z = 0)
    {
        Inner = new Unsafe.CVector3
        {
            x = x,
            y = y,
            z = z
        };
    }

    public override string? ToString()
    {
        return $"{{[ X: {X}, Y: {Y}, Z: {Z} ]}}";
    }
}