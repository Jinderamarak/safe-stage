using Avalonia;
using Avalonia.Controls;
using Avalonia.Markup.Xaml;

namespace ServiceApp.Avalonia.Config.Fields;

public partial class DoubleField : UserControl, IField
{
    public UserControl Control => this;

    public static readonly StyledProperty<string?> LabelProperty =
        AvaloniaProperty.Register<DoubleField, string?>(nameof(Label));

    public static readonly StyledProperty<double> ValueProperty =
        AvaloniaProperty.Register<DoubleField, double>(nameof(Value));

    public string? Label
    {
        get => GetValue(LabelProperty);
        init => SetValue(LabelProperty, value);
    }

    public double Value
    {
        get => GetValue(ValueProperty);
        set => SetValue(ValueProperty, value);
    }

    public DoubleField()
    {
        DataContext = this;
        AvaloniaXamlLoader.Load(this);
    }

    public object GetValue()
    {
        return Value;
    }

    public void SetValue(object value)
    {
        if (value is double d)
            Value = d;
    }

    public object Clone()
    {
        return new DoubleField
        {
            Label = Label,
            Value = Value
        };
    }
}