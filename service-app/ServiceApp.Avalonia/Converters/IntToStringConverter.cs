using System;
using System.Globalization;
using Avalonia.Data.Converters;

namespace ServiceApp.Avalonia.Converters;

public class IntToStringConverter : IValueConverter
{
    public object Convert(object? value, Type targetType, object? parameter, CultureInfo culture)
    {
        if (value is int intValue) return intValue.ToString(culture);

        return string.Empty;
    }

    public object ConvertBack(object? value, Type targetType, object? parameter, CultureInfo culture)
    {
        if (value is string stringValue &&
            int.TryParse(stringValue, NumberStyles.Any, culture, out var result))
            return result;
        return 0;
    }
}