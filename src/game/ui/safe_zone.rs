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
    ui_scale: Res<UiScale>,
) {
    for change in safe_zone_reader.read() {
        let (top, bottom, left, right) = (
            Val::Px(DEFAULT_HUD_PADDING + change.top / ui_scale.0),
            Val::Px(DEFAULT_HUD_PADDING + change.bottom / ui_scale.0),
            Val::Px(DEFAULT_HUD_PADDING + change.left / ui_scale.0),
            Val::Px(DEFAULT_HUD_PADDING + change.right / ui_scale.0),
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
