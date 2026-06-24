use bevy::{dev_tools::fps_overlay::FpsOverlayConfig, prelude::*};

pub fn keyboard_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut fps_config: ResMut<FpsOverlayConfig>,
    mut ui_config: ResMut<UiDebugOptions>,
) {
    if keyboard.just_pressed(KeyCode::F3) {
        fps_config.enabled = !fps_config.enabled;
        fps_config.frame_time_graph_config.enabled = !fps_config.frame_time_graph_config.enabled;
    }

    if keyboard.just_pressed(KeyCode::F4) {
        ui_config.enabled = !ui_config.enabled;
    }
}
