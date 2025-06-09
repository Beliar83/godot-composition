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

    public void DoForAllComponents(Callable func)
    {
        Call(_cached_do_for_all_components, func);
    }


    public Array<Dictionary> GetAllComponents()
    {
        return Call(_cached_get_all_components).As<Array<Dictionary>>();
    }


    public Array<StringName> SetComponents(Array<Dictionary> components)
    {
        return Call(_cached_set_components, components).As<Array<StringName>>();
    }


    public bool HasComponentOfClass(StringName componentClass)
    {
        return Call(_cached_has_component_of_class, componentClass).As<bool>();
    }


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
    private static readonly StringName _cached_component_changed = "component_changed";
}