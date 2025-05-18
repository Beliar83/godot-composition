use crate::components_editor::ComponentsEditor;

use godot::classes::{EditorInspectorPlugin, IEditorInspectorPlugin};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, tool, base=EditorInspectorPlugin)]
pub struct InspectorPlugin {
    base: Base<EditorInspectorPlugin>,
}

#[godot_api]
impl IEditorInspectorPlugin for InspectorPlugin {
    fn can_handle(&self, object: Option<Gd<Object>>) -> bool {
        match object {
            None => false,
            Some(object) => object.is_class(&Node::class_name().to_gstring()),
        }
    }

    fn parse_end(&mut self, object: Option<Gd<Object>>) {
        match object {
            None => {}
            Some(object) => {
                if let Ok(node) = object.try_cast::<Node>() {
                    let components_editor = ComponentsEditor::create_for_node(node);

                    self.base_mut().add_custom_control(&components_editor);
                }
            }
        }
    }
}
