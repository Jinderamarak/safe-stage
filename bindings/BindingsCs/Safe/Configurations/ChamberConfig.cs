namespace BindingsCs.Safe.Configurations;

public class ChamberConfig
{
    internal readonly Unsafe.ChamberConfig InnerConfig;

    private ChamberConfig(Unsafe.ChamberConfig innerConfig)
    {
        InnerConfig = innerConfig;
    }

    /// <inheritdoc cref="Unsafe.NativeMethods.thesis_chamber"/>
    public static ChamberConfig ThesisChamber()
    {
        return new ChamberConfig(Unsafe.NativeMethods.thesis_chamber());
    }
}