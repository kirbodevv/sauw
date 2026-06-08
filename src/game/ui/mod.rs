use crate::game::ui::hud::HudPlugin;

use bevy::prelude::*;

pub mod hud;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(HudPlugin).insert_resource(UiDebugOptions {
            enabled: true,
            ..default()
        });
    }
}
