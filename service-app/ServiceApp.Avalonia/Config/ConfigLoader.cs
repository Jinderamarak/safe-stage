using System;
using System.Collections.Generic;
using System.Reflection;

namespace ServiceApp.Avalonia.Config;

public static class ConfigLoader
{
    public static IEnumerable<ConfigVariant> LoadConfigurations(Type source)
    {
        var methods = source.GetMethods(BindingFlags.Static | BindingFlags.Public);
        foreach (var method in methods)
        {
            if (method.ReturnType == typeof(void)) continue;
            yield return ConfigVariant.FromMethod(source, method);
        }
    }
}