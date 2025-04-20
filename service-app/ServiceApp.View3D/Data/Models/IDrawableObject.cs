using Silk.NET.Vulkan;

namespace ServiceApp.View3D.Data.Models;

internal interface IDrawableObject
{
    void Draw(Vk api, CommandBuffer cmd, PipelineLayout pipeline);
}