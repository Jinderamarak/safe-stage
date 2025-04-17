using System;
using System.Collections.Generic;
using System.Collections.ObjectModel;
using System.Linq;
using System.Reactive.Linq;
using System.Threading.Tasks;
using Avalonia;
using Avalonia.Controls;
using Avalonia.Controls.Primitives;
using Avalonia.Input;
using Avalonia.Interactivity;
using Avalonia.Markup.Xaml;
using Avalonia.Rendering;
using Avalonia.Threading;
using BindingsCs.Safe;
using BindingsCs.Safe.Configurations;
using BindingsCs.Safe.Types;
using MsBox.Avalonia;
using MsBox.Avalonia.Enums;
using ServiceApp.Avalonia.Config;
using ServiceApp.Avalonia.Config.Fields;
using ServiceApp.Avalonia.Config.Presets;
using ServiceApp.Avalonia.Tasks;
using ServiceApp.Avalonia.Utility;
using ServiceApp.Avalonia.Utility.Extensions;
using ServiceApp.View3D.Controls;
using Vertex = System.Numerics.Vector3;
using MsgBoxIcon = MsBox.Avalonia.Enums.Icon;

namespace ServiceApp.Avalonia.Windows;

/// <summary>
///     Interaction logic for MainWindow.xaml
/// </summary>
public partial class MainWindow : Window, IDisposable
{
    public static readonly StyledProperty<string> StatusTextProperty =
        AvaloniaProperty.Register<MainWindow, string>(nameof(StatusText));

    public static readonly StyledProperty<bool> IsUpdatingProperty =
        AvaloniaProperty.Register<MainWindow, bool>(nameof(IsUpdating));

    public static readonly DirectProperty<MainWindow, bool> HasConfigurationProperty =
        AvaloniaProperty.RegisterDirect<MainWindow, bool>(
            nameof(HasConfiguration),
            o => o.HasConfiguration);

    public static readonly StyledProperty<string> ConfigurationTitleProperty =
        AvaloniaProperty.Register<MainWindow, string>(nameof(ConfigurationTitle));

    public static readonly StyledProperty<string> HolderTitleProperty =
        AvaloniaProperty.Register<MainWindow, string>(nameof(HolderTitle));

    public static readonly StyledProperty<Id?> SelectedRetractProperty =
        AvaloniaProperty.Register<MainWindow, Id?>(nameof(SelectedRetract));

    public static readonly DirectProperty<MainWindow, bool> HasPathProperty =
        AvaloniaProperty.RegisterDirect<MainWindow, bool>(
            nameof(HasPath),
            o => o.HasPath);

    public static readonly DirectProperty<MainWindow, int> StageInterpolationMaxProperty =
        AvaloniaProperty.RegisterDirect<MainWindow, int>(
            nameof(StageInterpolationMax),
            o => o.StageInterpolationMax);

    public static readonly StyledProperty<int> ObstructionLevelProperty =
        AvaloniaProperty.Register<MainWindow, int>(nameof(ObstructionLevel));

    public string StatusText
    {
        get => GetValue(StatusTextProperty);
        set => SetValue(StatusTextProperty, value);
    }

    public bool IsUpdating
    {
        get => GetValue(IsUpdatingProperty) || _microscope is null;
        set => SetValue(IsUpdatingProperty, value);
    }

    public bool HasConfiguration => _microscope is not null;

    public string ConfigurationTitle
    {
        get => GetValue(ConfigurationTitleProperty);
        set => SetValue(ConfigurationTitleProperty, value);
    }

    public string HolderTitle
    {
        get => GetValue(HolderTitleProperty);
        set => SetValue(HolderTitleProperty, value);
    }

    public ObservableCollection<Id> Retracts { get; } = new();

    public Id? SelectedRetract
    {
        get => GetValue(SelectedRetractProperty);
        set => SetValue(SelectedRetractProperty, value);
    }

    public bool HasPath => (_interestedInStage && _lastStagePath is not null)
                           || (!_interestedInStage && _lastRetractPath is not null);

    public int StageInterpolationMax => _interestedInStage
        ? _lastStagePath?.Nodes.Count() - 1 ?? 0
        : _lastRetractPath?.Nodes.Count() - 1 ?? 0;

