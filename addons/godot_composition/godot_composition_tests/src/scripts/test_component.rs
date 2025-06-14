use godot::classes::Node;
use godot_composition_core::component::Component;
use godot_composition_core::node_entity::NodeEntity;
use godot_rust_script::{godot::prelude::Gd, godot_script_impl, GodotScript};

#[derive(Debug, GodotScript)]
#[script(base = Component)]
pub(crate) struct TestComponent {
    #[export]
    pub process_calls: i64,
    #[export]
    pub physics_process_calls: i64,
}

#[godot_script_impl]
impl TestComponent {
    pub fn _process(&mut self, _delta: f64, _node: Gd<Node>, _node_entity: Gd<NodeEntity>) {
        self.process_calls += 1;
    }

    pub fn _physics_process(&mut self, _delta: f64, _node: Gd<Node>, _node_entity: Gd<NodeEntity>) {
        self.physics_process_calls += 1;
    }
}
