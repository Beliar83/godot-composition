use crate::component::Component;
use godot::prelude::*;
use std::hash::{Hash, Hasher};
use std::sync::LazyLock;

static COMPONENT_CLASS_NAME: &str = "component_class";
static COMPONENT_NAME: &str = "component";
static COMPONENT_CLASS_STRING_NAME: LazyLock<StringName> =
    LazyLock::new(|| StringName::from(COMPONENT_CLASS_NAME));
static COMPONENT_STRING_NAME: LazyLock<StringName> =
    LazyLock::new(|| StringName::from(COMPONENT_NAME));

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
            COMPONENT_STRING_NAME.to_variant(),
            self.component.to_variant(),
        );
        dict
    }
}

impl FromGodot for ComponentWithClass {
    fn try_from_godot(via: Self::Via) -> Result<Self, ConvertError> {
        let class_name_key = COMPONENT_CLASS_STRING_NAME.to_variant();
        let component_key = COMPONENT_STRING_NAME.to_variant();
        let keys_array = VariantArray::from(&[class_name_key.clone(), component_key.clone()]);
        if via.contains_all_keys(&keys_array) {
            let class_name = via
                .get(class_name_key)
                .expect("The engine said the key is present")
                .try_to::<StringName>();
            let component = via
                .get(component_key)
                .expect("The engine said the key is present")
                .try_to::<Gd<Component>>();

            match (class_name, component) {
                (Ok(class_name), Ok(component)) => Ok(Self::create(class_name, component)),
                (Ok(_), Err(component_error)) => Err(ConvertError::new(format!(
                    "Invalid component: {}",
                    component_error
                ))),
                (Err(class_name_error), Ok(_)) => Err(ConvertError::new(format!(
                    "Invalid class_name: {}",
                    class_name_error
                ))),
                (Err(class_name_error), Err(component_error)) => Err(ConvertError::new(format!(
                    "Invalid class_name: {}, Invalid component: {}",
                    class_name_error, component_error
                ))),
            }
        } else {
            Err(ConvertError::new("Dictionary does not contain all keys"))
        }
    }
}
