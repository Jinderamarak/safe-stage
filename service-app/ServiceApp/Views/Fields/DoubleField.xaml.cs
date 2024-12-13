using System.Windows.Controls;
using ServiceApp.Models;
using ServiceApp.Utility;

namespace ServiceApp.Views.Fields;

public partial class DoubleField : ReactiveUserControl, IField
{
    public UserControl Control => this;

    public string? Label
    {
        get => _label;
        init => SetField(ref _label, value);
    }

    public double Value
    {
        get => _value;
        set => SetField(ref _value, value);
    }

    private string? _label;
    private double _value;

    public DoubleField()
    {
        DataContext = this;
        InitializeComponent();
    }

    public object GetValue()
    {
        return Value;
    }

    public void SetValue(object value)
    {
        Value = (double)value;
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