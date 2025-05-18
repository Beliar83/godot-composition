use crate::component::Component;
use crate::component_with_class::{ComponentWithClass, ComponentWithClassForNode};
use crate::node_entity::NodeEntity;
use godot::classes::Engine;
use godot::prelude::*;
use std::collections::{HashMap, HashSet};
use std::sync::RwLock;

const COMPONENTS_META_NAME: &str = "godot_composition_components";
const NODE_ENTITIES_META_NAME: &str = "godot_composition_node_entities";

#[derive(GodotClass)]
#[class(init, base=Node, tool)]
/// Manages components of nodes
pub struct GodotCompositionWorld {
    to_add: RwLock<HashSet<ComponentWithClassForNode>>,
    to_remove: RwLock<HashSet<ComponentWithClassForNode>>,
    instances_by_component_class_godot: HashMap<StringName, Array<Gd<Component>>>,
    all_instances_internal: Vec<(Gd<NodeEntity>, Gd<Node>, Gd<Component>)>,
    pub(crate) all_instances: Dictionary,
    pub(crate) instances_by_component_class: HashMap<StringName, HashSet<Gd<Component>>>,
    pub(crate) node_entities: HashMap<InstanceId, Gd<NodeEntity>>,
    base: Base<Node>,
}

#[godot_api]
impl GodotCompositionWorld {
    #[func]
    pub fn get_singleton() -> Gd<Self> {
        let engine = Engine::singleton();
        engine
            .get_singleton(&Self::class_name().to_string_name())
            .expect("World was not created")
            .cast::<Self>()
    }

    #[signal]
    /// Emitted when the entity for a node is created
    pub fn node_entity_created(node_entity: Gd<NodeEntity>);

    #[signal]
    /// Emitted when a component is added to a node
    pub fn component_added(
        node_entity: Gd<NodeEntity>,
        component_class: StringName,
        component: Gd<Component>,
    );

    #[signal]
    /// Emitted when a component is removed from a node
    pub fn component_removed(
        node_entity: Gd<NodeEntity>,
        component_class: StringName,
        component: Gd<Component>,
    );

    #[func]
    /// Return all components, grouped by Node
    pub fn get_all_components(&mut self) -> Dictionary {
        self.all_instances.duplicate_shallow()
    }

    #[func]
    /// Execute a callable for all components
    ///
    /// The signature of the callable must be:
    /// (component_class: StringName, component: Component)
    pub fn do_for_all_components(&mut self, func: Callable) {
        if func.get_argument_count() != 2 {
            godot_error!(
                "Expected signature of func: (component_class: StringName, component: Component)"
            );
            return;
        }

        for (component_class, components) in self.instances_by_component_class.iter_mut() {
            let component_class = component_class.to_variant();
            for component in components.iter() {
                func.call(&[component_class.clone(), component.to_variant()]);
            }
        }
    }

    #[func]
    /// Execute a callable for all components of a component class
    ///
    /// The signature of the callable must be:
    /// (component: Component)
    pub fn do_for_all_components_of_class(&self, component_name: StringName, func: Callable) {
        if func.get_argument_count() != 1 {
            godot_error!("Expected signature of func: (component: Component)");
            return;
        }

        for component in self
            .instances_by_component_class
            .get(&component_name)
            .cloned()
            .unwrap_or_default()
            .iter()
        {
            func.call(&[component.to_variant()]);
        }
    }

    #[func]
    /// Returns all components of a component class
    pub fn get_all_components_of_class(&self, component_name: StringName) -> Array<Gd<Component>> {
        match self.instances_by_component_class_godot.get(&component_name) {
            None => {
                godot_warn!("No components of class {}", component_name);
                Array::new()
            }
            Some(components) => components.duplicate_shallow(),
        }
    }

