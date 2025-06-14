use crate::tests::test_object::TestObject;
use gd_rehearse::itest::gditest;
use godot::classes::object::ConnectFlags;
use godot::classes::{ResourceLoader, Script};
use godot::prelude::*;
use godot_composition_core::component::Component;
use godot_composition_core::godot_composition_world::GodotCompositionWorld;
use godot_composition_core::node_entity::NodeEntity;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

#[gditest]
fn get_node_entity_or_null_should_return_none_for_non_existing_node_entities() {
    let instance = GodotCompositionWorld::get_singleton();
    let node = Node::new_alloc();

    assert!(
        instance
            .bind()
            .get_node_entity_or_null(node.clone())
            .is_none()
    );

    node.free();
}

#[gditest]
fn get_or_create_node_entity_should_create_a_node_entity_if_it_does_not_exist() {
    let mut instance = GodotCompositionWorld::get_singleton();
    let node = Node::new_alloc();

    let entity = instance.bind_mut().get_or_create_node_entity(node.clone());
    assert!(entity.is_instance_valid());

    node.free();
    instance
        .bind_mut()
        .remove_all_entities_and_pending_changes()
}

#[gditest]
fn get_or_create_node_entity_should_return_existing_node_entity_for_same_node() {
    let mut instance = GodotCompositionWorld::get_singleton();
    let node = Node::new_alloc();

    let first_call_result = instance.bind_mut().get_or_create_node_entity(node.clone());
    let second_call_result = instance.bind_mut().get_or_create_node_entity(node.clone());
    assert_eq!(first_call_result, second_call_result);

    node.free();
    instance
        .bind_mut()
        .remove_all_entities_and_pending_changes()
}

#[gditest]
fn get_or_create_node_entity_should_emit_a_signal_when_a_new_node_entity_is_created() {
    let mut instance = GodotCompositionWorld::get_singleton();
    let node = Node::new_alloc();

    {
        let node = node.clone();
        instance
            .bind_mut()
            .signals()
            .node_entity_created()
            .builder()
            .flags(ConnectFlags::ONE_SHOT)
            .connect(move |node_entity: Gd<NodeEntity>| {
                assert_eq!(node_entity.bind().node.clone().unwrap(), node);
            });
    }

    instance.bind_mut().get_or_create_node_entity(node.clone());

    node.free();
    instance
        .bind_mut()
        .remove_all_entities_and_pending_changes()
}

#[gditest]
fn get_node_entity_or_null_should_return_existing_instance_for_existing_node_entities() {
    let mut instance = GodotCompositionWorld::get_singleton();
    let node = Node::new_alloc();
    let entity = instance.bind_mut().get_or_create_node_entity(node.clone());

    let found_entity = instance.bind().get_node_entity_or_null(node.clone());
    assert!(found_entity.is_some());
    assert_eq!(found_entity.unwrap(), entity);

    node.free();
    instance
        .bind_mut()
        .remove_all_entities_and_pending_changes();
}

#[gditest]
fn set_component_of_node_adds_component_to_node_entity_after_process() {
    let mut instance = GodotCompositionWorld::get_singleton();
    let component = Component::new_gd();

    let node = Node::new_alloc();

    let component_class = StringName::from("test");
    let entity = instance.bind_mut().get_or_create_node_entity(node.clone());

    instance.bind_mut().set_component_of_node(
        node.clone(),
        component_class.clone(),
        Some(component.clone()),
    );
    instance.bind_mut().process(0f64);
    assert!(
        entity
            .bind()
            .has_component_of_class(component_class.clone())
    );

    node.free();
    instance
        .bind_mut()
        .remove_all_entities_and_pending_changes();
}

#[gditest]
fn set_component_of_node_removes_component_from_node_entity_after_process() {
    let mut instance = GodotCompositionWorld::get_singleton();
    let component = Component::new_gd();

    let node = Node::new_alloc();
    let component_class = StringName::from("test");
    let mut entity = instance.bind_mut().get_or_create_node_entity(node.clone());
    entity
        .bind_mut()
        .set_component(component_class.clone(), Some(component));

    let result =
        instance
            .bind_mut()
            .set_component_of_node(node.clone(), component_class.clone(), None);
    assert!(result);
    instance.bind_mut().process(0f64);
    assert!(
        !entity
            .bind()
            .has_component_of_class(component_class.clone())
    );

    instance
        .bind_mut()
        .remove_all_entities_and_pending_changes();
    node.free();
}

