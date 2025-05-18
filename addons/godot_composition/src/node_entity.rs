use crate::component::Component;
use crate::component_with_class::ComponentWithClass;
use godot::prelude::*;
use std::collections::HashSet;

#[derive(GodotClass)]
#[class(init, base=RefCounted)]
/// Allows access to the components of a Node
pub struct NodeEntity {
    pub(crate) components: HashSet<ComponentWithClass>,
    #[var]
    pub node: Option<Gd<Node>>,
    base: Base<RefCounted>,
}

impl NodeEntity {
    pub fn create_for_node(node: Gd<Node>) -> Gd<Self> {
        let mut instance = Self::new_gd();
        instance.bind_mut().node = Some(node);
        instance
    }

    pub fn add_component(&mut self, component_with_class: ComponentWithClass) -> bool {
        if !self.components.insert(component_with_class.clone()) {
            godot_error!("NodeEntity already has that component");
            false
        } else {
            let component = component_with_class.component;
            self.set_entity_of_component_to_self(&component);
            self.base_mut().notify_property_list_changed();
            self.signals()
                .component_added()
                .emit(&component_with_class.component_class, &component);
            true
        }
    }

    fn set_entity_of_component_to_self(&self, component: &Gd<Component>) {
        Callable::from_local_fn("set_component_entity", move |args| {
            let entity = args
                .first()
                .expect("expected first argument to be a node")
                .to::<Gd<NodeEntity>>();
            let mut component = args
                .get(1)
                .expect("expected second argument to be a Component")
                .to::<Gd<Component>>();
            component.bind_mut().set_node_entity(Some(entity));
            Ok(Variant::nil())
        })
        .call_deferred(&[self.to_gd().to_variant(), component.to_variant()]);
    }

    pub fn remove_component(&mut self, component_class: StringName) -> bool {
        let key = ComponentWithClass::create(component_class.clone(), Gd::default());
        let removed = self.components.remove(&key);
        self.signals().component_removed().emit(&component_class);
        removed
    }
}

#[godot_api]
impl NodeEntity {
    #[signal]
    /// Emitted when a component was removed from this Entity
    fn component_removed(component_class: StringName);

    #[signal]
    /// Emitted when a component was added to this Entity
    fn component_added(component_class: StringName, component: Gd<Component>);

    #[func]
    /// Execute a callable for all components of this Entity
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
        for component in &self.components {
            func.call_deferred(&[
                component.component_class.to_variant(),
                component.component.to_variant(),
            ]);
        }
    }

    #[func]
    /// Return all components as a list of dictionaries
    pub fn get_all_components(&self) -> Vec<Dictionary> {
        self.components.iter().map(|x| x.to_godot()).collect()
    }

    #[func]
    /// Sets components from a list of dictionaries
    pub fn set_components(&mut self, components: Vec<Dictionary>) -> Vec<StringName> {
        let mut component_classes = Vec::new();
        let components: Vec<_> = components
            .iter()
            .map(|x| {
                let component_with_class = ComponentWithClass::from_godot(x.clone());
                self.set_entity_of_component_to_self(&component_with_class.component);
                component_with_class
            })
            .collect();
        component_classes.extend(components.iter().map(|x| x.component_class.clone()));
        self.components = HashSet::from_iter(components);
        component_classes
    }

    #[func]
    /// Returns if the Entity has a component of the given component class
    pub fn has_component_of_class(&self, component_class: StringName) -> bool {
        let key = ComponentWithClass::create(component_class, Gd::default());
        self.components.contains(&key)
    }

    #[func]
    /// Returns the component of the given component class, or null if the Entity does not have a component of that class
    pub fn get_component_of_class_or_null(
        &self,
        component_class: StringName,
    ) -> Option<Gd<Component>> {
        let key = ComponentWithClass::create(component_class, Gd::default());
        self.components.get(&key).map(|x| x.component.clone())
    }
}
