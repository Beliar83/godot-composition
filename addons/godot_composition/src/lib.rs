mod component;
mod component_with_class;
mod components_editor;
mod godot_composition_editor_plugin;
mod godot_composition_world;
mod inspector_plugin;
mod node_entity;
mod property_info;

use crate::godot_composition_world::GodotCompositionWorld;
use godot::classes::Engine;
use godot::prelude::*;

struct GodotComposition;

#[gdextension]
unsafe impl ExtensionLibrary for GodotComposition {
    fn on_level_init(level: InitLevel) {
        if level == InitLevel::Scene {
            let world = GodotCompositionWorld::new_alloc();
            Engine::singleton().register_singleton(
                &GodotCompositionWorld::class_name().to_string_name(),
                &world,
            );
            Callable::from_local_fn("setup_world", move |_| {
                let scene_tree = Engine::singleton()
                    .get_main_loop()
                    .expect("no main loop")
                    .cast::<SceneTree>();
                scene_tree.get_root().expect("no root").add_child(&world);
                Ok(Variant::nil())
            })
            .call_deferred(&[]);
        }
    }

    fn on_level_deinit(level: InitLevel) {
        if level == InitLevel::Scene {
            let mut engine = Engine::singleton();
            if engine.has_singleton(&GodotCompositionWorld::class_name().to_string_name()) {
                engine.unregister_singleton(&GodotCompositionWorld::class_name().to_string_name());
            }
        }
    }
}
