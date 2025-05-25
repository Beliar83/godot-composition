using System.Diagnostics.CodeAnalysis;
using Godot;

namespace GDExtension.Wrappers;

[Tool]
[SuppressMessage("ReSharper", "VirtualMemberNeverOverridden.Global")]
public partial class Component
{
    /// <summary>
    ///     Called during _Process of the normal engine loop
    /// </summary>
    protected virtual void _Process(float delta, Node node, NodeEntity nodeEntity)
    { }

    private void _process(float delta, Node node, RefCounted nodeEntity)
    {
        _Process(delta, node, NodeEntity.Bind(nodeEntity));
    }

    /// <summary>
    ///     Called during _PhysicsProcess of the normal engine loop
    /// </summary>
    protected virtual void _PhysicsProcess(float delta, Node node, NodeEntity nodeEntity)
    { }

    private void _physics_process(float delta, Node node, RefCounted nodeEntity)
    {
        _PhysicsProcess(delta, node, NodeEntity.Bind(nodeEntity));
    }

    /// <summary>
    ///     Called when the entity if the component is changed
    /// </summary>
    protected virtual void _EntityChanged(NodeEntity nodeEntity)
    { }

    private void _entity_changed(RefCounted nodeEntity)
    {
        NodeEntity entity = NodeEntity.Bind(nodeEntity);
        _EntityChanged(entity);
    }
}