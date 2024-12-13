using System.ComponentModel;
using System.Windows;
using System.Windows.Input;
using System.Windows.Media;
using System.Windows.Media.Imaging;
using ServiceApp.Utility;

namespace ServiceApp.Windows;

public partial class HeightMapWindow : ReactiveWindow
{
    public bool Cleared { get; private set; } = false;

    public double RealX
    {
        get => _realX;
        set => SetField(ref _realX, value);
    }

    public double RealY
    {
        get => _realY;
        set => SetField(ref _realY, value);
    }

    public double MaxSampleHeight
    {
        get => _maxSampleHeight;
        set
        {
            SetField(ref _maxSampleHeight, value);
            ClipHeightMap();
            RenderHeightMap();
        }
    }

    public int ResolutionX
    {
        get => _resolutionX;
        set
        {
            var oldX = _resolutionX;
            SetField(ref _resolutionX, value);
            ResizeHeightMap(oldX, ResolutionY);
            UpdateBitmap();
        }
    }

    public int ResolutionY
    {
        get => _resolutionY;
        set
        {
            var oldY = _resolutionY;
            SetField(ref _resolutionY, value);
            ResizeHeightMap(ResolutionX, oldY);
            UpdateBitmap();
        }
    }

    public int BrushRadius
    {
        get => _brushRadius;
        set => SetField(ref _brushRadius, value);
    }

    public double BrushHeight
    {
        get => _brushHeight;
        set => SetField(ref _brushHeight, value);
    }

    public double[] HeightMap
    {
        get => _heightMap;
        set
        {
            SetField(ref _heightMap, value);
            RenderHeightMap();
        }
    }

    private double _realX = 100e-3;
    private double _realY = 100e-3;
    private double _maxSampleHeight = 20e-3;
    private int _resolutionX = 32;
    private int _resolutionY = 32;
    private int _brushRadius = 4;
    private double _brushHeight;

    private double[] _heightMap = null!;
    private WriteableBitmap _bitmap = null!;

    public HeightMapWindow()
    {
        DataContext = this;
        InitializeComponent();
        InitializeHeightMap();
        UpdateBitmap();
    }

    private void InitializeHeightMap()
    {
        _heightMap = Enumerable.Repeat(0.0, ResolutionX * ResolutionY).ToArray();
    }

    private void ResizeHeightMap(int oldX, int oldY)
    {
        var oldHeightMap = _heightMap;
        _heightMap = new double[ResolutionX * ResolutionY];
        for (var y = 0; y < Math.Min(oldY, ResolutionY); y++)
        for (var x = 0; x < Math.Min(oldX, ResolutionX); x++)
        {
            var i = x + y * ResolutionX;
            _heightMap[i] = oldHeightMap[x + y * oldX];
        }
    }

    private void ClipHeightMap()
    {
        _heightMap = _heightMap.Select(h => Math.Max(0.0, Math.Min(MaxSampleHeight, h))).ToArray();
    }

    private void UpdateBitmap()
    {
        _bitmap = new WriteableBitmap(ResolutionX, ResolutionY, 96, 96, PixelFormats.Gray8, null);
        RenderHeightMap();
        HeightMapImage.Source = _bitmap;
    }

    private void RenderHeightMap()
    {
        var pixels = _heightMap.Select(h => (byte)(h / MaxSampleHeight * byte.MaxValue)).ToArray();
        _bitmap.WritePixels(new Int32Rect(0, 0, ResolutionX, ResolutionY), pixels, ResolutionX * sizeof(byte), 0);
    }

    private void HeightMapImage_MouseMove(object sender, MouseEventArgs e)
    {
        if (e.LeftButton == MouseButtonState.Pressed)
        {
            var position = e.GetPosition(HeightMapImage);
            var centerX = (int)(position.X * ResolutionX / HeightMapImage.ActualWidth);
            var centerY = (int)(position.Y * ResolutionY / HeightMapImage.ActualHeight);

            ApplyBrush(centerX, centerY);
            RenderHeightMap();
        }
    }

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
        DialogResult = true;
        Close();
    }

    private void OnClear(object sender, RoutedEventArgs e)
    {
        InitializeHeightMap();
        Cleared = true;
        DialogResult = true;
        Close();
    }

    private void OnCancel(object sender, RoutedEventArgs e)
    {
        DialogResult = false;
        Close();
    }
}