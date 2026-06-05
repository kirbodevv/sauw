use crate::game::ui::health::HealthPlugin;
use crate::game::ui::joystick::JoystickPlugin;

use bevy::prelude::*;

pub mod health;
pub mod joystick;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((HealthPlugin, JoystickPlugin));
    }
}
