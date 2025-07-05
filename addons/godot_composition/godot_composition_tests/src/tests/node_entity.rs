use crate::tests::godot_composition_world::{INT_FIELD_KEY, STRING_FIELD_KEY};
use gd_rehearse::itest::gditest;
use godot::builtin::StringName;
use godot::obj::{NewGd, WithUserSignals};
use godot::prelude::*;
use godot_composition_core::component::Component;
use godot_composition_core::node_entity::NodeEntity;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

#[gditest]
fn set_component_should_add_a_non_existing_component() {
    let mut entity = NodeEntity::new_gd();
    let component = Component::new_gd();
    let component_class = StringName::from("Test");
    let result = entity
        .bind_mut()
        .set_component(component_class.clone(), Some(component.clone()));
    assert!(result);
    assert!(entity.bind().has_component_of_class(component_class));
    assert_eq!(component.bind().get_node_entity().unwrap(), entity);
}

#[gditest]
fn set_component_should_remove_an_existing_component() {
    let mut entity = NodeEntity::new_gd();
    let component = Component::new_gd();
    let component_class = StringName::from("Test");
    entity
        .bind_mut()
        .set_component(component_class.clone(), Some(component.clone()));
    let result = entity
        .bind_mut()
        .set_component(component_class.clone(), None);
    assert!(result);
    assert!(!entity.bind().has_component_of_class(component_class));
    assert!(component.bind().get_node_entity().is_none())
}

#[gditest]
fn set_component_should_replace_an_existing_component() {
    let mut entity = NodeEntity::new_gd();
    let existing_component = Component::new_gd();
    let component_class = StringName::from("Test");
    entity
        .bind_mut()
        .set_component(component_class.clone(), Some(existing_component.clone()));
    let new_component = Component::new_gd();
    let result = entity
        .bind_mut()
        .set_component(component_class.clone(), Some(new_component.clone()));
    assert!(result);
    let component = entity
        .bind()
        .get_component_of_class_or_null(component_class.clone())
        .unwrap();
    assert_eq!(component, new_component);

    assert!(existing_component.bind().get_node_entity().is_none());
    assert_eq!(new_component.bind().get_node_entity().unwrap(), entity);
}

#[gditest]
fn set_component_should_emit_a_signal_when_a_new_component_is_added() {
    let mut entity = NodeEntity::new_gd();
    let component = Component::new_gd();
    let component_class = StringName::from("Test");

    let called = Rc::new(RefCell::new(false));
    {
        let component = component.clone();
        let component_class = component_class.clone();
        let entity_copy = entity.clone();
        let called = called.clone();
        entity.bind_mut().signals().component_changed().connect(
            move |p_entity: Gd<NodeEntity>,
                  p_component_class: StringName,
                  p_component: Option<Gd<Component>>,
                  old_component: Option<Gd<Component>>| {
                called.replace(true);
                assert_eq!(p_entity, entity_copy);
                assert_eq!(p_component_class, component_class);
                assert_eq!(p_component.unwrap(), component.clone());
                assert!(old_component.is_none());
            },
        );
    }

    entity
        .bind_mut()
        .set_component(component_class.clone(), Some(component));
    let called = *called.borrow();
    assert!(called);
}

#[gditest]
fn set_component_should_emit_signal_when_a_component_is_removed() {
    let mut entity = NodeEntity::new_gd();
    let component = Component::new_gd();
    let component_class = StringName::from("Test");

    entity
        .bind_mut()
        .set_component(component_class.clone(), Some(component.clone()));
    let called = Rc::new(RefCell::new(false));
    {
        let component_class = component_class.clone();
        let entity_copy = entity.clone();
        let called = called.clone();
        entity.bind_mut().signals().component_changed().connect(
            move |p_entity: Gd<NodeEntity>,
                  p_component_class: StringName,
                  p_component: Option<Gd<Component>>,
                  old_component: Option<Gd<Component>>| {
                called.replace(true);
                assert_eq!(p_entity, entity_copy);
                assert_eq!(p_component_class, component_class);
                assert!(p_component.is_none());
                assert_eq!(old_component.unwrap(), component);
            },
        );
    }

    entity
        .bind_mut()
        .set_component(component_class.clone(), None);

    let called = *called.borrow();
    assert!(called);
}

