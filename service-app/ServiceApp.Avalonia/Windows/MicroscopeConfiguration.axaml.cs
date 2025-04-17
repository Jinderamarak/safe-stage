using System.Collections.Generic;
using System.Collections.ObjectModel;
using System.Linq;
using Avalonia;
using Avalonia.Controls;
using Avalonia.Interactivity;
using Avalonia.Markup.Xaml;
using BindingsCs.Safe.Configurations;
using BindingsCs.Safe.Types;
using MsBox.Avalonia;
using MsBox.Avalonia.Enums;
using ServiceApp.Avalonia.Config;
using ServiceApp.Avalonia.Config.Presets;
using ServiceApp.Avalonia.Views;
using MsgBoxIcon = MsBox.Avalonia.Enums.Icon;

namespace ServiceApp.Avalonia.Windows;

public partial class MicroscopeConfiguration : Window
{
    public ObservableCollection<Preset> Presets { get; }
    public ObservableCollection<ConfigVariant> ChamberVariants { get; }
    public ObservableCollection<ConfigVariant> StageVariants { get; }
    public ObservableCollection<ConfigVariant> StageResolverVariants { get; }
    public ObservableCollection<ConfigVariant> EquipmentVariants { get; }
    public ObservableCollection<ConfigVariant> RetractVariants { get; }

    public static readonly StyledProperty<ConfigVariant?> ChamberVariantProperty =
        AvaloniaProperty.Register<MicroscopeConfiguration, ConfigVariant?>(nameof(ChamberVariant));

    public static readonly StyledProperty<ConfigVariant?> StageVariantProperty =
        AvaloniaProperty.Register<MicroscopeConfiguration, ConfigVariant?>(nameof(StageVariant));

    public static readonly StyledProperty<ConfigVariant?> StageResolverVariantProperty =
        AvaloniaProperty.Register<MicroscopeConfiguration, ConfigVariant?>(nameof(StageResolverVariant));

    public ConfigVariant? ChamberVariant
    {
        get => GetValue(ChamberVariantProperty);
        set => SetValue(ChamberVariantProperty, value);
    }

    public ConfigVariant? StageVariant
    {
        get => GetValue(StageVariantProperty);
        set => SetValue(StageVariantProperty, value);
    }

    public ConfigVariant? StageResolverVariant
    {
        get => GetValue(StageResolverVariantProperty);
        set => SetValue(StageResolverVariantProperty, value);
    }

    public ObservableCollection<EquipmentEntry> Equipment { get; } = new();
    public ObservableCollection<RetractEntry> Retracts { get; } = new();

    private List<ConfigVariant> _retractResolversVariants;

    private ulong _idCounter = 0;

    private ComboBox EquipmentVariantsCombo { get; }
    private ComboBox RetractVariantsCombo { get; }
    private ComboBox PresetsCombo { get; }

    public MicroscopeConfiguration()
    {
        Presets = new ObservableCollection<Preset>(Preset.AllPresets());
        ChamberVariants =
            new ObservableCollection<ConfigVariant>(ConfigLoader.LoadConfigurations(typeof(ChamberConfig)));
        EquipmentVariants =
            new ObservableCollection<ConfigVariant>(ConfigLoader.LoadConfigurations(typeof(EquipmentConfig)));
        StageVariants = new ObservableCollection<ConfigVariant>(ConfigLoader.LoadConfigurations(typeof(StageConfig)));
        StageResolverVariants =
            new ObservableCollection<ConfigVariant>(ConfigLoader.LoadConfigurations(typeof(ResolverStageConfig)));
        RetractVariants =
            new ObservableCollection<ConfigVariant>(ConfigLoader.LoadConfigurations(typeof(RetractConfig)));

        _retractResolversVariants = ConfigLoader.LoadConfigurations(typeof(ResolverRetractConfig)).ToList();

        DataContext = this;
        AvaloniaXamlLoader.Load(this);

        EquipmentVariantsCombo = this.FindControl<ComboBox>("EquipmentVariantsComboAva");
        RetractVariantsCombo = this.FindControl<ComboBox>("RetractVariantsComboAva");
        PresetsCombo = this.FindControl<ComboBox>("PresetsComboAva");
    }

    private void OnEquipmentVariantSelected(object sender, SelectionChangedEventArgs e)
    {
        var config = (ConfigVariant)EquipmentVariantsCombo.SelectedItem;
        if (config == null) return;

        EquipmentVariantsCombo.SelectedItem = null;

        Equipment.Add(new EquipmentEntry(OnRemoveEquipmentEntry)
        {
            Equipment = config.Cloned()
        });
    }

    private void OnRemoveEquipmentEntry(EquipmentEntry entry)
    {
        Equipment.Remove(entry);
    }

    private void OnRetractVariantSelected(object sender, SelectionChangedEventArgs e)
    {
        var config = (ConfigVariant)RetractVariantsCombo.SelectedItem;
        if (config == null) return;

        RetractVariantsCombo.SelectedItem = null;

        var resolverVariants = _retractResolversVariants.Select(r => r.Cloned()).ToList();
        Retracts.Add(new RetractEntry(OnRemoveRetractEntry)
        {
            Id = new Id(_idCounter++),
            Retract = config.Cloned(),
            RetractResolversVariants = resolverVariants
        });
    }

    private void OnRemoveRetractEntry(RetractEntry entry)
    {
        Retracts.Remove(entry);
    }

    private async void OnSave(object? sender, RoutedEventArgs routedEventArgs)
    {
        var errorCases = new List<string>();
        if (ChamberVariant is null) errorCases.Add("Chamber configuration is not selected");
        if (StageVariant is null) errorCases.Add("Stage configuration is not selected");
        if (StageResolverVariant is null) errorCases.Add("Stage resolver configuration is not selected");
        foreach (var retractEntry in Retracts)
            if (retractEntry.Resolver is null)
                errorCases.Add($"Resolver for retract {retractEntry.Id} is not selected");

        if (errorCases.Count > 0)
        {
            await MessageBoxManager
                .GetMessageBoxStandard("Error", string.Join("\n", errorCases), ButtonEnum.Ok, MsgBoxIcon.Error)
                .ShowAsync();
            return;
        }

        Close(true);
    }

    private void OnCancel(object sender, RoutedEventArgs e)
    {
        Close(false);
    }

    private void OnPresetChanged(object? sender, SelectionChangedEventArgs selectionChangedEventArgs)
    {
        if (PresetsCombo.SelectedItem is not Preset preset) return;
        PresetsCombo.SelectedItem = null;

        ApplyPreset(preset);
    }

    public void ApplyPreset(Preset preset)
    {
        ChamberVariant = preset.ChamberVariant.Cloned();
        StageVariant = preset.StageVariant.Cloned();
        StageResolverVariant = preset.StageResolverVariant.Cloned();

        Equipment.Clear();
        foreach (var equipment in preset.Equipment)
            Equipment.Add(new EquipmentEntry(OnRemoveEquipmentEntry)
            {
                Equipment = equipment.Cloned()
            });

        Retracts.Clear();
        foreach (var (id, retract, resolver) in preset.Retracts)
        {
            var resolverVariants = _retractResolversVariants.Select(r => r.Cloned()).ToList();
            Retracts.Add(new RetractEntry(OnRemoveRetractEntry)
            {
                Id = id,
                Retract = retract.Cloned(),
                Resolver = resolver.Cloned(),
                RetractResolversVariants = resolverVariants
            });
        }
    }
}