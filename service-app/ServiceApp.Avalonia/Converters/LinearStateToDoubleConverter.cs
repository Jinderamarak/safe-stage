using System;
using System.Globalization;
using Avalonia.Data.Converters;
using BindingsCs.Safe.Types;

namespace ServiceApp.Avalonia.Converters;

public class LinearStateToDoubleConverter : IValueConverter
{
    public object Convert(object? value, Type targetType, object? parameter, CultureInfo culture)
    {
        if (value is LinearState linearStateValue)
            return linearStateValue.T;
        return 0.0;
    }

    public object ConvertBack(object? value, Type targetType, object? parameter, CultureInfo culture)
    {
        if (value is double doubleValue)
            return new LinearState(doubleValue);
        return new LinearState();
    }
}