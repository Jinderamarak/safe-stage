namespace BindingsCs.Safe;

public static class Utility
{
    /// <inheritdoc cref="Unsafe.NativeMethods.init_logger"/>
    public static void InitializeNativeLogging()
    {
        Unsafe.NativeMethods.init_logger();
    }
}