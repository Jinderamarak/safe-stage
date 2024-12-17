namespace BindingsCs.Safe.Configurations;

public class HolderConfig
{
    internal readonly Unsafe.HolderConfig InnerConfig;

    private HolderConfig(Unsafe.HolderConfig innerConfig)
    {
        InnerConfig = innerConfig;
    }

    /// <inheritdoc cref="Unsafe.NativeMethods.thesis_holder_circle"/>
    public static HolderConfig ThesisHolderCircle()
    {
        return new HolderConfig(Unsafe.NativeMethods.thesis_holder_circle());
    }

    /// <inheritdoc cref="Unsafe.NativeMethods.thesis_holder_square"/>
    public static HolderConfig ThesisHolderSquare()
    {
        return new HolderConfig(Unsafe.NativeMethods.thesis_holder_square());
    }
}