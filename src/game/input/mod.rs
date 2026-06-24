use crate::game::ui::hud::joystick::USE_JOYSTICK;
use bevy::{input::InputSystems, prelude::*};

mod debug;
mod inventory;
mod player;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            (
                player::keyboard_input,
                debug::keyboard_input,
                inventory::keyboard_input,
            )
                .after(InputSystems),
        );

        if USE_JOYSTICK {
            app.add_systems(
                PreUpdate,
                player::joystick_input.after(player::keyboard_input),
            );
        }
    }
}
