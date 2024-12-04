using BindingsCs.Safe.Configurations;
using BindingsCs.Safe.Types;

namespace BindingsCs.Safe;

public class Configuration: IDisposable
{
    internal readonly Unsafe.Configuration Inner;
    
    private bool _disposed;
    
    internal Configuration(Unsafe.Configuration inner)
    {
        Inner = inner;
        _disposed = false;
    }

    public void Dispose()
    {
        if (_disposed) return;
        Unsafe.NativeMethods.configuration_drop(Inner);
        _disposed = true;
    }
}

public class ConfigurationBuilder : IDisposable
{
    internal Unsafe.ConfigurationBuilder Inner;
    
    private bool _disposed;
    
    public ConfigurationBuilder()
    {
        Inner = Unsafe.NativeMethods.builder_new();
        _disposed = false;
    }

    public ConfigurationBuilder WithChamber(ChamberConfig chamberConfig)
    {
        ObjectDisposedException.ThrowIf(_disposed, this);
        Inner = Unsafe.NativeMethods.builder_with_chamber(Inner, chamberConfig.InnerConfig);
        return this;
    }
    
    public ConfigurationBuilder WithStage(StageConfig stageConfig, ResolverStageConfig resolverStageConfig)
    {
        ObjectDisposedException.ThrowIf(_disposed, this);
        Inner = Unsafe.NativeMethods.builder_with_stage(Inner, stageConfig.InnerConfig, resolverStageConfig.InnerConfig);
        return this;
    }
    
    public ConfigurationBuilder WithEquipment(EquipmentConfig equipmentConfig)
    {
        ObjectDisposedException.ThrowIf(_disposed, this);
        Inner = Unsafe.NativeMethods.builder_with_equipment(Inner, equipmentConfig.InnerConfig);
        return this;
    }

    public ConfigurationBuilder WithRetract(Id id, RetractConfig retractConfig,
        ResolverRetractConfig resolverRetractConfig)
    {
        ObjectDisposedException.ThrowIf(_disposed, this);
        Inner = Unsafe.NativeMethods.builder_with_retract(Inner, id.Inner, retractConfig.InnerConfig,
            resolverRetractConfig.InnerConfig);
        return this;
    }

    public Configuration Build()
    {
        ObjectDisposedException.ThrowIf(_disposed, this);
        unsafe
        {
            var config = new Unsafe.Configuration();
            var result = Unsafe.NativeMethods.builder_build(Inner, &config);
            _disposed = true;   //  Method `builder_build` takes ownership of the builder
            
            #pragma warning disable CS8524
            return result switch
            {
                Unsafe.ConfigBuilderResult.Success => new Configuration(config),
                Unsafe.ConfigBuilderResult.MissingChamber => throw new InvalidOperationException(
                    "Missing chamber config"),
                Unsafe.ConfigBuilderResult.MissingStage => throw new InvalidOperationException(
                    "Missing stage config"),
            };
            #pragma warning restore
        }
    }
    
    public void Dispose()
    {
        if (_disposed) return;
        Unsafe.NativeMethods.builder_drop(Inner);
        _disposed = true;
    }
}