#nullable disable

using System;
using Godot;

namespace GDExtension.Wrappers;

public partial class GodotCompositionEditorPlugin : EditorPlugin
{
    public static readonly StringName GDExtensionName = "GodotCompositionEditorPlugin";

    [Obsolete(
        "Wrapper classes cannot be constructed with Ctor (it only instantiate the underlying EditorPlugin), please use the Instantiate() method instead.")]
    protected GodotCompositionEditorPlugin()
    { }

    /// <summary>
    ///     Creates an instance of the GDExtension <see cref="GodotCompositionEditorPlugin" /> type, and attaches the wrapper
    ///     script to it.
    /// </summary>
    /// <returns>The wrapper instance linked to the underlying GDExtension type.</returns>
    public static GodotCompositionEditorPlugin Instantiate()
    {
        return GDExtensionHelper.Instantiate<GodotCompositionEditorPlugin>(GDExtensionName);
    }

    /// <summary>
    ///     Try to cast the script on the supplied <paramref name="godotObject" /> to the
    ///     <see cref="GodotCompositionEditorPlugin" /> wrapper type,
    ///     if no script has attached to the type, or the script attached to the type does not inherit the
    ///     <see cref="GodotCompositionEditorPlugin" /> wrapper type,
    ///     a new instance of the <see cref="GodotCompositionEditorPlugin" /> wrapper script will get attaches to the
    ///     <paramref name="godotObject" />.
    /// </summary>
    /// <remarks>
    ///     The developer should only supply the <paramref name="godotObject" /> that represents the correct underlying
    ///     GDExtension type.
    /// </remarks>
    /// <param name="godotObject">The <paramref name="godotObject" /> that represents the correct underlying GDExtension type.</param>
    /// <returns>
    ///     The existing or a new instance of the <see cref="GodotCompositionEditorPlugin" /> wrapper script attached to
    ///     the supplied <paramref name="godotObject" />.
    /// </returns>
    public static GodotCompositionEditorPlugin Bind(GodotObject godotObject)
    {
        return godotObject is not null
            ? GDExtensionHelper.Bind<GodotCompositionEditorPlugin>(godotObject)
            : null;
    }

    #region Methods

    public bool RegisterComponentScript(string scriptPath)
    {
        return Call(_cached_register_component_script, scriptPath).As<bool>();
    }


    public void UnregisterComponentScript(string scriptPath)
    {
        Call(_cached_unregister_component_script, scriptPath);
    }


    public static GodotCompositionEditorPlugin GetInstance()
    {
        return Bind(GDExtensionHelper.Call(GDExtensionName, _cached_get_instance).As<GodotObject>());
    }


    public void SceneChanged(Node newScene)
    {
        Call(_cached_scene_changed, newScene);
    }

    #endregion

    private static readonly StringName _cached_register_component_script = "register_component_script";
    private static readonly StringName _cached_unregister_component_script = "unregister_component_script";
    private static readonly StringName _cached_get_instance = "get_instance";
    private static readonly StringName _cached_scene_changed = "scene_changed";
}