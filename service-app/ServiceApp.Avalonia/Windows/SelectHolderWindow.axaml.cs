using System.Collections.ObjectModel;
using Avalonia;
using Avalonia.Controls;
using Avalonia.Interactivity;
using Avalonia.Markup.Xaml;
using BindingsCs.Safe.Configurations;
using ServiceApp.Avalonia.Config;

namespace ServiceApp.Avalonia.Windows;

public partial class SelectHolderWindow : Window
{
    public ObservableCollection<ConfigVariant> HolderVariants { get; }

    public static readonly StyledProperty<ConfigVariant?> SelectedHolderProperty =
        AvaloniaProperty.Register<SelectHolderWindow, ConfigVariant?>(nameof(SelectedHolder));

    public ConfigVariant? SelectedHolder
    {
        get => GetValue(SelectedHolderProperty);
        set => SetValue(SelectedHolderProperty, value);
    }

    public SelectHolderWindow()
    {
        HolderVariants =
            new ObservableCollection<ConfigVariant>(ConfigLoader.LoadConfigurations(typeof(HolderConfig)));

        DataContext = this;
        AvaloniaXamlLoader.Load(this);
    }

    private void OnSave(object? sender, RoutedEventArgs routedEventArgs)
    {
        Close(true);
    }

    private void OnNoHolder(object sender, RoutedEventArgs e)
    {
        SelectedHolder = null;
        Close(true);
    }

    private void OnCancel(object sender, RoutedEventArgs e)
    {
        Close(true);
    }
}