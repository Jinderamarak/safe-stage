using BindingsCs.Safe.Types;

namespace BindingsCs.Safe.Configurations;

public class ResolverStageConfig
{
    internal readonly Unsafe.ResolverStageConfig InnerConfig;

    private ResolverStageConfig(Unsafe.ResolverStageConfig innerConfig)
    {
        InnerConfig = innerConfig;
    }

    /// <inheritdoc cref="Unsafe.NativeMethods.stage_linear_resolver"/>
    public static ResolverStageConfig Linear(SixAxis step)
    {
        return new ResolverStageConfig(Unsafe.NativeMethods.stage_linear_resolver(step.Inner));
    }

    /// <inheritdoc cref="Unsafe.NativeMethods.down_rotate_find_resolver"/>
    public static ResolverStageConfig DownRotateFind(Vector3 downPoint, SixAxis downStep, Vector3 moveSpeed,
        Vector3 sampleMin, Vector3 sampleMax, Vector3 sampleStep, Vector3 sampleEpsilon, Vector3 losStep,
        SixAxis smoothingStep)
    {
        return new ResolverStageConfig(Unsafe.NativeMethods.down_rotate_find_resolver(downPoint.Inner, downStep.Inner,
            moveSpeed.Inner, sampleMin.Inner, sampleMax.Inner, sampleStep.Inner, sampleEpsilon.Inner, losStep.Inner,
            smoothingStep.Inner));
    }
}