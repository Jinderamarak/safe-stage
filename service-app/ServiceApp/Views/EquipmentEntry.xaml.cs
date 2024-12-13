using System.Windows;
using ServiceApp.Models;
using ServiceApp.Utility;

namespace ServiceApp.Views;

public partial class EquipmentEntry : ReactiveUserControl
{
    public ConfigVariant Equipment
    {
        get => _equipment!;
        init => SetField(ref _equipment, value);
    }

    private ConfigVariant? _equipment;

    private readonly Action<EquipmentEntry> _removeHandler;

    public EquipmentEntry(Action<EquipmentEntry> removeHandler)
    {
        _removeHandler = removeHandler;
        DataContext = this;
        InitializeComponent();
    }

    private void OnRemoveEntry(object sender, RoutedEventArgs e)
    {
        _removeHandler(this);
    }

    public override string ToString()
    {
        return $"{Equipment}";
    }
}