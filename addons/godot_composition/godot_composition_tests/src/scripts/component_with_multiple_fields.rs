use godot::prelude::*;
use godot_composition_core::component::Component;
use godot_rust_script::{godot_script_impl, GodotScript};

#[derive(Debug, GodotScript)]
#[script(base = Component)]
pub(crate) struct ComponentWithMultipleFields {
    #[export]
    pub int_field: i64,
    #[export]
    pub string_field: GString,
    #[export]
    pub resource_field: Gd<Resource>,
}

#[godot_script_impl]
impl ComponentWithMultipleFields {}
