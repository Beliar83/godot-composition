use crate::godot_composition_world::GodotCompositionWorld;
use godot::classes::Engine;
use godot::prelude::*;

pub fn init_plugin(level: InitLevel) {
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

pub fn deinit_plugin(level: InitLevel) {
    if level == InitLevel::Scene {
        let mut engine = Engine::singleton();
        if engine.has_singleton(&GodotCompositionWorld::class_name().to_string_name()) {
            engine.unregister_singleton(&GodotCompositionWorld::class_name().to_string_name());
        }
    }
}
