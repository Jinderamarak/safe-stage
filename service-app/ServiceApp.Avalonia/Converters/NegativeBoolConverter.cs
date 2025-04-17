using System;
using System.Globalization;
using Avalonia.Data.Converters;

namespace ServiceApp.Avalonia.Converters;

public class NegativeBoolConverter : IValueConverter
{
    public object Convert(object? value, Type targetType, object? parameter, CultureInfo culture)
    {
        if (value is bool boolValue) return !boolValue;
        return default(bool);
    }

    public object ConvertBack(object? value, Type targetType, object? parameter, CultureInfo culture)
    {
        if (value is bool boolValue) return !boolValue;
        return default(bool);
    }
}