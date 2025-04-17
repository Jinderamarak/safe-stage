using System.Collections.Generic;
using BindingsCs.Safe;
using BindingsCs.Safe.Configurations;
using ServiceApp.Avalonia.Views;

namespace ServiceApp.Avalonia.Config;

public static class ConfigBuilder
{
    public static Microscope BuildMicroscope(ConfigVariant chamberVariant, ConfigVariant stageVariant,
        ConfigVariant stageResolverVariant, IReadOnlyList<EquipmentEntry> equipment,
        IReadOnlyList<RetractEntry> retracts)
    {
        using var builder = new ConfigurationBuilder();

        var chamber = chamberVariant.Construct<ChamberConfig>();
        builder.WithChamber(chamber!);

        var stage = stageVariant.Construct<StageConfig>();
        var stageResolver = stageResolverVariant.Construct<ResolverStageConfig>();
        builder.WithStage(stage!, stageResolver!);

        foreach (var entry in equipment)
        {
            var equipmentConfig = entry.Equipment.Construct<EquipmentConfig>();
            builder.WithEquipment(equipmentConfig!);
        }

        foreach (var entry in retracts)
        {
            var id = entry.Id;
            var retract = entry.Retract.Construct<RetractConfig>();
            var retractResolver = entry.Resolver!.Construct<ResolverRetractConfig>();
            builder.WithRetract(id, retract!, retractResolver!);
        }

        var configuration = builder.Build();
        return Microscope.FromConfiguration(configuration);
    }
}