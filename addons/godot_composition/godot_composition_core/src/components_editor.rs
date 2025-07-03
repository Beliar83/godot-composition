use crate::component::Component;
use crate::godot_composition_editor_plugin::GodotCompositionEditorPlugin;
use crate::godot_composition_world::GodotCompositionWorld;
use crate::property_info::create_property_from_dictionary;
use godot::classes::box_container::AlignmentMode;
use godot::classes::control::{LayoutPreset, SizeFlags};
use godot::classes::{
    Button, ClassDb, EditorInspector, EditorInterface, HBoxContainer, Label,
    PanelContainer, PopupMenu, ResourceLoader, VBoxContainer,
};
use godot::global::{HorizontalAlignment, PropertyUsageFlags};
use godot::prelude::*;
use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

pub(crate) static PROPERTIES_TO_IGNORE: LazyLock<HashSet<StringName>> =
    LazyLock::<HashSet<StringName>>::new(|| {
        let mut property_names = HashSet::new();
        property_names.insert(StringName::from("script"));
        property_names
    });

enum ComponentType {
    Native(StringName),
    Script { name: StringName, path: GString },
}

#[derive(GodotClass)]
#[class(init, tool, base=PanelContainer)]
pub struct ComponentsEditor {
    node: Option<Gd<Node>>,
    components: HashMap<StringName, Gd<Component>>,
    paste_components_button: Option<Gd<Button>>,
    paste_values_button: Option<Gd<Button>>,
    base: Base<PanelContainer>,
}

impl ComponentsEditor {
    /// Returns a manually managed instance of a ComponentsEditor for a component
    pub(crate) fn create_for_node(node: Gd<Node>) -> Gd<Self> {
        let mut instance = Gd::from_init_fn(|base| Self {
            components: HashMap::new(),
            node: Some(node.clone()),
            paste_components_button: None,
            paste_values_button: None,
            base,
        });
        instance.call_deferred("setup_ui", &[]);
        instance
    }
}

#[godot_api]
impl ComponentsEditor {
    #[func]
    fn store_components(&mut self) {
        if let Some(mut plugin) = GodotCompositionEditorPlugin::get_instance() {
            let node = match &self.node {
                None => {
                    return;
                }
                Some(node) => node,
            };
            let node_entity = match GodotCompositionWorld::get_singleton()
                .bind()
                .get_node_entity_or_null(node.clone())
            {
                None => {
                    return;
                }
                Some(node_entity) => node_entity,
            };
            plugin.bind_mut().store_components(node_entity);
            if let Some(ref mut paste_button) = self.paste_components_button {
                paste_button.set_disabled(false);
            }
            if let Some(ref mut paste_button) = self.paste_values_button {
                paste_button.set_disabled(false);
            }
        }
    }

    fn paste_components(&mut self) {
        if let Some(plugin) = GodotCompositionEditorPlugin::get_instance() {
            let node = match &mut self.node {
                None => {
                    return;
                }
                Some(node) => node,
            };

            plugin.bind().paste_component(node.clone());
            node.notify_property_list_changed();
        }
    }

    fn paste_values(&mut self) {
        if let Some(plugin) = GodotCompositionEditorPlugin::get_instance() {
            let node = match &mut self.node {
                None => {
                    return;
                }
                Some(node) => node,
            };

            plugin.bind().paste_values(node.clone());
            node.notify_property_list_changed();
        }
    }

