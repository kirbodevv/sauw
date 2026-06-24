use bevy::prelude::*;

pub mod components;
pub mod drop_item;
pub mod pickup;
pub mod spawn;

use crate::game::GameState;

pub use components::*;

pub struct DropPlugin;

impl Plugin for DropPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<SpawnDrop>()
            .add_message::<DropItem>()
            .add_systems(
                Update,
                (
                    spawn::spawn_drop,
                    pickup::collect_drops,
                    pickup::move_drops,
                    pickup::unlock_drops,
                    drop_item::drop_item,
                )
                    .run_if(in_state(GameState::Gaming)),
            );
    }
}
