use bevy::prelude::*;

use crate::game::ui::hud::{DEFAULT_HUD_PADDING, HudRoot};

#[derive(Message, Default)]
pub struct ChangeSafeZone {
    pub top: f32,
    pub bottom: f32,
    pub left: f32,
    pub right: f32,
}

pub fn apply_safe_zone(
    mut safe_zone_reader: MessageReader<ChangeSafeZone>,
    mut hud_root: Single<&mut Node, With<HudRoot>>,
    window: Single<&mut Window>,
) {
    for change in safe_zone_reader.read() {
        let scale = window.physical_width() as f32 / window.width() as f32;

        let (top, bottom, left, right) = (
            Val::Px(DEFAULT_HUD_PADDING + change.top / scale),
            Val::Px(DEFAULT_HUD_PADDING + change.bottom / scale),
            Val::Px(DEFAULT_HUD_PADDING + change.left / scale),
            Val::Px(DEFAULT_HUD_PADDING + change.right / scale),
        );

        hud_root.padding = UiRect::new(left, right, top, bottom);
    }
}

pub struct SafeZonePlugin;

impl Plugin for SafeZonePlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<ChangeSafeZone>()
            .add_systems(Update, apply_safe_zone);
    }
}
