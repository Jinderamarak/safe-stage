using System;
using System.Globalization;
using Avalonia.Data.Converters;

namespace ServiceApp.Avalonia.Converters;

public class DoubleToStringConverter : IValueConverter
{
    public object Convert(object? value, Type targetType, object? parameter, CultureInfo culture)
    {
        if (value is double doubleValue)
        {
            var multiplier = 1.0;
            if (parameter is string paramText) double.TryParse(paramText, out multiplier);
            if (parameter is double paramValue) multiplier = paramValue;
            return (doubleValue * multiplier).ToString(culture);
        }

        return string.Empty;
    }

    public object ConvertBack(object? value, Type targetType, object? parameter, CultureInfo culture)
    {
        if (value is string stringValue &&
            double.TryParse(stringValue, NumberStyles.Any, culture, out var result))
        {
            var multiplier = 1.0;
            if (parameter is string paramText) double.TryParse(paramText, out multiplier);
            if (parameter is double paramValue) multiplier = paramValue;
            return result / multiplier;
        }

        return 0.0;
    }
}