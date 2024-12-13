using System.Collections.ObjectModel;
using System.Windows;
using BindingsCs.Safe.Configurations;
using ServiceApp.Models;
using ServiceApp.Utility;

namespace ServiceApp.Windows;

public partial class SelectHolderWindow : ReactiveWindow
{
    public ObservableCollection<ConfigVariant> HolderVariants { get; }

    public ConfigVariant? SelectedHolder
    {
        get => _selectedHolder;
        set => SetField(ref _selectedHolder, value);
    }

    private ConfigVariant? _selectedHolder;

    public SelectHolderWindow()
    {
        HolderVariants =
            new ObservableCollection<ConfigVariant>(ConfigLoader.LoadConfigurations(typeof(HolderConfig)));

        DataContext = this;
        InitializeComponent();
    }

    private void OnSave(object sender, RoutedEventArgs e)
    {
        DialogResult = true;
        Close();
    }

    private void OnNoHolder(object sender, RoutedEventArgs e)
    {
        SelectedHolder = null;
        DialogResult = true;
        Close();
    }

    private void OnCancel(object sender, RoutedEventArgs e)
    {
        DialogResult = false;
        Close();
    }
}