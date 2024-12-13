namespace BindingsCs.Safe.Configurations;

public class HolderConfig
{
    internal readonly Unsafe.HolderConfig InnerConfig;

    private HolderConfig(Unsafe.HolderConfig innerConfig)
    {
        InnerConfig = innerConfig;
    }

    public static HolderConfig ThesisHolderCircle()
    {
        return new HolderConfig(Unsafe.NativeMethods.thesis_holder_circle());
    }

    public static HolderConfig ThesisHolderSquare()
    {
        return new HolderConfig(Unsafe.NativeMethods.thesis_holder_square());
    }
}