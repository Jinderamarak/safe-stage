namespace BindingsCs.Safe.Types;

public enum PathResult
{
    Success,
    InvalidStart,
    UnreachableEnd,
}

internal static class PathResultExtension
{
    internal static PathResult FromNative(Unsafe.PathResultState state)
    {
        #pragma warning disable CS8524
        return state switch
        {
            Unsafe.PathResultState.Path => PathResult.Success,
            Unsafe.PathResultState.InvalidStart => PathResult.InvalidStart,
            Unsafe.PathResultState.UnreachableEnd => PathResult.UnreachableEnd,
        };
        #pragma warning restore
    }
}

public class PathResultSixAxis
{
    public PathResult Result { get; private set; }
    public IEnumerable<SixAxis> Nodes => _nodes;

    private readonly SixAxis[] _nodes;

    internal PathResultSixAxis(Unsafe.CPathResultSixAxis nativePath)
    {
        Result = PathResultExtension.FromNative(nativePath.state);
        _nodes = new SixAxis[nativePath.len];
        unsafe
        {
            for (uint i = 0; i < nativePath.len; i++)
            {
                _nodes[i] = new SixAxis(nativePath.nodes[i]);
            }
        }
        
        Unsafe.NativeMethods.cpathresultsixaxis_drop(nativePath);
    }
}

public class PathResultLinearState
{
    public PathResult Result { get; private set; }
    public IEnumerable<LinearState> Nodes => _nodes;
    
    private readonly LinearState[] _nodes;
    
    internal PathResultLinearState(Unsafe.CPathResultLinearState nativePath)
    {
        Result = PathResultExtension.FromNative(nativePath.state);
        _nodes = new LinearState[nativePath.len];
        unsafe
        {
            for (uint i = 0; i < nativePath.len; i++)
            {
                _nodes[i] = new LinearState(nativePath.nodes[i]);
            }
        }
        
        Unsafe.NativeMethods.cpathresultlinearstate_drop(nativePath);
    }
}