namespace BindingsCs.Safe;

public static class Utility
{
    public static void InitializeNativeLogging()
    {
        Unsafe.NativeMethods.init_logger();
    }
}