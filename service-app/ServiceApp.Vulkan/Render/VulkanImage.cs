using System.Runtime.InteropServices;
using Avalonia;
using Avalonia.Platform;
using Silk.NET.Vulkan;
using Silk.NET.Vulkan.Extensions.KHR;
using Device = Silk.NET.Vulkan.Device;
using Format = Silk.NET.Vulkan.Format;

namespace ServiceApp.Vulkan.Render;

internal unsafe class VulkanImage : IDisposable
{
    private readonly VulkanCommandBufferPool _commandBufferPool;
    private readonly Device _device;
    private readonly Instance _instance;
    private AccessFlags _currentAccessFlags;
    private ImageLayout _currentLayout;

    public VulkanImage(VulkanContext vk, uint format, PixelSize size,
        bool exportable, IReadOnlyList<string> supportedHandleTypes)
    {
        _instance = vk.Instance;
        _device = vk.Device;
        var physicalDevice = vk.PhysicalDevice;
        _commandBufferPool = vk.Pool;
        Format = (Format)format;
        Api = vk.Api;
        Size = size;
        MipLevels = 1;
        ImageUsageFlags =
            ImageUsageFlags.ColorAttachmentBit | ImageUsageFlags.TransferDstBit |
            ImageUsageFlags.TransferSrcBit | ImageUsageFlags.SampledBit;

        var handleType = RuntimeInformation.IsOSPlatform(OSPlatform.Windows)
            ? supportedHandleTypes.Contains(KnownPlatformGraphicsExternalImageHandleTypes.D3D11TextureNtHandle)
              && !supportedHandleTypes.Contains(KnownPlatformGraphicsExternalImageHandleTypes.VulkanOpaqueNtHandle)
                ? ExternalMemoryHandleTypeFlags.D3D11TextureBit
                : ExternalMemoryHandleTypeFlags.OpaqueWin32Bit
            : ExternalMemoryHandleTypeFlags.OpaqueFDBit;

        var externalMemoryCreateInfo = new ExternalMemoryImageCreateInfo
        {
            SType = StructureType.ExternalMemoryImageCreateInfo,
            HandleTypes = handleType
        };

        var imageCreateInfo = new ImageCreateInfo
        {
            PNext = exportable ? &externalMemoryCreateInfo : null,
            SType = StructureType.ImageCreateInfo,
            ImageType = ImageType.Type2D,
            Format = Format,
            Extent =
                new Extent3D((uint?)Size.Width,
                    (uint?)Size.Height, 1),
            MipLevels = MipLevels,
            ArrayLayers = 1,
            Samples = SampleCountFlags.Count1Bit,
            Tiling = Tiling,
            Usage = ImageUsageFlags,
            SharingMode = SharingMode.Exclusive,
            InitialLayout = ImageLayout.Undefined,
            Flags = ImageCreateFlags.CreateMutableFormatBit
        };

        Api
            .CreateImage(_device, imageCreateInfo, null, out var image).ThrowOnError();
        InternalHandle = image;

        Api.GetImageMemoryRequirements(_device, InternalHandle,
            out var memoryRequirements);

        var dedicatedAllocation = new MemoryDedicatedAllocateInfoKHR
        {
            SType = StructureType.MemoryDedicatedAllocateInfoKhr,
            Image = image
        };

        var fdExport = new ExportMemoryAllocateInfo
        {
            HandleTypes = handleType, SType = StructureType.ExportMemoryAllocateInfo,
            PNext = &dedicatedAllocation
        };

        ImportMemoryWin32HandleInfoKHR handleImport = default;
        if (handleType == ExternalMemoryHandleTypeFlags.D3D11TextureBit && exportable)
            throw new NotSupportedException("Vulkan D3DDevice wasn't created");

        var memoryAllocateInfo = new MemoryAllocateInfo
        {
            PNext =
                exportable ? handleImport.Handle != IntPtr.Zero ? &handleImport : &fdExport : null,
            SType = StructureType.MemoryAllocateInfo,
            AllocationSize = memoryRequirements.Size,
            MemoryTypeIndex = (uint)VulkanMemoryHelper.FindSuitableMemoryTypeIndex(
                Api,
                physicalDevice,
                memoryRequirements.MemoryTypeBits, MemoryPropertyFlags.DeviceLocalBit)
        };

        Api.AllocateMemory(_device, memoryAllocateInfo, null,
            out var imageMemory).ThrowOnError();

        ImageMemory = imageMemory;


        MemorySize = memoryRequirements.Size;

        Api.BindImageMemory(_device, InternalHandle, ImageMemory, 0).ThrowOnError();
        var componentMapping = new ComponentMapping(
            ComponentSwizzle.Identity,
            ComponentSwizzle.Identity,
            ComponentSwizzle.Identity,
            ComponentSwizzle.Identity);

        AspectFlags = ImageAspectFlags.ColorBit;

        var subresourceRange = new ImageSubresourceRange(AspectFlags, 0, MipLevels, 0, 1);

        var imageViewCreateInfo = new ImageViewCreateInfo
        {
            SType = StructureType.ImageViewCreateInfo,
            Image = InternalHandle,
            ViewType = ImageViewType.Type2D,
            Format = Format,
            Components = componentMapping,
            SubresourceRange = subresourceRange
        };

        Api
            .CreateImageView(_device, imageViewCreateInfo, null, out var imageView)
            .ThrowOnError();

        ImageView = imageView;

        _currentLayout = ImageLayout.Undefined;

        TransitionLayout(ImageLayout.ColorAttachmentOptimal, AccessFlags.NoneKhr);
    }

