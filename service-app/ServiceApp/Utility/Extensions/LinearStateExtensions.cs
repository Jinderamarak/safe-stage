using BindingsCs.Safe.Types;

namespace ServiceApp.Utility.Extensions;

public static class LinearStateExtensions
{
    public static string ToFormattedString(this LinearState state, bool inserting = true)
    {
        return (state.T, inserting) switch
        {
            (0, _) => "Retracted",
            (1, _) => "Inserted",
            (_, true) => $"{state.T:P0} inserted",
            (_, false) => $"{1 - state.T:P0} retracted"
        };
    }
}