    #[func]
    fn setup_ui(&mut self) {
        for i in 0..self.base().get_child_count() {
            let mut child = self
                .base_mut()
                .get_child(i)
                .expect("Unexpected missing child");
            child.queue_free();
        }

        // self.reset_timer = None;
        let mut world = GodotCompositionWorld::get_singleton();
        let node = self.node.clone().expect("Node was not set");
        let mut main_container = VBoxContainer::new_alloc();
        self.components.clear();
        let mut node_entity = world.bind_mut().get_or_create_node_entity(node.clone());

        #[allow(clippy::mutable_key_type)]
        let components = &node_entity.bind_mut().components.clone();
        for component in components.iter() {
            let name = component.component_class.clone();
            let component = &component.component;
            self.components.insert(name.clone(), component.clone());
            let mut component_container = VBoxContainer::new_alloc();
            let mut header = PanelContainer::new_alloc();
            let mut header_layout = HBoxContainer::new_alloc();
            let inspector = EditorInterface::singleton()
                .get_inspector()
                .expect("Could not get inspector");
            let stylebox = inspector
                .get_theme_stylebox_ex("bg")
                .theme_type("EditorInspectorCategory")
                .done()
                .expect("Could not get EditorInspectorCategory stylebox");
            let font_color = inspector
                .get_theme_color_ex("font_color")
                .theme_type("Editor")
                .done();
            let label_font = inspector
                .get_theme_font_ex("title")
                .theme_type("EditorFonts")
                .done()
                .expect("Could not get editor font (title)");
            let label_font_size = inspector
                .get_theme_font_size_ex("title_size")
                .theme_type("EditorFonts")
                .done();
            header.add_theme_stylebox_override("panel", &stylebox);
            header_layout.set_alignment(AlignmentMode::CENTER);

            header_layout.set_anchors_preset(LayoutPreset::FULL_RECT);
            header.set_anchors_preset(LayoutPreset::HCENTER_WIDE);
            let mut label = Label::new_alloc();
            label.set_horizontal_alignment(HorizontalAlignment::CENTER);
            label.set_text(&name.to_string());
            label.set_h_size_flags(SizeFlags::EXPAND_FILL);
            label.add_theme_color_override("font_color", font_color);
            label.add_theme_font_override("font", &label_font);
            label.add_theme_font_size_override("font_size", label_font_size);
            header_layout.add_child(&label);
            let mut remove_button = Button::new_alloc();
            remove_button.set_text("X");
            remove_button.set_custom_minimum_size(Vector2::new(25f32, 0f32));
            remove_button.set_text_alignment(HorizontalAlignment::CENTER);
            remove_button.add_theme_stylebox_override("focus", &stylebox);
            remove_button.add_theme_stylebox_override("normal", &stylebox);
            remove_button.add_theme_stylebox_override("disabled", &stylebox);
            remove_button.add_theme_stylebox_override("hover", &stylebox);
            remove_button.add_theme_stylebox_override("hover_pressed", &stylebox);
            remove_button.add_theme_stylebox_override("pressed", &stylebox);
            remove_button.add_theme_stylebox_override("normal_mirrored", &stylebox);
            remove_button.add_theme_stylebox_override("disabled_mirrored", &stylebox);
            remove_button.add_theme_stylebox_override("hover_mirrored", &stylebox);
            remove_button.add_theme_stylebox_override("hover_pressed_mirrored", &stylebox);
            remove_button.add_theme_stylebox_override("pressed_mirrored", &stylebox);
            remove_button.set_h_size_flags(SizeFlags::FILL);
            let mut node = node.clone();
            let mut world = world.clone();
            {
                let name = name.clone();
                remove_button.signals().pressed().connect(move || {
                    world
                        .bind_mut()
                        .set_component_of_node(node.clone(), name.clone(), None);
                    node.notify_property_list_changed();
                });
            }

            header_layout.add_child(&remove_button);

            header.add_child(&header_layout);
            component_container.add_child(&header);

            let mut property_editors = VBoxContainer::new_alloc();

            for property in component.get_property_list().iter_shared() {
                let property = create_property_from_dictionary(property);

                if PROPERTIES_TO_IGNORE.contains(&property.property_name) {
                    continue;
                }
                if property.usage.ord() & PropertyUsageFlags::EDITOR.ord()
                    == PropertyUsageFlags::EDITOR.ord()
                {
                    match EditorInspector::instantiate_property_editor(
                        &component.clone(),
                        property.variant_type,
                        &property.property_name.to_string(),
                        property.hint_info.hint,
                        &property.hint_info.hint_string,
                        property.usage.ord() as u32,
                    ) {
                        None => {}
                        Some(mut editor) => {
                            editor.set_object_and_property(
                                &component.clone(),
                                &property.property_name,
                            );
                            editor.update_property();
                            editor.set_label(&property.property_name.to_string());
                            let mut component = component.clone();
                            editor.signals().property_changed().connect(
                                move |path, value, _, _| {
                                    component.set_deferred(&path, &value);
                                    EditorInterface::singleton().mark_scene_as_unsaved();
                                },
                            );
                            editor.set_read_only(
                                property.usage.ord() & PropertyUsageFlags::READ_ONLY.ord()
                                    == PropertyUsageFlags::READ_ONLY.ord(),
                            );

                            property_editors.add_child(&editor);
                        }
                    }
                }
            }

            component_container.add_child(&property_editors);
            main_container.add_child(&component_container);
        }

        let menu = PopupMenu::new_alloc();

        let mut add_component_button = Button::new_alloc();
        add_component_button.set_text("Add component");
        main_container.add_child(&add_component_button);

        self.update_menu(world.clone(), menu, add_component_button);

        self.base_mut()
            .set_anchors_preset(LayoutPreset::HCENTER_WIDE);

        let mut copy_button = Button::new_alloc();
        copy_button.set_text("Copy Components");
        copy_button
            .signals()
            .pressed()
            .connect_other(self, Self::store_components);

        copy_button.set_disabled(node_entity.bind().components.is_empty());

        main_container.add_child(&copy_button);

        let mut paste_components_button = Button::new_alloc();
        paste_components_button.set_text("Paste Components");
        paste_components_button
            .signals()
            .pressed()
            .connect_other(self, Self::paste_components);

        let editor_plugin = GodotCompositionEditorPlugin::get_instance().expect("No editor plugin");
        paste_components_button.set_disabled(!editor_plugin.bind().has_stored_component());

        main_container.add_child(&paste_components_button);

        self.paste_components_button = Some(paste_components_button);

        let mut paste_values_button = Button::new_alloc();
        paste_values_button.set_text("Paste values");
        paste_values_button
            .signals()
            .pressed()
            .connect_other(self, Self::paste_values);

        let editor_plugin = GodotCompositionEditorPlugin::get_instance().expect("No editor plugin");
        paste_values_button.set_disabled(!editor_plugin.bind().has_stored_component());

        main_container.add_child(&paste_values_button);

        self.paste_values_button = Some(paste_values_button);

        self.base_mut().add_child(&main_container);
    }

