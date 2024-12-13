using System.Windows;

namespace ServiceApp;

/// <summary>
/// Interaction logic for App.xaml
/// </summary>
public partial class App : Application
{
    protected override void OnStartup(StartupEventArgs e)
    {
        base.OnStartup(e);
        BindingsCs.Safe.Utility.InitializeNativeLogging();
    }
}