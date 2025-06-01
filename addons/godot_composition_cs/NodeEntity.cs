using System;
using System.Collections.Generic;
using Godot;

namespace GDExtension.Wrappers;

[Tool]
public partial class NodeEntity
{
    private static GodotCompositionWorld World => GodotCompositionWorld.GetSingleton();

    private readonly Dictionary<StringName, Component?> components = new();

    [Obsolete(
        "Wrapper classes cannot be constructed with Ctor (it only instantiate the underlying RefCounted), please use the Instantiate() method instead.")]
    protected NodeEntity()
    {
        ComponentAdded += OnComponentAdded;
        ComponentRemoved += OnComponentRemoved;
        ComponentReplaced += OnComponentReplaced;
    }

    /// <inheritdoc />
    protected override void Dispose(bool disposing)
    {
        try
        {
            ComponentAdded -= OnComponentAdded;
            ComponentRemoved -= OnComponentRemoved;
            ComponentReplaced -= OnComponentReplaced;
        }
        catch (ObjectDisposedException)
        { }

        base.Dispose(disposing);
    }

    /// <inheritdoc />
    public override void _Notification(int what)
    {
        if (what == NotificationPredelete)
        { }
    }

    /// <summary>
    ///     Emitted when a component was added to this Entity
    /// </summary>
    private void OnComponentAdded(StringName componentClass, Component component)
    {
        components[componentClass] = component;
    }

    /// <summary>
    ///     Emitted when a component was removed from this Entity
    /// </summary>
    private void OnComponentRemoved(StringName componentClass)
    {
        components.Remove(componentClass);
    }

    /// <summary>
    ///     /// Emitted when a component was replaced on this Entity
    /// </summary>
    private void OnComponentReplaced(StringName componentClass, Component component)
    {
        components[componentClass] = component;
    }


    /// <summary>
    ///     Returns whether this Entity has a component of the given class
    /// </summary>
    public bool HasComponent<T>() where T : Component
    {
        StringName componentClassName = World.GetComponentClassName<T>();
        return components.ContainsKey(componentClassName) || HasComponentOfClass(componentClassName);
    }

    /// <summary>
    ///     Returns the component of the given class, or null if this Entity does not have a component of the given class
    /// </summary>
    public T? GetComponentOrNull<T>() where T : Component
    {
        if (components.TryGetValue(World.GetComponentClassName<T>(), out Component? foundComponent))
        {
            return foundComponent as T;
        }

        StringName componentClassName = World.GetComponentClassName<T>();
        T? component = GetComponentOfClassOrNull(componentClassName) as T;
        components[componentClassName] = component;
        return component;
    }
}