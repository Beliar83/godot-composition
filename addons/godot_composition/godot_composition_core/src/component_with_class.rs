use crate::component::Component;
use godot::classes::{ClassDb, ResourceLoader, Script};
use godot::prelude::*;
use std::hash::{Hash, Hasher};
use std::sync::LazyLock;

static COMPONENT_CLASS_NAME: &str = "component_class";
static COMPONENT_NAME: &str = "component";
static VALUES_NAME: &str = "values";
static BASE_CLASS_NAME: &str = "base_class";
static SCRIPT_NAME: &str = "script";
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
        let mut component_dict = Dictionary::new();
        component_dict.set(
            BASE_CLASS_NAME,
            StringName::from(self.component.get_class()),
        );
        component_dict.set(
            VALUES_NAME.to_variant(),
            self.component.clone().bind_mut().get_values(),
        );
        if !self.component.get_script().is_nil() {
            component_dict.set(SCRIPT_NAME, self.component.get_script().to::<Gd<Script>>().get_path());
        }
        dict.set(COMPONENT_STRING_NAME.to_variant(), component_dict);
        dict
    }
}

impl FromGodot for ComponentWithClass {
    fn try_from_godot(via: Self::Via) -> Result<Self, ConvertError> {
        let class_name_key = COMPONENT_CLASS_STRING_NAME.to_variant();
        let component_key = COMPONENT_STRING_NAME.to_variant();
        let keys_array = VariantArray::from(&[class_name_key.clone(), component_key.clone()]);
        if via.contains_all_keys(&keys_array) {
            let class_name = via.at(class_name_key).try_to::<StringName>();
            let component = via.at(component_key).try_to::<Dictionary>();

            match (class_name, component) {
                (Ok(class_name), Ok(component_dict)) => {
                    let keys_array = VariantArray::from(&[
                        BASE_CLASS_NAME.to_variant(),
                        VALUES_NAME.to_variant(),
                    ]);
                    if !component_dict.contains_all_keys(&keys_array) {
                        Err(ConvertError::new(
                            "Component dictionary does not contain all keys",
                        ))
                    } else {
                        let base_class = component_dict.at(BASE_CLASS_NAME).try_to::<StringName>();
                        let values = component_dict.at(VALUES_NAME).try_to::<Dictionary>();
                        match (base_class, values) {
                            (Ok(base_class), Ok(values)) => {
                                let mut component = ClassDb::singleton()
                                    .instantiate(&base_class)
                                    .to::<Gd<Component>>();
                                if component_dict.contains_key(SCRIPT_NAME) {
                                    let script_path = component_dict.at(SCRIPT_NAME).to::<GString>();
                                    match ResourceLoader::singleton().load(&script_path) {
                                        None => {
                                            godot_print!("Could not load script: {}", script_path);   
                                        }
                                        Some(script) => {
                                            component.set_script(&script.to_variant());                                            
                                        }
                                    };
                                }
                                component.bind_mut().set_values(values);
                                Ok(Self::create(class_name, component))
                            }
                            (Ok(_), Err(values_error)) => Err(ConvertError::new(format!(
                                "Invalid values: {}",
                                values_error
                            ))),
                            (Err(base_class_error), Ok(_)) => Err(ConvertError::new(format!(
                                "Invalid base_class: {}",
                                base_class_error
                            ))),
                            (Err(base_class_error), Err(values_error)) => {
                                Err(ConvertError::new(format!(
                                    "Invalid base_class: {}, Invalid values: {}",
                                    base_class_error, values_error
                                )))
                            }
                        }
                    }
                }
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
