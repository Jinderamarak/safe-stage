namespace BindingsCs.Safe.Types;

public struct Id : IEquatable<Id>
{
    internal readonly Unsafe.Id Inner;

    public Id(ulong id)
    {
        Inner = Unsafe.NativeMethods.id_new(id);
    }

    public override string? ToString()
    {
        return $"{Inner.Item1}";
    }

    public bool Equals(Id other)
    {
        return Inner.Item1.Equals(other.Inner.Item1);
    }

    public override bool Equals(object? obj)
    {
        return obj is Id other && Equals(other);
    }

    public override int GetHashCode()
    {
        return Inner.Item1.GetHashCode();
    }
}