#[gditest]
fn set_component_of_node_replaces_component_from_node_entity_after_process() {
    let mut instance = GodotCompositionWorld::get_singleton();
    let existing_component = Component::new_gd();

    let node = Node::new_alloc();
    let component_class = StringName::from("test");
    let mut entity = instance.bind_mut().get_or_create_node_entity(node.clone());
    entity
        .bind_mut()
        .set_component(component_class.clone(), Some(existing_component));

    let mut new_component = Component::new_gd();
    new_component.set_meta("test", &Variant::from(999));

    instance.bind_mut().set_component_of_node(
        node.clone(),
        component_class.clone(),
        Some(new_component),
    );
    instance.bind_mut().process(0f64);

    let component = entity
        .bind()
        .get_component_of_class_or_null(component_class.clone())
        .unwrap();
    assert_eq!(component.get_meta("test").to::<i64>(), 999);

    instance
        .bind_mut()
        .remove_all_entities_and_pending_changes();
    node.free();
}

#[gditest]
fn set_component_of_node_does_not_allow_changing_a_component_of_a_class_that_is_already_staged_for_changing()
 {
    let mut instance = GodotCompositionWorld::get_singleton();
    let mut component = Component::new_gd();
    let node = Node::new_alloc();
    let component_class = StringName::from("test");
    let entity = instance.bind_mut().get_or_create_node_entity(node.clone());
    component.set_meta("test", &Variant::from(999));
    instance.bind_mut().set_component_of_node(
        node.clone(),
        component_class.clone(),
        Some(component),
    );
    let component = Component::new_gd();

    let result = instance.bind_mut().set_component_of_node(
        node.clone(),
        component_class.clone(),
        Some(component.clone()),
    );
    assert!(!result);

    instance.bind_mut().process(0f64);
    let component = entity
        .bind()
        .get_component_of_class_or_null(component_class.clone())
        .unwrap();
    assert_eq!(component.get_meta("test").to::<i64>(), 999);

    let component = Component::new_gd();
    instance.bind_mut().set_component_of_node(
        node.clone(),
        component_class.clone(),
        Some(component),
    );

    let result =
        instance
            .bind_mut()
            .set_component_of_node(node.clone(), component_class.clone(), None);
    assert!(!result);
    instance.bind_mut().process(0f64);
    assert!(
        entity
            .bind()
            .has_component_of_class(component_class.clone())
    );

    node.free();
    instance
        .bind_mut()
        .remove_all_entities_and_pending_changes();
}

#[gditest]
fn set_component_emits_signal_when_a_new_component_is_added() {
    let mut instance = GodotCompositionWorld::get_singleton();
    let component = Component::new_gd();

    let node = Node::new_alloc();

    let component_class = StringName::from("test");
    let called = Rc::new(RefCell::new(false));
    {
        let component = component.clone();
        let component_class = component_class.clone();
        let node = node.clone();
        let called = called.clone();
        instance
            .bind_mut()
            .signals()
            .component_changed()
            .builder()
            .flags(ConnectFlags::ONE_SHOT)
            .connect(
                move |node_entity: Gd<NodeEntity>,
                      p_component_class: StringName,
                      p_component: Option<Gd<Component>>,
                      old_component: Option<Gd<Component>>| {
                    called.replace(true);
                    let mut instance = GodotCompositionWorld::get_singleton();
                    let entity = instance.bind_mut().get_or_create_node_entity(node.clone());
                    assert_eq!(node_entity, entity);
                    assert_eq!(p_component_class, component_class);
                    assert_eq!(p_component.unwrap(), component.clone());
                    assert!(old_component.is_none());
                },
            );
    }

    instance.bind_mut().set_component_of_node(
        node.clone(),
        component_class.clone(),
        Some(component.clone()),
    );
    instance.bind_mut().process(0f64);

    let called = *called.borrow();
    assert!(called);

    instance
        .bind_mut()
        .remove_all_entities_and_pending_changes();
    node.free();
}

