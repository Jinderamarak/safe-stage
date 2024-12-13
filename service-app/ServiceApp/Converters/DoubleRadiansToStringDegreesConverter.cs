using System.Globalization;
using System.Windows.Data;

namespace ServiceApp.Converters;

public class DoubleRadiansToStringDegreesConverter : IValueConverter
{
    public object Convert(object? value, Type targetType, object? parameter, CultureInfo culture)
    {
        if (value is double doubleValue) return (doubleValue / Math.PI * 180).ToString(culture);
        return string.Empty;
    }

    public object ConvertBack(object? value, Type targetType, object? parameter, CultureInfo culture)
    {
        if (value is string stringValue &&
            double.TryParse(stringValue, NumberStyles.Any, culture, out var result)) return result * Math.PI / 180;
        return 0.0;
    }
}