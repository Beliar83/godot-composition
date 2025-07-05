use crate::components_editor::PROPERTIES_TO_IGNORE;
use crate::node_entity::NodeEntity;
use crate::property_info::create_property_from_dictionary;
use godot::classes::ClassDb;
use godot::global::PropertyUsageFlags;
use godot::meta::PropertyInfo;
use godot::prelude::*;
use std::collections::HashSet;
use std::sync::LazyLock;

static BASE_PROPERTIES: LazyLock<HashSet<StringName>> =
    LazyLock::<HashSet<StringName>>::new(|| {
        let mut property_names = HashSet::new();
        let properties = ClassDb::singleton()
            .class_get_property_list_ex(&Component::class_name().to_string_name())
            .no_inheritance(false)
            .done();
        for property in properties.iter_shared() {
            let name = create_property_from_dictionary(property).property_name;
            property_names.insert(name);
        }
        property_names
    });

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
        let mut self_gd = self.to_gd();
        Callable::from_local_fn("emit_entity_change", move |_| {
            self_gd.bind_mut().entity_changed(node_entity.clone());
            Ok(Variant::nil())
        })
        .call_deferred(&[]);
    }

    pub fn get_values(&mut self) -> Dictionary {
        let mut values = Dictionary::new();
        for property in self.base().get_property_list().iter_shared() {
            let property = create_property_from_dictionary(property);
            if PROPERTIES_TO_IGNORE.contains(&property.property_name) {
                continue;
            }
            if property.usage.ord() & PropertyUsageFlags::STORAGE.ord()
                == PropertyUsageFlags::STORAGE.ord()
            {
                values.set(
                    property.property_name.clone(),
                    self.base().get(&property.property_name),
                );
            }
        }
        values
    }

    pub fn set_values(&mut self, values: Dictionary) {
        for (property_name, value) in values.iter_shared() {
            let property_name = property_name.to::<StringName>();
            if PROPERTIES_TO_IGNORE.contains(&property_name) {
                continue;
            }
            self.base_mut().set(&property_name, &value)
        }
    }
}

#[godot_api]
impl IRefCounted for Component {
    fn validate_property(&self, property: &mut PropertyInfo) {
        if BASE_PROPERTIES.contains(&property.property_name) {
            property.usage = PropertyUsageFlags::from_ord(
                property.usage.ord() & !PropertyUsageFlags::EDITOR.ord(),
            );
        }
    }
}
