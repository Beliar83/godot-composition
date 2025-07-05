use crate::component::Component;
use godot::classes::{ClassDb, ResourceLoader, Script};
use godot::prelude::*;
use std::hash::{Hash, Hasher};
use std::sync::LazyLock;

static COMPONENT_CLASS_NAME: &str = "component_class";
pub static VALUES_NAME: &str = "values";
pub static BASE_CLASS_NAME: &str = "base_class";
pub static SCRIPT_NAME: &str = "script";
pub static COMPONENT_CLASS_STRING_NAME: LazyLock<StringName> =
    LazyLock::new(|| StringName::from(COMPONENT_CLASS_NAME));

#[derive(Clone, Eq, Default)]
pub struct ComponentWithClass {
    pub component_class: StringName,
    pub component: Gd<Component>,
}

impl ComponentWithClass {
    pub fn create(component_class: StringName, component: Gd<Component>) -> Self {
        Self {
            component,
            component_class,
        }
    }
}

impl Hash for ComponentWithClass {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Hash::hash(&self.component_class, state);
    }
}

impl PartialEq for ComponentWithClass {
    fn eq(&self, other: &Self) -> bool {
        self.component_class.eq(&other.component_class)
    }
}

impl GodotConvert for ComponentWithClass {
    type Via = Dictionary;
}

impl ToGodot for ComponentWithClass {
    type ToVia<'v> = Self::Via;

    fn to_godot(&self) -> Self::Via {
        let mut dict = Dictionary::new();
        dict.set(
            COMPONENT_CLASS_STRING_NAME.to_variant(),
            self.component_class.to_variant(),
        );
        dict.set(
            BASE_CLASS_NAME,
            StringName::from(self.component.get_class()),
        );
        if !self.component.get_script().is_nil() {
            dict.set(
                SCRIPT_NAME,
                self.component.get_script().to::<Gd<Script>>().get_path(),
            );
        }
        dict.set(
            VALUES_NAME.to_variant(),
            self.component.clone().bind_mut().get_values(),
        );
        dict
    }
}

impl FromGodot for ComponentWithClass {
    fn try_from_godot(via: Self::Via) -> Result<Self, ConvertError> {
        let class_name = match via
            .at(COMPONENT_CLASS_STRING_NAME.to_variant())
            .try_to::<StringName>()
        {
            Ok(class_name) => class_name,
            Err(error) => {
                return Err(ConvertError::new(format!("Invalid class_name: {}", error)));
            }
        };
        let base_class = match via.at(BASE_CLASS_NAME.to_variant()).try_to::<StringName>() {
            Ok(base_class) => base_class,
            Err(error) => {
                return Err(ConvertError::new(format!("Invalid base_class: {}", error)));
            }
        };

        let mut component = ClassDb::singleton()
            .instantiate(&base_class)
            .to::<Gd<Component>>();
        if via.contains_key(SCRIPT_NAME) {
            let script_path = via.at(SCRIPT_NAME).to::<GString>();
            match ResourceLoader::singleton().load(&script_path) {
                None => {
                    godot_print!("Could not load script: {}", script_path);
                }
                Some(script) => {
                    component.set_script(&script.to_variant());
                }
            };
        }
        let values = match via.at(VALUES_NAME.to_variant()).try_to::<Dictionary>() {
            Ok(values) => values,
            Err(error) => {
                return Err(ConvertError::new(format!("values: {}", error)));
            }
        };
        component.bind_mut().set_values(values);
        Ok(Self::create(class_name, component))
    }
}
