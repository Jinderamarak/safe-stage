using System;
using System.Globalization;
using Avalonia.Data.Converters;
using BindingsCs.Safe.Types;

namespace ServiceApp.Avalonia.Converters;

public class LinearStateToStringConverter : IValueConverter
{
    public object Convert(object? value, Type targetType, object? parameter, CultureInfo culture)
    {
        if (value is LinearState linearStateValue)
            return (linearStateValue.T * 100).ToString("F1", culture);
        return string.Empty;
    }

    public object ConvertBack(object? value, Type targetType, object? parameter, CultureInfo culture)
    {
        if (value is string stringValue
            && double.TryParse(stringValue, culture, out var result))
            return new LinearState(result / 100);
        return new LinearState();
    }
}