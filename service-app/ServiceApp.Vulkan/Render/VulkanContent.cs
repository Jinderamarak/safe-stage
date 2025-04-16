using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using Avalonia;
using ServiceApp.Vulkan.Data;
using ServiceApp.Vulkan.Render.Shaders;
using Silk.NET.Vulkan;
using Buffer = Silk.NET.Vulkan.Buffer;
using Image = Silk.NET.Vulkan.Image;

namespace ServiceApp.Vulkan.Render;

internal unsafe class VulkanContent : IDisposable
{
    private readonly VulkanContext _context;

    private readonly ShaderModule _fragShader;
    private readonly ShaderModule _vertShader;

    private VulkanImage? _colorAttachment;
    private Image _depthImage;
    private DeviceMemory _depthImageMemory;
    private ImageView _depthImageView;
    private DescriptorSet _descriptorSet;
    private DescriptorSetLayout _descriptorSetLayout;
    private Framebuffer _framebuffer;
    private bool _isInit;
    private Pipeline _pipeline;
    private PipelineLayout _pipelineLayout;

    private PixelSize? _previousImageSize = PixelSize.Empty;
    private RenderPass _renderPass;
    private Buffer _uniformBuffer;
    private DeviceMemory _uniformBufferMemory;

    public VulkanContent(VulkanContext context)
    {
        _context = context;

        var api = _context.Api;
        var device = _context.Device;
        var vertShaderData = EmbeddedShaders.LoadVertexShader();
        var fragShaderData = EmbeddedShaders.LoadDiffuseShader();

        fixed (byte* ptr = vertShaderData)
        {
            var shaderCreateInfo = new ShaderModuleCreateInfo
            {
                SType = StructureType.ShaderModuleCreateInfo,
                CodeSize = (nuint)vertShaderData.Length,
                PCode = (uint*)ptr
            };

            api.CreateShaderModule(device, shaderCreateInfo, null, out _vertShader);
        }

        fixed (byte* ptr = fragShaderData)
        {
            var shaderCreateInfo = new ShaderModuleCreateInfo
            {
                SType = StructureType.ShaderModuleCreateInfo,
                CodeSize = (nuint)fragShaderData.Length,
                PCode = (uint*)ptr
            };

            api.CreateShaderModule(device, shaderCreateInfo, null, out _fragShader);
        }
    }

    public void Dispose()
    {
        if (_isInit)
        {
            var api = _context.Api;
            var device = _context.Device;

            DestroyTemporalObjects();

            api.DestroyShaderModule(device, _vertShader, null);
            api.DestroyShaderModule(device, _fragShader, null);
        }

        _isInit = false;
    }

