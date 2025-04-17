using System.Collections.ObjectModel;
using System.Collections.Specialized;
using Avalonia;
using Avalonia.Controls;
using Avalonia.LogicalTree;
using Avalonia.Rendering.Composition;
using Avalonia.VisualTree;
using ServiceApp.Vulkan.Data;
using ServiceApp.Vulkan.Render;

namespace ServiceApp.Vulkan.Controls;

public class SimpleView3D : Control
{
    private Compositor? _compositor;
    private CompositionDrawingSurface? _drawing;
    private bool _initialized;
    private VulkanResources? _resources;
    private bool _updateQueued;
    private CompositionSurfaceVisual? _visual;

    protected override void OnAttachedToVisualTree(VisualTreeAttachmentEventArgs e)
    {
        base.OnAttachedToVisualTree(e);
        _ = Initialize();
    }

    protected override void OnDetachedFromLogicalTree(LogicalTreeAttachmentEventArgs e)
    {
        if (_initialized)
        {
            _drawing?.Dispose();
            FreeGraphicsResources();
        }

        _initialized = false;
        base.OnDetachedFromLogicalTree(e);
    }

    private async Task Initialize()
    {
        var selfVisual = ElementComposition.GetElementVisual(this)!;
        try
        {
            _compositor = selfVisual.Compositor;
            _drawing = _compositor.CreateDrawingSurface();
            _visual = _compositor.CreateSurfaceVisual();
            _visual.Size = new Vector(Bounds.Width, Bounds.Height);
            _visual.Surface = _drawing;
            ElementComposition.SetElementChildVisual(this, _visual);

            await DoInitialize(_compositor, _drawing);
            _initialized = true;
            QueueNextFrame();
        }
        catch (Exception e)
        {
            //  TODO: Handle exception
        }
    }

    public void UpdateFrame()
    {
        _updateQueued = false;
        var root = this.GetVisualRoot();
        if (root == null)
            return;

        _visual!.Size = new Vector(Bounds.Width, Bounds.Height);
        var size = PixelSize.FromSize(Bounds.Size, root.RenderScaling);
        RenderFrame(size);
    }

    private void QueueNextFrame()
    {
        if (_initialized && !_updateQueued && _compositor != null)
        {
            _updateQueued = true;
            _compositor?.RequestCompositionUpdate(UpdateFrame);
        }
    }

    private async Task DoInitialize(Compositor compositor,
        CompositionDrawingSurface compositionDrawingSurface)
    {
        var interop = await compositor.TryGetCompositionGpuInterop();
        if (interop == null)
            throw new Exception("Compositor doesn't support interop for the current backend");
        InitializeGraphicsResources(compositor, compositionDrawingSurface, interop);
    }

    private void InitializeGraphicsResources(Compositor compositor,
        CompositionDrawingSurface compositionDrawingSurface, ICompositionGpuInterop gpuInterop)
    {
        var context = VulkanContext.Create(gpuInterop);
        var content = new VulkanContent(context);
        _resources = new VulkanResources(context, new VulkanSwapchain(context, gpuInterop, compositionDrawingSurface),
            content);
    }

    private void FreeGraphicsResources()
    {
        _resources?.DisposeAsync();
        _resources = null;
    }

    private Camera3D? _camera;

    public static readonly DirectProperty<SimpleView3D, Camera3D?> CameraProperty =
        AvaloniaProperty.RegisterDirect<SimpleView3D, Camera3D?>(nameof(Camera), o => o.Camera, (o, v) => o.Camera = v);

    public Camera3D? Camera
    {
        get => _camera;
        set => SetAndRaise(CameraProperty, ref _camera, value);
    }

    private PointLight? _light;

    public static readonly DirectProperty<SimpleView3D, PointLight?> LightProperty =
        AvaloniaProperty.RegisterDirect<SimpleView3D, PointLight?>(nameof(Light), o => o.Light, (o, v) => o.Light = v);

    public PointLight? Light
    {
        get => _light;
        set => SetAndRaise(LightProperty, ref _light, value);
    }

    private ObservableCollection<ModelGroup> _modelGroups = new();

    public static readonly DirectProperty<SimpleView3D, ObservableCollection<ModelGroup>> ModelGroupsProperty =
        AvaloniaProperty.RegisterDirect<SimpleView3D, ObservableCollection<ModelGroup>>(nameof(ModelGroups),
            o => o.ModelGroups, (o, v) => o.ModelGroups = v);

    public ObservableCollection<ModelGroup> ModelGroups
    {
        get => _modelGroups;
        set => SetAndRaise(ModelGroupsProperty, ref _modelGroups, value);
    }

    protected override void OnPropertyChanged(AvaloniaPropertyChangedEventArgs change)
    {
        if (change.Property == ModelGroupsProperty)
        {
            if (change.OldValue is ObservableCollection<ModelGroup> oldModelGroups)
                oldModelGroups.CollectionChanged -= ModelGroupsCollectionChanged;
            if (change.NewValue is ObservableCollection<ModelGroup> newModelGroups)
                newModelGroups.CollectionChanged += ModelGroupsCollectionChanged;
        }

        if (change.Property == BoundsProperty
            || change.Property == CameraProperty
            || change.Property == LightProperty
            || change.Property == ModelGroupsProperty)
            QueueNextFrame();
        base.OnPropertyChanged(change);
    }

    private void RenderFrame(PixelSize pixelSize)
    {
        if (_resources == null)
            return;

        var camera = Camera?.GetCameraData() ?? new CameraData();
        var light = Light?.GetLightData() ?? new LightData();
        var objects = _modelGroups
            .SelectMany(g => g.Models
                .Select(m => m.GetOrCreateBuffered(_resources.Context)));

        using (_resources.Swapchain.BeginDraw(pixelSize, out var image))
        {
            Console.WriteLine("RENDER");
            _resources.Content.Render(image, camera, light, objects);
        }
    }

    public SimpleView3D()
    {
        ModelGroups.CollectionChanged += ModelGroupsCollectionChanged;
    }

    ~SimpleView3D()
    {
        ModelGroups.CollectionChanged -= ModelGroupsCollectionChanged;
    }

    private void ModelGroupsCollectionChanged(object? sender, NotifyCollectionChangedEventArgs e)
    {
        QueueNextFrame();
        if (e.NewItems is not null)
            foreach (ModelGroup item in e.NewItems)
                item.PropertyChanged += ModelGroupChanged;

        if (e.OldItems is not null)
            foreach (ModelGroup item in e.OldItems)
                item.PropertyChanged -= ModelGroupChanged;
    }

    private void ModelGroupChanged(object? sender, AvaloniaPropertyChangedEventArgs e)
    {
        QueueNextFrame();
    }
}