    public int ObstructionLevel
    {
        get => GetValue(ObstructionLevelProperty);
        set => SetValue(ObstructionLevelProperty, value);
    }

    private bool _interestedInStage;
    private PathResult<SixAxis>? _lastStagePath;
    private PathResult<LinearState>? _lastRetractPath;
    private Id? _lastRetractId;

    private Preset? _lastPreset;
    private ConfigVariant? _lastHolder;
    private Microscope? _microscope;

    private readonly Dictionary<Id, int> _retractIdToGeometryIndex = new();

    private readonly OrbitControls _orbitControls;

    private Slider PathInterpolationSlider { get; }
    private ModelGroup StageModelGroup { get; }
    private ModelGroup StaticModelGroup { get; }
    private ModelGroup RetractModelGroup { get; }
    private ModelGroup PathModelGroup { get; }
    private SixAxisField StageCurrent { get; }
    private SixAxisField StageTarget { get; }

    public MainWindow()
    {
        DataContext = this;
        AvaloniaXamlLoader.Load(this);

#if DEBUG
        this.AttachDevTools();
        RendererDiagnostics.DebugOverlays = RendererDebugOverlays.Fps;
#endif

        PathInterpolationSlider = this.Find<Slider>("PathInterpolationSliderAva")!;
        StageModelGroup = this.Find<ModelGroup>("StageModelGroupAva")!;
        StaticModelGroup = this.Find<ModelGroup>("StaticModelGroupAva")!;
        RetractModelGroup = this.Find<ModelGroup>("RetractModelGroupAva")!;
        PathModelGroup = this.Find<ModelGroup>("PathModelGroupAva")!;
        StageCurrent = this.Find<SixAxisField>("StageCurrentAva")!;
        StageTarget = this.Find<SixAxisField>("StageTargetAva")!;

        var camera = this.Find<Camera3D>("CameraAva")!;
        var light = this.Find<PointLight>("LightAva")!;

        _orbitControls = new OrbitControls(
            camera, light,
            2, 1.2, -0.9)
        {
            ScrollSensitivity = 1e-3,
            DragSensitivity = 5e-3
        };

        Observable.FromEventPattern<EventHandler<RangeBaseValueChangedEventArgs>, RangeBaseValueChangedEventArgs>(
                h => PathInterpolationSlider.ValueChanged += h,
                h => PathInterpolationSlider.ValueChanged -= h)
            .Select(args => args.EventArgs.NewValue)
            .Buffer(TimeSpan.FromMilliseconds(100))
            .Where(values => values.Count > 0)
            .Select(values => values.Last())
            .Subscribe(OnPathInterpolationChanged);
    }

    public void Dispose()
    {
        _microscope?.Dispose();
    }

    private void RunTaskChain(TaskChain chain)
    {
        chain.Execute(Dispatcher.UIThread);
    }

    private void RunUpdatingChain(TaskChain chain)
    {
        IsUpdating = true;
        chain.OnUi(() => IsUpdating = false)
            .Execute(Dispatcher.UIThread);
    }

    private void OnViewportMouseMove(object? sender, PointerEventArgs pointerEventArgs)
    {
        _orbitControls.MouseMoved(sender, pointerEventArgs);
    }

    private void OnViewportMouseWheel(object? sender, PointerWheelEventArgs pointerWheelEventArgs)
    {
        _orbitControls.MouseScrolled(pointerWheelEventArgs);
    }

    private TaskChain UpdateStageGeometry(SixAxis? state = null)
    {
        return new TaskChain()
            .InBack(() =>
            {
                var models =
                    state.HasValue
                        ? _microscope!.PresentStageAt(state.Value)
                        : _microscope!.PresentStage();
                return models.Select(Shapes.TrianglesToPointsList).ToList();
            })
            .OnUi((List<IEnumerable<Vertex>> positions) =>
            {
                if (StageModelGroup.Models.Count != positions.Count)
                {
                    StageModelGroup.Models.Clear();
                    foreach (var _ in positions)
                        StageModelGroup.Models.Add(new GeometryModel());
                }

                var materials = Materials.StageRange(positions.Count).ToList();
                for (var i = 0; i < positions.Count; i++)
                {
                    StageModelGroup.Models[i].Vertices = positions[i];
                    StageModelGroup.Models[i].Color = materials[i];
                }
            });
    }

