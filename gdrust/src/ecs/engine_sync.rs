use crate::ecs::engine_sync::events::SpawnSignal;
use bevy::prelude::{CoreStage, ParallelSystemDescriptorCoercion, Plugin, SystemStage};

use self::{
    events::{DespawnPlayingGame, SpawnGame, SpawnNode, UserInput},
    resources::{GameOver, IdleDelta, PhysicsDelta},
    stages::SyncStages,
    systems::{despawn_playing_game, end_game, spawn_game},
};

pub mod components;
pub mod events;
pub mod resources;
pub mod stages;
pub mod systems;

pub struct EngineSyncPlugin;

impl Plugin for EngineSyncPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<UserInput>()
            .add_event::<SpawnGame>()
            .add_event::<SpawnNode>()
            .add_event::<DespawnPlayingGame>()
            .add_event::<SpawnSignal>()
            .add_system_to_stage(CoreStage::Last, spawn_game)
            .add_stage_after(
                CoreStage::Last,
                SyncStages::UpdateBevy,
                SystemStage::parallel(),
            )
            .add_stage_after(
                SyncStages::UpdateBevy,
                SyncStages::UpdateBevyPhysics,
                SystemStage::single_threaded(),
            )
            .add_system_to_stage(SyncStages::UpdateBevy, end_game.label("end_game"))
            .add_system_to_stage(
                SyncStages::UpdateBevy,
                despawn_playing_game.after("end_game"),
            )
            .init_resource::<IdleDelta>()
            .init_resource::<PhysicsDelta>()
            .init_resource::<Option<GameOver>>();
    }
}
