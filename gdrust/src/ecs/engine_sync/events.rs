use bevy::prelude::Entity;
use bevy::{ecs::system::Resource, prelude::World};
use gdnative::prelude::VariantArray;
use gdnative::{
    api::{InputEvent, Node},
    prelude::{Ref, TRef},
};

use super::resources::Delta;

pub struct SpawnGame {
    pub entity: Entity,
}

pub struct SpawnNode {
    pub node: Ref<Node>,
    pub name: String,
}

pub struct DespawnPlayingGame;

pub struct UserInput {
    pub input: Ref<InputEvent>,
}

pub struct SpawnSignal {
    pub name: String,
    pub vars: VariantArray,
}

pub fn update_delta_resource<T: Resource + Delta>(world: &mut World, delta: f32) {
    world
        .get_resource_mut::<T>()
        .expect("Umm, there should be a godot time here!")
        .set_delta(delta);
}

pub fn user_input(world: &mut World, event: TRef<InputEvent>) {
    world
        .get_resource_mut::<bevy::app::Events<UserInput>>()
        .expect("should be a user input event")
        .send(UserInput {
            input: event.claim(),
        });
}

pub fn spawn_game(world: &mut World, entity: Entity) {
    world
        .get_resource_mut::<bevy::app::Events<SpawnGame>>()
        .expect("No world spawn game event, did you forget to add Spawn Game into your events?")
        .send(SpawnGame { entity });
}

pub fn spawn_node<T: Resource>(world: &mut World, spawn: T) {
    world
        .get_resource_mut::<bevy::app::Events<T>>()
        .expect("No world spawn node event, did you forget to add Spawn node into your events?")
        .send(spawn);
}
