namespace BindingsCs.Safe.Configurations;

public class EquipmentConfig
{
    internal readonly Unsafe.EquipmentConfig InnerConfig;

    private EquipmentConfig(Unsafe.EquipmentConfig innerConfig)
    {
        InnerConfig = innerConfig;
    }

    /// <inheritdoc cref="Unsafe.NativeMethods.thesis_detector_alpha"/>
    public static EquipmentConfig ThesisDetectorAlpha()
    {
        return new EquipmentConfig(Unsafe.NativeMethods.thesis_detector_alpha());
    }

    /// <inheritdoc cref="Unsafe.NativeMethods.thesis_detector_beta"/>
    public static EquipmentConfig ThesisDetectorBeta()
    {
        return new EquipmentConfig(Unsafe.NativeMethods.thesis_detector_beta());
    }
}