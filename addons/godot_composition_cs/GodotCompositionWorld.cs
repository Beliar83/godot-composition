using System;
using Godot;
using Godot.Collections;

namespace GDExtension.Wrappers;

[Tool]
public partial class GodotCompositionWorld
{
    private readonly System.Collections.Generic.Dictionary<Type, StringName> typeNames = [];

    /// <summary>
    ///     Execute the given function for all components
    /// </summary>
    public void DoForAllComponents(Action<StringName, Component> func)
    {
        DoForAllComponents(Callable.From((StringName componentClass, RefCounted component) =>
        {
            func.Invoke(componentClass, Component.Bind(component));
        }));
    }

    /// <summary>
    ///     Execute the given function for all components of the given class
    /// </summary>
    public void DoForAllComponents<T>(Action<T> func) where T : Component
    {
        StringName componentName = GetComponentClassName<T>();
        DoForAllComponentsOfClass(componentName,
            Callable.From((RefCounted component) => func.Invoke(GDExtensionHelper.Bind<T>(component))));
    }

    /// <summary>
    ///     Get all components of the given class
    /// </summary>
    public Array<Component> GetAllComponents<T>() where T : Component
    {
        StringName componentName = GetComponentClassName<T>();
        return GetAllComponentsOfClass(componentName);
    }


    /// <summary>
    ///     Get the component class name for the given Component
    /// </summary>
    public StringName GetComponentClassName<T>() where T : Component
    {
        if (typeNames.TryGetValue(typeof(T), out StringName? componentName))
        {
            return componentName;
        }

        componentName = new StringName(typeof(T).Name);
        typeNames.Add(typeof(T), componentName);

        return componentName;
    }
}