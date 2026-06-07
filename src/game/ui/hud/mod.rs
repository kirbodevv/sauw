use crate::game::ui::hud::{
    health::HealthPlugin,
    joystick::{JoystickPlugin, USE_JOYSTICK},
};

use bevy::prelude::*;

pub mod health;
pub mod joystick;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(HealthPlugin);

        if USE_JOYSTICK {
            app.add_plugins(JoystickPlugin);
        }
    }
}