    private TaskChain UpdateStaticGeometry()
    {
        var level = ObstructionLevel;
        return new TaskChain()
            .InBack(() =>
            {
                var models = level switch
                {
                    0 => _microscope!.PresentStaticNonObstructive(),
                    1 => _microscope!.PresentStaticLessObstructive(),
                    _ => _microscope!.PresentStaticFull()
                };
                return models.Select(Shapes.TrianglesToPointsList).ToList();
            })
            .OnUi((List<IEnumerable<Vertex>> positions) =>
            {
                if (StaticModelGroup.Models.Count != positions.Count)
                {
                    StaticModelGroup.Models.Clear();
                    foreach (var _ in positions)
                        StaticModelGroup.Models.Add(new GeometryModel());
                }


                var materials = Materials.StaticRange(positions.Count).ToList();
                for (var i = 0; i < positions.Count; i++)
                {
                    StaticModelGroup.Models[i].Vertices = positions[i];
                    StaticModelGroup.Models[i].Color = materials[i];
                }
            });
    }

    private void InitializeRetractMapping()
    {
        _retractIdToGeometryIndex.Clear();
        RetractModelGroup.Models.Clear();

        for (var i = 0; i < Retracts.Count; i++)
        {
            var id = Retracts[i];
            _retractIdToGeometryIndex[id] = i;
            RetractModelGroup.Groups.Add(new ModelGroup());
        }
    }

    private TaskChain UpdateRetractGeometry(Id id, LinearState? state = null)
    {
        return new TaskChain()
            .InBack(() =>
            {
                var index = _retractIdToGeometryIndex[id];
                var model =
                    state.HasValue
                        ? _microscope!.PresentRetractAt(id, state.Value)
                        : _microscope!.PresentRetract(id);
                var positions = model.Select(Shapes.TrianglesToPointsList).ToList();
                return (index, positions);
            })
            .OnUi(((int, List<IEnumerable<Vertex>>) pair) =>
            {
                var group = RetractModelGroup.Groups[pair.Item1];
                if (group.Models.Count != pair.Item2.Count)
                {
                    group.Models.Clear();
                    foreach (var _ in pair.Item2)
                        group.Models.Add(new GeometryModel());
                }

                var materials = Materials.RetractRange(pair.Item1, pair.Item2.Count).ToList();
                for (var i = 0; i < pair.Item2.Count; i++)
                {
                    group.Models[i].Vertices = pair.Item2[i];
                    group.Models[i].Color = materials[i];
                }
            });
    }

