use bevy::{camera::ScalingMode, prelude::*};
use bevy_firefly::data::{FireflyConfig, NormalMode};

use crate::{constants::VIEWPORT_WIDTH, game::world::camera::MainCamera};

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic({
            let mut ortho = OrthographicProjection::default_2d();
            ortho.scaling_mode = ScalingMode::FixedHorizontal {
                viewport_width: VIEWPORT_WIDTH,
            };
            ortho
        }),
        MainCamera,
        FireflyConfig {
            normal_mode: NormalMode::TopDownY,
            enable_32bit_stencils: true,
            ..default()
        },
        Msaa::Off,
    ));
}
