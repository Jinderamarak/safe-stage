using Avalonia;
using Avalonia.Controls;
using Avalonia.Markup.Xaml;
using BindingsCs.Safe.Types;

namespace ServiceApp.Avalonia.Config.Fields;

public partial class SixAxisField : UserControl, IField
{
    public UserControl Control => this;

    public static readonly StyledProperty<string?> LabelProperty =
        AvaloniaProperty.Register<SixAxisField, string?>(nameof(Label));

    public static readonly StyledProperty<double> XProperty =
        AvaloniaProperty.Register<SixAxisField, double>(nameof(X));

    public static readonly StyledProperty<double> YProperty =
        AvaloniaProperty.Register<SixAxisField, double>(nameof(Y));

    public static readonly StyledProperty<double> ZProperty =
        AvaloniaProperty.Register<SixAxisField, double>(nameof(Z));

    public static readonly StyledProperty<double> RxProperty =
        AvaloniaProperty.Register<SixAxisField, double>(nameof(Rx));

    public static readonly StyledProperty<double> RyProperty =
        AvaloniaProperty.Register<SixAxisField, double>(nameof(Ry));

    public static readonly StyledProperty<double> RzProperty =
        AvaloniaProperty.Register<SixAxisField, double>(nameof(Rz));

    public string? Label
    {
        get => GetValue(LabelProperty);
        init => SetValue(LabelProperty, value);
    }

    public double X
    {
        get => GetValue(XProperty);
        set => SetValue(XProperty, value);
    }

    public double Y
    {
        get => GetValue(YProperty);
        set => SetValue(YProperty, value);
    }

    public double Z
    {
        get => GetValue(ZProperty);
        set => SetValue(ZProperty, value);
    }

    public double Rx
    {
        get => GetValue(RxProperty);
        set => SetValue(RxProperty, value);
    }

    public double Ry
    {
        get => GetValue(RyProperty);
        set => SetValue(RyProperty, value);
    }

    public double Rz
    {
        get => GetValue(RzProperty);
        set => SetValue(RzProperty, value);
    }

    public SixAxisField()
    {
        DataContext = this;
        AvaloniaXamlLoader.Load(this);
    }

    public object GetValue()
    {
        return new SixAxis(X, Y, Z, Rx, Ry, Rz);
    }

    public void SetValue(object value)
    {
        if (value is SixAxis sixAxis)
        {
            X = sixAxis.X;
            Y = sixAxis.Y;
            Z = sixAxis.Z;
            Rx = sixAxis.Rx;
            Ry = sixAxis.Ry;
            Rz = sixAxis.Rz;
        }
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