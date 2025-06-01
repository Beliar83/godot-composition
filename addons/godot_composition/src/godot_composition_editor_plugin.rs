use crate::component::Component;
use crate::component_with_class::ComponentWithClass;
use crate::godot_composition_world::GodotCompositionWorld;
use crate::inspector_plugin::InspectorPlugin;
use crate::node_entity::NodeEntity;
use godot::classes::{
    ClassDb, ConfirmationDialog, EditorInterface, EditorPlugin, Engine, IEditorPlugin, Json,
    ResourceLoader, Script,
};
use godot::prelude::*;
use std::collections::HashMap;

#[derive(GodotClass)]
#[class(init, tool, base=EditorPlugin)]
/// Plugin to make adding components to nodes with the editor easier.
pub struct GodotCompositionEditorPlugin {
    pub(crate) inspector_plugin: Option<Gd<InspectorPlugin>>,
    registered_scripts: HashMap<GString, StringName>,
    pub(crate) registered_components: HashMap<StringName, GString>,
    first_scene_setup_completed: bool,
    current_scene: Option<Gd<Node>>,
    pub(crate) component_clipboard: Option<GString>,
    base: Base<EditorPlugin>,
}

#[godot_api]
impl GodotCompositionEditorPlugin {
    #[func]
    /// Registers a script as a component to be shown in "Add Component"
    /// The script should inherit directly from Component, or from RefCounted, if the language does not support inheriting from extension types
    ///
    /// If the script has a global name (class_name in gdscript) it will be registered under that name,
    /// otherwise the filename, without extension, in PascalCase will be used.
    fn register_component_script(&mut self, script_path: GString) -> bool {
        if let std::collections::hash_map::Entry::Vacant(entry) =
            self.registered_scripts.entry(script_path.clone())
        {
            match ResourceLoader::singleton()
                .load_ex(&script_path)
                .type_hint("Script")
                .done()
                .map(|script| script.cast::<Script>())
            {
                None => {
                    godot_error!("{} is not a Script", script_path);
                    false
                }
                Some(script) => {
                    let base_type = script.get_instance_base_type();
                    if base_type == RefCounted::class_name().to_string_name()
                        || base_type == Component::class_name().to_string_name()
                    {
                        let script_name = script.get_global_name();

                        let script_name = if script_name.is_empty() {
                            let path = script_path.clone().get_file();
                            let extension = format!(".{}", path.get_extension());
                            StringName::from(path.replace(&extension, "").to_pascal_case())
                        } else {
                            script_name
                        };
                        if self.registered_components.contains_key(&script_name) {
                            godot_error!(
                                "The component name '{}' is already registered to {}",
                                script_name,
                                script_path
                            );
                            false
                        } else if ClassDb::singleton().class_exists(&script_name)
                            && ClassDb::singleton().is_parent_class(
                                &script_name,
                                &Component::class_name().to_string_name(),
                            )
                        {
                            godot_error!(
                                "There already is a Component '{}' in the ClassDb",
                                script_name
                            );
                            false
                        } else {
                            self.registered_components
                                .insert(script_name.clone(), script_path.clone());
                            entry.insert(script_name);
                            true
                        }
                    } else {
                        godot_error!("{} does not extend Component or RefCounted", script_path);
                        false
                    }
                }
            }
        } else {
            godot_error!("Script {} is already registered", script_path);
            false
        }
    }

    #[func]
    /// Removes the script as a registered component.
    ///
    /// This does NOT affect existing components of that script, only visibility in the "Add Component" menu
    fn unregister_component_script(&mut self, script_path: GString) {
        if let Some(component_name) = self.registered_scripts.remove(&script_path) {
            self.registered_components.remove(&component_name);
        } else {
            godot_error!("Script {} is not registered", script_path);
        }
    }

    #[func]
    pub fn get_instance() -> Option<Gd<Self>> {
        Engine::singleton()
            .get_singleton(&GodotCompositionEditorPlugin::class_name().to_string_name())
            .map(|s| s.cast::<GodotCompositionEditorPlugin>())
    }

    #[func]
    fn scene_changed(&mut self, new_scene: Option<Gd<Node>>) {
        self.first_scene_setup_completed = true;
        let engine = Engine::singleton();
        let mut world = engine
            .get_singleton(&GodotCompositionWorld::class_name().to_string_name())
            .expect("The engine said the {} singleton exists")
            .cast::<GodotCompositionWorld>();
        if let Some(scene) = self.current_scene.take() {
            world.bind().store_entities_to_scene(scene);
        }

        if let Some(new_scene) = new_scene {
            world.bind_mut().set_entities_from_scene(new_scene.clone());
            Callable::from_object_method(&world, "clear_entities_from_scene")
                .call_deferred(&[new_scene.to_variant()]);
            self.current_scene = Some(new_scene);
        }
    }

