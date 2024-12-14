using System.Reflection;
using BindingsCs.Safe.Types;
using ServiceApp.Config.Fields;

namespace ServiceApp.Config;

/// <summary>
/// Represents a variant of a configuration (usually static method) as configurable object with fields.
/// </summary>
public class ConfigVariant
{
    public string Name => _methodInfo.Name;
    public IReadOnlyList<IField> Fields => _fields.AsReadOnly();

    private readonly Type _classType;
    private readonly MethodInfo _methodInfo;
    private readonly List<IField> _fields;

    private ConfigVariant(Type classType, MethodInfo methodInfo, List<IField> fields)
    {
        _classType = classType;
        _methodInfo = methodInfo;
        _fields = fields;
    }

    public T? Construct<T>() where T : class
    {
        var constructed = _methodInfo.Invoke(null, _fields.Select((f) => f.GetValue()).ToArray());
        return constructed as T;
    }

    public static ConfigVariant FromMethodName(Type classType, string methodName)
    {
        var methodInfo = classType.GetMethod(methodName);
        if (methodInfo == null) throw new ArgumentException($"Method {methodName} not found in {classType.Name}");

        return FromMethod(classType, methodInfo);
    }

    public static ConfigVariant FromMethod(Type classType, MethodInfo methodInfo)
    {
        var fields = new List<IField>();
        foreach (var parameter in methodInfo.GetParameters())
        {
            if (parameter.ParameterType == typeof(double))
            {
                fields.Add(new DoubleField
                {
                    Label = parameter.Name ?? "Unnammed"
                });
                continue;
            }

            if (parameter.ParameterType == typeof(LinearState))
            {
                fields.Add(new LinearStateField
                {
                    Label = parameter.Name ?? "Unnammed"
                });
                continue;
            }

            if (parameter.ParameterType == typeof(Vector3))
            {
                fields.Add(new VectorField
                {
                    Label = parameter.Name ?? "Unnammed"
                });
                continue;
            }

            if (parameter.ParameterType == typeof(SixAxis))
            {
                fields.Add(new SixAxisField
                {
                    Label = parameter.Name ?? "Unnammed"
                });
                continue;
            }

            throw new NotSupportedException($"Type {parameter.ParameterType} is not supported");
        }

        return new ConfigVariant(classType, methodInfo, fields);
    }

    public ConfigVariant Cloned()
    {
        return new ConfigVariant(_classType, _methodInfo, _fields.Select((f) => (IField)f.Clone()).ToList());
    }

    public override bool Equals(object? obj)
    {
        if (obj is not ConfigVariant other) return false;
        if (_classType != other._classType) return false;
        if (_methodInfo != other._methodInfo) return false;
        return true;
    }

    public override int GetHashCode()
    {
        return HashCode.Combine(_classType, _methodInfo);
    }

    public override string ToString()
    {
        return Name;
    }
}