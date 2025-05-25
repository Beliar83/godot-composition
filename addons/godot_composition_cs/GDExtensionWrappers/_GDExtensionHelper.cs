#nullable disable

using System;
using System.Collections.Concurrent;
using System.Linq;
using System.Reflection;
using Godot;
using Godot.Collections;

public static class GDExtensionHelper
{
    private static readonly ConcurrentDictionary<string, GodotObject> _instances = [];
    private static readonly ConcurrentDictionary<Type, Variant> _scripts = [];

    /// <summary>
    ///     Calls a static method within the given type.
    /// </summary>
    /// <param name="className">The type name.</param>
    /// <param name="method">The method name.</param>
    /// <param name="arguments">The arguments.</param>
    /// <returns>The return value of the method.</returns>
    public static Variant Call(StringName className, StringName method, params Variant[] arguments)
    {
        return _instances.GetOrAdd(className, InstantiateStaticFactory).Call(method, arguments);
    }

    private static GodotObject InstantiateStaticFactory(string className)
    {
        return ClassDB.Instantiate(className).As<GodotObject>();
    }

    /// <summary>
    ///     Try to cast the script on the supplied <paramref name="godotObject" /> to the <typeparamref name="T" /> wrapper
    ///     type,
    ///     if no script has attached to the type, or the script attached to the type does not inherit the
    ///     <typeparamref name="T" /> wrapper type,
    ///     a new instance of the <typeparamref name="T" /> wrapper script will get attaches to the
    ///     <paramref name="godotObject" />.
    /// </summary>
    /// <remarks>
    ///     The developer should only supply the <paramref name="godotObject" /> that represents the correct underlying
    ///     GDExtension type.
    /// </remarks>
    /// <param name="godotObject">The <paramref name="godotObject" /> that represents the correct underlying GDExtension type.</param>
    /// <returns>
    ///     The existing or a new instance of the <typeparamref name="T" /> wrapper script attached to the supplied
    ///     <paramref name="godotObject" />.
    /// </returns>
    public static T Bind<T>(GodotObject godotObject) where T : GodotObject
    {
#if DEBUG
        if (!GodotObject.IsInstanceValid(godotObject))
        {
            throw new ArgumentException(nameof(godotObject), "The supplied GodotObject is not valid.");
        }
#endif
        if (godotObject is T wrapperScript)
        {
            return wrapperScript;
        }

        Type type = typeof(T);
#if DEBUG
        string className = godotObject.GetClass();
        bool inherits = ClassDB.IsParentClass(type.Name, className);
        if (!inherits)
        {
            Type baseType = type.BaseType;
            while (baseType is not null && !inherits)
            {
                inherits = ClassDB.IsParentClass(baseType.Name, className);
                baseType = type.BaseType;
            }
        }

        if (!inherits)
        {
            throw new ArgumentException(nameof(godotObject),
                $"The supplied GodotObject {className} is not a {type.Name}.");
        }
#endif
        Variant script = _scripts.GetOrAdd(type, GetScriptFactory);
        ulong instanceId = godotObject.GetInstanceId();
        godotObject.SetScript(script);
        return (T)GodotObject.InstanceFromId(instanceId);
    }

    private static Variant GetScriptFactory(Type type)
    {
        ScriptPathAttribute scriptPath = type.GetCustomAttributes<ScriptPathAttribute>().FirstOrDefault();
        return scriptPath is null ? null : ResourceLoader.Load(scriptPath.Path);
    }

    public static Array<T> Cast<[MustBeVariant] T>(Array<GodotObject> godotObjects) where T : GodotObject
    {
        return new Array<T>(godotObjects.Select(Bind<T>));
    }

    /// <summary>
    ///     Creates an instance of the GDExtension
    ///     <typeparam name="T" />
    ///     type, and attaches the wrapper script to it.
    /// </summary>
    /// <returns>The wrapper instance linked to the underlying GDExtension type.</returns>
    public static T Instantiate<T>(StringName className) where T : GodotObject
    {
        return Bind<T>(ClassDB.Instantiate(className).As<GodotObject>());
    }
}