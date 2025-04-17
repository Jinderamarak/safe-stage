﻿using Avalonia;
using Avalonia.Logging;
using Avalonia.Vulkan;

namespace GpuInterop;

public class Program
{
    private static void Main(string[] args)
    {
        BuildAvaloniaApp()
            .StartWithClassicDesktopLifetime(args);
    }

    public static AppBuilder BuildAvaloniaApp()
    {
        return AppBuilder.Configure<App>()
            .UsePlatformDetect()
            .With(new Win32PlatformOptions
            {
                RenderingMode = new[]
                {
                    Win32RenderingMode.Vulkan
                }
            })
            .With(new X11PlatformOptions() { RenderingMode = new[] { X11RenderingMode.Vulkan } })
            .With(new VulkanOptions()
            {
                VulkanInstanceCreationOptions = new VulkanInstanceCreationOptions()
                {
                    UseDebug = true
                }
            })
            .LogToTrace(LogEventLevel.Debug, "Vulkan");
    }
}