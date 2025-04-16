using Silk.NET.Vulkan;

namespace ServiceApp.Vulkan.Render;

internal static class VulkanExtensions
{
    public static void ThrowOnError(this Result result)
    {
        if (result != Result.Success) throw new Exception($"Unexpected Vulkan API error \"{result}\".");
    }
}