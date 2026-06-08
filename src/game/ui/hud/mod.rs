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
        app.add_systems(Startup, spawn_root_node);

        app.add_plugins(HealthPlugin);

        if USE_JOYSTICK {
            app.add_plugins(JoystickPlugin);
        }
    }
}

#[derive(Component)]
pub struct HudRoot;

#[derive(Component)]
pub struct HudTop;

#[derive(Component)]
pub struct HudBottom;

pub fn spawn_root_node(mut commands: Commands) {
    commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceBetween,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                padding: UiRect {
                    top: Val::Px(16.0),
                    left: Val::Px(16.0),
                    bottom: Val::Px(24.0),
                    right: Val::Px(16.0),
                },
                ..default()
            },
            HudRoot,
        ))
        .with_children(|parent| {
            parent.spawn((Node::default(), HudTop));
            parent.spawn((Node::default(), HudBottom));
        });
}
