﻿using Avalonia;
using System;
using Avalonia.Vulkan;

namespace ServiceApp.Avalonia;

internal class Program
{
    // Initialization code. Don't use any Avalonia, third-party APIs or any
    // SynchronizationContext-reliant code before AppMain is called: things aren't initialized
    // yet and stuff might break.
    [STAThread]
    public static void Main(string[] args)
    {
        BuildAvaloniaApp()
            .StartWithClassicDesktopLifetime(args);
    }

    // Avalonia configuration, don't remove; also used by visual designer.
    public static AppBuilder BuildAvaloniaApp()
        => AppBuilder.Configure<App>()
            .UsePlatformDetect()
            .WithInterFont()
            .With(new Win32PlatformOptions
            {
                RenderingMode = new[]
                {
                    Win32RenderingMode.Vulkan
                }
            })
            .With(new X11PlatformOptions
            {
                RenderingMode = new[]
                {
                    X11RenderingMode.Vulkan
                }
            })
            .With(new VulkanOptions()
            {
                VulkanInstanceCreationOptions = new VulkanInstanceCreationOptions
                {
#if DEBUG
                    UseDebug = true
#endif
                }
            })
            .LogToTrace();
}