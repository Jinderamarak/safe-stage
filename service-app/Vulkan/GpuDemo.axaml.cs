using System.Drawing;
using Avalonia;
using Avalonia.Controls;
using Avalonia.Markup.Xaml;
using Avalonia.Threading;
using BindingsCs.Safe;
using BindingsCs.Safe.Configurations;
using BindingsCs.Safe.Types;
using ServiceApp.Vulkan.Controls;
using Vector3 = System.Numerics.Vector3;

namespace GpuInterop;

public partial class GpuDemo : UserControl
{
    private DispatcherTimer _timer;

    public GpuDemo()
    {
        AvaloniaXamlLoader.Load(this);
        using var configBuilder = new ConfigurationBuilder();
        var config = configBuilder.WithChamber(ChamberConfig.ThesisChamber())
            .WithStage(StageConfig.ThesisStage(), ResolverStageConfig.Linear(new SixAxis()))
            .Build();

        var microscope = Microscope.FromConfiguration(config);

        var holder = HolderConfig.ThesisHolderCircle();
        microscope.UpdateHolder(holder);

        microscope.UpdateResolvers();

        var stageGroup = this.Find<ModelGroup>("StageGroup")!;
        var colors = new[]
        {
            Color.Red,
            Color.Blue,
            Color.Green,
            Color.Yellow,
            Color.Orange,
            Color.Purple,
            Color.Cyan,
            Color.Magenta
        };
        var timeTick = 0;

        _timer = new DispatcherTimer
        {
            Interval = TimeSpan.FromMilliseconds(300)
        };
        _timer.Tick += (sender, args) =>
        {
            Console.WriteLine("TICK");

            timeTick++;
            var stage = microscope.PresentStageAt(new SixAxis
            {
                Rz = double.DegreesToRadians(timeTick * 10)
            });

            if (stage.Count == stageGroup.Models.Count)
            {
                for (var i = 0; i < stageGroup.Models.Count; i++)
                {
                    stageGroup.Models[i].Color = colors[i % colors.Length];
                    stageGroup.Models[i].Vertices =
                        stage[i].Buffer.Select(v => new Vector3((float)v.X, (float)v.Y, (float)v.Z));
                }
            }
            else
            {
                stageGroup.Models.Clear();
                for (var i = 0; i < stage.Count; i++)
                {
                    var model = new GeometryModel
                    {
                        Color = colors[i % colors.Length],
                        Vertices = stage[i].Buffer.Select(v => new Vector3((float)v.X, (float)v.Y, (float)v.Z))
                    };
                    stageGroup.Models.Add(model);
                }
            }
        };
        _timer.Start();
    }
}