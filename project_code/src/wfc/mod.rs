pub mod components;
pub mod systems;

use bevy::prelude::*;
use components::*;
use systems::*;
use crate::components::{Background, GameworldState};
use crate::level::components::*;
use crate::level::systems::*;
use crate::handle_door_translation;

pub struct WFCPlugin;

impl Plugin for WFCPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WFCSettings>()
            .add_systems(Startup, (
                init_wfc_resources,
                load_dungeon,
            ).chain())
            .add_systems(OnEnter(GameworldState::Dungeon), 
               (create_patterns_from_template, generate_dungeon, despawn_with::<Background>).chain())
            .add_systems(OnExit(GameworldState::Dungeon),(despawn_with::<Tile>, cleanup_debug_markers))
            .add_systems(OnExit(GameworldState::Dungeon), cleanup_debug_markers);

    }
}