    private async void OnConfigureMicroscope(object? sender, RoutedEventArgs routedEventArgs)
    {
        var window = new MicroscopeConfiguration();
        if (_lastPreset is not null) window.ApplyPreset(_lastPreset);

        if (await window.ShowDialog<bool>(this))
        {
            var task = new TaskChain()
                .OnUi(() => StatusText = "Building microscope configuration ...")
                .OnUi(() =>
                {
                    _microscope?.Dispose();
                    _microscope = ConfigBuilder.BuildMicroscope(window.ChamberVariant!,
                        window.StageVariant!,
                        window.StageResolverVariant!,
                        window.Equipment.AsReadOnly(),
                        window.Retracts.AsReadOnly());
                })
                .OnUi(() => StatusText = "Updating resolvers ...")
                .InBack(() =>
                {
                    try
                    {
                        _microscope!.UpdateResolvers();
                    }
                    catch (Exception ex)
                    {
                        return ex;
                    }

                    return null;
                })
                .OnUi((object? result) =>
                {
                    if (result is Exception ex)
                        Task.WaitAll(
                            MessageBoxManager.GetMessageBoxStandard("Cannot Change Configuration", ex.Message,
                                    ButtonEnum.Ok, MsgBoxIcon.Error)
                                .ShowAsync()
                        );
                })
                .Chain(UpdateStageGeometry())
                .Chain(UpdateStaticGeometry())
                .OnUi(() =>
                {
                    _lastPreset = new Preset
                    {
                        Name = "Last Preset",
                        ChamberVariant = window.ChamberVariant!.Cloned(),
                        StageVariant = window.StageVariant!.Cloned(),
                        StageResolverVariant = window.StageResolverVariant!.Cloned(),
                        Equipment = window.Equipment.Select(e => e.Equipment.Cloned()).ToList(),
                        Retracts = window.Retracts.Select(e => (e.Id, e.Retract.Cloned(), e.Resolver!.Cloned()))
                            .ToList()
                    };
                    _lastRetractId = null;

                    Retracts.Clear();
                    var retractIds = window.Retracts.Select(e => e.Id);
                    foreach (var id in retractIds) Retracts.Add(id);

                    ConfigurationTitle = $"With {Retracts.Count} Retracts";
                    HolderTitle = "No Holder";
                    SelectedRetract = null;

                    RaisePropertyChanged(HasConfigurationProperty, HasConfiguration, HasConfiguration);
                    RaisePropertyChanged(HasPathProperty, HasPath, HasPath);
                    RaisePropertyChanged(StageInterpolationMaxProperty, StageInterpolationMax, StageInterpolationMax);

                    InitializeRetractMapping();
                    StatusText = "Microscope configuration updated";
                })
                .InBack(() =>
                {
                    var updateRetracts = new TaskChain();
                    foreach (var id in Retracts)
                        updateRetracts.Chain(UpdateRetractGeometry(id));
                    RunTaskChain(updateRetracts);
                });
            RunUpdatingChain(task);
        }
    }

    private async void OnChangeHolder(object sender, RoutedEventArgs e)
    {
        var holderDialog = new SelectHolderWindow
        {
            SelectedHolder = _lastHolder
        };

        if (await holderDialog.ShowDialog<bool>(this))
        {
            var task = new TaskChain()
                .OnUi(() => StatusText = "Updating holder and resolvers ...")
                .InBack(() =>
                {
                    if (holderDialog.SelectedHolder is not null)
                    {
                        _lastHolder = holderDialog.SelectedHolder;
                        var holder = holderDialog.SelectedHolder.Construct<HolderConfig>();
                        _microscope!.UpdateHolder(holder!);
                    }
                    else
                    {
                        _lastHolder = null;
                        _microscope!.RemoveHolder();
                    }

                    try
                    {
                        _microscope!.UpdateResolvers();
                    }
                    catch (Exception ex)
                    {
                        return ex;
                    }

                    return null;
                })
                .OnUi((object? result) =>
                {
                    if (result is Exception ex)
                        Task.WaitAll(
                            MessageBoxManager.GetMessageBoxStandard("Cannot Change Holder", ex.Message, ButtonEnum.Ok,
                                MsgBoxIcon.Error).ShowAsync()
                        );
                })
                .Chain(UpdateStageGeometry())
                .OnUi(() => HolderTitle = _lastHolder is not null ? _lastHolder.ToString() : "No Holder")
                .OnUi(() => StatusText = "Holder changed");
            RunUpdatingChain(task);
        }
    }

