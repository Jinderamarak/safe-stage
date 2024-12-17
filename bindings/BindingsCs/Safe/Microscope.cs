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

    /// <inheritdoc cref="Unsafe.NativeMethods.microscope_from_config"/>
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

    /// <inheritdoc cref="Unsafe.NativeMethods.microscope_update_holder"/>
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

    /// <inheritdoc cref="Unsafe.NativeMethods.microscope_remove_holder"/>
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

    /// <inheritdoc cref="Unsafe.NativeMethods.microscope_update_sample_height_map"/>
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

    /// <inheritdoc cref="Unsafe.NativeMethods.microscope_clear_sample"/>
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

    /// <inheritdoc cref="Unsafe.NativeMethods.microscope_update_stage_state"/>
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

    /// <inheritdoc cref="Unsafe.NativeMethods.microscope_update_retract_state"/>
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

    /// <inheritdoc cref="Unsafe.NativeMethods.microscope_update_resolvers"/>
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

    /// <inheritdoc cref="Unsafe.NativeMethods.microscope_find_stage_path"/>
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

    /// <inheritdoc cref="Unsafe.NativeMethods.microscope_find_retract_path"/>
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

    /// <inheritdoc cref="Unsafe.NativeMethods.microscope_present_static_full"/>
    public List<TriangleBuffer> PresentStaticFull()
    {
        ObjectDisposedException.ThrowIf(_disposed, this);
        using var guard = _lock.LockRef();
        unsafe
        {
            fixed (Unsafe.Microscope* microscope = &_microscope)
            {
                return TriangleBuffer.FromNativeVec(Unsafe.NativeMethods.microscope_present_static_full(microscope));
            }
        }
    }

    /// <inheritdoc cref="Unsafe.NativeMethods.microscope_present_static_less_obstructive"/>
    public List<TriangleBuffer> PresentStaticLessObstructive()
    {
        ObjectDisposedException.ThrowIf(_disposed, this);
        using var guard = _lock.LockRef();
        unsafe
        {
            fixed (Unsafe.Microscope* microscope = &_microscope)
            {
                return TriangleBuffer.FromNativeVec(
                    Unsafe.NativeMethods.microscope_present_static_less_obstructive(microscope));
            }
        }
    }

    /// <inheritdoc cref="Unsafe.NativeMethods.microscope_present_static_non_obstructive"/>
    public List<TriangleBuffer> PresentStaticNonObstructive()
    {
        ObjectDisposedException.ThrowIf(_disposed, this);
        using var guard = _lock.LockRef();
        unsafe
        {
            fixed (Unsafe.Microscope* microscope = &_microscope)
            {
                return TriangleBuffer.FromNativeVec(
                    Unsafe.NativeMethods.microscope_present_static_non_obstructive(microscope));
            }
        }
    }

    /// <inheritdoc cref="Unsafe.NativeMethods.microscope_present_stage"/>
    public List<TriangleBuffer> PresentStage()
    {
        ObjectDisposedException.ThrowIf(_disposed, this);
        using var guard = _lock.LockRef();
        unsafe
        {
            fixed (Unsafe.Microscope* microscope = &_microscope)
            {
                return TriangleBuffer.FromNativeVec(Unsafe.NativeMethods.microscope_present_stage(microscope));
            }
        }
    }

    /// <inheritdoc cref="Unsafe.NativeMethods.microscope_present_stage_at"/>
    public List<TriangleBuffer> PresentStageAt(SixAxis state)
    {
        ObjectDisposedException.ThrowIf(_disposed, this);
        using var guard = _lock.LockRef();
        unsafe
        {
            fixed (Unsafe.Microscope* microscope = &_microscope)
            {
                return TriangleBuffer.FromNativeVec(
                    Unsafe.NativeMethods.microscope_present_stage_at(microscope, &state.Inner));
            }
        }
    }

    /// <inheritdoc cref="Unsafe.NativeMethods.microscope_present_retract"/>
    public List<TriangleBuffer> PresentRetract(Id id)
    {
        ObjectDisposedException.ThrowIf(_disposed, this);
        using var guard = _lock.LockRef();
        unsafe
        {
            fixed (Unsafe.Microscope* microscope = &_microscope)
            {
                return TriangleBuffer.FromNativeVec(
                    Unsafe.NativeMethods.microscope_present_retract(microscope, id.Inner));
            }
        }
    }

    /// <inheritdoc cref="Unsafe.NativeMethods.microscope_present_retract_at"/>
    public List<TriangleBuffer> PresentRetractAt(Id id, LinearState state)
    {
        ObjectDisposedException.ThrowIf(_disposed, this);
        using var guard = _lock.LockRef();
        unsafe
        {
            fixed (Unsafe.Microscope* microscope = &_microscope)
            {
                return TriangleBuffer.FromNativeVec(
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