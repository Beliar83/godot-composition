#nullable disable

using System;
using Godot;

namespace GDExtension.Wrappers;

public partial class ComponentsEditor : PanelContainer
{
    public static readonly StringName GDExtensionName = "ComponentsEditor";

    [Obsolete(
        "Wrapper classes cannot be constructed with Ctor (it only instantiate the underlying PanelContainer), please use the Instantiate() method instead.")]
    protected ComponentsEditor()
    { }

    /// <summary>
    ///     Creates an instance of the GDExtension <see cref="ComponentsEditor" /> type, and attaches the wrapper script to it.
    /// </summary>
    /// <returns>The wrapper instance linked to the underlying GDExtension type.</returns>
    public static ComponentsEditor Instantiate()
    {
        return GDExtensionHelper.Instantiate<ComponentsEditor>(GDExtensionName);
    }

    /// <summary>
    ///     Try to cast the script on the supplied <paramref name="godotObject" /> to the <see cref="ComponentsEditor" />
    ///     wrapper type,
    ///     if no script has attached to the type, or the script attached to the type does not inherit the
    ///     <see cref="ComponentsEditor" /> wrapper type,
    ///     a new instance of the <see cref="ComponentsEditor" /> wrapper script will get attaches to the
    ///     <paramref name="godotObject" />.
    /// </summary>
    /// <remarks>
    ///     The developer should only supply the <paramref name="godotObject" /> that represents the correct underlying
    ///     GDExtension type.
    /// </remarks>
    /// <param name="godotObject">The <paramref name="godotObject" /> that represents the correct underlying GDExtension type.</param>
    /// <returns>
    ///     The existing or a new instance of the <see cref="ComponentsEditor" /> wrapper script attached to the supplied
    ///     <paramref name="godotObject" />.
    /// </returns>
    public static ComponentsEditor Bind(GodotObject godotObject)
    {
        return godotObject is not null
            ? GDExtensionHelper.Bind<ComponentsEditor>(godotObject)
            : null;
    }

    #region Methods

    public void StoreComponents()
    {
        Call(_cached_store_components);
    }


    public void SetupUi()
    {
        Call(_cached_setup_ui);
    }

    #endregion

    private static readonly StringName _cached_store_components = "store_components";
    private static readonly StringName _cached_setup_ui = "setup_ui";
}