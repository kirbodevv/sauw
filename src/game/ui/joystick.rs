use crate::game::{GameState, ImageAssets};
use bevy::prelude::*;
use virtual_joystick::*;

#[cfg(any(target_os = "android", target_os = "ios"))]
pub const USE_JOYSTICK: bool = true;

#[cfg(not(any(target_os = "android", target_os = "ios")))]
pub const USE_JOYSTICK: bool = false;

#[derive(Default, Debug, Reflect, Hash, Clone, PartialEq, Eq)]
pub enum JoystickControllerID {
    #[default]
    Main,
}

pub struct JoystickPlugin;

impl Plugin for JoystickPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(VirtualJoystickPlugin::<JoystickControllerID>::default())
            .add_systems(OnEnter(GameState::Bootstrap), spawn_joystick);
    }
}

fn spawn_joystick(mut commands: Commands, assets: Res<ImageAssets>) {
    create_joystick(
        &mut commands,
        JoystickControllerID::Main,
        assets.ui_joystick_handle.clone(),
        assets.ui_joystick_base.clone(),
        None,
        None,
        None,
        Vec2::new(75., 75.),
        Vec2::new(150., 150.),
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            position_type: PositionType::Absolute,
            left: Val::Percent(10.),
            top: Val::Percent(50.),
            ..default()
        },
        JoystickFixed,
        NoAction,
    );
}
