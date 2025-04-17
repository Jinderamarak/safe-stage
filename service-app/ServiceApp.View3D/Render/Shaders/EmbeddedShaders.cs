using shaderc;

namespace ServiceApp.View3D.Render.Shaders;

internal static class EmbeddedShaders
{
    public static byte[] LoadVertexShader()
    {
        return ShaderLoader.LoadCompiledShader("ServiceApp.View3D.Assets.Shaders.vert.glsl",
            ShaderKind.GlslVertexShader);
    }

    public static byte[] LoadDiffuseShader()
    {
        return ShaderLoader.LoadCompiledShader("ServiceApp.View3D.Assets.Shaders.diffuse.glsl",
            ShaderKind.GlslFragmentShader);
    }
}