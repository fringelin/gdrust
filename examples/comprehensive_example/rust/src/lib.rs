use bevy::app::PluginGroupBuilder;
use bevy::prelude::{Plugin, PluginGroup};
use gdrust::gd_ecs_controller;

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
