using BindingsCs.Safe.Configurations;
using BindingsCs.Safe.Types;

namespace BindingsCs.Safe;

public class Microscope : IDisposable
{
    private readonly Unsafe.Microscope _microscope;
    private bool _disposed;
    
    private Microscope(Unsafe.Microscope microscope)
    {
        _microscope = microscope;
        _disposed = false;
    }

    public static Microscope FromConfiguration(Configuration configuration)
    {
        unsafe
        {
            fixed (Unsafe.Configuration* innerPtr = &configuration.Inner)
            {
                return new Microscope(Unsafe.NativeMethods.microscope_from_config(innerPtr));
            }
        }
    }

    public void UpdateHolder(HolderConfig holder)
    {
        ObjectDisposedException.ThrowIf(_disposed, this);
        unsafe
        {
            fixed (Unsafe.Microscope* microscope = &_microscope)
            fixed (Unsafe.HolderConfig* holderConfig = &holder.InnerConfig)
            {
                var result = Unsafe.NativeMethods.microscope_update_holder(microscope, holderConfig);
                ThrowIfStateUpdateError(result);
            }
        }
    }

    public void RemoveHolder()
    {
        ObjectDisposedException.ThrowIf(_disposed, this);
        unsafe
        {
            fixed (Unsafe.Microscope* microscope = &_microscope)
            {
                var result = Unsafe.NativeMethods.microscope_remove_holder(microscope);
                ThrowIfStateUpdateError(result);
            }
        }
    }

    public void UpdateSampleHeightMap(double[] heightMap, nuint sizeX, nuint sizeY, double realX, double realY)
    {
        ObjectDisposedException.ThrowIf(_disposed, this);
        unsafe
        {
            fixed (Unsafe.Microscope* microscope = &_microscope)
            fixed (double* heightMapPtr = heightMap)
            {
                var result = Unsafe.NativeMethods.microscope_update_sample_height_map(microscope, heightMapPtr, sizeX, sizeY, realX, realY);
                ThrowIfStateUpdateError(result);
            }
        }
    }

    public void ClearSample()
    {
        ObjectDisposedException.ThrowIf(_disposed, this);
        unsafe
        {
            fixed (Unsafe.Microscope* microscope = &_microscope)
            {
                var result = Unsafe.NativeMethods.microscope_clear_sample(microscope);
                ThrowIfStateUpdateError(result);
            }
        }
    }

    public void UpdateStageState(SixAxis state)
    {
        ObjectDisposedException.ThrowIf(_disposed, this);
        unsafe
        {
            fixed (Unsafe.Microscope* microscope = &_microscope)
            {
                var result = Unsafe.NativeMethods.microscope_update_stage_state(microscope, &state.Inner);
                ThrowIfStateUpdateError(result, state: nameof(state));
            }
        }
    }

    public void UpdateRetractState(Id id, LinearState state)
    {
        ObjectDisposedException.ThrowIf(_disposed, this);
        unsafe
        {
            fixed (Unsafe.Microscope* microscope = &_microscope)
            {
                var result = Unsafe.NativeMethods.microscope_update_retract_state(microscope, id.Inner, &state.Inner);
                ThrowIfStateUpdateError(result, state: nameof(state), id: nameof(id));
            }
        }
    }

    public PathResultSixAxis FindStagePath(SixAxis target)
    {
        ObjectDisposedException.ThrowIf(_disposed, this);
        unsafe
        {
            fixed (Unsafe.Microscope* microscope = &_microscope)
            {
                return new PathResultSixAxis(Unsafe.NativeMethods.microscope_find_stage_path(microscope, &target.Inner));
            }
        }
    }
    
    public PathResultLinearState FindRetractPath(Id id, LinearState target)
    {
        ObjectDisposedException.ThrowIf(_disposed, this);
        unsafe
        {
            fixed (Unsafe.Microscope* microscope = &_microscope)
            {
                return new PathResultLinearState(Unsafe.NativeMethods.microscope_find_retract_path(microscope, id.Inner, &target.Inner));
            }
        }
    }

    public TriangleBuffer PresentStatic()
    {
        ObjectDisposedException.ThrowIf(_disposed, this);
        unsafe
        {
            fixed (Unsafe.Microscope* microscope = &_microscope)
            {
                return new TriangleBuffer(Unsafe.NativeMethods.microscope_present_static(microscope));
            }
        }
    }

    public TriangleBuffer PresentStage()
    {
        ObjectDisposedException.ThrowIf(_disposed, this);
        unsafe
        {
            fixed (Unsafe.Microscope* microscope = &_microscope)
            {
                return new TriangleBuffer(Unsafe.NativeMethods.microscope_present_stage(microscope));
            }
        }
    }

    public TriangleBuffer PresentRetract(Id id)
    {
        ObjectDisposedException.ThrowIf(_disposed, this);
        unsafe
        {
            fixed (Unsafe.Microscope* microscope = &_microscope)
            {
                return new TriangleBuffer(Unsafe.NativeMethods.microscope_present_retract(microscope, id.Inner));
            }
        }
    }
    
    public void Dispose()
    {
        if (_disposed) return;
        Unsafe.NativeMethods.microscope_drop(_microscope);
        _disposed = true;
    }
    
    private static void ThrowIfStateUpdateError(Unsafe.StateUpdateError error, string? state = null, string? id = null)
    {
        switch (error)
        {
            case Unsafe.StateUpdateError.Ok:
                return;
            case Unsafe.StateUpdateError.InvalidState:
                throw new ArgumentException("Changing the state resulted in one of the resolvers failing to update.", state);
            case Unsafe.StateUpdateError.InvalidId:
                throw new ArgumentException("Provided ID was not valid in the given context.", id);
        }
    }
}