    private async void OnChangeSampleHeight(object sender, RoutedEventArgs e)
    {
        var dialog = new HeightMapWindow();
        var result = await dialog.ShowDialog<bool>(this);

        var task = new TaskChain()
            .OnUi(() => StatusText = "Updating sample height map and resolvers ...");

        IsUpdating = true;
        if (result && !dialog.Cleared)
            task.InBack(() =>
                {
                    _microscope!.UpdateSampleHeightMap(
                        dialog.HeightMap,
                        (nuint)dialog.ResolutionX,
                        (nuint)dialog.ResolutionY,
                        dialog.RealX,
                        dialog.RealY
                    );

                    try
                    {
                        _microscope!.UpdateResolvers();
                    }
                    catch (Exception ex)
                    {
                        return ex;
                    }

                    return null;
                })
                .OnUi((object? result) =>
                {
                    if (result is Exception ex)
                        Task.WaitAll(
                            MessageBoxManager.GetMessageBoxStandard("Cannot Change Sample", ex.Message, ButtonEnum.Ok,
                                    MsgBoxIcon.Error)
                                .ShowAsync()
                        );
                })
                .Chain(UpdateStageGeometry())
                .OnUi(() => { StatusText = "Sample height map updated"; });
        else if (result && dialog.Cleared)
            task.InBack(() =>
                {
                    _microscope!.ClearSample();
                    try
                    {
                        _microscope!.UpdateResolvers();
                    }
                    catch (Exception ex)
                    {
                        return ex;
                    }

                    return null;
                })
                .OnUi((object? result) =>
                {
                    if (result is Exception ex)
                        Task.WaitAll(
                            MessageBoxManager.GetMessageBoxStandard("Cannot Clear Sample", ex.Message, ButtonEnum.Ok,
                                    MsgBoxIcon.Error)
                                .ShowAsync()
                        );
                })
                .Chain(UpdateStageGeometry())
                .OnUi(() => { StatusText = "Sample height map cleared"; });
        else
            task.OnUi(() => { StatusText = "Sample height map not updated"; });
        RunUpdatingChain(task);
    }

    private void StageUpdateCurrent(object sender, RoutedEventArgs e)
    {
        var current = (SixAxis)StageCurrent.GetValue();
        var task = new TaskChain()
            .OnUi(() => StatusText = "Updating stage state and resolvers ...")
            .InBack(() =>
            {
                try
                {
                    _microscope!.UpdateStageState(current);
                    _microscope!.UpdateResolvers();
                    _lastStagePath = null;
                    return null;
                }
                catch (Exception ex)
                {
                    return ex;
                }
            })
            .OnUi((Exception? ex) =>
            {
                RaisePropertyChanged(HasPathProperty, HasPath, HasPath);
                RaisePropertyChanged(StageInterpolationMaxProperty, StageInterpolationMax, StageInterpolationMax);
                StatusText = $"Stage state updated to {current.ToFormattedString()}";

                if (ex is not null)
                    Task.WaitAll(
                        MessageBoxManager
                            .GetMessageBoxStandard("Cannot Stage State", ex.Message, ButtonEnum.Ok, MsgBoxIcon.Error)
                            .ShowAsync()
                    );
            })
            .Chain(UpdateStageGeometry());
        RunUpdatingChain(task);
    }

    private void StageFindPath(object sender, RoutedEventArgs e)
    {
        var target = (SixAxis)StageTarget.GetValue();
        var task = new TaskChain()
            .OnUi(() => StatusText = "Finding stage path ...")
            .InBack(() =>
            {
                try
                {
                    (_lastStagePath, var stopwatch) = Timed.Run(() => _microscope!.FindStagePath(target));
                    _interestedInStage = true;
                    return (object)stopwatch.ElapsedMilliseconds;
                }
                catch (Exception ex)
                {
                    return ex;
                }
            })
            .OnUi((object? result) =>
            {
                if (result is long time)
                {
                    RaisePropertyChanged(HasPathProperty, HasPath, HasPath);
                    RaisePropertyChanged(StageInterpolationMaxProperty, StageInterpolationMax, StageInterpolationMax);

                    StatusText = $"Path found in {time} ms. {_lastStagePath?.ToStatusMessage()}";
                    _lastStagePath?.ShowMessageBox();

                    PathModelGroup.Models.Clear();
                    foreach (var geometry in Shapes.CreatePathGeometries(_lastStagePath!.Nodes.ToList()))
                        PathModelGroup.Models.Add(geometry);
                }
                else if (result is Exception ex)
                {
                    Task.WaitAll(
                        MessageBoxManager.GetMessageBoxStandard("Invalid Stage Target", ex.Message, ButtonEnum.Ok,
                                MsgBoxIcon.Error)
                            .ShowAsync()
                    );
                }
            });
        RunUpdatingChain(task);
    }

