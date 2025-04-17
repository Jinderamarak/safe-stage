using System;
using System.Linq;
using System.Threading.Tasks;
using BindingsCs.Safe.Types;
using MsBox.Avalonia;
using MsBox.Avalonia.Enums;

namespace ServiceApp.Avalonia.Utility.Extensions;

public static class PathResultExtensions
{
    public static string ToStatusMessage(this PathResult<SixAxis> path)
    {
        return (path.ResultState, path.Nodes.Any()) switch
        {
            (PathResultState.Success, _) => "Found full path.",
            (PathResultState.InvalidStart, _) => "Invalid start state, no path available.",
            (PathResultState.UnreachableEnd, false) => "Unreachable end state, no path available.",
            (PathResultState.UnreachableEnd, true) =>
                $"Unreachable end state, found partial path to {path.Nodes.Last().ToFormattedString()}",
            _ => throw new ArgumentOutOfRangeException()
        };
    }

    public static void ShowMessageBox(this PathResult<SixAxis> path)
    {
        if (path.ResultState is PathResultState.Success)
            return;

        var icon = (path.ResultState, path.Nodes.Any()) switch
        {
            (PathResultState.InvalidStart, _) => Icon.Error,
            (PathResultState.UnreachableEnd, false) => Icon.Error,
            (PathResultState.UnreachableEnd, true) => Icon.Warning,
            _ => throw new ArgumentOutOfRangeException()
        };

        Task.WaitAll(
            MessageBoxManager.GetMessageBoxStandard("Path Finding Failed", path.ToStatusMessage(), ButtonEnum.Ok, icon)
                .ShowAsync()
        );
    }

    public static bool? IsInserting(this PathResult<LinearState> path)
    {
        var nodes = path.Nodes.ToList();
        if (nodes.Count < 2)
            return null;

        var first = nodes.First();
        var last = nodes.Last();
        return first.T < last.T;
    }

    public static string ToStatusMessage(this PathResult<LinearState> path)
    {
        return (path.ResultState, path.Nodes.Any(), path.IsInserting()) switch
        {
            (PathResultState.Success, _, true) => "Found full path, can be fully inserted.",
            (PathResultState.Success, _, false) => "Found full path, can be fully retracted.",
            (PathResultState.InvalidStart, _, _) => "Invalid start state, no action available.",
            (PathResultState.UnreachableEnd, false, _) or (PathResultState.UnreachableEnd, true, null) =>
                "Unreachable end state, no action available.",
            (PathResultState.UnreachableEnd, true, true) =>
                $"Unreachable end state, can be moved to {path.Nodes.Last().ToFormattedString()}.",
            (PathResultState.UnreachableEnd, true, false) =>
                $"Unreachable end state, can be moved to {path.Nodes.Last().ToFormattedString(false)}.",
            _ => throw new ArgumentOutOfRangeException()
        };
    }

    public static void ShowMessageBox(this PathResult<LinearState> path)
    {
        var icon = (path.ResultState, path.Nodes.Any()) switch
        {
            (PathResultState.Success, _) => Icon.Info,
            (PathResultState.InvalidStart, _) => Icon.Error,
            (PathResultState.UnreachableEnd, false) => Icon.Error,
            (PathResultState.UnreachableEnd, true) => Icon.Warning,
            _ => throw new ArgumentOutOfRangeException()
        };

        Task.WaitAll(
            MessageBoxManager.GetMessageBoxStandard("Path Finding Result", path.ToStatusMessage(), ButtonEnum.Ok, icon)
                .ShowAsync()
        );
    }
}