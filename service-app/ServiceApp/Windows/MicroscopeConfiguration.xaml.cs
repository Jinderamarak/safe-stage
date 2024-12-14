using System.Collections.ObjectModel;
using System.Windows;
using System.Windows.Controls;
using BindingsCs.Safe.Configurations;
using BindingsCs.Safe.Types;
using ServiceApp.Config;
using ServiceApp.Config.Presets;
using ServiceApp.Utility;
using ServiceApp.Views;

namespace ServiceApp.Windows;

public partial class MicroscopeConfiguration : ReactiveWindow
{
    public ObservableCollection<Preset> Presets { get; }
    public ObservableCollection<ConfigVariant> ChamberVariants { get; }
    public ObservableCollection<ConfigVariant> StageVariants { get; }
    public ObservableCollection<ConfigVariant> StageResolverVariants { get; }
    public ObservableCollection<ConfigVariant> EquipmentVariants { get; }
    public ObservableCollection<ConfigVariant> RetractVariants { get; }

    public ConfigVariant? ChamberVariant
    {
        get => _chamberVariant;
        set => SetField(ref _chamberVariant, value);
    }

    public ConfigVariant? StageVariant
    {
        get => _stageVariant;
        set => SetField(ref _stageVariant, value);
    }

    public ConfigVariant? StageResolverVariant
    {
        get => _stageResolverVariant;
        set => SetField(ref _stageResolverVariant, value);
    }

    public ObservableCollection<EquipmentEntry> Equipment { get; } = new();
    public ObservableCollection<RetractEntry> Retracts { get; } = new();

    private ConfigVariant? _chamberVariant;
    private ConfigVariant? _stageVariant;
    private ConfigVariant? _stageResolverVariant;

    private List<ConfigVariant> _retractResolversVariants;

    private ulong _idCounter = 0;

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
        InitializeComponent();
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

    private void OnSave(object sender, RoutedEventArgs e)
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
            MessageBox.Show(string.Join("\n", errorCases), "Error", MessageBoxButton.OK, MessageBoxImage.Error);
            return;
        }

        DialogResult = true;
        Close();
    }

    private void OnCancel(object sender, RoutedEventArgs e)
    {
        DialogResult = false;
        Close();
    }

    private void OnPresetChanged(object sender, SelectionChangedEventArgs e)
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