    pub(crate) fn store_components(&mut self, node_entity: Gd<NodeEntity>) {
        let components =
            Json::from_native_ex(&node_entity.bind().get_all_components().to_variant())
                .full_objects(true)
                .done();
        self.component_clipboard = Some(Json::stringify(&components))
    }

    pub(crate) fn paste_component(&self, node: Gd<Node>) {
        if let Some(components) = &self.component_clipboard {
            let components = Json::to_native_ex(&Json::parse_string(components))
                .allow_objects(true)
                .done();
            let components = components.to::<Vec<Dictionary>>();
            let components: Vec<_> = components
                .iter()
                .map(|x| ComponentWithClass::from_godot(x.clone()))
                .collect();
            let mut world = GodotCompositionWorld::get_singleton();
            for component in components {
                if !world
                    .bind()
                    .node_has_component_of_class(node.clone(), component.component_class.clone())
                {
                    world.bind_mut().add_component_to_node(
                        component.component.clone(),
                        node.clone(),
                        component.component_class.clone(),
                    );
                }
            }
        }
    }

    pub(crate) fn paste_values(&self, mut node: Gd<Node>) {
        if let Some(components) = &self.component_clipboard {
            let components = Json::to_native_ex(&Json::parse_string(components))
                .allow_objects(true)
                .done();
            let components = components.to::<Vec<Dictionary>>();
            let components: Vec<_> = components
                .iter()
                .map(|x| ComponentWithClass::from_godot(x.clone()))
                .collect();
            let mut dialog = ConfirmationDialog::new_alloc();
            let component_list = components
                .iter()
                .map(|x| x.component_class.to_string())
                .collect::<Vec<String>>();
            let text = format!(
                "This will overwrite the values of existing components on {} with values from stored components ({})",
                node.get_name(),
                component_list.join(", ")
            );
            dialog.set_autowrap(true);
            dialog.set_title("Paste values");
            dialog.set_text(text.as_str());
            let components = components.clone();
            let mut dialog_confirmed = dialog.clone();
            let mut dialog_canceled = dialog.clone();
            dialog.clone().signals().confirmed().connect(move || {
                let mut world = GodotCompositionWorld::get_singleton();
                for component in &components {
                    world.bind_mut().replace_component_of_node(
                        component.component.clone(),
                        node.clone(),
                        component.component_class.clone(),
                    );
                }
                dialog_confirmed.queue_free();
                node.notify_property_list_changed();
            });

            dialog.clone().signals().canceled().connect(move || {
                dialog_canceled.queue_free();
            });

            dialog.popup_exclusive_centered(&self.to_gd());
        }
    }

    pub(crate) fn has_stored_component(&self) -> bool {
        self.component_clipboard.is_some()
    }
}

#[godot_api]
impl IEditorPlugin for GodotCompositionEditorPlugin {
    fn apply_changes(&mut self) {
        match EditorInterface::singleton().get_edited_scene_root() {
            None => {}
            Some(scene) => {
                let world = Engine::singleton()
                    .get_singleton(&GodotCompositionWorld::class_name().to_string_name())
                    .map(|world| world.cast::<GodotCompositionWorld>())
                    .expect("No GodotCompositionWorld singleton found");
                world.bind().store_entities_to_scene(scene.clone());

                Callable::from_object_method(&world, "clear_entities_from_scene")
                    .call_deferred(&[scene.to_variant()]);
            }
        }
    }

    fn process(&mut self, _: f64) {
        if !self.first_scene_setup_completed {
            if let Some(current_scene) = EditorInterface::singleton().get_edited_scene_root() {
                self.scene_changed(Some(current_scene));
            }
        }
    }

    fn enter_tree(&mut self) {
        let world = GodotCompositionWorld::get_singleton();
        world
            .signals()
            .component_added()
            .connect(|_, _, _| EditorInterface::singleton().mark_scene_as_unsaved());
        world
            .signals()
            .component_removed()
            .connect(|_, _, _| EditorInterface::singleton().mark_scene_as_unsaved());
        world
            .signals()
            .component_replaced()
            .connect(|_, _, _| EditorInterface::singleton().mark_scene_as_unsaved());
        let self_gd = self.to_gd();
        Engine::singleton().register_singleton(
            &GodotCompositionEditorPlugin::class_name().to_string_name(),
            &self_gd,
        );

        let self_gd = self.to_gd();
        // Can't use signals(), as that does not support receiving null values yet
        self.base_mut().connect(
            "scene_changed",
            &Callable::from_object_method(&self_gd, "scene_changed"),
        );

        let inspector_plugin = InspectorPlugin::new_gd();
        self.base_mut().add_inspector_plugin(&inspector_plugin);
        self.inspector_plugin = Some(inspector_plugin);
    }

    fn exit_tree(&mut self) {
        Engine::singleton()
            .unregister_singleton(&GodotCompositionEditorPlugin::class_name().to_string_name());
        match self.inspector_plugin.take() {
            None => {}
            Some(plugin) => {
                self.base_mut().remove_inspector_plugin(&plugin);
            }
        }
    }
}
