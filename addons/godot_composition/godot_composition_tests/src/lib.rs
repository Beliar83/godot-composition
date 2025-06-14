mod scripts;
mod tests;

use godot::prelude::*;
use godot_composition_core::plugin::{deinit_plugin, init_plugin};

pub struct GodotCompositionTests;

#[gdextension]
unsafe impl ExtensionLibrary for GodotCompositionTests {
    fn on_level_init(level: InitLevel) {
        init_plugin(level);
        match level {
            InitLevel::Core => (),
            InitLevel::Servers => (),
            InitLevel::Scene => godot_rust_script::init!(scripts),
            InitLevel::Editor => (),
        }
    }

    fn on_level_deinit(level: InitLevel) {
        match level {
            InitLevel::Editor => (),
            InitLevel::Scene => godot_rust_script::deinit!(),
            InitLevel::Servers => (),
            InitLevel::Core => (),
        }
        deinit_plugin(level);
    }
}
