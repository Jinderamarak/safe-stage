using BindingsCs.Safe.Types;

namespace BindingsCs.Safe.Configurations;

public class RetractConfig
{
    internal readonly Unsafe.RetractConfig InnerConfig;

    private RetractConfig(Unsafe.RetractConfig innerConfig)
    {
        InnerConfig = innerConfig;
    }

    /// <inheritdoc cref="Unsafe.NativeMethods.thesis_retract"/>
    public static RetractConfig ThesisRetract()
    {
        return new RetractConfig(Unsafe.NativeMethods.thesis_retract());
    }
}