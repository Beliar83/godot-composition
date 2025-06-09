#nullable disable

using System;
using Godot;
using Godot.Collections;

namespace GDExtension.Wrappers;

public partial class GodotCompositionWorld : Node
{
    public static readonly StringName GDExtensionName = "GodotCompositionWorld";

    [Obsolete(
        "Wrapper classes cannot be constructed with Ctor (it only instantiate the underlying Node), please use the Instantiate() method instead.")]
    protected GodotCompositionWorld()
    { }

    /// <summary>
    ///     Creates an instance of the GDExtension <see cref="GodotCompositionWorld" /> type, and attaches the wrapper script
    ///     to it.
    /// </summary>
    /// <returns>The wrapper instance linked to the underlying GDExtension type.</returns>
    public static GodotCompositionWorld Instantiate()
    {
        return GDExtensionHelper.Instantiate<GodotCompositionWorld>(GDExtensionName);
    }

    /// <summary>
    ///     Try to cast the script on the supplied <paramref name="godotObject" /> to the <see cref="GodotCompositionWorld" />
    ///     wrapper type,
    ///     if no script has attached to the type, or the script attached to the type does not inherit the
    ///     <see cref="GodotCompositionWorld" /> wrapper type,
    ///     a new instance of the <see cref="GodotCompositionWorld" /> wrapper script will get attaches to the
    ///     <paramref name="godotObject" />.
    /// </summary>
    /// <remarks>
    ///     The developer should only supply the <paramref name="godotObject" /> that represents the correct underlying
    ///     GDExtension type.
    /// </remarks>
    /// <param name="godotObject">The <paramref name="godotObject" /> that represents the correct underlying GDExtension type.</param>
    /// <returns>
    ///     The existing or a new instance of the <see cref="GodotCompositionWorld" /> wrapper script attached to the
    ///     supplied <paramref name="godotObject" />.
    /// </returns>
    public static GodotCompositionWorld Bind(GodotObject godotObject)
    {
        return godotObject is not null
            ? GDExtensionHelper.Bind<GodotCompositionWorld>(godotObject)
            : null;
    }

    #region Signals

    private void nodeEntityCreatedCall(NodeEntity nodeEntity)
    {
        NodeEntity arg0 = GDExtensionHelper.Bind<NodeEntity>(nodeEntity);
        _nodeEntityCreated_backing?.Invoke(arg0);
    }

    public delegate void NodeEntityCreatedHandler(NodeEntity nodeEntity);

    private NodeEntityCreatedHandler _nodeEntityCreated_backing;
    private Callable _nodeEntityCreated_backing_callable;

    public event NodeEntityCreatedHandler NodeEntityCreated
    {
        add
        {
            if (_nodeEntityCreated_backing == null)
            {
                _nodeEntityCreated_backing_callable = new Callable(this, MethodName.nodeEntityCreatedCall);
                Connect(_cached_node_entity_created, _nodeEntityCreated_backing_callable);
            }

            _nodeEntityCreated_backing += value;
        }
        remove
        {
            _nodeEntityCreated_backing -= value;

            if (_nodeEntityCreated_backing == null)
            {
                Disconnect(_cached_node_entity_created, _nodeEntityCreated_backing_callable);
                _nodeEntityCreated_backing_callable = default;
            }
        }
    }

    private void componentChangedCall(NodeEntity nodeEntity, StringName componentClass, Component component,
        Component oldComponent)
    {
        NodeEntity arg0 = GDExtensionHelper.Bind<NodeEntity>(nodeEntity);
        StringName arg1 = componentClass;
        Component arg2 = GDExtensionHelper.Bind<Component>(component);
        Component arg3 = GDExtensionHelper.Bind<Component>(oldComponent);
        _componentChanged_backing?.Invoke(arg0, arg1, arg2, arg3);
    }

    public delegate void ComponentChangedHandler(NodeEntity nodeEntity, StringName componentClass, Component component,
        Component oldComponent);

    private ComponentChangedHandler _componentChanged_backing;
    private Callable _componentChanged_backing_callable;

