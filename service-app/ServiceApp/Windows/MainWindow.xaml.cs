using System.Collections.ObjectModel;
using System.Reactive.Linq;
using System.Windows;
using System.Windows.Input;
using System.Windows.Media;
using System.Windows.Media.Media3D;
using BindingsCs.Safe;
using BindingsCs.Safe.Configurations;
using BindingsCs.Safe.Types;
using ServiceApp.Models;
using ServiceApp.Models.Presets;
using ServiceApp.Tasks;
using ServiceApp.Utility;
using ServiceApp.Utility.Extensions;

namespace ServiceApp.Windows;

/// <summary>
///     Interaction logic for MainWindow.xaml
/// </summary>
public partial class MainWindow : ReactiveWindow, IDisposable
{
    public string StatusText
    {
        get => _statusText;
        set => SetField(ref _statusText, value);
    }

    public bool IsUpdating
    {
        get => _isUpdating || _microscope is null;
        set => SetField(ref _isUpdating, value);
    }

    public bool HasConfiguration => _microscope is not null;

    public string ConfigurationTitle
    {
        get => _configurationTitle;
        set => SetField(ref _configurationTitle, value);
    }

    public string HolderTitle
    {
        get => _holderTitle;
        set => SetField(ref _holderTitle, value);
    }

    public ObservableCollection<Id> Retracts { get; } = new();

    public Id? SelectedRetract
    {
        get => _selectedRetract;
        set => SetField(ref _selectedRetract, value);
    }

    public bool HasPath => (_interestedInStage && _lastStagePath is not null)
                           || (!_interestedInStage && _lastRetractPath is not null);

    public int StageInterpolationMax => _interestedInStage
        ? _lastStagePath?.Nodes.Count() - 1 ?? 0
        : _lastRetractPath?.Nodes.Count() - 1 ?? 0;

    public int ObstructionLevel
    {
        get => _obstructionLevel;
        set => SetField(ref _obstructionLevel, value);
    }

    private string _statusText = "Ready";
    private bool _isUpdating;

    private string _configurationTitle = "No Configuration";
    private string _holderTitle = "No Holder";

    private Id? _selectedRetract;

    private bool _interestedInStage;
    private PathResult<SixAxis>? _lastStagePath;
    private PathResult<LinearState>? _lastRetractPath;
    private Id? _lastRetractId;

    private int _obstructionLevel;

    private Preset? _lastPreset;
    private ConfigVariant? _lastHolder;
    private Microscope? _microscope;

    private readonly Dictionary<Id, int> _retractIdToGeometryIndex = new();

    private readonly OrbitControls _orbitControls;

