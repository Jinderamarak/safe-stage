namespace BindingsCs.Safe;

public class MutRefLock
{
    private readonly ReaderWriterLockSlim _rwl = new();

    public MutLockGuard LockMut()
    {
        return new MutLockGuard(_rwl);
    }

    public RefLockGuard LockRef()
    {
        return new RefLockGuard(_rwl);
    }
}

public class MutLockGuard : IDisposable
{
    private readonly ReaderWriterLockSlim _rwl;

    internal MutLockGuard(ReaderWriterLockSlim rwl)
    {
        _rwl = rwl;
        _rwl.EnterWriteLock();
    }

    public void Dispose()
    {
        _rwl.ExitWriteLock();
    }
}

public class RefLockGuard : IDisposable
{
    private readonly ReaderWriterLockSlim _rwl;

    internal RefLockGuard(ReaderWriterLockSlim rwl)
    {
        _rwl = rwl;
        _rwl.EnterReadLock();
    }

    public void Dispose()
    {
        _rwl.ExitReadLock();
    }
}