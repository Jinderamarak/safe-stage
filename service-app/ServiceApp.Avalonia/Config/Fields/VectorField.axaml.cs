using Avalonia;
using Avalonia.Controls;
using Avalonia.Markup.Xaml;
using BindingsCs.Safe.Types;

namespace ServiceApp.Avalonia.Config.Fields;

public partial class VectorField : UserControl, IField
{
    public UserControl Control => this;

    public static readonly StyledProperty<string?> LabelProperty =
        AvaloniaProperty.Register<VectorField, string?>(nameof(Label));

    public static readonly StyledProperty<double> XProperty =
        AvaloniaProperty.Register<VectorField, double>(nameof(X));

    public static readonly StyledProperty<double> YProperty =
        AvaloniaProperty.Register<VectorField, double>(nameof(Y));

    public static readonly StyledProperty<double> ZProperty =
        AvaloniaProperty.Register<VectorField, double>(nameof(Z));

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

    public VectorField()
    {
        DataContext = this;
        AvaloniaXamlLoader.Load(this);
    }

    public object GetValue()
    {
        return new Vector3(X, Y, Z);
    }

    public void SetValue(object value)
    {
        if (value is Vector3 vector)
        {
            X = vector.X;
            Y = vector.Y;
            Z = vector.Z;
        }
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