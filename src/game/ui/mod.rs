use crate::game::ui::health::HealthPlugin;
use crate::game::ui::joystick::{JoystickPlugin, USE_JOYSTICK};

use bevy::prelude::*;

pub mod health;
pub mod joystick;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(HealthPlugin);

        if USE_JOYSTICK {
            app.add_plugins(JoystickPlugin);
        }
    }
}
