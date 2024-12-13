using System.Windows.Controls;
using BindingsCs.Safe.Types;
using ServiceApp.Models;
using ServiceApp.Utility;

namespace ServiceApp.Views.Fields;

public partial class LinearStateField : ReactiveUserControl, IField
{
    public UserControl Control => this;

    public string? Label
    {
        get => _label;
        init => SetField(ref _label, value);
    }

    public double Value
    {
        get => _fieldValue.T;
        set
        {
            _fieldValue.T = Math.Clamp(value, 0, 1);
            OnPropertyChanged();
        }
    }

    private string? _label;
    private LinearState _fieldValue;

    public LinearStateField()
    {
        DataContext = this;
        InitializeComponent();
    }

    public object GetValue()
    {
        return _fieldValue;
    }

    public void SetValue(object value)
    {
        _fieldValue = (LinearState)value;
        OnPropertyChanged(nameof(Value));
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