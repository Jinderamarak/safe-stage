using System.Collections.ObjectModel;
using System.Collections.Specialized;
using Avalonia;
using Avalonia.Controls;
using ServiceApp.View3D.Controls.Models;

namespace ServiceApp.View3D.Controls;

public class ModelGroup : Control
{
    public ObservableCollection<IGeometryModel> Models { get; } = new();

    public static readonly DirectProperty<ModelGroup, ObservableCollection<IGeometryModel>> ModelsProperty =
        AvaloniaProperty.RegisterDirect<ModelGroup, ObservableCollection<IGeometryModel>>(nameof(Models), o => o.Models);

    public ObservableCollection<ModelGroup> Groups { get; } = new();

    public static readonly DirectProperty<ModelGroup, ObservableCollection<ModelGroup>> GroupsProperty =
        AvaloniaProperty.RegisterDirect<ModelGroup, ObservableCollection<ModelGroup>>(nameof(Groups), o => o.Groups);

    public ModelGroup()
    {
        Models.CollectionChanged += ModelsCollectionChanged;
        Groups.CollectionChanged += GroupsCollectionChanged;
    }

    ~ModelGroup()
    {
        Models.CollectionChanged -= ModelsCollectionChanged;
        Groups.CollectionChanged -= GroupsCollectionChanged;
    }

    private void ModelsCollectionChanged(object? sender, NotifyCollectionChangedEventArgs e)
    {
        RaisePropertyChanged(ModelsProperty, Models, Models);
        if (e.NewItems is not null)
            foreach (IGeometryModel item in e.NewItems)
            {
                if (item is GeometryModel model)
                    model.PropertyChanged += GeometryModelPropertyChanged;
                if (item is IndexedGeometryModel indexed)
                    indexed.PropertyChanged += GeometryModelPropertyChanged;
            }

        if (e.OldItems is not null)
            foreach (IGeometryModel item in e.OldItems)
            {
                if (item is GeometryModel model)
                    model.PropertyChanged -= GeometryModelPropertyChanged;
                if (item is IndexedGeometryModel indexed)
                    indexed.PropertyChanged -= GeometryModelPropertyChanged;
            }
    }

    private void GroupsCollectionChanged(object? sender, NotifyCollectionChangedEventArgs e)
    {
        RaisePropertyChanged(GroupsProperty, Groups, Groups);
        if (e.NewItems is not null)
            foreach (ModelGroup item in e.NewItems)
                item.PropertyChanged += GeometryModelPropertyChanged;

        if (e.OldItems is not null)
            foreach (ModelGroup item in e.OldItems)
                item.PropertyChanged -= GeometryModelPropertyChanged;
    }

    private void GeometryModelPropertyChanged(object? sender, AvaloniaPropertyChangedEventArgs e)
    {
        RaisePropertyChanged(ModelsProperty, Models, Models);
    }
}