    public MainWindow()
    {
        DataContext = this;
        InitializeComponent();

        _orbitControls = new OrbitControls(
            Camera, DirectLight,
            2, 1.2, -0.9)
        {
            ScrollSensitivity = 1e-3,
            DragSensitivity = 5e-3
        };

        Observable.FromEventPattern<RoutedPropertyChangedEventHandler<double>, RoutedPropertyChangedEventArgs<double>>(
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
        chain.Execute(Dispatcher);
    }

    private void RunUpdatingChain(TaskChain chain)
    {
        IsUpdating = true;
        chain.OnUi(() => IsUpdating = false)
            .Execute(Dispatcher);
    }

    private void OnViewportMouseMove(object sender, MouseEventArgs e)
    {
        _orbitControls.MouseMoved(e.GetPosition(Viewport), e);
    }

    private void OnViewportMouseWheel(object sender, MouseWheelEventArgs e)
    {
        _orbitControls.MouseScrolled(e);
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
            .OnUi((List<List<Point3D>> positions) =>
            {
                StageModelGroup.Children.Clear();

                var materials = Materials.StageRange(positions.Count);
                foreach (var (points, material) in positions.Zip(materials))
                    StageModelGroup.Children.Add(new GeometryModel3D
                    {
                        Material = material,
                        Geometry = new MeshGeometry3D
                        {
                            Positions = new Point3DCollection(points)
                        }
                    });
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
            .OnUi((List<List<Point3D>> positions) =>
            {
                StaticModelGroup.Children.Clear();

                var materials = Materials.StaticRange(positions.Count);
                foreach (var (points, material) in positions.Zip(materials))
                    StaticModelGroup.Children.Add(new GeometryModel3D
                    {
                        Material = material,
                        Geometry = new MeshGeometry3D
                        {
                            Positions = new Point3DCollection(points)
                        }
                    });
            });
    }

    private void InitializeRetractMapping()
    {
        _retractIdToGeometryIndex.Clear();
        RetractModelGroup.Children.Clear();

        for (var i = 0; i < Retracts.Count; i++)
        {
            var id = Retracts[i];
            _retractIdToGeometryIndex[id] = i;
            RetractModelGroup.Children.Add(new Model3DGroup());
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
            .OnUi(((int, List<List<Point3D>>) pair) =>
            {
                var model = RetractModelGroup.Children[pair.Item1];
                if (model is Model3DGroup group)
                {
                    group.Children.Clear();

                    var materials = Materials.RetractRange(pair.Item1, pair.Item2.Count);
                    foreach (var (points, material) in pair.Item2.Zip(materials))
                        group.Children.Add(new GeometryModel3D
                        {
                            Material = material,
                            Geometry = new MeshGeometry3D
                            {
                                Positions = new Point3DCollection(points)
                            }
                        });
                }
            });
    }

    private void OnConfigureMicroscope(object sender, RoutedEventArgs e)
    {
        var window = new MicroscopeConfiguration();
        if (_lastPreset is not null) window.ApplyPreset(_lastPreset);
        var result = window.ShowDialog();

        if (result ?? false)
        {
            var task = new TaskChain()
                .OnUi(() => StatusText = "Building microscope configuration ...")
                .InBack(() =>
                {
                    _microscope?.Dispose();
                    _microscope = ConfigBuilder.BuildMicroscope(window.ChamberVariant!,
                        window.StageVariant!,
                        window.StageResolverVariant!,
                        window.Equipment.AsReadOnly(),
                        window.Retracts.AsReadOnly());
                })
                .OnUi(() => StatusText = "Updating resolvers ...")
                .InBack(() => _microscope!.UpdateResolvers())
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

                    OnPropertyChanged(nameof(HasConfiguration));
                    OnPropertyChanged(nameof(IsUpdating));
                    OnPropertyChanged(nameof(HasPath));
                    OnPropertyChanged(nameof(StageInterpolationMax));

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

    private void OnChangeHolder(object sender, RoutedEventArgs e)
    {
        var holderDialog = new SelectHolderWindow
        {
            SelectedHolder = _lastHolder
        };

        if (holderDialog.ShowDialog() ?? false)
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

                    _microscope!.UpdateResolvers();
                })
                .Chain(UpdateStageGeometry())
                .OnUi(() => HolderTitle = _lastHolder is not null ? _lastHolder.ToString() : "No Holder")
                .OnUi(() => StatusText = "Holder changed");
            RunUpdatingChain(task);
        }
    }

    private void OnChangeSampleHeight(object sender, RoutedEventArgs e)
    {
        var dialog = new HeightMapWindow();
        var result = dialog.ShowDialog();

        var task = new TaskChain()
            .OnUi(() => StatusText = "Updating sample height map and resolvers ...");

        IsUpdating = true;
        if ((result ?? false) && !dialog.Cleared)
            task.InBack(() =>
                {
                    _microscope!.UpdateSampleHeightMap(
                        dialog.HeightMap,
                        (nuint)dialog.ResolutionX,
                        (nuint)dialog.ResolutionY,
                        dialog.RealX,
                        dialog.RealY
                    );
                    _microscope!.UpdateResolvers();
                })
                .Chain(UpdateStageGeometry())
                .OnUi(() => { StatusText = "Sample height map updated"; });
        else if ((result ?? false) && dialog.Cleared)
            task.InBack(() =>
                {
                    _microscope!.ClearSample();
                    _microscope!.UpdateResolvers();
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
                OnPropertyChanged(nameof(HasPath));
                OnPropertyChanged(nameof(StageInterpolationMax));
                StatusText = $"Stage state updated to {current.ToFormattedString()}";

                if (ex is not null)
                    MessageBox.Show(ex.Message, "Invalid Stage State", MessageBoxButton.OK, MessageBoxImage.Error);
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
                    OnPropertyChanged(nameof(HasPath));
                    OnPropertyChanged(nameof(StageInterpolationMax));

                    StatusText = $"Path found in {time}ms. {_lastStagePath?.ToStatusMessage()}";
                    _lastStagePath?.ShowMessageBox();

                    PathModelGroup.Children.Clear();
                    foreach (var geometry in Shapes.CreatePathGeometries(_lastStagePath!.Nodes.ToList()))
                        PathModelGroup.Children.Add(geometry);
                }
                else if (result is Exception ex)
                {
                    MessageBox.Show(ex.Message, "Invalid Stage Target",
                        MessageBoxButton.OK,
                        MessageBoxImage.Error);
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
                OnPropertyChanged(nameof(HasPath));
                OnPropertyChanged(nameof(StageInterpolationMax));
                StatusText = $"Retract {id} updated";

                if (ex is not null)
                    MessageBox.Show(ex.Message, "Invalid Retract State", MessageBoxButton.OK, MessageBoxImage.Error);
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
                    OnPropertyChanged(nameof(HasPath));
                    OnPropertyChanged(nameof(StageInterpolationMax));

                    StatusText = $"Path found in {time}ms. {_lastRetractPath?.ToStatusMessage()}";
                    _lastRetractPath?.ShowMessageBox();

                    PathModelGroup.Children.Clear();
                }
                else if (result is Exception ex)
                {
                    MessageBox.Show(ex.Message, "Invalid Retract Target",
                        MessageBoxButton.OK,
                        MessageBoxImage.Error);
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

    private void OnObstructionLevelChanged(object sender, RoutedPropertyChangedEventArgs<double> e)
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