using System;
using System.Linq;
using Avalonia;
using Avalonia.Controls;
using Avalonia.Interactivity;
using Avalonia.Markup.Xaml;
using Avalonia.Media.Imaging;

namespace ServiceApp.Avalonia.Windows;

public partial class HeightMapWindow : Window
{
    public static readonly StyledProperty<double> RealXProperty =
        AvaloniaProperty.Register<HeightMapWindow, double>(nameof(RealX));

    public static readonly StyledProperty<double> RealYProperty =
        AvaloniaProperty.Register<HeightMapWindow, double>(nameof(RealY));

    public static readonly StyledProperty<double> MaxSampleHeightProperty =
        AvaloniaProperty.Register<HeightMapWindow, double>(nameof(MaxSampleHeight));

    public static readonly StyledProperty<int> ResolutionXProperty =
        AvaloniaProperty.Register<HeightMapWindow, int>(nameof(ResolutionX));

    public static readonly StyledProperty<int> ResolutionYProperty =
        AvaloniaProperty.Register<HeightMapWindow, int>(nameof(ResolutionY));

    public static readonly StyledProperty<int> BrushRadiusProperty =
        AvaloniaProperty.Register<HeightMapWindow, int>(nameof(BrushRadius));

    public static readonly StyledProperty<double> BrushHeightProperty =
        AvaloniaProperty.Register<HeightMapWindow, double>(nameof(BrushHeight));

    public static readonly StyledProperty<double[]> HeightMapProperty =
        AvaloniaProperty.Register<HeightMapWindow, double[]>(nameof(HeightMap));

    public bool Cleared { get; private set; } = false;

    public double RealX
    {
        get => GetValue(RealXProperty);
        set => SetValue(RealXProperty, value);
    }

    public double RealY
    {
        get => GetValue(RealYProperty);
        set => SetValue(RealYProperty, value);
    }

    public double MaxSampleHeight
    {
        get => GetValue(MaxSampleHeightProperty);
        set
        {
            SetValue(MaxSampleHeightProperty, value);
            ClipHeightMap();
            RenderHeightMap();
        }
    }

    public int ResolutionX
    {
        get => GetValue(ResolutionXProperty);
        set
        {
            var oldX = ResolutionX;
            SetValue(ResolutionXProperty, value);
            ResizeHeightMap(oldX, ResolutionY);
            UpdateBitmap();
        }
    }

    public int ResolutionY
    {
        get => GetValue(ResolutionYProperty);
        set
        {
            var oldY = ResolutionY;
            SetValue(ResolutionYProperty, value);
            ResizeHeightMap(ResolutionX, oldY);
            UpdateBitmap();
        }
    }

    public int BrushRadius
    {
        get => GetValue(BrushRadiusProperty);
        set => SetValue(BrushRadiusProperty, value);
    }

    public double BrushHeight
    {
        get => GetValue(BrushHeightProperty);
        set => SetValue(BrushHeightProperty, value);
    }

    public double[] HeightMap
    {
        get => GetValue(HeightMapProperty);
        set
        {
            SetValue(HeightMapProperty, value);
            RenderHeightMap();
        }
    }

    private WriteableBitmap _bitmap;
    private Image _heightMapImage;

    public HeightMapWindow()
    {
        DataContext = this;
        AvaloniaXamlLoader.Load(this);
        _heightMapImage = this.FindControl<Image>("HeightMapImage")!;
        InitializeHeightMap();
        UpdateBitmap();
    }

    private void InitializeHeightMap()
    {
        HeightMap = Enumerable.Repeat(0.0, ResolutionX * ResolutionY).ToArray();
    }

    private void ResizeHeightMap(int oldX, int oldY)
    {
        var oldHeightMap = HeightMap;
        var newHeightMap = new double[ResolutionX * ResolutionY];
        for (var y = 0; y < Math.Min(oldY, ResolutionY); y++)
        for (var x = 0; x < Math.Min(oldX, ResolutionX); x++)
        {
            var i = x + y * ResolutionX;
            newHeightMap[i] = oldHeightMap[x + y * oldX];
        }

        HeightMap = newHeightMap;
    }

    private void ClipHeightMap()
    {
        HeightMap = HeightMap.Select(h => Math.Max(0.0, Math.Min(MaxSampleHeight, h))).ToArray();
    }

    private void UpdateBitmap()
    {
        _bitmap = new WriteableBitmap(new PixelSize(ResolutionX, ResolutionY), new Vector(96, 96));
        RenderHeightMap();
        _heightMapImage.Source = _bitmap;
    }

    private void RenderHeightMap()
    {
        // var pixels = HeightMap.Select(h => (byte)(h / MaxSampleHeight * byte.MaxValue)).ToArray();
        // _bitmap.WritePixels(new Int32Rect(0, 0, ResolutionX, ResolutionY), pixels, ResolutionX * sizeof(byte), 0);
    }

    // private void HeightMapImage_MouseMove(object sender, MouseEventArgs e)
    // {
    //     if (e.LeftButton == MouseButtonState.Pressed)
    //     {
    //         var position = e.GetPosition(HeightMapImage);
    //         var centerX = (int)(position.X * ResolutionX / HeightMapImage.ActualWidth);
    //         var centerY = (int)(position.Y * ResolutionY / HeightMapImage.ActualHeight);
    //
    //         ApplyBrush(centerX, centerY);
    //         RenderHeightMap();
    //     }
    // }

    private void ApplyBrush(int centerX, int centerY)
    {
        for (var y = -BrushRadius; y <= BrushRadius; y++)
        for (var x = -BrushRadius; x <= BrushRadius; x++)
        {
            var targetX = centerX + x;
            var targetY = centerY + y;

            if (targetX < 0 || targetX >= ResolutionX)
                continue;
            if (targetY < 0 || targetY >= ResolutionY)
                continue;
            if (x * x + y * y > BrushRadius * BrushRadius)
                continue;
            var i = targetX + targetY * ResolutionX;
            HeightMap[i] = BrushHeight;
        }
    }

    private void OnResetHeightMap(object sender, RoutedEventArgs e)
    {
        InitializeHeightMap();
        RenderHeightMap();
    }

    private void OnSave(object sender, RoutedEventArgs e)
    {
        Close(true);
    }

    private void OnClear(object sender, RoutedEventArgs e)
    {
        InitializeHeightMap();
        Cleared = true;
        Close(true);
    }

    private void OnCancel(object sender, RoutedEventArgs e)
    {
        Close(false);
    }
}