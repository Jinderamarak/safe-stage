using BindingsCs.Safe.Configurations;
using BindingsCs.Safe.Types;

namespace BindingsCs.Safe;

public class Microscope : IDisposable
{
    private bool _disposed;
    private readonly MutRefLock _lock = new();
    private readonly Unsafe.Microscope _microscope;

    private Microscope(Unsafe.Microscope microscope)
    {
        _disposed = false;
        _microscope = microscope;
    }

    public static Microscope FromConfiguration(Configuration configuration)
    {
        lock (configuration)
        {
            unsafe
            {
                fixed (Unsafe.Configuration* innerPtr = &configuration.Inner)
                {
                    return new Microscope(Unsafe.NativeMethods.microscope_from_config(innerPtr));
                }
            }
        }
    }

    public void UpdateHolder(HolderConfig holder)
    {
        ObjectDisposedException.ThrowIf(_disposed, this);
        using var guard = _lock.LockMut();
        unsafe
        {
            fixed (Unsafe.Microscope* microscope = &_microscope)
            fixed (Unsafe.HolderConfig* holderConfig = &holder.InnerConfig)
            {
                Unsafe.NativeMethods.microscope_update_holder(microscope, holderConfig);
            }
        }
    }

    public void RemoveHolder()
    {
        ObjectDisposedException.ThrowIf(_disposed, this);
        using var guard = _lock.LockMut();
        unsafe
        {
            fixed (Unsafe.Microscope* microscope = &_microscope)
            {
                Unsafe.NativeMethods.microscope_remove_holder(microscope);
            }
        }
    }

    public void UpdateSampleHeightMap(double[] heightMap, nuint sizeX, nuint sizeY, double realX, double realY)
    {
        ObjectDisposedException.ThrowIf(_disposed, this);
        using var guard = _lock.LockMut();
        unsafe
        {
            fixed (Unsafe.Microscope* microscope = &_microscope)
            fixed (double* heightMapPtr = heightMap)
            {
                Unsafe.NativeMethods.microscope_update_sample_height_map(microscope, heightMapPtr, sizeX, sizeY,
                    realX, realY);
            }
        }
    }

    public void ClearSample()
    {
        ObjectDisposedException.ThrowIf(_disposed, this);
        using var guard = _lock.LockMut();
        unsafe
        {
            fixed (Unsafe.Microscope* microscope = &_microscope)
            {
                Unsafe.NativeMethods.microscope_clear_sample(microscope);
            }
        }
    }

    public void UpdateStageState(SixAxis state)
    {
        ObjectDisposedException.ThrowIf(_disposed, this);
        using var guard = _lock.LockMut();
        unsafe
        {
            fixed (Unsafe.Microscope* microscope = &_microscope)
            {
                Unsafe.NativeMethods.microscope_update_stage_state(microscope, &state.Inner);
            }
        }
    }

    public void UpdateRetractState(Id id, LinearState state)
    {
        ObjectDisposedException.ThrowIf(_disposed, this);
        using var guard = _lock.LockMut();
        unsafe
        {
            fixed (Unsafe.Microscope* microscope = &_microscope)
            {
                var result =
                    Unsafe.NativeMethods.microscope_update_retract_state(microscope, id.Inner, &state.Inner);
                ThrowIfStateUpdateError(result, nameof(state), nameof(id));
            }
        }
    }

    public void UpdateResolvers()
    {
        ObjectDisposedException.ThrowIf(_disposed, this);
        using var guard = _lock.LockMut();
        unsafe
        {
            fixed (Unsafe.Microscope* microscope = &_microscope)
            {
                var result = Unsafe.NativeMethods.microscope_update_resolvers(microscope);
                ThrowIfStateUpdateError(result);
            }
        }
    }

