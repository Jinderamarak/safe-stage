using System.Reactive.Disposables;
using Avalonia;
using Avalonia.Rendering.Composition;

namespace ServiceApp.Vulkan.Render;

/// <summary>
///     A helper class for composition-backed swapchains, should not be a public API yet
/// </summary>
internal abstract class SwapchainBase<TImage> : IAsyncDisposable where TImage : class, ISwapchainImage
{
    private readonly List<TImage> _pendingImages = new();

    protected SwapchainBase(ICompositionGpuInterop interop, CompositionDrawingSurface target)
    {
        Interop = interop;
        Target = target;
    }

    protected ICompositionGpuInterop Interop { get; }
    protected CompositionDrawingSurface Target { get; }

    public async ValueTask DisposeAsync()
    {
        foreach (var img in _pendingImages)
            await img.DisposeAsync();
    }

    private static bool IsBroken(TImage image)
    {
        return image.LastPresent?.IsFaulted == true;
    }

    private static bool IsReady(TImage image)
    {
        return image.LastPresent == null || image.LastPresent.Status == TaskStatus.RanToCompletion;
    }

    private TImage? CleanupAndFindNextImage(PixelSize size)
    {
        TImage? firstFound = null;
        var foundMultiple = false;

        for (var c = _pendingImages.Count - 1; c > -1; c--)
        {
            var image = _pendingImages[c];
            var ready = IsReady(image);
            var matches = image.Size == size;
            if (IsBroken(image) || (!matches && ready))
            {
                image.DisposeAsync();
                _pendingImages.RemoveAt(c);
            }

            if (matches && ready)
            {
                if (firstFound == null)
                    firstFound = image;
                else
                    foundMultiple = true;
            }
        }

        // We are making sure that there was at least one image of the same size in flight
        // Otherwise we might encounter UI thread lockups
        return foundMultiple ? firstFound : null;
    }

    protected abstract TImage CreateImage(PixelSize size);

    protected IDisposable BeginDrawCore(PixelSize size, out TImage image)
    {
        var img = CleanupAndFindNextImage(size) ?? CreateImage(size);

        img.BeginDraw();
        _pendingImages.Remove(img);
        image = img;
        return Disposable.Create(() =>
        {
            img.Present();
            _pendingImages.Add(img);
        });
    }
}

internal interface ISwapchainImage : IAsyncDisposable
{
    PixelSize Size { get; }
    Task? LastPresent { get; }
    void BeginDraw();
    void Present();
}