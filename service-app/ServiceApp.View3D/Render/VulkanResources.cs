namespace ServiceApp.View3D.Render;

internal class VulkanResources : IAsyncDisposable
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