    fn update_menu(
        &mut self,
        mut world: Gd<GodotCompositionWorld>,
        mut menu: Gd<PopupMenu>,
        mut button: Gd<Button>,
    ) {
        if let Some(plugin) = GodotCompositionEditorPlugin::get_instance() {
            let mut possible_components: HashMap<i32, ComponentType> = HashMap::new();
            let mut index: i32 = 0;

            for inheritor in ClassDb::singleton()
                .get_inheriters_from_class(&Component::class_name().to_string_name())
                .as_slice()
            {
                if !self.components.contains_key(&StringName::from(inheritor)) {
                    menu.add_item_ex(&format!("{}", inheritor)).id(index).done();
                    possible_components
                        .insert(index, ComponentType::Native(StringName::from(inheritor)));

                    index += 1;
                }
            }

            for (name, path) in &plugin.bind().registered_components {
                if !self.components.contains_key(name) {
                    possible_components.insert(
                        index,
                        ComponentType::Script {
                            name: name.clone(),
                            path: path.clone(),
                        },
                    );
                    menu.add_item_ex(&format!("{}", name)).id(index).done();

                    index += 1;
                }
            }
            let mut selected_node = self.node.clone().expect("Node was empty");

            menu.signals().id_pressed().connect(move |id: i64| {
                let (_, component) = possible_components
                    .remove_entry(&(id as i32))
                    .expect("No entry for id was found");
                let (component, name) = match component {
                    ComponentType::Native(class_name) => (
                        ClassDb::singleton()
                            .instantiate(&class_name)
                            .to::<Gd<Component>>(),
                        class_name,
                    ),
                    ComponentType::Script { name, path } => {
                        let mut component = Component::new_gd();
                        let script = ResourceLoader::singleton()
                            .load_ex(&path)
                            .type_hint("Script")
                            .done()
                            .unwrap_or_else(|| panic!("Could not load {} as script", path));
                        component.set_script(&script.to_variant());
                        (component, name)
                    }
                };
                world.bind_mut().set_component_of_node(
                    selected_node.clone(),
                    name,
                    Some(component),
                );
                selected_node.notify_property_list_changed();
            });
        }

        if menu.get_item_count() == 0 {
            button.set_disabled(true);
            menu.free();
        } else {
            button.add_child(&menu);
            button.clone().signals().pressed().connect(move || {
                let menu_rect = button.get_global_rect().cast_int();
                menu.popup_on_parent(menu_rect);
            });
        };
    }
}

// #[godot_api]
// impl IPanelContainer for ComponentsEditor {
//     fn ready(&mut self) {}
// }