    public PathResult<SixAxis> FindStagePath(SixAxis target)
    {
        ObjectDisposedException.ThrowIf(_disposed, this);
        using var guard = _lock.LockMut();
        unsafe
        {
            fixed (Unsafe.Microscope* microscope = &_microscope)
            {
                return PathResult<SixAxis>.FromNative(
                    Unsafe.NativeMethods.microscope_find_stage_path(microscope, &target.Inner));
            }
        }
    }

    public PathResult<LinearState> FindRetractPath(Id id, LinearState target)
    {
        ObjectDisposedException.ThrowIf(_disposed, this);
        using var guard = _lock.LockMut();
        unsafe
        {
            fixed (Unsafe.Microscope* microscope = &_microscope)
            {
                return PathResult<LinearState>.FromNative(
                    Unsafe.NativeMethods.microscope_find_retract_path(microscope, id.Inner, &target.Inner));
            }
        }
    }

    public TriangleBuffer PresentStaticFull()
    {
        ObjectDisposedException.ThrowIf(_disposed, this);
        using var guard = _lock.LockRef();
        unsafe
        {
            fixed (Unsafe.Microscope* microscope = &_microscope)
            {
                return new TriangleBuffer(Unsafe.NativeMethods.microscope_present_static_full(microscope));
            }
        }
    }

    public TriangleBuffer PresentStaticLessObstructive()
    {
        ObjectDisposedException.ThrowIf(_disposed, this);
        using var guard = _lock.LockRef();
        unsafe
        {
            fixed (Unsafe.Microscope* microscope = &_microscope)
            {
                return new TriangleBuffer(
                    Unsafe.NativeMethods.microscope_present_static_less_obstructive(microscope));
            }
        }
    }

    public TriangleBuffer PresentStaticNonObstructive()
    {
        ObjectDisposedException.ThrowIf(_disposed, this);
        using var guard = _lock.LockRef();
        unsafe
        {
            fixed (Unsafe.Microscope* microscope = &_microscope)
            {
                return new TriangleBuffer(
                    Unsafe.NativeMethods.microscope_present_static_non_obstructive(microscope));
            }
        }
    }

    public TriangleBuffer PresentStage()
    {
        ObjectDisposedException.ThrowIf(_disposed, this);
        using var guard = _lock.LockRef();
        unsafe
        {
            fixed (Unsafe.Microscope* microscope = &_microscope)
            {
                return new TriangleBuffer(Unsafe.NativeMethods.microscope_present_stage(microscope));
            }
        }
    }

    public TriangleBuffer PresentStageAt(SixAxis state)
    {
        ObjectDisposedException.ThrowIf(_disposed, this);
        using var guard = _lock.LockRef();
        unsafe
        {
            fixed (Unsafe.Microscope* microscope = &_microscope)
            {
                return new TriangleBuffer(Unsafe.NativeMethods.microscope_present_stage_at(microscope, &state.Inner));
            }
        }
    }

    public TriangleBuffer PresentRetract(Id id)
    {
        ObjectDisposedException.ThrowIf(_disposed, this);
        using var guard = _lock.LockRef();
        unsafe
        {
            fixed (Unsafe.Microscope* microscope = &_microscope)
            {
                return new TriangleBuffer(Unsafe.NativeMethods.microscope_present_retract(microscope, id.Inner));
            }
        }
    }

    public TriangleBuffer PresentRetractAt(Id id, LinearState state)
    {
        ObjectDisposedException.ThrowIf(_disposed, this);
        using var guard = _lock.LockRef();
        unsafe
        {
            fixed (Unsafe.Microscope* microscope = &_microscope)
            {
                return new TriangleBuffer(
                    Unsafe.NativeMethods.microscope_present_retract_at(microscope, id.Inner, &state.Inner));
            }
        }
    }

    public void Dispose()
    {
        if (_disposed) return;
        using var guard = _lock.LockMut();
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
                throw new ArgumentException("Changing the state resulted in one of the resolvers failing to update.",
                    state);
            case Unsafe.StateUpdateError.InvalidId:
                throw new ArgumentException("Provided ID was not valid in the given context.", id);
        }
    }
}