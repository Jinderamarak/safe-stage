using System.Windows.Controls;
using BindingsCs.Safe.Types;
using ServiceApp.Config;
using ServiceApp.Utility;

namespace ServiceApp.Config.Fields;

public partial class VectorField : ReactiveUserControl, IField
{
    public UserControl Control => this;

    public string? Label
    {
        get => _label;
        init => SetField(ref _label, value);
    }

    public double X
    {
        get => _fieldValue.X;
        set
        {
            _fieldValue.X = value;
            OnPropertyChanged();
        }
    }

    public double Y
    {
        get => _fieldValue.Y;
        set
        {
            _fieldValue.Y = value;
            OnPropertyChanged();
        }
    }

    public double Z
    {
        get => _fieldValue.Z;
        set
        {
            _fieldValue.Z = value;
            OnPropertyChanged();
        }
    }

    private string? _label;
    private Vector3 _fieldValue;

    public VectorField()
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
        _fieldValue = (Vector3)value;
        OnPropertyChanged(nameof(X));
        OnPropertyChanged(nameof(Y));
        OnPropertyChanged(nameof(Z));
    }

    public object Clone()
    {
        return new VectorField
        {
            Label = Label,
            X = X,
            Y = Y,
            Z = Z
        };
    }
}