    #[func]
    /// Store current entity data to a scene
    pub fn store_entities_to_scene(&self, mut scene: Gd<Node>) {
        let mut entities = Array::new();
        for node_entity in self.node_entities.values() {
            if !node_entity.bind().components.is_empty() {
                let mut node = node_entity
                    .bind()
                    .node
                    .clone()
                    .expect("Node entity has no node");
                node.set_meta(
                    COMPONENTS_META_NAME,
                    &node_entity.bind().get_all_components().to_variant(),
                );
                let node_path = scene.get_path_to(&node);
                entities.push(&node_path);
            }
        }
        scene.set_meta(NODE_ENTITIES_META_NAME, &entities.to_variant());
    }

    #[func]
    /// Set entities from data stored to a scene
    pub fn set_entities_from_scene(&mut self, new_scene: Gd<Node>) {
        if new_scene.has_meta(NODE_ENTITIES_META_NAME) {
            self.remove_all_entities_and_pending_changes();
            let mut node_entities = HashSet::<Gd<NodeEntity>>::new();
            let mut component_classes = HashSet::<StringName>::new();
            let entity_paths = new_scene
                .get_meta(NODE_ENTITIES_META_NAME)
                .to::<Vec<NodePath>>();
            for path in entity_paths {
                let node = new_scene
                    .get_node_or_null(&path)
                    .expect("Node for stored entity path not found");
                let mut node_entity = self.get_or_create_node_entity(node.clone());
                let components = node.get_meta(COMPONENTS_META_NAME).to::<Vec<Dictionary>>();
                component_classes.extend(node_entity.bind_mut().set_components(components));
                node_entities.insert(node_entity);
            }
            self.update_caches(node_entities, component_classes);
        }
    }

    #[func]
    /// Remove stored entities from a scene
    pub fn clear_entities_from_scene(&self, mut scene: Gd<Node>) {
        for node_entity in self.node_entities.values() {
            if !node_entity.bind().components.is_empty() {
                let mut node = node_entity
                    .bind()
                    .node
                    .clone()
                    .expect("Node entity has no node");
                node.remove_meta(COMPONENTS_META_NAME);
            }
        }
        scene.remove_meta(NODE_ENTITIES_META_NAME);
    }
    #[func]
    /// Adds a component to a node
    pub fn add_component_to_node(
        &mut self,
        component: Gd<Component>,
        node: Gd<Node>,
        component_class: StringName,
    ) -> bool {
        if self
            .get_or_create_node_entity(node.clone())
            .bind()
            .has_component_of_class(component_class.clone())
        {
            godot_warn!(
                "Component {} already exists for {}",
                component_class,
                node.get_name()
            );
            false
        } else {
            let to_add = self.to_add.get_mut().unwrap_or_else(|err| {
                godot_warn!(
                        "The lock for the to_add queue was poisoned. A component might not have been added"
                    );
                err.into_inner()
            });

            let component_with_class =
                ComponentWithClass::create(component_class.clone(), component);
            let to_add_data = ComponentWithClassForNode {
                node,
                component_with_class,
            };

            if to_add.contains(&to_add_data) {
                godot_warn!(
                    "Component {} is already being added to {}",
                    to_add_data.component_with_class.component_class,
                    to_add_data.node.get_name()
                );
                self.to_add.clear_poison();
                false
            } else {
                to_add.insert(to_add_data);
                self.to_add.clear_poison();
                true
            }
        }
    }

    /// Removes the component from the node, if present
    #[func]
    pub fn remove_component_from_node(
        &mut self,
        node: Gd<Node>,
        component_class: StringName,
    ) -> bool {
        if let Some(node_entity) = self.node_entities.get(&node.instance_id()) {
            if let Some(component) = node_entity
                .bind()
                .get_component_of_class_or_null(component_class.clone())
            {
                let component_with_class = ComponentWithClass::create(component_class, component);

                let to_remove = self.to_remove.get_mut().unwrap_or_else(|err| {
                    godot_warn!(
                            "The lock for the to_remove queue was poisoned. A component might not have been removed"
                        );
                    err.into_inner()
                });

                let to_remove_data = ComponentWithClassForNode {
                    node,
                    component_with_class,
                };
                if to_remove.contains(&to_remove_data) {
                    godot_warn!(
                        "Component {} was already marked as to be removed for {}",
                        to_remove_data.component_with_class.component_class,
                        to_remove_data.node.get_name()
                    );
                    self.to_remove.clear_poison();
                    false
                } else {
                    to_remove.insert(to_remove_data);
                    self.to_remove.clear_poison();
                    true
                }
            } else {
                godot_warn!(
                    "Node {} does not have component {}",
                    node.get_name(),
                    component_class
                );
                false
            }
        } else {
            godot_warn!("Node {} does have any entities", node.get_name());
            false
        }
    }

