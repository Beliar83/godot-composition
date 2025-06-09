use crate::node_entity::NodeEntity;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=RefCounted, tool)]
/// A component that can be added to a Node to add additional functionality
pub struct Component {
    node_entity: Option<Gd<NodeEntity>>,
    base: Base<RefCounted>,
}

#[godot_api]
impl Component {
    #[func(virtual)]
    /// Called when the entity if the component is changed
    pub fn entity_changed(&mut self, _node_entity: Option<Gd<NodeEntity>>) {}
    #[func(virtual)]
    /// Called during _process of the normal engine loop
    pub fn process(&mut self, _delta: f64, _node: Gd<Node>, _node_entity: Gd<NodeEntity>) {}
    #[func(virtual)]
    /// Called during _physics_process of the normal engine loop
    pub fn physics_process(&mut self, _delta: f64, _node: Gd<Node>, _node_entity: Gd<NodeEntity>) {}

    #[func]
    /// Gets the NodeEntity of this Component. Warning: Calling this in _process or _physics_process of the component will result in an error
    pub fn get_node_entity(&self) -> Option<Gd<NodeEntity>> {
        self.node_entity.clone()
    }
    pub fn set_node_entity(&mut self, node_entity: Option<Gd<NodeEntity>>) {
        self.node_entity = node_entity.clone();
        self.entity_changed(node_entity);
    }
}
