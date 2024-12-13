using BindingsCs.Safe.Types;

namespace BindingsCs.Safe.Configurations;

public class ResolverRetractConfig
{
    internal readonly Unsafe.ResolverRetractConfig InnerConfig;

    private ResolverRetractConfig(Unsafe.ResolverRetractConfig innerConfig)
    {
        InnerConfig = innerConfig;
    }

    public static ResolverRetractConfig Linear(LinearState stepSize)
    {
        return new ResolverRetractConfig(Unsafe.NativeMethods.retract_linear_resolver(stepSize.Inner));
    }
}