#[gditest]
fn set_component_emits_signal_when_a_component_is_removed() {
    let mut instance = GodotCompositionWorld::get_singleton();
    let component = Component::new_gd();

    let node = Node::new_alloc();
    let component_class = StringName::from("test");
    let mut entity = instance.bind_mut().get_or_create_node_entity(node.clone());
    entity
        .bind_mut()
        .set_component(component_class.clone(), Some(component.clone()));
    let called = Rc::new(RefCell::new(false));
    {
        let component_class = component_class.clone();
        let node = node.clone();
        let called = called.clone();
        instance
            .bind_mut()
            .signals()
            .component_changed()
            .builder()
            .flags(ConnectFlags::ONE_SHOT)
            .connect(
                move |node_entity: Gd<NodeEntity>,
                      p_component_class: StringName,
                      p_component: Option<Gd<Component>>,
                      old_component: Option<Gd<Component>>| {
                    called.replace(true);
                    let mut instance = GodotCompositionWorld::get_singleton();
                    let entity = instance.bind_mut().get_or_create_node_entity(node.clone());
                    assert_eq!(node_entity, entity);
                    assert_eq!(p_component_class, component_class);
                    assert!(p_component.is_none());
                    assert_eq!(old_component.unwrap(), component);
                },
            );
    }

    instance
        .bind_mut()
        .set_component_of_node(node.clone(), component_class.clone(), None);
    instance.bind_mut().process(0f64);
    let called = *called.borrow();
    assert!(called);

    instance
        .bind_mut()
        .remove_all_entities_and_pending_changes();
    node.free();
}

#[gditest]
fn set_component_emits_signal_when_a_component_is_replaced() {
    let mut instance = GodotCompositionWorld::get_singleton();
    let existing_component = Component::new_gd();

    let node = Node::new_alloc();
    let component_class = StringName::from("test");
    let mut entity = instance.bind_mut().get_or_create_node_entity(node.clone());
    entity
        .bind_mut()
        .set_component(component_class.clone(), Some(existing_component.clone()));

    let new_component = Component::new_gd();
    let called = Rc::new(RefCell::new(false));
    {
        let component_class = component_class.clone();
        let existing_component = existing_component.clone();
        let new_component = new_component.clone();
        let called = called.clone();
        instance
            .bind_mut()
            .signals()
            .component_changed()
            .builder()
            .flags(ConnectFlags::ONE_SHOT)
            .connect(
                move |node_entity: Gd<NodeEntity>,
                      p_component_class: StringName,
                      p_component: Option<Gd<Component>>,
                      old_component: Option<Gd<Component>>| {
                    called.replace(true);
                    assert_eq!(node_entity, entity);
                    assert_eq!(p_component_class, component_class);
                    assert_eq!(p_component.unwrap(), new_component);
                    assert_eq!(old_component.unwrap(), existing_component);
                },
            );
    }

    instance.bind_mut().set_component_of_node(
        node.clone(),
        component_class.clone(),
        Some(new_component),
    );
    instance.bind_mut().process(0f64);

    let called = *called.borrow();
    assert!(called);

    instance
        .bind_mut()
        .remove_all_entities_and_pending_changes();
    node.free();
}

#[gditest]
fn calling_update_should_update_all_instances() {
    let mut instance = GodotCompositionWorld::get_singleton();

    let component = Component::new_gd();
    let mut nodes = Vec::new();
    let node = Node::new_alloc();
    nodes.push(node.clone());
    let component_class = StringName::from("test");
    let mut entity = instance.bind_mut().get_or_create_node_entity(node.clone());
    entity
        .bind_mut()
        .set_component(component_class.clone(), Some(component.clone()));
    instance.bind_mut().update_caches(
        HashSet::from([entity]),
        HashSet::from([component_class.clone()]),
    );

    let components: Vec<_> = extract_components(&mut instance);
    assert_eq!(components.len(), 1);

    let node = Node::new_alloc();
    nodes.push(node.clone());
    let mut entity = instance.bind_mut().get_or_create_node_entity(node.clone());
    entity
        .bind_mut()
        .set_component(component_class.clone(), Some(component.clone()));
    instance.bind_mut().update_caches(
        HashSet::from([entity.clone()]),
        HashSet::from([component_class.clone()]),
    );

    let components: Vec<_> = extract_components(&mut instance);
    assert_eq!(components.len(), 2);

    entity
        .bind_mut()
        .set_component(component_class.clone(), None);
    godot_print!(
        "{}",
        entity
            .bind()
            .has_component_of_class(component_class.clone())
    );
    instance.bind_mut().update_caches(
        HashSet::from([entity]),
        HashSet::from([component_class.clone()]),
    );

    let components: Vec<_> = extract_components(&mut instance);
    assert_eq!(components.len(), 1);

    for node in nodes {
        node.free();
    }
    instance
        .bind_mut()
        .remove_all_entities_and_pending_changes();
}