    fn create_node_entity(&mut self, node: Gd<Node>) -> Gd<NodeEntity> {
        let entity = NodeEntity::create_for_node(node.clone());
        self.node_entities
            .insert(node.instance_id(), entity.clone());
        self.signals().node_entity_created().emit(&entity.clone());
        entity
    }

    #[func]
    /// Returns the NodeEntity of a Node, or creates one if it does not exist yet
    pub fn get_or_create_node_entity(&mut self, node: Gd<Node>) -> Gd<NodeEntity> {
        if !self.node_entities.contains_key(&node.instance_id()) {
            self.create_node_entity(node.clone())
        } else {
            self.node_entities.get(&node.instance_id()).unwrap().clone()
        }
    }

    #[func]
    /// Returns the NodeEntity of a Node, or null if the Node does not have a NodeEntity
    pub fn get_node_entity_or_null(&self, node: Gd<Node>) -> Option<Gd<NodeEntity>> {
        self.node_entities.get(&node.instance_id()).cloned()
    }

    #[func]
    /// Returns all existing node entities
    pub fn get_all_node_entity(&self) -> Vec<Gd<NodeEntity>> {
        self.node_entities.values().cloned().collect()
    }

    #[func]
    /// Removes all node entities and their components and clears any pending changes
    ///
    /// Note that components that have an active reference will remain accessible but won't have a node entity
    pub fn remove_all_entities_and_pending_changes(&mut self) {
        let mut changed_nodes: HashSet<Gd<NodeEntity>> = HashSet::new();
        for node_entity in self.node_entities.values_mut() {
            for mut component in node_entity.bind_mut().components.clone() {
                component.component.bind_mut().set_node_entity(None);
            }
            changed_nodes.insert(node_entity.clone());
        }
        self.node_entities.clear();
        self.to_add
            .get_mut()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .clear();
        self.to_remove
            .get_mut()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .clear();
        self.update_caches(
            changed_nodes,
            self.instances_by_component_class.keys().cloned().collect(),
        );
    }
}

#[godot_api]
impl INode for GodotCompositionWorld {
    fn process(&mut self, delta: f64) {
        if !Engine::singleton().is_editor_hint() {
            for (node_entity, node, component) in &mut self.all_instances_internal {
                component
                    .bind_mut()
                    .process(delta, node.clone(), node_entity.clone());
            }
        }

        self.process_changes();
    }

    fn physics_process(&mut self, delta: f64) {
        if !Engine::singleton().is_editor_hint() {
            for (node_entity, node, component) in &mut self.all_instances_internal {
                component
                    .bind_mut()
                    .physics_process(delta, node.clone(), node_entity.clone());
            }
        }
    }

    fn ready(&mut self) {
        self.process_changes();
    }
}

