namespace BindingsCs.Safe.Types;

public enum PathResultState
{
    Success,
    InvalidStart,
    UnreachableEnd
}

internal static class PathResultExtension
{
    internal static PathResultState FromNative(Unsafe.PathResultState state)
    {
#pragma warning disable CS8524
        return state switch
        {
            Unsafe.PathResultState.Path => PathResultState.Success,
            Unsafe.PathResultState.InvalidStart => PathResultState.InvalidStart,
            Unsafe.PathResultState.UnreachableEnd => PathResultState.UnreachableEnd
        };
#pragma warning restore
    }
}

public class PathResult<T>
{
    public PathResultState ResultState { get; private set; }
    public IEnumerable<T> Nodes => _nodes;

    private readonly T[] _nodes;

    private PathResult(PathResultState state, T[] nodes)
    {
        ResultState = state;
        _nodes = nodes;
    }

    internal static PathResult<SixAxis> FromNative(Unsafe.CPathResultSixAxis nativePath)
    {
        var state = PathResultExtension.FromNative(nativePath.state);
        var nodes = new SixAxis[nativePath.len];
        unsafe
        {
            for (uint i = 0; i < nativePath.len; i++) nodes[i] = new SixAxis(nativePath.nodes[i]);
        }

        Unsafe.NativeMethods.cpathresultsixaxis_drop(nativePath);
        return new PathResult<SixAxis>(state, nodes);
    }

    internal static PathResult<LinearState> FromNative(Unsafe.CPathResultLinearState nativePath)
    {
        var state = PathResultExtension.FromNative(nativePath.state);
        var nodes = new LinearState[nativePath.len];
        unsafe
        {
            for (uint i = 0; i < nativePath.len; i++) nodes[i] = new LinearState(nativePath.nodes[i]);
        }

        Unsafe.NativeMethods.cpathresultlinearstate_drop(nativePath);
        return new PathResult<LinearState>(state, nodes);
    }
}