#nullable disable

using Godot;
using Godot.Collections;

namespace GDExtension.Wrappers;

public partial class NodeEntity : RefCounted
{
    public static readonly StringName GDExtensionName = "NodeEntity";

    /// <summary>
    ///     Creates an instance of the GDExtension <see cref="NodeEntity" /> type, and attaches the wrapper script to it.
    /// </summary>
    /// <returns>The wrapper instance linked to the underlying GDExtension type.</returns>
    public static NodeEntity Instantiate()
    {
        return GDExtensionHelper.Instantiate<NodeEntity>(GDExtensionName);
    }

    /// <summary>
    ///     Try to cast the script on the supplied <paramref name="godotObject" /> to the <see cref="NodeEntity" /> wrapper
    ///     type,
    ///     if no script has attached to the type, or the script attached to the type does not inherit the
    ///     <see cref="NodeEntity" /> wrapper type,
    ///     a new instance of the <see cref="NodeEntity" /> wrapper script will get attaches to the
    ///     <paramref name="godotObject" />.
    /// </summary>
    /// <remarks>
    ///     The developer should only supply the <paramref name="godotObject" /> that represents the correct underlying
    ///     GDExtension type.
    /// </remarks>
    /// <param name="godotObject">The <paramref name="godotObject" /> that represents the correct underlying GDExtension type.</param>
    /// <returns>
    ///     The existing or a new instance of the <see cref="NodeEntity" /> wrapper script attached to the supplied
    ///     <paramref name="godotObject" />.
    /// </returns>
    public static NodeEntity Bind(GodotObject godotObject)
    {
        return godotObject is not null
            ? GDExtensionHelper.Bind<NodeEntity>(godotObject)
            : null;
    }

    #region Properties

    public Node Node
    {
        get => (Node)Get(_cached_node);
        set => Set(_cached_node, Variant.From(value));
    }

    #endregion

    #region Signals

    private void componentRemovedCall(StringName componentClass)
    {
        StringName arg0 = componentClass;
        _componentRemoved_backing?.Invoke(arg0);
    }

    public delegate void ComponentRemovedHandler(StringName componentClass);

    private ComponentRemovedHandler _componentRemoved_backing;
    private Callable _componentRemoved_backing_callable;

    /// <summary>
    ///     Emitted when a component was removed from this Entity
    /// </summary>
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

    private void componentAddedCall(StringName componentClass, Component component)
    {
        StringName arg0 = componentClass;
        Component arg1 = GDExtensionHelper.Bind<Component>(component);
        _componentAdded_backing?.Invoke(arg0, arg1);
    }

    public delegate void ComponentAddedHandler(StringName componentClass, Component component);

    private ComponentAddedHandler _componentAdded_backing;
    private Callable _componentAdded_backing_callable;

    /// <summary>
    ///     Emitted when a component was added to this Entity
    /// </summary>
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

    #endregion

    #region Methods

    /// <summary>
    ///     Execute a callable for all components of this Entity
    ///     The signature of the callable must be:
    ///     (component_class: StringName, component: Component)
    /// </summary>
    public void DoForAllComponents(Callable func)
    {
        Call(_cached_do_for_all_components, func);
    }


    /// <summary>
    ///     Return all components as a list of dictionaries
    /// </summary>
    public Array<Dictionary> GetAllComponents()
    {
        return Call(_cached_get_all_components).As<Array<Dictionary>>();
    }


    /// <summary>
    ///     Sets components from a list of dictionaries
    /// </summary>
    public Array<StringName> SetComponents(Array<Dictionary> components)
    {
        return Call(_cached_set_components, components).As<Array<StringName>>();
    }


    /// <summary>
    ///     Returns if the Entity has a component of the given component class
    /// </summary>
    /// <param name="componentClass"></param>
    /// <returns></returns>
    public bool HasComponentOfClass(StringName componentClass)
    {
        return Call(_cached_has_component_of_class, componentClass).As<bool>();
    }


    /// <summary>
    ///     Returns the component of the given component class, or null if the Entity does not have a component of that class
    /// </summary>
    /// <param name="componentClass"></param>
    /// <returns></returns>
    public Component GetComponentOfClassOrNull(StringName componentClass)
    {
        return Component.Bind(Call(_cached_get_component_of_class_or_null, componentClass).As<GodotObject>());
    }


    public Node GetNode()
    {
        return GDExtensionHelper.Bind<Node>(Call(_cached_get_node).As<GodotObject>());
    }


    public void SetNode(Node node)
    {
        Call(_cached_set_node, node);
    }

    #endregion

    private static readonly StringName _cached_node = "node";
    private static readonly StringName _cached_do_for_all_components = "do_for_all_components";
    private static readonly StringName _cached_get_all_components = "get_all_components";
    private static readonly StringName _cached_set_components = "set_components";
    private static readonly StringName _cached_has_component_of_class = "has_component_of_class";
    private static readonly StringName _cached_get_component_of_class_or_null = "get_component_of_class_or_null";
    private static readonly StringName _cached_get_node = "get_node";
    private static readonly StringName _cached_set_node = "set_node";
    private static readonly StringName _cached_component_removed = "component_removed";
    private static readonly StringName _cached_component_added = "component_added";
}