    private ImageUsageFlags ImageUsageFlags { get; }
    private ImageView ImageView { get; set; }
    private DeviceMemory ImageMemory { get; set; }

    internal Image InternalHandle { get; private set; }
    internal Format Format { get; }
    internal ImageAspectFlags AspectFlags { get; }

    public ulong Handle => InternalHandle.Handle;
    public ulong ViewHandle => ImageView.Handle;
    public uint UsageFlags => (uint)ImageUsageFlags;
    public ulong MemoryHandle => ImageMemory.Handle;
    public DeviceMemory DeviceMemory => ImageMemory;
    public uint MipLevels { get; }
    public Vk Api { get; }
    public PixelSize Size { get; }
    public ulong MemorySize { get; }
    public uint CurrentLayout => (uint)_currentLayout;

    public ImageTiling Tiling => ImageTiling.Optimal;

    public void Dispose()
    {
        Api.DestroyImageView(_device, ImageView, null);
        Api.DestroyImage(_device, InternalHandle, null);
        Api.FreeMemory(_device, ImageMemory, null);

        ImageView = default;
        InternalHandle = default;
        ImageMemory = default;
    }

    public int ExportFd()
    {
        if (!Api.TryGetDeviceExtension<KhrExternalMemoryFd>(_instance, _device, out var ext))
            throw new InvalidOperationException();
        var info = new MemoryGetFdInfoKHR
        {
            Memory = ImageMemory,
            SType = StructureType.MemoryGetFDInfoKhr,
            HandleType = ExternalMemoryHandleTypeFlags.OpaqueFDBit
        };
        ext.GetMemoryF(_device, info, out var fd).ThrowOnError();
        return fd;
    }

    public IntPtr ExportOpaqueNtHandle()
    {
        if (!Api.TryGetDeviceExtension<KhrExternalMemoryWin32>(_instance, _device, out var ext))
            throw new InvalidOperationException();
        var info = new MemoryGetWin32HandleInfoKHR
        {
            Memory = ImageMemory,
            SType = StructureType.MemoryGetWin32HandleInfoKhr,
            HandleType = ExternalMemoryHandleTypeFlags.OpaqueWin32Bit
        };
        ext.GetMemoryWin32Handle(_device, info, out var fd).ThrowOnError();
        return fd;
    }

    public IPlatformHandle Export()
    {
        if (RuntimeInformation.IsOSPlatform(OSPlatform.Windows))
            return new PlatformHandle(ExportOpaqueNtHandle(),
                KnownPlatformGraphicsExternalImageHandleTypes.VulkanOpaqueNtHandle);

        return new PlatformHandle(new IntPtr(ExportFd()),
            KnownPlatformGraphicsExternalImageHandleTypes.VulkanOpaquePosixFileDescriptor);
    }

    internal void TransitionLayout(CommandBuffer commandBuffer,
        ImageLayout fromLayout, AccessFlags fromAccessFlags,
        ImageLayout destinationLayout, AccessFlags destinationAccessFlags)
    {
        VulkanMemoryHelper.TransitionLayout(Api, commandBuffer, InternalHandle,
            fromLayout,
            fromAccessFlags,
            destinationLayout, destinationAccessFlags,
            MipLevels);

        _currentLayout = destinationLayout;
        _currentAccessFlags = destinationAccessFlags;
    }

    internal void TransitionLayout(CommandBuffer commandBuffer,
        ImageLayout destinationLayout, AccessFlags destinationAccessFlags)
    {
        TransitionLayout(commandBuffer, _currentLayout, _currentAccessFlags, destinationLayout,
            destinationAccessFlags);
    }


    internal void TransitionLayout(ImageLayout destinationLayout, AccessFlags destinationAccessFlags)
    {
        var commandBuffer = _commandBufferPool.CreateCommandBuffer();
        commandBuffer.BeginRecording();
        TransitionLayout(commandBuffer.InternalHandle, destinationLayout, destinationAccessFlags);
        commandBuffer.EndRecording();
        commandBuffer.Submit();
    }

    public void TransitionLayout(uint destinationLayout, uint destinationAccessFlags)
    {
        TransitionLayout((ImageLayout)destinationLayout, (AccessFlags)destinationAccessFlags);
    }
}