    public void Render(VulkanImage image, CameraData camera, LightData light, IEnumerable<SimpleObject3D> objects)
    {
        var api = _context.Api;

        if (image.Size != _previousImageSize)
            CreateTemporalObjects(image.Size, camera, light);

        _previousImageSize = image.Size;

        var commandBuffer = _context.Pool.CreateCommandBuffer();
        commandBuffer.BeginRecording();

        _colorAttachment!.TransitionLayout(commandBuffer.InternalHandle,
            ImageLayout.Undefined, AccessFlags.None,
            ImageLayout.ColorAttachmentOptimal, AccessFlags.ColorAttachmentWriteBit);

        var commandBufferHandle = new CommandBuffer(commandBuffer.Handle);

        api.CmdSetViewport(commandBufferHandle, 0, 1,
            new Viewport
            {
                Width = image.Size.Width,
                Height = image.Size.Height,
                MaxDepth = 1,
                MinDepth = 0,
                X = 0,
                Y = 0
            });

        var scissor = new Rect2D
        {
            Extent = new Extent2D((uint?)image.Size.Width, (uint?)image.Size.Height)
        };

        api.CmdSetScissor(commandBufferHandle, 0, 1, &scissor);

        var clearValues = new ClearValue[]
        {
            new()
            {
                //  TODO: use background color
                Color = new ClearColorValue { Float32_0 = 0.2f, Float32_1 = 0.2f, Float32_2 = 0.2f, Float32_3 = 0.1f }
            },
            new() { DepthStencil = new ClearDepthStencilValue { Depth = 1, Stencil = 0 } }
        };

        fixed (ClearValue* clearValue = clearValues)
        {
            var beginInfo = new RenderPassBeginInfo
            {
                SType = StructureType.RenderPassBeginInfo,
                RenderPass = _renderPass,
                Framebuffer = _framebuffer,
                RenderArea = new Rect2D(new Offset2D(0, 0),
                    new Extent2D((uint?)image.Size.Width, (uint?)image.Size.Height)),
                ClearValueCount = (uint)clearValues.Length,
                PClearValues = clearValue
            };

            api.CmdBeginRenderPass(commandBufferHandle, beginInfo, SubpassContents.Inline);
        }

        api.CmdBindPipeline(commandBufferHandle, PipelineBindPoint.Graphics, _pipeline);

        var dset = _descriptorSet;
        api.CmdBindDescriptorSets(commandBufferHandle, PipelineBindPoint.Graphics,
            _pipelineLayout, 0, 1, &dset, null);

        foreach (var simpleObject in objects) simpleObject.Draw(api, commandBufferHandle, _pipelineLayout);

        api.CmdEndRenderPass(commandBufferHandle);

        _colorAttachment.TransitionLayout(commandBuffer.InternalHandle, ImageLayout.TransferSrcOptimal,
            AccessFlags.TransferReadBit);
        image.TransitionLayout(commandBuffer.InternalHandle, ImageLayout.TransferDstOptimal,
            AccessFlags.TransferWriteBit);

        var srcBlitRegion = new ImageBlit
        {
            SrcOffsets = new ImageBlit.SrcOffsetsBuffer
            {
                Element0 = new Offset3D(0, 0, 0),
                Element1 = new Offset3D(image.Size.Width, image.Size.Height, 1)
            },
            DstOffsets = new ImageBlit.DstOffsetsBuffer
            {
                Element0 = new Offset3D(0, 0, 0),
                Element1 = new Offset3D(image.Size.Width, image.Size.Height, 1)
            },
            SrcSubresource =
                new ImageSubresourceLayers
                {
                    AspectMask = ImageAspectFlags.ColorBit,
                    BaseArrayLayer = 0,
                    LayerCount = 1,
                    MipLevel = 0
                },
            DstSubresource = new ImageSubresourceLayers
            {
                AspectMask = ImageAspectFlags.ColorBit,
                BaseArrayLayer = 0,
                LayerCount = 1,
                MipLevel = 0
            }
        };

        api.CmdBlitImage(commandBuffer.InternalHandle, _colorAttachment.InternalHandle,
            ImageLayout.TransferSrcOptimal,
            image.InternalHandle, ImageLayout.TransferDstOptimal, 1, srcBlitRegion, Filter.Linear);

        commandBuffer.Submit();
    }

    private void DestroyTemporalObjects()
    {
        if (_isInit)
            if (_renderPass.Handle != 0)
            {
                var api = _context.Api;
                var device = _context.Device;
                api.FreeDescriptorSets(_context.Device, _context.DescriptorPool, new[] { _descriptorSet });

                api.DestroyImageView(device, _depthImageView, null);
                api.DestroyImage(device, _depthImage, null);
                api.FreeMemory(device, _depthImageMemory, null);

                api.DestroyFramebuffer(device, _framebuffer, null);
                api.DestroyPipeline(device, _pipeline, null);
                api.DestroyPipelineLayout(device, _pipelineLayout, null);
                api.DestroyRenderPass(device, _renderPass, null);
                api.DestroyDescriptorSetLayout(device, _descriptorSetLayout, null);

                api.DestroyBuffer(device, _uniformBuffer, null);
                api.FreeMemory(device, _uniformBufferMemory, null);
                _colorAttachment?.Dispose();

                _colorAttachment = null;
                _depthImage = default;
                _depthImageView = default;
                _depthImageView = default;
                _framebuffer = default;
                _pipeline = default;
                _renderPass = default;
                _pipelineLayout = default;
                _descriptorSetLayout = default;
                _uniformBuffer = default;
                _uniformBufferMemory = default;
            }
    }

