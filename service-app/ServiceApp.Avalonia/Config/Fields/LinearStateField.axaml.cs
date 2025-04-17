using Avalonia;
using Avalonia.Controls;
using Avalonia.Markup.Xaml;
using BindingsCs.Safe.Types;

namespace ServiceApp.Avalonia.Config.Fields;

public partial class LinearStateField : UserControl, IField
{
    public UserControl Control => this;

    public static readonly StyledProperty<string?> LabelProperty =
        AvaloniaProperty.Register<LinearStateField, string?>(nameof(Label));

    public static readonly StyledProperty<LinearState> ValueProperty =
        AvaloniaProperty.Register<LinearStateField, LinearState>(nameof(Value));

    public string? Label
    {
        get => GetValue(LabelProperty);
        init => SetValue(LabelProperty, value);
    }

    public LinearState Value
    {
        get => GetValue(ValueProperty);
        set => SetValue(ValueProperty, value);
    }

    public LinearStateField()
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
        if (value is LinearState linear)
            Value = linear;
    }

    public object Clone()
    {
        return new LinearStateField
        {
            Label = Label,
            Value = Value
        };
    }
}