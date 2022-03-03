use bevy::app::PluginGroupBuilder;
use bevy::prelude::{Component, Plugin, PluginGroup};
use gdrust::gd_ecs_controller;
use gdrust::gdrust_macros::gdcomponent;
use gdrust::unsafe_functions::{NodeExt, RefExt};

#[gdcomponent(extends = Node2D)]
struct ComponentA {
    #[node]
    node: Ref<Node2D>,
    #[property("x")]
    x: i32,
    y: f32,
    #[value(0.0)]
    z: f32,
}

#[gdcomponent(extends = Node2D)]
struct ComponentB {
    #[node]
    node: Ref<Node2D>,
    #[component("ComponentA")]
    component_a: ComponentA,
}

struct DefaultPlugin;

impl Plugin for DefaultPlugin {
    fn build(&self, app: &mut App) {}
}

struct GamePlugin;

impl PluginGroup for GamePlugin {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group.add(DefaultPlugin);
    }
}

gd_ecs_controller!(GamePlugin);
