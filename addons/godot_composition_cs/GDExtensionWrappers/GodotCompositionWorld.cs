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

    private void componentAddedCall(NodeEntity nodeEntity, StringName componentClass, Component component)
    {
        NodeEntity arg0 = GDExtensionHelper.Bind<NodeEntity>(nodeEntity);
        StringName arg1 = componentClass;
        Component arg2 = GDExtensionHelper.Bind<Component>(component);
        _componentAdded_backing?.Invoke(arg0, arg1, arg2);
    }

    public delegate void ComponentAddedHandler(NodeEntity nodeEntity, StringName componentClass, Component component);

    private ComponentAddedHandler _componentAdded_backing;
    private Callable _componentAdded_backing_callable;

    public event ComponentAddedHandler ComponentAdded
    {
        add
        {
            if (_componentAdded_backing == null)
            {
                _componentAdded_backing_callable = new Callable(this, MethodName.componentAddedCall);
                Connect(_cached_component_added, _componentAdded_backing_callable);
            }

            _componentAdded_backing += value;
        }
        remove
        {
            _componentAdded_backing -= value;

            if (_componentAdded_backing == null)
            {
                Disconnect(_cached_component_added, _componentAdded_backing_callable);
                _componentAdded_backing_callable = default;
            }
        }
    }

    private void componentRemovedCall(NodeEntity nodeEntity, StringName componentClass, Component component)
    {
        NodeEntity arg0 = GDExtensionHelper.Bind<NodeEntity>(nodeEntity);
        StringName arg1 = componentClass;
        Component arg2 = GDExtensionHelper.Bind<Component>(component);
        _componentRemoved_backing?.Invoke(arg0, arg1, arg2);
    }

    public delegate void ComponentRemovedHandler(NodeEntity nodeEntity, StringName componentClass, Component component);

    private ComponentRemovedHandler _componentRemoved_backing;
    private Callable _componentRemoved_backing_callable;

    public event ComponentRemovedHandler ComponentRemoved
    {
        add
        {
            if (_componentRemoved_backing == null)
            {
                _componentRemoved_backing_callable = new Callable(this, MethodName.componentRemovedCall);
                Connect(_cached_component_removed, _componentRemoved_backing_callable);
            }

            _componentRemoved_backing += value;
        }
        remove
        {
            _componentRemoved_backing -= value;

            if (_componentRemoved_backing == null)
            {
                Disconnect(_cached_component_removed, _componentRemoved_backing_callable);
                _componentRemoved_backing_callable = default;
            }
        }
    }

    private void componentReplacedCall(NodeEntity nodeEntity, StringName componentClass, Component component)
    {
        NodeEntity arg0 = GDExtensionHelper.Bind<NodeEntity>(nodeEntity);
        StringName arg1 = componentClass;
        Component arg2 = GDExtensionHelper.Bind<Component>(component);
        _componentReplaced_backing?.Invoke(arg0, arg1, arg2);
    }

    public delegate void
        ComponentReplacedHandler(NodeEntity nodeEntity, StringName componentClass, Component component);

    private ComponentReplacedHandler _componentReplaced_backing;
    private Callable _componentReplaced_backing_callable;

    public event ComponentReplacedHandler ComponentReplaced
    {
        add
        {
            if (_componentReplaced_backing == null)
            {
                _componentReplaced_backing_callable = new Callable(this, MethodName.componentReplacedCall);
                Connect(_cached_component_replaced, _componentReplaced_backing_callable);
            }

            _componentReplaced_backing += value;
        }
        remove
        {
            _componentReplaced_backing -= value;

            if (_componentReplaced_backing == null)
            {
                Disconnect(_cached_component_replaced, _componentReplaced_backing_callable);
                _componentReplaced_backing_callable = default;
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


    public bool AddComponentToNode(Component component, Node node, StringName componentClass)
    {
        return Call(_cached_add_component_to_node, (RefCounted)component, node, componentClass).As<bool>();
    }


    public bool RemoveComponentFromNode(Node node, StringName componentClass)
    {
        return Call(_cached_remove_component_from_node, node, componentClass).As<bool>();
    }


    public bool ReplaceComponentOfNode(Component component, Node node, StringName componentClass)
    {
        return Call(_cached_replace_component_of_node, (RefCounted)component, node, componentClass).As<bool>();
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


    public Array<NodeEntity> GetAllNodeEntity()
    {
        return GDExtensionHelper.Cast<NodeEntity>(Call(_cached_get_all_node_entity).As<Array<GodotObject>>());
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
    private static readonly StringName _cached_add_component_to_node = "add_component_to_node";
    private static readonly StringName _cached_remove_component_from_node = "remove_component_from_node";
    private static readonly StringName _cached_replace_component_of_node = "replace_component_of_node";
    private static readonly StringName _cached_node_has_component_of_class = "node_has_component_of_class";
    private static readonly StringName _cached_get_or_create_node_entity = "get_or_create_node_entity";
    private static readonly StringName _cached_get_node_entity_or_null = "get_node_entity_or_null";
    private static readonly StringName _cached_get_all_node_entity = "get_all_node_entity";

    private static readonly StringName _cached_remove_all_entities_and_pending_changes =
        "remove_all_entities_and_pending_changes";

    private static readonly StringName _cached_node_entity_created = "node_entity_created";
    private static readonly StringName _cached_component_added = "component_added";
    private static readonly StringName _cached_component_removed = "component_removed";
    private static readonly StringName _cached_component_replaced = "component_replaced";
}