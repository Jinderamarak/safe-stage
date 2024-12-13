namespace BindingsCs.Safe.Types;

public struct SixAxis
{
    internal Unsafe.CSixAxis Inner;

    public double X
    {
        get => Inner.x;
        set
        {
            if (double.IsNaN(value)) throw new ArgumentOutOfRangeException("Value must be a number.");
            Inner.x = value;
        }
    }

    public double Y
    {
        get => Inner.y;
        set
        {
            if (double.IsNaN(value)) throw new ArgumentOutOfRangeException("Value must be a number.");
            Inner.y = value;
        }
    }

    public double Z
    {
        get => Inner.z;
        set
        {
            if (double.IsNaN(value)) throw new ArgumentOutOfRangeException("Value must be a number.");
            Inner.z = value;
        }
    }

    public double Rx
    {
        get => Inner.rx;
        set
        {
            if (double.IsNaN(value)) throw new ArgumentOutOfRangeException("Value must be a number.");
            Inner.rx = value;
        }
    }

    public double Ry
    {
        get => Inner.ry;
        set
        {
            if (double.IsNaN(value)) throw new ArgumentOutOfRangeException("Value must be a number.");
            Inner.ry = value;
        }
    }

    public double Rz
    {
        get => Inner.rz;
        set
        {
            if (double.IsNaN(value)) throw new ArgumentOutOfRangeException("Value must be a number.");
            Inner.rz = value;
        }
    }

    public double T
    {
        get => Ry;
        set => Ry = value;
    }

    public double R
    {
        get => Rz;
        set => Rz = value;
    }

    internal SixAxis(Unsafe.CSixAxis inner)
    {
        Inner = inner;
    }

    public SixAxis(double x = 0, double y = 0, double z = 0, double rx = 0, double ry = 0, double rz = 0)
    {
        if (double.IsNaN(x) || double.IsNaN(y) || double.IsNaN(z) || double.IsNaN(rx) || double.IsNaN(ry) ||
            double.IsNaN(rz)) throw new ArgumentOutOfRangeException("Value must be a number.");

        Inner = new Unsafe.CSixAxis
        {
            x = x,
            y = y,
            z = z,
            rx = rx,
            ry = ry,
            rz = rz
        };
    }

    public override string? ToString()
    {
        return $"{{[ X: {X}, Y: {Y}, Z: {Z}, Rx: {Rx}, Ry: {Ry}, Rz: {Rz} ]}}";
    }
}