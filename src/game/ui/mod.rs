use bevy::prelude::*;

pub mod hud;
pub mod safe_zone;
pub mod scale;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            hud::HudPlugin,
            scale::ScalePlugin,
            safe_zone::SafeZonePlugin,
        ))
        .insert_resource(UiDebugOptions {
            enabled: false,
            ..default()
        });
    }
}
