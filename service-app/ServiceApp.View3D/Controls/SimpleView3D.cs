using Avalonia;
using Avalonia.Controls;
using Avalonia.LogicalTree;
using Avalonia.Rendering.Composition;
using Avalonia.VisualTree;
using ServiceApp.View3D.Data;
using ServiceApp.View3D.Render;

namespace ServiceApp.View3D.Controls;

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
            Console.WriteLine(e);
        }
    }

    private void UpdateFrame()
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

    private ModelGroup? _modelGroup;

    public static readonly DirectProperty<SimpleView3D, ModelGroup?> ModelGroupProperty =
        AvaloniaProperty.RegisterDirect<SimpleView3D, ModelGroup?>(nameof(ModelGroup), o => o.ModelGroup,
            (o, v) => o.ModelGroup = v);

    public ModelGroup? ModelGroup
    {
        get => _modelGroup;
        set => SetAndRaise(ModelGroupProperty, ref _modelGroup, value);
    }

    protected override void OnPropertyChanged(AvaloniaPropertyChangedEventArgs change)
    {
        if (change.Property == CameraProperty)
        {
            if (change.OldValue is Camera3D oldCamera)
                oldCamera.PropertyChanged -= ChildChanged;
            if (change.NewValue is Camera3D newCamera)
                newCamera.PropertyChanged += ChildChanged;
        }

        if (change.Property == LightProperty)
        {
            if (change.OldValue is PointLight oldLight)
                oldLight.PropertyChanged -= ChildChanged;
            if (change.NewValue is PointLight newLight)
                newLight.PropertyChanged += ChildChanged;
        }

        if (change.Property == ModelGroupProperty)
        {
            if (change.OldValue is ModelGroup oldGroup)
                oldGroup.PropertyChanged -= ChildChanged;
            if (change.NewValue is ModelGroup newGroup)
                newGroup.PropertyChanged += ChildChanged;
        }

        if (change.Property == BoundsProperty
            || change.Property == CameraProperty
            || change.Property == LightProperty
            || change.Property == ModelGroupProperty)
            QueueNextFrame();
        base.OnPropertyChanged(change);
    }

    private void RenderFrame(PixelSize pixelSize)
    {
        if (_resources == null)
            return;

        var camera = Camera?.GetCameraData() ?? new CameraData();
        var light = Light?.GetLightData() ?? new LightData();
        var objects = FlattenGroup(ModelGroup);

        using (_resources.Swapchain.BeginDraw(pixelSize, out var image))
        {
            _resources.Content.Render(image, camera, light, objects);
        }
    }

    private IEnumerable<BufferedObject> FlattenGroup(ModelGroup? group)
    {
        if (group is null || _resources is null)
            yield break;

        foreach (var model in group.Models)
            yield return model.GetOrCreateBuffered(_resources.Context);
        foreach (var nested in group.Groups)
        foreach (var inner in FlattenGroup(nested))
            yield return inner;
    }

    private void ChildChanged(object? sender, AvaloniaPropertyChangedEventArgs e)
    {
        QueueNextFrame();
    }
}