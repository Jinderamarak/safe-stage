namespace BindingsCs.Safe.Types;

public struct Id
{
    internal readonly Unsafe.Id Inner;
    
    public Id(ulong id)
    {
        Inner = Unsafe.NativeMethods.id_new(id);
    }

    public override string? ToString()
        => $"{Inner.Item1}";
}