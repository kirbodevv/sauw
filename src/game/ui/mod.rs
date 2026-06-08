use bevy::prelude::*;

pub mod hud;
pub mod scale;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((hud::HudPlugin, scale::ScalePlugin))
            .insert_resource(UiDebugOptions {
                enabled: true,
                ..default()
            });
    }
}
