using Avalonia;
using Avalonia.Controls;
using Avalonia.Markup.Xaml;
using Avalonia.Threading;

namespace GpuInterop;

public partial class GpuDemo : UserControl
{
    private DispatcherTimer _timer;
    
    public GpuDemo()
    {
        AvaloniaXamlLoader.Load(this);
        _timer = new DispatcherTimer
        {
            Interval = TimeSpan.FromSeconds(1)
        };
        _timer.Tick += (sender, args) =>
        {
            TimeTick++;
        };
        _timer.Start();
    }
    
    private float _yaw = 0;

    public static readonly DirectProperty<GpuDemo, float> YawProperty =
        AvaloniaProperty.RegisterDirect<GpuDemo, float>("Yaw", o => o.Yaw, (o, v) => o.Yaw = v);

    public float Yaw
    {
        get => _yaw;
        set => SetAndRaise(YawProperty, ref _yaw, value);
    }

    private float _pitch = 0;

    public static readonly DirectProperty<GpuDemo, float> PitchProperty =
        AvaloniaProperty.RegisterDirect<GpuDemo, float>("Pitch", o => o.Pitch, (o, v) => o.Pitch = v);

    public float Pitch
    {
        get => _pitch;
        set => SetAndRaise(PitchProperty, ref _pitch, value);
    }


    private float _roll = 0;

    public static readonly DirectProperty<GpuDemo, float> RollProperty =
        AvaloniaProperty.RegisterDirect<GpuDemo, float>("Roll", o => o.Roll, (o, v) => o.Roll = v);

    public float Roll
    {
        get => _roll;
        set => SetAndRaise(RollProperty, ref _roll, value);
    }


    private float _disco;

    public static readonly DirectProperty<GpuDemo, float> DiscoProperty =
        AvaloniaProperty.RegisterDirect<GpuDemo, float>("Disco", o => o.Disco, (o, v) => o.Disco = v);

    public float Disco
    {
        get => _disco;
        set => SetAndRaise(DiscoProperty, ref _disco, value);
    }
    
    public static readonly DirectProperty<GpuDemo, int> TimeTickProperty =
        AvaloniaProperty.RegisterDirect<GpuDemo, int>("TimeTick", o => o.TimeTick, (o, v) => o.TimeTick = v);
    
    private int _timeTick = 0;
    public int TimeTick
    {
        get => _timeTick;
        set => SetAndRaise(TimeTickProperty, ref _timeTick, value);
    }

    private string _info = string.Empty;

    public static readonly DirectProperty<GpuDemo, string> InfoProperty =
        AvaloniaProperty.RegisterDirect<GpuDemo, string>("Info", o => o.Info, (o, v) => o.Info = v);

    public string Info
    {
        get => _info;
        set => SetAndRaise(InfoProperty, ref _info, value);
    }
    
    private bool _discoVisible;

    public static readonly DirectProperty<GpuDemo, bool> DiscoVisibleProperty =
        AvaloniaProperty.RegisterDirect<GpuDemo, bool>("DiscoVisible", o => o.DiscoVisible,
            (o, v) => o._discoVisible = v);

    public bool DiscoVisible
    {
        get => _discoVisible;
        set => SetAndRaise(DiscoVisibleProperty, ref _discoVisible, value);
    }
    
    private IGpuDemo? _demo;

    public static readonly DirectProperty<GpuDemo, IGpuDemo?> DemoProperty =
        AvaloniaProperty.RegisterDirect<GpuDemo, IGpuDemo?>("Demo", o => o.Demo,
            (o, v) => o._demo = v);

    public IGpuDemo? Demo
    {
        get => _demo;
        set => SetAndRaise(DemoProperty, ref _demo, value);
    }

    protected override void OnPropertyChanged(AvaloniaPropertyChangedEventArgs change)
    {
        if (change.Property == YawProperty
            || change.Property == PitchProperty
            || change.Property == RollProperty
            || change.Property == DiscoProperty
            || change.Property == DemoProperty
           )
        {
            if (change.Property == DemoProperty)
                ((IGpuDemo?)change.OldValue)?.Update(null, 0, 0, 0, 0, 0);
            _demo?.Update(this, Yaw, Pitch, Roll, Disco, TimeTick);
        }

        base.OnPropertyChanged(change);
    }
}

public interface IGpuDemo
{
    void Update(GpuDemo? parent, float yaw, float pitch, float roll, float disco, int timeTick);
}
