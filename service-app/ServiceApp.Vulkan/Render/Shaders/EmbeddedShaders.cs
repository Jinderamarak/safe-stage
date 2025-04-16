using shaderc;

namespace ServiceApp.Vulkan.Render.Shaders;

internal static class EmbeddedShaders
{
    public static byte[] LoadVertexShader()
    {
        return ShaderLoader.LoadCompiledShader("ServiceApp.Vulkan.Assets.Shaders.vert.glsl",
            ShaderKind.GlslVertexShader);
    }

    public static byte[] LoadDiffuseShader()
    {
        return ShaderLoader.LoadCompiledShader("ServiceApp.Vulkan.Assets.Shaders.diffuse.glsl",
            ShaderKind.GlslFragmentShader);
    }
}