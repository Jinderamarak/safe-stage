using System;
using System.Threading.Tasks;
using Avalonia;
using Avalonia.Rendering.Composition;
using BindingsCs.Safe;
using BindingsCs.Safe.Configurations;
using BindingsCs.Safe.Types;

namespace GpuInterop.VulkanDemo;

public class VulkanDemoControl : DrawingSurfaceDemoBase
{
    class VulkanResources : IAsyncDisposable
    {
        public VulkanContext Context { get; }
        public VulkanSwapchain Swapchain { get; }
        public VulkanContent Content { get; }

        public VulkanResources(VulkanContext context, VulkanSwapchain swapchain, VulkanContent content)
        {
            Context = context;
            Swapchain = swapchain;
            Content = content;
        }
        public async ValueTask DisposeAsync()
        {
            Context.Pool.FreeUsedCommandBuffers();
            Content.Dispose();
            await Swapchain.DisposeAsync();
            Context.Dispose();
        }
    }

    protected override bool SupportsDisco => true;

    private VulkanResources? _resources;

    private Microscope _microscope;
    private Timer _timer;
    
    public VulkanDemoControl()
    {
        using var configBuilder = new ConfigurationBuilder();
        var config = configBuilder.WithChamber(ChamberConfig.ThesisChamber())
            .WithStage(StageConfig.ThesisStage(), ResolverStageConfig.Linear(new SixAxis()))
            .Build();
        
        _microscope = Microscope.FromConfiguration(config);

        var holder = HolderConfig.ThesisHolderCircle();
        _microscope.UpdateHolder(holder);
        
        _microscope.UpdateResolvers();

        var counter = 0;
        _timer = new Timer(state =>
            {
                if (_resources is null)
                    return;
                
                counter++;
                var position = new SixAxis(0, 0, 0, 0, 0, double.DegreesToRadians(counter * 1 % 360));
                
                var stage = _microscope.PresentStageAt(position);
                if (_resources?.Content != null)
                {
                    _resources.Content.UpdateStage(stage);
                }
            },
            null,
            TimeSpan.FromSeconds(1),
            TimeSpan.FromMilliseconds(1)
        );
    }

    protected override (bool success, string info) InitializeGraphicsResources(Compositor compositor,
        CompositionDrawingSurface compositionDrawingSurface, ICompositionGpuInterop gpuInterop)
    {
        var (context, info) = VulkanContext.TryCreate(gpuInterop);
        if (context == null)
            return (false, info);
        try
        {
            var content = new VulkanContent(context);
            _resources = new VulkanResources(context,
                new VulkanSwapchain(context, gpuInterop, compositionDrawingSurface), content);

            var stage = _microscope.PresentStage();
            content.UpdateStage(stage);
            
            return (true, info);
        }
        catch(Exception e)
        {
            return (false, e.ToString());
        }
    }

    protected override void FreeGraphicsResources()
    {
        _resources?.DisposeAsync();
        _resources = null;
    }

    protected override unsafe void RenderFrame(PixelSize pixelSize)
    {
        if (_resources == null)
            return;
        using (_resources.Swapchain.BeginDraw(pixelSize, out var image))
        {
            _resources.Content.Render(image, Yaw, Pitch, Roll, Disco, TimeTick);
        }
    }
}
