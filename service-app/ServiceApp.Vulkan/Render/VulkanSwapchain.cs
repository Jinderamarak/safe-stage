using Avalonia;
using Avalonia.Platform;
using Avalonia.Rendering.Composition;
using Silk.NET.Vulkan;

namespace ServiceApp.Vulkan.Render;

internal class VulkanSwapchain : SwapchainBase<VulkanSwapchainImage>
{
    private readonly VulkanContext _vk;

    public VulkanSwapchain(VulkanContext vk, ICompositionGpuInterop interop, CompositionDrawingSurface target) : base(
        interop, target)
    {
        _vk = vk;
    }

    protected override VulkanSwapchainImage CreateImage(PixelSize size)
    {
        return new VulkanSwapchainImage(_vk, size, Interop, Target);
    }

    public IDisposable BeginDraw(PixelSize size, out VulkanImage image)
    {
        _vk.Pool.FreeUsedCommandBuffers();
        var rv = BeginDrawCore(size, out var swapchainImage);
        image = swapchainImage.Image;
        return rv;
    }
}

internal class VulkanSwapchainImage : ISwapchainImage
{
    private readonly ICompositionGpuInterop _interop;
    private readonly VulkanSemaphorePair _semaphorePair;
    private readonly CompositionDrawingSurface _target;
    private readonly VulkanContext _vk;
    private ICompositionImportedGpuSemaphore? _availableSemaphore, _renderCompletedSemaphore;
    private ICompositionImportedGpuImage? _importedImage;
    private bool _initial = true;

    public VulkanSwapchainImage(VulkanContext vk, PixelSize size, ICompositionGpuInterop interop,
        CompositionDrawingSurface target)
    {
        _vk = vk;
        _interop = interop;
        _target = target;
        Size = size;
        Image = new VulkanImage(vk, (uint)Format.R8G8B8A8Unorm, size, true, interop.SupportedImageHandleTypes);
        _semaphorePair = new VulkanSemaphorePair(vk, true);
    }

    public VulkanImage Image { get; }

    public async ValueTask DisposeAsync()
    {
        if (LastPresent != null)
            await LastPresent;
        if (_importedImage != null)
            await _importedImage.DisposeAsync();
        if (_availableSemaphore != null)
            await _availableSemaphore.DisposeAsync();
        if (_renderCompletedSemaphore != null)
            await _renderCompletedSemaphore.DisposeAsync();
        _semaphorePair.Dispose();
        Image.Dispose();
    }

    public PixelSize Size { get; }

    public Task? LastPresent { get; private set; }

    public void BeginDraw()
    {
        var buffer = _vk.Pool.CreateCommandBuffer();
        buffer.BeginRecording();

        Image.TransitionLayout(buffer.InternalHandle,
            ImageLayout.Undefined, AccessFlags.None,
            ImageLayout.ColorAttachmentOptimal, AccessFlags.ColorAttachmentReadBit);

        if (_initial)
        {
            _initial = false;
            buffer.Submit();
        }
        else
        {
            buffer.Submit(new[] { _semaphorePair.ImageAvailableSemaphore },
                new[]
                {
                    PipelineStageFlags.AllGraphicsBit
                });
        }
    }


    public void Present()
    {
        var buffer = _vk.Pool.CreateCommandBuffer();
        buffer.BeginRecording();
        Image.TransitionLayout(buffer.InternalHandle, ImageLayout.TransferSrcOptimal, AccessFlags.TransferWriteBit);


        buffer.Submit(null, null, new[] { _semaphorePair.RenderFinishedSemaphore });

        _availableSemaphore ??= _interop.ImportSemaphore(_semaphorePair.Export(false));

        _renderCompletedSemaphore ??= _interop.ImportSemaphore(_semaphorePair.Export(true));

        _importedImage ??= _interop.ImportImage(Image.Export(),
            new PlatformGraphicsExternalImageProperties
            {
                Format = PlatformGraphicsExternalImageFormat.R8G8B8A8UNorm,
                Width = Size.Width,
                Height = Size.Height,
                MemorySize = Image.MemorySize
            });

        LastPresent =
            _target.UpdateWithSemaphoresAsync(_importedImage, _renderCompletedSemaphore, _availableSemaphore);
    }
}