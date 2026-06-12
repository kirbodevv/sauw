use bevy::prelude::*;

pub mod health;
pub mod hotbar;
pub mod joystick;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_root_node);

        app.add_plugins((health::HealthPlugin, hotbar::HotbarPlugin));

        if joystick::USE_JOYSTICK {
            app.add_plugins(joystick::JoystickPlugin);
        }
    }
}

#[derive(Component)]
pub struct HudRoot;

#[derive(Component)]
pub struct HudTop;

#[derive(Component)]
pub struct HudBottom;

pub const DEFAULT_HUD_PADDING: f32 = 16.0;

pub fn spawn_root_node(mut commands: Commands) {
    commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::SpaceBetween,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                padding: Val::Px(DEFAULT_HUD_PADDING).all(),
                ..default()
            },
            HudRoot,
        ))
        .with_children(|parent| {
            parent.spawn((Node::default(), HudTop));
            parent.spawn((Node::default(), HudBottom));
        });
}
