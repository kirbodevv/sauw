use crate::game::{GameState, assets::resource::ImageAssets, ui::hud::HudBottom};
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

fn spawn_joystick(
    mut cmd: Commands,
    assets: Res<ImageAssets>,
    hud_bottom: Single<Entity, With<HudBottom>>,
) {
    let id = JoystickControllerID::Main;
    let knob_img = assets.ui_joystick_handle.clone();
    let base_img = assets.ui_joystick_base.clone();
    let knob_size = Vec2::new(75., 75.);
    let base_size = Vec2::new(150., 150.);

    cmd.entity(*hud_bottom).with_children(|parent| {
        let mut spawn = parent.spawn(
            VirtualJoystickBundle::new(
                VirtualJoystickNode::<JoystickControllerID>::default()
                    .with_id(id)
                    .with_behavior(JoystickFixed)
                    .with_action(NoAction),
            )
            .set_style(Node {
                width: Val::Px(base_size.x),
                height: Val::Px(base_size.y),
                ..default()
            }),
        );

        spawn.with_children(|parent| {
            parent.spawn((
                VirtualJoystickInteractionArea,
                Node {
                    width: Val::Px(base_size.x),
                    height: Val::Px(base_size.y),
                    ..default()
                },
            ));

            parent.spawn((
                VirtualJoystickUIKnob,
                ImageNode {
                    color: Color::WHITE.with_alpha(1.0),
                    image: knob_img,
                    ..default()
                },
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Px(knob_size.x),
                    height: Val::Px(knob_size.y),
                    ..default()
                },
                ZIndex(1),
            ));

            parent.spawn((
                VirtualJoystickUIBackground,
                ImageNode {
                    color: Color::WHITE.with_alpha(1.0),
                    image: base_img,
                    ..default()
                },
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Px(base_size.x),
                    height: Val::Px(base_size.y),
                    ..default()
                },
                ZIndex(0),
            ));
        });
    });
}
