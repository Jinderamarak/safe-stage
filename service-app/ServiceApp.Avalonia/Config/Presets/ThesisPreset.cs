using System;
using System.Collections.Generic;
using System.Linq;
using BindingsCs.Safe.Configurations;
using BindingsCs.Safe.Types;

namespace ServiceApp.Avalonia.Config.Presets;

public static class ThesisPreset
{
    public static Preset CreateThesisPreset()
    {
        var chamber = ConfigVariant.FromMethodName(typeof(ChamberConfig), nameof(ChamberConfig.ThesisChamber));
        var stage = ConfigVariant.FromMethodName(typeof(StageConfig), nameof(StageConfig.ThesisStage));
        var stageResolver =
            ConfigVariant.FromMethodName(typeof(ResolverStageConfig), nameof(ResolverStageConfig.DownRotateFind));
        var equipmentA =
            ConfigVariant.FromMethodName(typeof(EquipmentConfig), nameof(EquipmentConfig.ThesisDetectorAlpha));
        var equipmentB =
            ConfigVariant.FromMethodName(typeof(EquipmentConfig), nameof(EquipmentConfig.ThesisDetectorBeta));
        var retractId = new Id(1);
        var retract = ConfigVariant.FromMethodName(typeof(RetractConfig), nameof(RetractConfig.ThesisRetract));
        var retractResolver =
            ConfigVariant.FromMethodName(typeof(ResolverRetractConfig), nameof(ResolverRetractConfig.Linear));

        var deg = Math.PI / 180;
        var fields = new object[]
        {
            new Vector3(0.0),
            new SixAxis(1e-3, 1e-3, 1e-3, deg, deg, deg),
            new Vector3(1.0, 1.0, 1.0),
            new Vector3(-135e-3, -125e-3, -125e-3),
            new Vector3(125e-3, 125e-3, 125e-3),
            new Vector3(10e-3, 10e-3, 10e-3),
            new Vector3(6e-3, 6e-3, 6e-3),
            new Vector3(1e-3, 1e-3, 1e-3),
            new SixAxis(1e-3, 1e-3, 1e-3, deg, deg, deg)
        };
        foreach (var (field, value) in stageResolver.Fields.Zip(fields)) field.SetValue(value);

        retractResolver.Fields[0].SetValue(new LinearState(1e-3));

        return new Preset()
        {
            Name = "Thesis Preset",
            ChamberVariant = chamber,
            StageVariant = stage,
            StageResolverVariant = stageResolver,
            Equipment = new List<ConfigVariant> { equipmentA, equipmentB },
            Retracts = new List<(Id, ConfigVariant, ConfigVariant)> { (retractId, retract, retractResolver) }
        };
    }
}