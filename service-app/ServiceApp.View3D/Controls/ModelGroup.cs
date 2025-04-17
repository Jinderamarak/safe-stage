using System.Collections.ObjectModel;
using System.Collections.Specialized;
using System.ComponentModel;
using Avalonia;
using Avalonia.Controls;

namespace ServiceApp.View3D.Controls;

public class ModelGroup : Control
{
    public ObservableCollection<GeometryModel> Models { get; } = new();

    public static readonly DirectProperty<ModelGroup, ObservableCollection<GeometryModel>> ModelsProperty =
        AvaloniaProperty.RegisterDirect<ModelGroup, ObservableCollection<GeometryModel>>(nameof(Models), o => o.Models);

    public ModelGroup()
    {
        Models.CollectionChanged += ModelsCollectionChanged;
    }

    ~ModelGroup()
    {
        Models.CollectionChanged -= ModelsCollectionChanged;
    }

    private void ModelsCollectionChanged(object? sender, NotifyCollectionChangedEventArgs e)
    {
        RaisePropertyChanged(ModelsProperty, Models, Models);
        if (e.NewItems is not null)
            foreach (GeometryModel item in e.NewItems)
                item.PropertyChanged += GeometryModelPropertyChanged;

        if (e.OldItems is not null)
            foreach (GeometryModel item in e.OldItems)
                item.PropertyChanged -= GeometryModelPropertyChanged;
    }

    private void GeometryModelPropertyChanged(object? sender, AvaloniaPropertyChangedEventArgs e)
    {
        RaisePropertyChanged(ModelsProperty, Models, Models);
    }
}