#[gditest]
fn set_component_should_emit_a_signal_when_a_component_is_replaced() {
    let mut entity = NodeEntity::new_gd();
    let existing_component = Component::new_gd();
    let component_class = StringName::from("Test");
    let new_component = Component::new_gd();

    entity
        .bind_mut()
        .set_component(component_class.clone(), Some(existing_component.clone()));
    let called = Rc::new(RefCell::new(false));
    {
        let component_class = component_class.clone();
        let existing_component = existing_component.clone();
        let new_component = new_component.clone();
        let entity_copy = entity.clone();
        let called = called.clone();
        entity.bind_mut().signals().component_changed().connect(
            move |p_entity: Gd<NodeEntity>,
                  p_component_class: StringName,
                  p_component: Option<Gd<Component>>,
                  old_component: Option<Gd<Component>>| {
                called.replace(true);
                assert_eq!(p_entity, entity_copy);
                assert_eq!(p_component_class, component_class);
                assert_eq!(p_component.unwrap(), new_component);
                assert_eq!(old_component.unwrap(), existing_component);
            },
        );
    }

    entity
        .bind_mut()
        .set_component(component_class.clone(), Some(new_component));

    let called = *called.borrow();
    assert!(called);
}

#[gditest]
fn set_components_should_replace_all_components() {
    let mut entity = NodeEntity::new_gd();

    let old_component_name = StringName::from("Old component");
    entity
        .bind_mut()
        .set_component(old_component_name.clone(), Some(Component::new_gd()));

    let mut components = Vec::<Dictionary>::new();

    let mut component_1_data = Dictionary::new();

    component_1_data.set(
        godot_composition_core::component_with_class::BASE_CLASS_NAME.to_variant(),
        Component::class_name().to_string_name().to_variant(),
    );

    let component_1_values = Dictionary::new();
    component_1_data.set(
        godot_composition_core::component_with_class::VALUES_NAME.to_variant(),
        component_1_values.to_variant(),
    );

    let component_1_name = StringName::from("another_component");
    component_1_data.set(
        godot_composition_core::component_with_class::COMPONENT_CLASS_STRING_NAME.to_variant(),
        component_1_name.to_variant(),
    );

    components.push(component_1_data);

    let mut component_2_data = Dictionary::new();

    component_2_data.set(
        godot_composition_core::component_with_class::BASE_CLASS_NAME.to_variant(),
        Component::class_name().to_string_name().to_variant(),
    );

    let mut component_2_values = Dictionary::new();

    component_2_values.set(INT_FIELD_KEY.to_variant(), 999.to_variant());
    component_2_values.set(STRING_FIELD_KEY.to_variant(), "Zero Escape");
    component_2_data.set(
        godot_composition_core::component_with_class::SCRIPT_NAME.to_variant(),
        "res://src/scripts/component_with_multiple_fields.rs".to_variant(),
    );

    component_2_data.set(
        godot_composition_core::component_with_class::VALUES_NAME.to_variant(),
        component_2_values.to_variant(),
    );
    let component_name = StringName::from("component_with_multiple_fields");
    component_2_data.set(
        godot_composition_core::component_with_class::COMPONENT_CLASS_STRING_NAME.to_variant(),
        component_name.to_variant(),
    );

    components.push(component_2_data);

    entity.bind_mut().set_components(components);

    assert_eq!(entity.bind_mut().get_all_components().len(), 2);

    assert_eq!(
        entity
            .bind_mut()
            .get_component_of_class_or_null(old_component_name),
        None
    );

    let component_1 = entity
        .bind_mut()
        .get_component_of_class_or_null(component_1_name)
        .unwrap();
    assert!(component_1.get_script().is_nil());
    let component_2 = entity
        .bind_mut()
        .get_component_of_class_or_null(component_name)
        .unwrap();
    assert!(!component_2.get_script().is_nil());
    assert_eq!(component_2.get(INT_FIELD_KEY.deref()), 999.to_variant());
    assert_eq!(
        component_2.get(STRING_FIELD_KEY.deref()),
        "Zero Escape".to_variant()
    );
}
