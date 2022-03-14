use bevy::prelude::{App, Plugin};
use gdnative::prelude::*;
use gdrust::ecs::app::init_ecs;
use gdrust::ecs::engine_controller::ECSController;

struct DefaultPlugin;

impl Plugin for DefaultPlugin {
    fn build(&self, _app: &mut App) {}
}

fn init(handle: InitHandle) {
    handle.add_class::<ECSController>();

    init_ecs(DefaultPlugin);
}
godot_init!(init);