    public event ComponentChangedHandler ComponentChanged
    {
        add
        {
            if (_componentChanged_backing == null)
            {
                _componentChanged_backing_callable = new Callable(this, MethodName.componentChangedCall);
                Connect(_cached_component_changed, _componentChanged_backing_callable);
            }

            _componentChanged_backing += value;
        }
        remove
        {
            _componentChanged_backing -= value;

            if (_componentChanged_backing == null)
            {
                Disconnect(_cached_component_changed, _componentChanged_backing_callable);
                _componentChanged_backing_callable = default;
            }
        }
    }

    #endregion

    #region Methods

    public static GodotCompositionWorld GetSingleton()
    {
        return Bind(GDExtensionHelper.Call(GDExtensionName, _cached_get_singleton).As<GodotObject>());
    }


    public Dictionary GetAllComponents()
    {
        return Call(_cached_get_all_components).As<Dictionary>();
    }


    public void DoForAllComponents(Callable func)
    {
        Call(_cached_do_for_all_components, func);
    }


    public void DoForAllComponentsOfClass(StringName componentName, Callable func)
    {
        Call(_cached_do_for_all_components_of_class, componentName, func);
    }


    public Array<Component> GetAllComponentsOfClass(StringName componentName)
    {
        return GDExtensionHelper.Cast<Component>(Call(_cached_get_all_components_of_class, componentName)
            .As<Array<GodotObject>>());
    }


    public void StoreEntitiesToScene(Node scene)
    {
        Call(_cached_store_entities_to_scene, scene);
    }


    public void SetEntitiesFromScene(Node newScene)
    {
        Call(_cached_set_entities_from_scene, newScene);
    }


    public void ClearEntitiesFromScene(Node scene)
    {
        Call(_cached_clear_entities_from_scene, scene);
    }


    public bool SetComponentOfNode(Node node, StringName componentClass, Component component)
    {
        return Call(_cached_set_component_of_node, node, componentClass, (RefCounted)component).As<bool>();
    }


    public bool NodeHasComponentOfClass(Node node, StringName componentClass)
    {
        return Call(_cached_node_has_component_of_class, node, componentClass).As<bool>();
    }


    public NodeEntity GetOrCreateNodeEntity(Node node)
    {
        return NodeEntity.Bind(Call(_cached_get_or_create_node_entity, node).As<GodotObject>());
    }


    public NodeEntity GetNodeEntityOrNull(Node node)
    {
        return NodeEntity.Bind(Call(_cached_get_node_entity_or_null, node).As<GodotObject>());
    }


    public Array<NodeEntity> GetAllNodeEntities()
    {
        return GDExtensionHelper.Cast<NodeEntity>(Call(_cached_get_all_node_entities).As<Array<GodotObject>>());
    }


    public void RemoveAllEntitiesAndPendingChanges()
    {
        Call(_cached_remove_all_entities_and_pending_changes);
    }

    #endregion

    private static readonly StringName _cached_get_singleton = "get_singleton";
    private static readonly StringName _cached_get_all_components = "get_all_components";
    private static readonly StringName _cached_do_for_all_components = "do_for_all_components";
    private static readonly StringName _cached_do_for_all_components_of_class = "do_for_all_components_of_class";
    private static readonly StringName _cached_get_all_components_of_class = "get_all_components_of_class";
    private static readonly StringName _cached_store_entities_to_scene = "store_entities_to_scene";
    private static readonly StringName _cached_set_entities_from_scene = "set_entities_from_scene";
    private static readonly StringName _cached_clear_entities_from_scene = "clear_entities_from_scene";
    private static readonly StringName _cached_set_component_of_node = "set_component_of_node";
    private static readonly StringName _cached_node_has_component_of_class = "node_has_component_of_class";
    private static readonly StringName _cached_get_or_create_node_entity = "get_or_create_node_entity";
    private static readonly StringName _cached_get_node_entity_or_null = "get_node_entity_or_null";
    private static readonly StringName _cached_get_all_node_entities = "get_all_node_entities";

    private static readonly StringName _cached_remove_all_entities_and_pending_changes =
        "remove_all_entities_and_pending_changes";

    private static readonly StringName _cached_node_entity_created = "node_entity_created";
    private static readonly StringName _cached_component_changed = "component_changed";
}