    private void CreateDepthAttachment(PixelSize size)
    {
        var imageCreateInfo = new ImageCreateInfo
        {
            SType = StructureType.ImageCreateInfo,
            ImageType = ImageType.Type2D,
            Format = Format.D32Sfloat,
            Extent =
                new Extent3D((uint?)size.Width,
                    (uint?)size.Height, 1),
            MipLevels = 1,
            ArrayLayers = 1,
            Samples = SampleCountFlags.Count1Bit,
            Tiling = ImageTiling.Optimal,
            Usage = ImageUsageFlags.DepthStencilAttachmentBit,
            SharingMode = SharingMode.Exclusive,
            InitialLayout = ImageLayout.Undefined,
            Flags = ImageCreateFlags.CreateMutableFormatBit
        };

        var api = _context.Api;
        var device = _context.Device;
        api
            .CreateImage(device, imageCreateInfo, null, out _depthImage).ThrowOnError();

        api.GetImageMemoryRequirements(device, _depthImage,
            out var memoryRequirements);

        var memoryAllocateInfo = new MemoryAllocateInfo
        {
            SType = StructureType.MemoryAllocateInfo,
            AllocationSize = memoryRequirements.Size,
            MemoryTypeIndex = (uint)FindSuitableMemoryTypeIndex(api,
                _context.PhysicalDevice,
                memoryRequirements.MemoryTypeBits, MemoryPropertyFlags.DeviceLocalBit)
        };

        api.AllocateMemory(device, memoryAllocateInfo, null,
            out _depthImageMemory).ThrowOnError();

        api.BindImageMemory(device, _depthImage, _depthImageMemory, 0);

        var componentMapping = new ComponentMapping(
            ComponentSwizzle.R,
            ComponentSwizzle.G,
            ComponentSwizzle.B,
            ComponentSwizzle.A);

        var subresourceRange = new ImageSubresourceRange(ImageAspectFlags.DepthBit,
            0, 1, 0, 1);

        var imageViewCreateInfo = new ImageViewCreateInfo
        {
            SType = StructureType.ImageViewCreateInfo,
            Image = _depthImage,
            ViewType = ImageViewType.Type2D,
            Format = Format.D32Sfloat,
            Components = componentMapping,
            SubresourceRange = subresourceRange
        };

        api
            .CreateImageView(device, imageViewCreateInfo, null, out _depthImageView)
            .ThrowOnError();
    }

