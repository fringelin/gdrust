use bevy::{ecs::system::Resource, prelude::World};
use gdnative::prelude::VariantArray;
use gdnative::{
    api::{InputEvent, Node},
    prelude::{Ref, TRef},
};

use super::resources::Delta;

pub struct SpawnGame {
    pub node: Ref<Node>,
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

pub fn spawn_game(world: &mut World, node: Ref<Node>) {
    world
        .get_resource_mut::<bevy::app::Events<SpawnGame>>()
        .expect("No world spawn game event, did you forget to add Spawn Game into your events?")
        .send(SpawnGame { node });
}

pub fn spawn_node(world: &mut World, node: Ref<Node>, name: String) {
    world
        .get_resource_mut::<bevy::app::Events<SpawnNode>>()
        .expect("No world spawn node event, did you forget to add Spawn node into your events?")
        .send(SpawnNode { node, name });
}

pub fn spawn_signal(world: &mut World, name: String, vars: VariantArray) {
    world
        .get_resource_mut::<bevy::app::Events<SpawnSignal>>()
        .expect("No world spawn signal event, did you forget to add Spawn signal into your events?")
        .send(SpawnSignal { name, vars });
}
