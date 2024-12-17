using System.Windows;
using BindingsCs.Safe.Types;
using ServiceApp.Config;
using ServiceApp.Utility;

namespace ServiceApp.Views;

public partial class RetractEntry : ReactiveUserControl
{
    public Id Id
    {
        get => _id;
        init => SetField(ref _id, value);
    }

    public ConfigVariant Retract
    {
        get => _retract!;
        init => SetField(ref _retract, value);
    }

    public ConfigVariant? Resolver
    {
        get => _resolver;
        set => SetField(ref _resolver, value);
    }

    public IReadOnlyList<ConfigVariant> RetractResolversVariants
    {
        get => _retractResolversVariants!;
        init => SetField(ref _retractResolversVariants, value);
    }

    private Id _id;
    private ConfigVariant? _retract;
    private ConfigVariant? _resolver;
    private IReadOnlyList<ConfigVariant>? _retractResolversVariants;

    private readonly Action<RetractEntry> _removeHandler;

    public RetractEntry(Action<RetractEntry> removeHandler)
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
        return $"{Retract}";
    }
}