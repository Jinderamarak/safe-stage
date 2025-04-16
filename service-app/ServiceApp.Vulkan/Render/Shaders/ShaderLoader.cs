using System.Runtime.InteropServices;
using shaderc;

namespace ServiceApp.Vulkan.Render.Shaders;

internal static class ShaderLoader
{
    public static byte[] LoadCompiledShader(string resourceName, ShaderKind kind)
    {
        var source = LoadEmbeddedContent(resourceName);
        return CompileShader(source, resourceName, kind);
    }

    private static string LoadEmbeddedContent(string resourceName)
    {
        using var stream = typeof(ShaderLoader).Assembly.GetManifestResourceStream(resourceName);
        if (stream is null)
            throw new ArgumentException("Resource not found", nameof(resourceName));

        using var reader = new StreamReader(stream);
        return reader.ReadToEnd();
    }

    private static byte[] CompileShader(string source, string name, ShaderKind kind)
    {
        using var compiler = new Compiler();
        using var result = compiler.Compile(source, name, kind);
        if (result.Status != Status.Success)
            throw new Exception($"Shader compilation failed: {result.ErrorMessage}");

        var data = new byte[result.CodeLength];
        Marshal.Copy(result.CodePointer, data, 0, (int)result.CodeLength);
        return data;
    }
}