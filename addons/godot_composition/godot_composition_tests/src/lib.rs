mod tests;

use godot::prelude::*;
use godot_composition_core::plugin::{deinit_plugin, init_plugin};


pub struct GodotCompositionTests;

#[gdextension]
unsafe impl ExtensionLibrary for GodotCompositionTests {
    fn on_level_init(level: InitLevel) {
        init_plugin(level);
    }

    fn on_level_deinit(level: InitLevel) {
        deinit_plugin(level);
    }
}

