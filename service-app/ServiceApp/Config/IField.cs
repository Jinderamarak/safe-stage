using System.Windows.Controls;

namespace ServiceApp.Config;

public interface IField : ICloneable
{
    public UserControl Control { get; }

    public object GetValue();
    public void SetValue(object value);
}