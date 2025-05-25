#nullable disable

using System;
using Godot;

namespace GDExtension.Wrappers;

public partial class InspectorPlugin : EditorInspectorPlugin
{
    public static readonly StringName GDExtensionName = "InspectorPlugin";

    [Obsolete(
        "Wrapper classes cannot be constructed with Ctor (it only instantiate the underlying EditorInspectorPlugin), please use the Instantiate() method instead.")]
    protected InspectorPlugin()
    { }

    /// <summary>
    ///     Creates an instance of the GDExtension <see cref="InspectorPlugin" /> type, and attaches the wrapper script to it.
    /// </summary>
    /// <returns>The wrapper instance linked to the underlying GDExtension type.</returns>
    public static InspectorPlugin Instantiate()
    {
        return GDExtensionHelper.Instantiate<InspectorPlugin>(GDExtensionName);
    }

    /// <summary>
    ///     Try to cast the script on the supplied <paramref name="godotObject" /> to the <see cref="InspectorPlugin" />
    ///     wrapper type,
    ///     if no script has attached to the type, or the script attached to the type does not inherit the
    ///     <see cref="InspectorPlugin" /> wrapper type,
    ///     a new instance of the <see cref="InspectorPlugin" /> wrapper script will get attaches to the
    ///     <paramref name="godotObject" />.
    /// </summary>
    /// <remarks>
    ///     The developer should only supply the <paramref name="godotObject" /> that represents the correct underlying
    ///     GDExtension type.
    /// </remarks>
    /// <param name="godotObject">The <paramref name="godotObject" /> that represents the correct underlying GDExtension type.</param>
    /// <returns>
    ///     The existing or a new instance of the <see cref="InspectorPlugin" /> wrapper script attached to the supplied
    ///     <paramref name="godotObject" />.
    /// </returns>
    public static InspectorPlugin Bind(GodotObject godotObject)
    {
        return godotObject is not null
            ? GDExtensionHelper.Bind<InspectorPlugin>(godotObject)
            : null;
    }
}