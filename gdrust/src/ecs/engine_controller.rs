use crate::ecs::app::{with_schedule, with_world};
use crate::ecs::engine_sync::{
    events::update_delta_resource,
    resources::{IdleDelta, PhysicsDelta},
};
use bevy::prelude::Stage;
use gdnative::prelude::*;

/// This ECSController acts as the middle man between Godot and Bevy, it's a singleton or "AutoLoad" script that
/// creates the entire Bevy ECS. Also, "Project Settings > Rendering > Threading" to turn on multi threading, which will work
/// nicely with the multi threading that Bevy offers, ie, if you want to render multiple things using ecs, then Godot will play nicely.
#[derive(NativeClass)]
#[inherit(Node)]
#[register_with(Self::register_builder)]
pub struct ECSController {
    name: String,
}

#[methods]
impl ECSController {
    fn register_builder(_builder: &ClassBuilder<Self>) {}

    fn new(_owner: &Node) -> Self {
        godot_print!("ECSController is created!");
        ECSController {
            name: "".to_string(),
        }
    }

    #[export]
    fn _ready(&mut self, _owner: &Node) {
        self.name = "ECSController".to_string();
    }

    #[export]
    fn _process(&mut self, _owner: &Node, delta: f32) {
        with_world(|w| {
            w.clear_trackers();
            update_delta_resource::<IdleDelta>(w, delta);
            with_schedule(|s| s.run(w));
        });
    }

    /// I created two Delta resources, one for the physics loop, and one for the Idle loop
    #[export]
    fn _physics_process(&mut self, _owner: &Node, delta: f32) {
        with_world(|w| {
            update_delta_resource::<PhysicsDelta>(w, delta);
        });
    }
}
