using System.Windows.Controls;
using BindingsCs.Safe.Types;
using ServiceApp.Models;
using ServiceApp.Utility;

namespace ServiceApp.Views.Fields;

public partial class SixAxisField : ReactiveUserControl, IField
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

    public double Rx
    {
        get => _fieldValue.Rx;
        set
        {
            _fieldValue.Rx = value;
            OnPropertyChanged();
        }
    }

    public double Ry
    {
        get => _fieldValue.Ry;
        set
        {
            _fieldValue.Ry = value;
            OnPropertyChanged();
        }
    }

    public double Rz
    {
        get => _fieldValue.Rz;
        set
        {
            _fieldValue.Rz = value;
            OnPropertyChanged();
        }
    }

    private string? _label;
    private SixAxis _fieldValue;

    public SixAxisField()
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
        _fieldValue = (SixAxis)value;
        OnPropertyChanged(nameof(X));
        OnPropertyChanged(nameof(Y));
        OnPropertyChanged(nameof(Z));
        OnPropertyChanged(nameof(Rx));
        OnPropertyChanged(nameof(Ry));
        OnPropertyChanged(nameof(Rz));
    }

    public object Clone()
    {
        return new SixAxisField
        {
            Label = Label,
            X = X,
            Y = Y,
            Z = Z,
            Rx = Rx,
            Ry = Ry,
            Rz = Rz
        };
    }
}