impl GodotCompositionWorld {
    fn process_changes(&mut self) {
        let mut changed_nodes: HashSet<Gd<NodeEntity>> = HashSet::new();
        let mut changed_component_classes: HashSet<StringName> = HashSet::new();
        let to_remove: Vec<ComponentWithClassForNode> = match self.to_remove.get_mut() {
            Ok(to_remove) => to_remove.drain().collect(),
            Err(err) => {
                godot_warn!(
                    "The lock for the to_remove queue was poisoned. A component might not have been removed"
                );
                err.into_inner().drain().collect()
            }
        };

        self.to_remove.clear_poison();

        for component_with_class_for_node in to_remove {
            if let Some(ref mut node_entity) =
                self.get_node_entity_or_null(component_with_class_for_node.node.clone())
            {
                node_entity.bind_mut().remove_component(
                    component_with_class_for_node
                        .component_with_class
                        .component_class
                        .clone(),
                );
                self.signals().component_removed().emit(
                    &node_entity.clone(),
                    &component_with_class_for_node
                        .component_with_class
                        .component_class
                        .clone(),
                    &component_with_class_for_node.component_with_class.component,
                );
                changed_nodes.insert(node_entity.clone());
            }
            changed_component_classes.insert(
                component_with_class_for_node
                    .component_with_class
                    .component_class
                    .clone(),
            );
        }

        let to_add: Vec<ComponentWithClassForNode> = match self.to_add.get_mut() {
            Ok(to_add) => to_add.drain().collect(),
            Err(err) => {
                godot_warn!(
                    "The lock for the to_add queue was poisoned. A component might not have been added"
                );
                err.into_inner().drain().collect()
            }
        };

        self.to_add.clear_poison();

        for component_with_class_for_node in to_add {
            let component_with_class = component_with_class_for_node.component_with_class;
            let component_class = component_with_class.component_class.clone();
            let mut node_entity =
                self.get_or_create_node_entity(component_with_class_for_node.node);
            node_entity
                .bind_mut()
                .add_component(component_with_class.clone());
            changed_component_classes.insert(component_class.clone());
            changed_nodes.insert(node_entity.clone());
            self.signals().component_added().emit(
                &node_entity,
                &component_class,
                &component_with_class.component,
            );
        }
        self.update_caches(changed_nodes, changed_component_classes);
    }

    pub fn update_caches(
        &mut self,
        changed_nodes: HashSet<Gd<NodeEntity>>,
        changed_component_classes: HashSet<StringName>,
    ) {
        if !changed_nodes.is_empty() {
            self.all_instances.clear();
            self.all_instances_internal.retain(|(_, _, x)| {
                changed_nodes.contains(&x.bind().get_node_entity().unwrap_or_default())
            });
            let mut instances_by_component_class: HashMap<StringName, HashSet<Gd<Component>>> =
                HashMap::new();

            //NOTE: Possible performance improvement by also doing a complete update when a majority (exact percentage to be determined when doing this) has changed
            let update_all_components = changed_component_classes.len() == self.node_entities.len();
            let update_all_nodes = changed_nodes.len() == self.node_entities.len();

            for node_entity in self.node_entities.values() {
                let node = node_entity
                    .bind()
                    .node
                    .clone()
                    .expect("Node entity had no node");
                for component in &node_entity.bind().components {
                    let instance = &component.component;
                    if update_all_components
                        || changed_component_classes.contains(&component.component_class)
                    {
                        instances_by_component_class
                            .entry(component.component_class.clone())
                            .or_default()
                            .insert(instance.clone());
                    }
                    self.all_instances_internal.push((
                        node_entity.clone(),
                        node.clone(),
                        instance.clone(),
                    ));
                }
                if update_all_nodes || changed_nodes.contains(&node_entity.clone()) {
                    self.all_instances
                        .set(node, node_entity.bind().get_all_components());
                }
            }
            let instances_by_component_class_godot = instances_by_component_class
                .iter()
                .map(|x| (x.0.clone(), x.1.iter().cloned().collect()))
                .collect();
            if update_all_components {
                self.instances_by_component_class = instances_by_component_class.clone();
                self.instances_by_component_class_godot = instances_by_component_class_godot;
            } else {
                self.instances_by_component_class
                    .retain(|k, _| !changed_component_classes.contains(k));
                self.instances_by_component_class
                    .extend(instances_by_component_class.clone());
                self.instances_by_component_class_godot
                    .retain(|k, _| !changed_component_classes.contains(k));
                self.instances_by_component_class_godot
                    .extend(instances_by_component_class_godot);
            }
        }
    }
}