fn extract_components(instance: &mut Gd<GodotCompositionWorld>) -> Vec<Dictionary> {
    instance
        .bind_mut()
        .get_all_components()
        .iter_shared()
        .typed::<Gd<Node>, Vec<Dictionary>>()
        .flat_map(|x| x.1)
        .collect()
}

#[gditest]
fn process_calls_process_on_components() {
    let script = ResourceLoader::singleton()
        .load("uid://2jgn8yhev0bx")
        .unwrap()
        .cast::<Script>();
    let mut instance = GodotCompositionWorld::get_singleton();
    let mut second_component = Component::new_gd();
    second_component.set_script(&script.to_variant());
    let mut first_component = Component::new_gd();
    first_component.set_script(&script.to_variant());

    let node = Node::new_alloc();
    let first_component_class = StringName::from("test1");
    let second_component_class = StringName::from("test2");
    let mut entity = instance.bind_mut().get_or_create_node_entity(node.clone());
    entity
        .bind_mut()
        .set_component(first_component_class.clone(), Some(first_component.clone()));
    entity.bind_mut().set_component(
        second_component_class.clone(),
        Some(second_component.clone()),
    );
    instance.bind_mut().update_caches(
        HashSet::from([entity]),
        HashSet::from([first_component_class.clone()]),
    );
    instance.bind_mut().process(0f64);
    assert_eq!(first_component.get("process_calls").to::<i64>(), 1);
    assert_eq!(second_component.get("process_calls").to::<i64>(), 1);

    node.free();
    instance
        .bind_mut()
        .remove_all_entities_and_pending_changes();
}

#[gditest]
fn physics_process_calls_physics_process_on_components() {
    let script = ResourceLoader::singleton()
        .load("uid://2jgn8yhev0bx")
        .unwrap()
        .cast::<Script>();
    let mut instance = GodotCompositionWorld::get_singleton();
    let mut second_component = Component::new_gd();
    second_component.set_script(&script.to_variant());
    let mut first_component = Component::new_gd();
    first_component.set_script(&script.to_variant());

    let node = Node::new_alloc();
    let first_component_class = StringName::from("test1");
    let second_component_class = StringName::from("test2");
    let mut entity = instance.bind_mut().get_or_create_node_entity(node.clone());
    entity
        .bind_mut()
        .set_component(first_component_class.clone(), Some(first_component.clone()));
    entity.bind_mut().set_component(
        second_component_class.clone(),
        Some(second_component.clone()),
    );
    instance.bind_mut().update_caches(
        HashSet::from([entity]),
        HashSet::from([first_component_class.clone()]),
    );
    instance.bind_mut().physics_process(0f64);
    assert_eq!(first_component.get("physics_process_calls").to::<i64>(), 1);
    assert_eq!(second_component.get("physics_process_calls").to::<i64>(), 1);

    node.free();
    instance
        .bind_mut()
        .remove_all_entities_and_pending_changes();
}

#[gditest]
fn do_for_all_components_calls_the_callable() {
    let mut instance = GodotCompositionWorld::get_singleton();
    let component = Component::new_gd();

    let node = Node::new_alloc();
    let component_class = StringName::from("test");
    let mut entity = instance.bind_mut().get_or_create_node_entity(node.clone());
    entity
        .bind_mut()
        .set_component(component_class.clone(), Some(component.clone()));
    instance.bind_mut().update_caches(
        HashSet::from([entity]),
        HashSet::from([component_class.clone()]),
    );

    let test_instance = TestObject::new_gd();
    let callable = Callable::from_object_method(
        &test_instance,
        &StringName::from("check_do_for_all_components_calls_the_callable"),
    );
    let callable = callable.bind(&[component_class.to_variant(), component.to_variant()]);

    instance.bind_mut().do_for_all_components(callable);
    let was_called = test_instance
        .get_meta("check_do_for_all_components_calls_the_callable")
        .booleanize();
    assert!(was_called);

    node.free();
    instance
        .bind_mut()
        .remove_all_entities_and_pending_changes();
}