    private void CreateTemporalObjects(PixelSize size, CameraData camera, LightData light)
    {
        DestroyTemporalObjects();

        _colorAttachment = new VulkanImage(_context, (uint)Format.R8G8B8A8Unorm, size, false, Array.Empty<string>());
        CreateDepthAttachment(size);

        var api = _context.Api;
        var device = _context.Device;

        // create renderpasses
        var colorAttachment = new AttachmentDescription
        {
            Format = Format.R8G8B8A8Unorm,
            Samples = SampleCountFlags.Count1Bit,
            LoadOp = AttachmentLoadOp.Clear,
            StoreOp = AttachmentStoreOp.Store,
            InitialLayout = ImageLayout.Undefined,
            FinalLayout = ImageLayout.ColorAttachmentOptimal,
            StencilLoadOp = AttachmentLoadOp.DontCare,
            StencilStoreOp = AttachmentStoreOp.DontCare
        };

        var depthAttachment = new AttachmentDescription
        {
            Format = Format.D32Sfloat,
            Samples = SampleCountFlags.Count1Bit,
            LoadOp = AttachmentLoadOp.Clear,
            StoreOp = AttachmentStoreOp.DontCare,
            InitialLayout = ImageLayout.Undefined,
            FinalLayout = ImageLayout.DepthStencilAttachmentOptimal,
            StencilLoadOp = AttachmentLoadOp.DontCare,
            StencilStoreOp = AttachmentStoreOp.DontCare
        };

        var subpassDependency = new SubpassDependency
        {
            SrcSubpass = Vk.SubpassExternal,
            DstSubpass = 0,
            SrcStageMask = PipelineStageFlags.ColorAttachmentOutputBit,
            SrcAccessMask = 0,
            DstStageMask = PipelineStageFlags.ColorAttachmentOutputBit,
            DstAccessMask = AccessFlags.ColorAttachmentWriteBit
        };

        var colorAttachmentReference = new AttachmentReference
        {
            Attachment = 0, Layout = ImageLayout.ColorAttachmentOptimal
        };

        var depthAttachmentReference = new AttachmentReference
        {
            Attachment = 1, Layout = ImageLayout.DepthStencilAttachmentOptimal
        };

        var subpassDescription = new SubpassDescription
        {
            PipelineBindPoint = PipelineBindPoint.Graphics,
            ColorAttachmentCount = 1,
            PColorAttachments = &colorAttachmentReference,
            PDepthStencilAttachment = &depthAttachmentReference
        };

        var attachments = new[] { colorAttachment, depthAttachment };

        fixed (AttachmentDescription* atPtr = attachments)
        {
            var renderPassCreateInfo = new RenderPassCreateInfo
            {
                SType = StructureType.RenderPassCreateInfo,
                AttachmentCount = (uint)attachments.Length,
                PAttachments = atPtr,
                SubpassCount = 1,
                PSubpasses = &subpassDescription,
                DependencyCount = 1,
                PDependencies = &subpassDependency
            };

            api.CreateRenderPass(device, renderPassCreateInfo, null, out _renderPass).ThrowOnError();


            // create framebuffer
            var frameBufferAttachments = new[] { new ImageView(_colorAttachment.ViewHandle), _depthImageView };

            fixed (ImageView* frAtPtr = frameBufferAttachments)
            {
                var framebufferCreateInfo = new FramebufferCreateInfo
                {
                    SType = StructureType.FramebufferCreateInfo,
                    RenderPass = _renderPass,
                    AttachmentCount = (uint)frameBufferAttachments.Length,
                    PAttachments = frAtPtr,
                    Width = (uint)size.Width,
                    Height = (uint)size.Height,
                    Layers = 1
                };

                api.CreateFramebuffer(device, framebufferCreateInfo, null, out _framebuffer).ThrowOnError();
            }
        }

        // Create pipeline
        var pname = Marshal.StringToHGlobalAnsi("main");
        var vertShaderStageInfo = new PipelineShaderStageCreateInfo
        {
            SType = StructureType.PipelineShaderStageCreateInfo,
            Stage = ShaderStageFlags.VertexBit,
            Module = _vertShader,
            PName = (byte*)pname
        };
        var fragShaderStageInfo = new PipelineShaderStageCreateInfo
        {
            SType = StructureType.PipelineShaderStageCreateInfo,
            Stage = ShaderStageFlags.FragmentBit,
            Module = _fragShader,
            PName = (byte*)pname
        };

        var stages = new[] { vertShaderStageInfo, fragShaderStageInfo };

        var bindingDescription = VertexInput.VertexInputBindingDescription;
        var attributeDescription = VertexInput.VertexInputAttributeDescription;

        fixed (VertexInputAttributeDescription* attrPtr = attributeDescription)
        {
            var vertextInputInfo = new PipelineVertexInputStateCreateInfo
            {
                SType = StructureType.PipelineVertexInputStateCreateInfo,
                VertexAttributeDescriptionCount = (uint)attributeDescription.Length,
                VertexBindingDescriptionCount = 1,
                PVertexAttributeDescriptions = attrPtr,
                PVertexBindingDescriptions = &bindingDescription
            };

            var inputAssembly = new PipelineInputAssemblyStateCreateInfo
            {
                SType = StructureType.PipelineInputAssemblyStateCreateInfo,
                Topology = PrimitiveTopology.TriangleList,
                PrimitiveRestartEnable = false
            };

            var viewport = new Viewport
            {
                X = 0,
                Y = 0,
                Width = size.Width,
                Height = size.Height,
                MinDepth = 0,
                MaxDepth = 1
            };

            var scissor = new Rect2D
            {
                Offset = new Offset2D(0, 0), Extent = new Extent2D((uint)viewport.Width, (uint)viewport.Height)
            };

            var pipelineViewPortCreateInfo = new PipelineViewportStateCreateInfo
            {
                SType = StructureType.PipelineViewportStateCreateInfo,
                ViewportCount = 1,
                PViewports = &viewport,
                ScissorCount = 1,
                PScissors = &scissor
            };

            var rasterizerStateCreateInfo = new PipelineRasterizationStateCreateInfo
            {
                SType = StructureType.PipelineRasterizationStateCreateInfo,
                DepthClampEnable = false,
                RasterizerDiscardEnable = false,
                PolygonMode = PolygonMode.Fill,
                LineWidth = 1,
                CullMode = CullModeFlags.None,
                DepthBiasEnable = false
            };

            var multisampleStateCreateInfo = new PipelineMultisampleStateCreateInfo
            {
                SType = StructureType.PipelineMultisampleStateCreateInfo,
                SampleShadingEnable = false,
                RasterizationSamples = SampleCountFlags.Count1Bit
            };

            var depthStencilCreateInfo = new PipelineDepthStencilStateCreateInfo
            {
                SType = StructureType.PipelineDepthStencilStateCreateInfo,
                StencilTestEnable = false,
                DepthCompareOp = CompareOp.Less,
                DepthTestEnable = true,
                DepthWriteEnable = true,
                DepthBoundsTestEnable = false
            };

            var colorBlendAttachmentState = new PipelineColorBlendAttachmentState
            {
                ColorWriteMask = ColorComponentFlags.ABit |
                                 ColorComponentFlags.RBit |
                                 ColorComponentFlags.GBit |
                                 ColorComponentFlags.BBit,
                BlendEnable = false
            };

            var colorBlendState = new PipelineColorBlendStateCreateInfo
            {
                SType = StructureType.PipelineColorBlendStateCreateInfo,
                LogicOpEnable = false,
                AttachmentCount = 1,
                PAttachments = &colorBlendAttachmentState
            };

            var dynamicStates = new[] { DynamicState.Viewport, DynamicState.Scissor };

            fixed (DynamicState* states = dynamicStates)
            {
                var dynamicStateCreateInfo = new PipelineDynamicStateCreateInfo
                {
                    SType = StructureType.PipelineDynamicStateCreateInfo,
                    DynamicStateCount = (uint)dynamicStates.Length,
                    PDynamicStates = states
                };

                var vertexPushConstantRange = new PushConstantRange
                {
                    Offset = 0,
                    Size = (uint)Marshal.SizeOf<VertexPushConstant>(),
                    StageFlags = ShaderStageFlags.VertexBit
                };

                var fragPushConstantRange = new PushConstantRange
                {
                    Offset = 0,
                    Size = (uint)Marshal.SizeOf<VertexPushConstant>(),
                    StageFlags = ShaderStageFlags.FragmentBit
                };

                var layoutBindingInfo = new DescriptorSetLayoutBinding
                {
                    Binding = 0,
                    StageFlags = ShaderStageFlags.VertexBit,
                    DescriptorCount = 1,
                    DescriptorType = DescriptorType.UniformBuffer
                };

                var layoutInfo = new DescriptorSetLayoutCreateInfo
                {
                    SType = StructureType.DescriptorSetLayoutCreateInfo,
                    BindingCount = 1,
                    PBindings = &layoutBindingInfo
                };

                api.CreateDescriptorSetLayout(device, &layoutInfo, null, out _descriptorSetLayout).ThrowOnError();

                VulkanBufferHelper.AllocateBuffer<UniformBufferObject>(_context, BufferUsageFlags.UniformBufferBit,
                    out _uniformBuffer,
                    out _uniformBufferMemory, new[]
                    {
                        new UniformBufferObject
                        {
                            Projection = camera.ProjectionView(size.Width, size.Height),
                            LightPosition = light.Position,
                            LightColor = light.ColorVector,
                            LightStrength = light.Strength
                        }
                    });

                var descriptorSetLayout = _descriptorSetLayout;
                var descriptorCreateInfo = new DescriptorSetAllocateInfo
                {
                    SType = StructureType.DescriptorSetAllocateInfo,
                    DescriptorPool = _context.DescriptorPool,
                    DescriptorSetCount = 1,
                    PSetLayouts = &descriptorSetLayout
                };
                api.AllocateDescriptorSets(device, &descriptorCreateInfo, out _descriptorSet).ThrowOnError();

                var descriptorBufferInfo = new DescriptorBufferInfo
                {
                    Buffer = _uniformBuffer,
                    Range = (ulong)Unsafe.SizeOf<UniformBufferObject>()
                };
                var descriptorWrite = new WriteDescriptorSet
                {
                    SType = StructureType.WriteDescriptorSet,
                    DstSet = _descriptorSet,
                    DescriptorType = DescriptorType.UniformBuffer,
                    DescriptorCount = 1,
                    PBufferInfo = &descriptorBufferInfo
                };
                api.UpdateDescriptorSets(device, 1, &descriptorWrite, 0, null);

                var constants = new[] { vertexPushConstantRange, fragPushConstantRange };

                fixed (PushConstantRange* constant = constants)
                {
                    var setLayout = _descriptorSetLayout;
                    var pipelineLayoutCreateInfo = new PipelineLayoutCreateInfo
                    {
                        SType = StructureType.PipelineLayoutCreateInfo,
                        PushConstantRangeCount = (uint)constants.Length,
                        PPushConstantRanges = constant,
                        SetLayoutCount = 1,
                        PSetLayouts = &setLayout
                    };

                    api.CreatePipelineLayout(device, pipelineLayoutCreateInfo, null, out _pipelineLayout)
                        .ThrowOnError();
                }


                fixed (PipelineShaderStageCreateInfo* stPtr = stages)
                {
                    var pipelineCreateInfo = new GraphicsPipelineCreateInfo
                    {
                        SType = StructureType.GraphicsPipelineCreateInfo,
                        StageCount = 2,
                        PStages = stPtr,
                        PVertexInputState = &vertextInputInfo,
                        PInputAssemblyState = &inputAssembly,
                        PViewportState = &pipelineViewPortCreateInfo,
                        PRasterizationState = &rasterizerStateCreateInfo,
                        PMultisampleState = &multisampleStateCreateInfo,
                        PDepthStencilState = &depthStencilCreateInfo,
                        PColorBlendState = &colorBlendState,
                        PDynamicState = &dynamicStateCreateInfo,
                        Layout = _pipelineLayout,
                        RenderPass = _renderPass,
                        Subpass = 0,
                        BasePipelineHandle = _pipeline.Handle != 0 ? _pipeline : new Pipeline(),
                        BasePipelineIndex = _pipeline.Handle != 0 ? 0 : -1
                    };

                    api.CreateGraphicsPipelines(device, new PipelineCache(), 1, &pipelineCreateInfo, null,
                        out _pipeline).ThrowOnError();
                }
            }
        }

        Marshal.FreeHGlobal(pname);
        _isInit = true;
    }

    private static int FindSuitableMemoryTypeIndex(Vk api, PhysicalDevice physicalDevice, uint memoryTypeBits,
        MemoryPropertyFlags flags)
    {
        api.GetPhysicalDeviceMemoryProperties(physicalDevice, out var properties);

        for (var i = 0; i < properties.MemoryTypeCount; i++)
        {
            var type = properties.MemoryTypes[i];

            if ((memoryTypeBits & (1 << i)) != 0 && type.PropertyFlags.HasFlag(flags)) return i;
        }

        return -1;
    }
}