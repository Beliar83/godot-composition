use gd_rehearse::itest::gditest;
use godot_composition_core::godot_composition_world::GodotCompositionWorld;

#[gditest]
fn should_register_a_singleton_of_the_composition_world() {
    let instance = GodotCompositionWorld::get_singleton();
    assert!(instance.is_instance_valid());
}
