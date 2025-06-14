use gd_rehearse::itest::gditest;
use godot::builtin::StringName;
use godot::obj::{NewGd, WithUserSignals};
use godot::prelude::*;
use godot_composition_core::component::Component;
use godot_composition_core::node_entity::NodeEntity;
use std::cell::RefCell;
use std::rc::Rc;

#[gditest]
fn set_component_adds_a_non_existing_component() {
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
fn set_component_removes_an_existing_component() {
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
fn set_component_replaces_an_existing_component() {
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
fn set_component_emits_signal_when_a_new_component_is_added() {
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
fn set_component_emits_signal_when_a_component_is_removed() {
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
fn set_component_emits_signal_when_a_component_is_replaced() {
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