    private void UpdateRetractState(Id id, LinearState state)
    {
        var task = new TaskChain()
            .OnUi(() => StatusText = $"Updating retract {id} and resolvers ...")
            .InBack(() =>
            {
                try
                {
                    _microscope!.UpdateRetractState(id, state);
                    _microscope!.UpdateResolvers();
                    _lastRetractPath = null;
                    return null;
                }
                catch (Exception ex)
                {
                    return ex;
                }
            })
            .OnUi((Exception? ex) =>
            {
                RaisePropertyChanged(HasPathProperty, HasPath, HasPath);
                RaisePropertyChanged(StageInterpolationMaxProperty, StageInterpolationMax, StageInterpolationMax);
                StatusText = $"Retract {id} updated";

                if (ex is not null)
                    Task.WaitAll(
                        MessageBoxManager.GetMessageBoxStandard("Invalid Retract State", ex.Message, ButtonEnum.Ok,
                                MsgBoxIcon.Error)
                            .ShowAsync()
                    );
            })
            .Chain(UpdateRetractGeometry(id));
        RunUpdatingChain(task);
    }

    private void OnRetractStateInserted(object sender, RoutedEventArgs e)
    {
        UpdateRetractState(SelectedRetract!.Value, LinearState.Full());
    }

    private void OnRetractStateRetracted(object sender, RoutedEventArgs e)
    {
        UpdateRetractState(SelectedRetract!.Value, LinearState.None());
    }

    private void RetractFindPath(Id id, LinearState target)
    {
        var task = new TaskChain()
            .OnUi(() => StatusText = "Finding retract path ...")
            .InBack(() =>
            {
                try
                {
                    (_lastRetractPath, var stopwatch) = Timed.Run(() => _microscope!.FindRetractPath(id, target));
                    _interestedInStage = false;
                    _lastRetractId = id;
                    return (object)stopwatch.ElapsedMilliseconds;
                }
                catch (Exception ex)
                {
                    return ex;
                }
            })
            .OnUi((object? result) =>
            {
                if (result is long time)
                {
                    RaisePropertyChanged(HasPathProperty, HasPath, HasPath);
                    RaisePropertyChanged(StageInterpolationMaxProperty, StageInterpolationMax, StageInterpolationMax);

                    StatusText = $"Path found in {time} ms. {_lastRetractPath?.ToStatusMessage()}";
                    _lastRetractPath?.ShowMessageBox();

                    PathModelGroup.Models.Clear();
                }
                else if (result is Exception ex)
                {
                    Task.WaitAll(
                        MessageBoxManager.GetMessageBoxStandard("Invalid Retract Target", ex.Message, ButtonEnum.Ok,
                                MsgBoxIcon.Error)
                            .ShowAsync()
                    );
                }
            });
        RunUpdatingChain(task);
    }

    private void OnRetractFindInserted(object sender, RoutedEventArgs e)
    {
        RetractFindPath(SelectedRetract!.Value, LinearState.Full());
    }

    private void OnRetractFindRetracted(object sender, RoutedEventArgs e)
    {
        RetractFindPath(SelectedRetract!.Value, LinearState.None());
    }

    private void OnPathInterpolationChanged(double interpolation)
    {
        var task = new TaskChain();
        if (_interestedInStage && _lastStagePath is not null)
        {
            var state = Maths.PathInterpolation(_lastStagePath, interpolation);
            task.Chain(UpdateStageGeometry(state));
            if (_lastRetractId.HasValue) task.Chain(UpdateRetractGeometry(_lastRetractId.Value));
        }
        else if (_lastRetractPath is not null && _lastRetractId.HasValue)
        {
            var state = Maths.PathInterpolation(_lastRetractPath, interpolation);
            task.Chain(UpdateRetractGeometry(_lastRetractId.Value, state));
            task.Chain(UpdateStageGeometry());
        }

        RunTaskChain(task);
    }

    private void OnObstructionLevelChanged(object? sender,
        RangeBaseValueChangedEventArgs rangeBaseValueChangedEventArgs)
    {
        RunTaskChain(UpdateStaticGeometry());
    }

    private void OnCopyTargetToCurrent(object sender, RoutedEventArgs e)
    {
        StageCurrent.SetValue(StageTarget.GetValue());
    }

    private void OnCopyCurrentToTarget(object sender, RoutedEventArgs e)
    {
        StageTarget.SetValue(StageCurrent.GetValue());
    }
}