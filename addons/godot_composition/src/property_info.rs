use godot::builtin::{Dictionary, StringName, VariantType};
use godot::global::{PropertyHint, PropertyUsageFlags};
use godot::meta::{ClassName, PropertyHintInfo, PropertyInfo};
use godot::obj::{EngineBitfield, EngineEnum};

pub(crate) fn create_property_from_dictionary(property: Dictionary) -> PropertyInfo {
    let name = property
        .get("name")
        .expect("Property in property list has no name")
        .stringify();
    let class_name = ClassName::none();
    let property_name = StringName::from(name);
    let usage = property
        .get("usage")
        .expect("Property in property list has no usage")
        .to::<u64>();
    let usage = PropertyUsageFlags::from_ord(usage);
    let variant_type = property
        .get("type")
        .expect("Property in property list has no type")
        .to::<VariantType>();
    let hint = property
        .get("hint")
        .expect("Property in property list has no hint")
        .to::<i32>();
    let hint = PropertyHint::from_ord(hint);
    let hint_string = property
        .get("hint_string")
        .expect("Property in property list has no hint_string")
        .stringify();
    let hint_info = PropertyHintInfo { hint, hint_string };
    PropertyInfo {
        class_name,
        property_name,
        usage,
        variant_type,
        hint_info,
    }
}
