namespace BindingsCs.Safe.Types;

public struct LinearState
{
    internal Unsafe.CLinearState Inner;

    public double T
    {
        get => Inner.t;
        set
        {
            if (value < 0 || value > 1)
            {
                throw new ArgumentOutOfRangeException(nameof(value), "Value must be between 0 and 1.");
            }
            Inner.t = value;
        }
    }

    internal LinearState(Unsafe.CLinearState inner)
    {
        Inner = inner;
    }
    
    public LinearState(double t = 0)
    {
        if (t < 0 || t > 1)
        {
            throw new ArgumentOutOfRangeException(nameof(t), "Value must be between 0 and 1.");
        }
        Inner = new Unsafe.CLinearState
        {
            t = t
        };
    }

    public static LinearState Full() => new LinearState(1.0);
    public static LinearState None() => new LinearState(0.0);

    public override string? ToString()
        => $"{{[ T: {T} ]}}";
}