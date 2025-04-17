using System;
using Avalonia.Controls;

namespace ServiceApp.Avalonia.Config;

public interface IField : ICloneable
{
    public UserControl Control { get; }

    public object GetValue();
    public void SetValue(object value);
}