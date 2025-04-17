using System;
using System.Threading.Tasks;
using Avalonia;
using Avalonia.Controls;
using Avalonia.LogicalTree;
using Avalonia.Rendering.Composition;
using Avalonia.VisualTree;

namespace GpuInterop;

public abstract class DrawingSurfaceDemoBase : Control
{
    private CompositionSurfaceVisual? _visual;
    private Compositor? _compositor;
    private readonly Action _update;
    private string _info = string.Empty;
    private bool _updateQueued;
    private bool _initialized;

    protected CompositionDrawingSurface? Surface { get; private set; }

    public DrawingSurfaceDemoBase()
    {
        _update = UpdateFrame;
    }

    protected override void OnAttachedToVisualTree(VisualTreeAttachmentEventArgs e)
    {
        base.OnAttachedToVisualTree(e);
        Initialize();
    }

    protected override void OnDetachedFromLogicalTree(LogicalTreeAttachmentEventArgs e)
    {
        if (_initialized)
        {
            Surface?.Dispose();
            FreeGraphicsResources();
        }

        _initialized = false;
        base.OnDetachedFromLogicalTree(e);
    }

    private async void Initialize()
    {
        try
        {
            var selfVisual = ElementComposition.GetElementVisual(this)!;
            _compositor = selfVisual.Compositor;

            Surface = _compositor.CreateDrawingSurface();
            _visual = _compositor.CreateSurfaceVisual();
            _visual.Size = new Vector(Bounds.Width, Bounds.Height);
            _visual.Surface = Surface;
            ElementComposition.SetElementChildVisual(this, _visual);
            var (res, info) = await DoInitialize(_compositor, Surface);
            _info = info;
            _initialized = res;
            QueueNextFrame();
        }
        catch (Exception e)
        {
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
        QueueNextFrame();
    }

    private void QueueNextFrame()
    {
        if (_initialized && !_updateQueued && _compositor != null)
        {
            _updateQueued = true;
            _compositor?.RequestCompositionUpdate(_update);
        }
    }

    protected override void OnPropertyChanged(AvaloniaPropertyChangedEventArgs change)
    {
        if (change.Property == BoundsProperty)
            QueueNextFrame();
        base.OnPropertyChanged(change);
    }

    private async Task<(bool success, string info)> DoInitialize(Compositor compositor,
        CompositionDrawingSurface compositionDrawingSurface)
    {
        var interop = await compositor.TryGetCompositionGpuInterop();
        if (interop == null)
            return (false, "Compositor doesn't support interop for the current backend");
        return InitializeGraphicsResources(compositor, compositionDrawingSurface, interop);
    }

    protected abstract (bool success, string info) InitializeGraphicsResources(Compositor compositor,
        CompositionDrawingSurface compositionDrawingSurface, ICompositionGpuInterop gpuInterop);

    protected abstract void FreeGraphicsResources();


    protected abstract void RenderFrame(PixelSize pixelSize);
    protected virtual bool SupportsDisco => false;

    public void Update(GpuDemo? parent, float yaw, float pitch, float roll, float disco, int timeTick)
    {
        ParentControl = parent;
        if (ParentControl != null)
        {
        }

        Yaw = yaw;
        Pitch = pitch;
        Roll = roll;
        Disco = disco;
        TimeTick = timeTick;
        QueueNextFrame();
    }

    public GpuDemo? ParentControl { get; private set; }

    public int TimeTick { get; private set; }

    public float Disco { get; private set; }

    public float Roll { get; private set; }

    public float Pitch { get; private set; }

    public float Yaw { get; private set; }
}