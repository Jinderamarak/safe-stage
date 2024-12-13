using BindingsCs.Safe.Types;

namespace ServiceApp.Models.Presets;

public class Preset
{
    public required string Name { get; init; }
    public required ConfigVariant ChamberVariant { get; init; }
    public required ConfigVariant StageVariant { get; init; }
    public required ConfigVariant StageResolverVariant { get; init; }
    public required IReadOnlyList<ConfigVariant> Equipment { get; init; }
    public required IReadOnlyList<(Id, ConfigVariant, ConfigVariant)> Retracts { get; init; }

    public override string ToString()
    {
        return Name;
    }

    public static List<Preset> AllPresets()
    {
        return new List<Preset>
        {
            ThesisPreset.CreateThesisPreset()
        };
    }
}