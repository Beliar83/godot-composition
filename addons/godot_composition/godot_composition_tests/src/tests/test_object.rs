use godot::builtin::{StringName, Variant};
use godot::classes::RefCounted;
use godot::obj::{Base, Gd, WithBaseField};
use godot::prelude::{godot_api, GodotClass};
use godot_composition_core::component::Component;

#[derive(GodotClass)]
#[class(init)]
pub(crate) struct TestObject {
    base: Base<RefCounted>,
}

#[godot_api]
impl TestObject {
    #[func]
    fn check_do_for_all_components_calls_the_callable(
        &mut self,
        expected_component_class: StringName,
        expected_component: Gd<Component>,
        component_class: StringName,
        component: Gd<Component>,
    ) {
        self.base_mut().set_meta(
            "check_do_for_all_components_calls_the_callable",
            &Variant::from(true),
        );
        assert_eq!(component_class, expected_component_class);
        assert_eq!(component, expected_component);
    }

    #[func]
    fn check_do_for_all_components_calls_the_callable_for_each_component(
        &mut self,
        _component_class: StringName,
        _component: Gd<Component>,
    ) {
        let calls = self
            .base()
            .get_meta_ex("check_do_for_all_components_calls_the_callable_for_each_component_calls")
            .default(&Variant::from(0))
            .done()
            .to::<i64>();
        self.base_mut().set_meta(
            "check_do_for_all_components_calls_the_callable_for_each_component_calls",
            &Variant::from(calls + 1),
        );
    }
}
