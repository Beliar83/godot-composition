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

    pub fn set_component(
        &mut self,
        component_class: StringName,
        component: Option<Gd<Component>>,
    ) -> bool {
        match component {
            None => match self.get_component_of_class_or_null(component_class.clone()) {
                None => false,
                Some(mut old_component) => {
                    self.components.remove(&ComponentWithClass::create(
                        component_class.clone(),
                        Gd::default(),
                    ));
                    old_component.bind_mut().set_node_entity(None);
                    let self_gd = self.to_gd();
                    self.signals().component_changed().emit(
                        &self_gd,
                        &component_class.clone(),
                        None,
                        Some(&old_component),
                    );
                    true
                }
            },
            Some(mut component) => {
                let component_with_class =
                    ComponentWithClass::create(component_class.clone(), component.clone());
                let old_component = self.components.replace(component_with_class);
                let old_component = match old_component {
                    None => None,
                    Some(mut old_component) => {
                        old_component.component.bind_mut().set_node_entity(None);
                        Some(old_component.component)
                    }
                };
                self.set_entity_of_component_to_self(&mut component);
                let self_gd = self.to_gd();
                self.signals().component_changed().emit(
                    &self_gd,
                    &component_class.clone(),
                    Some(&component),
                    old_component.as_ref(),
                );
                true
            }
        }
    }

    fn set_entity_of_component_to_self(&self, component: &mut Gd<Component>) {
        component.bind_mut().set_node_entity(Some(self.to_gd()));
    }
}

#[godot_api]
impl NodeEntity {
    #[signal]
    /// Emitted when a component was changed on this Entity
    pub fn component_changed(
        node_entity: Gd<NodeEntity>,
        component_class: StringName,
        component: Option<Gd<Component>>,
        old_component: Option<Gd<Component>>,
    );

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
                let mut component_with_class = ComponentWithClass::from_godot(x.clone());
                self.set_entity_of_component_to_self(&mut component_with_class.component);
                component_with_class
            })
            .collect();
        component_classes.extend(components.iter().map(|x| x.component_class.clone()));
        for component in self.components.clone() {
            self.set_component(component.component_class.clone(), None);
        }
        for component in components {
            self.set_component(component.component_class.clone(), Some(component.component));
        }
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
