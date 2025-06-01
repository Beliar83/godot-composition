#nullable disable

using System;
using Godot;

namespace GDExtension.Wrappers;

public partial class Component : RefCounted
{
    public static readonly StringName GDExtensionName = "Component";

    [Obsolete(
        "Wrapper classes cannot be constructed with Ctor (it only instantiate the underlying RefCounted), please use the Instantiate() method instead.")]
    protected Component()
    { }

    /// <summary>
    ///     Creates an instance of the GDExtension <see cref="Component" /> type, and attaches the wrapper script to it.
    /// </summary>
    /// <returns>The wrapper instance linked to the underlying GDExtension type.</returns>
    public static Component Instantiate()
    {
        return GDExtensionHelper.Instantiate<Component>(GDExtensionName);
    }

    /// <summary>
    ///     Try to cast the script on the supplied <paramref name="godotObject" /> to the <see cref="Component" /> wrapper
    ///     type,
    ///     if no script has attached to the type, or the script attached to the type does not inherit the
    ///     <see cref="Component" /> wrapper type,
    ///     a new instance of the <see cref="Component" /> wrapper script will get attaches to the
    ///     <paramref name="godotObject" />.
    /// </summary>
    /// <remarks>
    ///     The developer should only supply the <paramref name="godotObject" /> that represents the correct underlying
    ///     GDExtension type.
    /// </remarks>
    /// <param name="godotObject">The <paramref name="godotObject" /> that represents the correct underlying GDExtension type.</param>
    /// <returns>
    ///     The existing or a new instance of the <see cref="Component" /> wrapper script attached to the supplied
    ///     <paramref name="godotObject" />.
    /// </returns>
    public static Component Bind(GodotObject godotObject)
    {
        return godotObject is not null
            ? GDExtensionHelper.Bind<Component>(godotObject)
            : null;
    }

    #region Methods

    public void EntityChanged(NodeEntity nodeEntity)
    {
        Call(_cached__entity_changed, (RefCounted)nodeEntity);
    }


    public void Process(float delta, Node node, NodeEntity nodeEntity)
    {
        Call(_cached__process, delta, node, (RefCounted)nodeEntity);
    }


    public void PhysicsProcess(float delta, Node node, NodeEntity nodeEntity)
    {
        Call(_cached__physics_process, delta, node, (RefCounted)nodeEntity);
    }


    public NodeEntity GetNodeEntity()
    {
        return NodeEntity.Bind(Call(_cached_get_node_entity).As<GodotObject>());
    }

    #endregion

    private static readonly StringName _cached__entity_changed = "_entity_changed";
    private static readonly StringName _cached__process = "_process";
    private static readonly StringName _cached__physics_process = "_physics_process";
    private static readonly StringName _cached_get_node_entity = "get_node_entity";
}