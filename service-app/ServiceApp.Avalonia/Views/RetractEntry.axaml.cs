using System;
using System.Collections.Generic;
using Avalonia;
using Avalonia.Controls;
using Avalonia.Interactivity;
using Avalonia.Markup.Xaml;
using BindingsCs.Safe.Types;
using ServiceApp.Avalonia.Config;

namespace ServiceApp.Avalonia.Views;

public partial class RetractEntry : UserControl
{
    public static readonly StyledProperty<Id> IdProperty =
        AvaloniaProperty.Register<RetractEntry, Id>(nameof(Id));

    public static readonly StyledProperty<ConfigVariant> RetractProperty =
        AvaloniaProperty.Register<RetractEntry, ConfigVariant>(nameof(Retract));

    public static readonly StyledProperty<ConfigVariant?> ResolverProperty =
        AvaloniaProperty.Register<RetractEntry, ConfigVariant?>(nameof(Resolver));

    public static readonly StyledProperty<IReadOnlyList<ConfigVariant>> RetractResolversVariantsProperty =
        AvaloniaProperty.Register<RetractEntry, IReadOnlyList<ConfigVariant>>(nameof(RetractResolversVariants));

    public Id Id
    {
        get => GetValue(IdProperty);
        init => SetValue(IdProperty, value);
    }

    public ConfigVariant Retract
    {
        get => GetValue(RetractProperty);
        init => SetValue(RetractProperty, value);
    }

    public ConfigVariant? Resolver
    {
        get => GetValue(ResolverProperty);
        set => SetValue(ResolverProperty, value);
    }

    public IReadOnlyList<ConfigVariant> RetractResolversVariants
    {
        get => GetValue(RetractResolversVariantsProperty);
        init => SetValue(RetractResolversVariantsProperty, value);
    }

    private readonly Action<RetractEntry> _removeHandler;

    public RetractEntry(Action<RetractEntry> removeHandler)
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
        return $"{Retract}";
    }
}