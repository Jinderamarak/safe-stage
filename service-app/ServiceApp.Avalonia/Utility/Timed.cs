using System;
using System.Diagnostics;

namespace ServiceApp.Avalonia.Utility;

public static class Timed
{
    public static (T, Stopwatch) Run<T>(Func<T> func)
    {
        var stopwatch = new Stopwatch();
        stopwatch.Start();
        var result = func();
        stopwatch.Stop();
        return (result, stopwatch);
    }
}