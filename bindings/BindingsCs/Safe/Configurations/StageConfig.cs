namespace BindingsCs.Safe.Configurations;

public class StageConfig
{
    internal readonly Unsafe.StageConfig InnerConfig;

    private StageConfig(Unsafe.StageConfig innerConfig)
    {
        InnerConfig = innerConfig;
    }

    /// <inheritdoc cref="Unsafe.NativeMethods.thesis_stage"/>
    public static StageConfig ThesisStage()
    {
        return new StageConfig(Unsafe.NativeMethods.thesis_stage());
    }
}