using System;
using System.Collections.Generic;
using System.Threading.Tasks;
using Avalonia.Threading;

namespace ServiceApp.Avalonia.Tasks;

public class TaskChain
{
    private List<(TaskVariant, Func<object?, object?>)> _tasks = new();

    public TaskChain OnUi(Action action)
    {
        AddTask(TaskVariant.Ui, action);
        return this;
    }

    public TaskChain OnUi<I>(Action<I> action)
    {
        AddTask(TaskVariant.Ui, action);
        return this;
    }

    public TaskChain OnUi<O>(Func<O> action)
    {
        AddTask(TaskVariant.Ui, action);
        return this;
    }

    public TaskChain OnUi<I, O>(Func<I, O> action)
    {
        AddTask(TaskVariant.Ui, action);
        return this;
    }

    public TaskChain InBack(Action action)
    {
        AddTask(TaskVariant.Background, action);
        return this;
    }

    public TaskChain InBack<I>(Action<I> action)
    {
        AddTask(TaskVariant.Background, action);
        return this;
    }

    public TaskChain InBack<O>(Func<O> action)
    {
        AddTask(TaskVariant.Background, action);
        return this;
    }

    public TaskChain InBack<I, O>(Func<I, O> action)
    {
        AddTask(TaskVariant.Background, action);
        return this;
    }

    public TaskChain Chain(TaskChain taskChain)
    {
        _tasks.AddRange(taskChain._tasks);
        return this;
    }

    public Task Execute(Dispatcher uiDispatch)
    {
        return Task.Run(() =>
        {
            object? last = null;
            foreach (var (variant, action) in _tasks)
                switch (variant)
                {
                    case TaskVariant.Ui:
                        last = uiDispatch.Invoke(() => action(last));
                        break;
                    case TaskVariant.Background:
                        last = action(last);
                        break;
                    default:
                        throw new ArgumentOutOfRangeException();
                }
        });
    }

    private void AddTask(TaskVariant variant, Action action)
    {
        _tasks.Add((variant, _ =>
        {
            action();
            return null;
        }));
    }

    private void AddTask<I>(TaskVariant variant, Action<I> action)
    {
        _tasks.Add((variant, arg =>
        {
            action((I)arg!);
            return null;
        }));
    }

    private void AddTask<O>(TaskVariant variant, Func<O> action)
    {
        _tasks.Add((variant, _ => action()));
    }

    private void AddTask<I, O>(TaskVariant variant, Func<I, O> action)
    {
        _tasks.Add((variant, arg => action((I)arg!)));
    }
}