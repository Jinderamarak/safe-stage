using System;
using Avalonia;
using Avalonia.Controls;
using Avalonia.Interactivity;
using Avalonia.Markup.Xaml;
using ServiceApp.Avalonia.Config;

namespace ServiceApp.Avalonia.Views;

public partial class EquipmentEntry : UserControl
{
    public static readonly StyledProperty<ConfigVariant> EquipmentProperty =
        AvaloniaProperty.Register<EquipmentEntry, ConfigVariant>(nameof(Equipment));

    public ConfigVariant Equipment
    {
        get => GetValue(EquipmentProperty);
        init => SetValue(EquipmentProperty, value);
    }

    private readonly Action<EquipmentEntry> _removeHandler;

    public EquipmentEntry(Action<EquipmentEntry> removeHandler)
    {
        _removeHandler = removeHandler;
        DataContext = this;
        AvaloniaXamlLoader.Load(this);
    }

    private void OnRemoveEntry(object? sender, RoutedEventArgs routedEventArgs)
    {
        _removeHandler(this);
    }

    public override string ToString()
    {
        return $"{Equipment}";
    }
}