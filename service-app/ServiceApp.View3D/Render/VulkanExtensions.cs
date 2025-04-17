using Silk.NET.Vulkan;

namespace ServiceApp.View3D.Render;

internal static class VulkanExtensions
{
    public static void ThrowOnError(this Result result)
    {
        if (result != Result.Success) throw new Exception($"Unexpected Vulkan API error \"{result}\".");
    }
}