#[gditest]
fn do_for_all_components_calls_the_callable_for_each_component() {
    let mut instance = GodotCompositionWorld::get_singleton();

    let classes = [
        StringName::from("test"),
        StringName::from("test2"),
        StringName::from("test3"),
    ];
    static ENTITY_COUNT: i64 = 3;

    let mut entities = Vec::with_capacity(ENTITY_COUNT as usize);
    let mut nodes = Vec::with_capacity(ENTITY_COUNT as usize);

    for _ in 0..ENTITY_COUNT {
        let node = Node::new_alloc();
        let mut entity = instance.bind_mut().get_or_create_node_entity(node.clone());
        let component = Component::new_gd();
        for class in classes.iter() {
            entity
                .bind_mut()
                .set_component(class.clone(), Some(component.clone()));
        }
        entities.push(entity);
        nodes.push(node);
    }

    instance.bind_mut().update_caches(
        entities.into_iter().collect(),
        HashSet::from(classes.clone()),
    );

    let test_instance = TestObject::new_gd();
    let callable = Callable::from_object_method(
        &test_instance,
        &StringName::from("check_do_for_all_components_calls_the_callable_for_each_component"),
    );

    instance.bind_mut().do_for_all_components(callable);
    let call_count = test_instance
        .get_meta("check_do_for_all_components_calls_the_callable_for_each_component_calls")
        .to::<i64>();
    let expected_calls = ENTITY_COUNT * classes.len() as i64;
    assert_eq!(call_count, expected_calls);
    for node in nodes {
        node.free();
    }
    instance
        .bind_mut()
        .remove_all_entities_and_pending_changes();
}

#[gditest]
fn do_for_all_components_of_class_calls_the_callable() {
    let mut instance = GodotCompositionWorld::get_singleton();
    let component = Component::new_gd();

    let node = Node::new_alloc();
    let class_to_use = StringName::from("test999");
    let other_class = StringName::from("test");
    let mut entity = instance.bind_mut().get_or_create_node_entity(node.clone());
    entity
        .bind_mut()
        .set_component(class_to_use.clone(), Some(component.clone()));
    entity
        .bind_mut()
        .set_component(other_class.clone(), Some(component.clone()));
    instance.bind_mut().update_caches(
        HashSet::from([entity]),
        HashSet::from([class_to_use.clone(), other_class.clone()]),
    );

    let test_instance = TestObject::new_gd();
    let callable = Callable::from_object_method(
        &test_instance,
        &StringName::from("check_do_for_all_components_of_class_calls_the_callable"),
    );
    let callable = callable.bind(&[component.to_variant()]);

    instance
        .bind_mut()
        .do_for_all_components_of_class(class_to_use, callable);
    let was_called = test_instance
        .get_meta("check_do_for_all_components_of_class_calls_the_callable")
        .booleanize();
    assert!(was_called);

    node.free();
    instance
        .bind_mut()
        .remove_all_entities_and_pending_changes();
}

#[gditest]
fn do_for_all_components_of_class_calls_the_callable_for_each_component_of_the_given_class() {
    let mut instance = GodotCompositionWorld::get_singleton();

    let class_to_use = StringName::from("test999");

    let classes = [
        StringName::from("test"),
        StringName::from("test2"),
        StringName::from("test3"),
        class_to_use.clone(),
    ];
    static ENTITY_COUNT: i64 = 3;

    let mut entities = Vec::with_capacity(ENTITY_COUNT as usize);
    let mut nodes = Vec::with_capacity(ENTITY_COUNT as usize);

    for _ in 0..ENTITY_COUNT {
        let node = Node::new_alloc();
        let mut entity = instance.bind_mut().get_or_create_node_entity(node.clone());
        let component = Component::new_gd();
        for class in classes.iter() {
            entity
                .bind_mut()
                .set_component(class.clone(), Some(component.clone()));
        }
        entities.push(entity);
        nodes.push(node);
    }

    instance.bind_mut().update_caches(
        entities.clone().into_iter().collect(),
        HashSet::from(classes.clone()),
    );

    let test_instance = TestObject::new_gd();
    let callable = Callable::from_object_method(
        &test_instance,
        &StringName::from(
            "check_do_for_all_components_of_class_calls_the_callable_for_each_component_of_the_given_class",
        ),
    );

    instance
        .bind_mut()
        .do_for_all_components_of_class(class_to_use, callable);
    let recorded_calls = test_instance
        .get_meta(
            "check_do_for_all_components_of_class_calls_the_callable_for_each_component_calls",
        )
        .to::<i64>();
    let expected_calls = entities.len() as i64;
    assert_eq!(recorded_calls, expected_calls);
    for node in nodes {
        node.free();
    }
    instance
        .bind_mut()
        .remove_all_entities_and_pending_changes();
}
