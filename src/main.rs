use bevy::{camera::ScalingMode, prelude::*};
const TILE_SIZE: f32 = 32.0;
const WORLD_WIDTH: f32 = 16.0;
const VIEWPORT_WIDTH: f32 = WORLD_WIDTH * TILE_SIZE;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .insert_resource(ClearColor(Color::linear_rgb(0.0, 0.0, 0.0)))
        .add_systems(Startup, setup)
        .add_systems(Update, (player_movement, camera_follow).chain())
        .run();
}

#[derive(Component)]
struct Player;
#[derive(Component)]
struct MainCamera;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
    ));
    commands.spawn((
        Sprite {
            color: Color::linear_rgb(0.0, 0.0, 0.0),
            custom_size: Some(Vec2::new(16.0, 32.0)),
            ..default()
        },
        Transform::from_xyz(300.0, 20.0, 50.0),
        Player,
    ));

    let tile_texture: Handle<Image> = asset_server.load("blocks/grass.png");
    for y in 0..9 {
        for x in 0..16 {
            commands.spawn((
                Sprite {
                    image: tile_texture.clone(),
                    custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                    ..default()
                },
                Transform::from_xyz(
                    x as f32 * TILE_SIZE + TILE_SIZE / 2.0,
                    y as f32 * TILE_SIZE + TILE_SIZE / 2.0,
                    0.0,
                ),
            ));
        }
    }
}

fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let mut dir = Vec2::ZERO;
    if keyboard_input.pressed(KeyCode::KeyW) {
        dir.y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyS) {
        dir.y -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyA) {
        dir.x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        dir.x += 1.0;
    }
    for mut transform in &mut query {
        transform.translation += (dir.normalize_or_zero() * 200.0 * time.delta_secs()).extend(0.0);
    }
}

fn camera_follow(
    mut camera: Single<&mut Transform, (With<MainCamera>, Without<Player>)>,
    player: Single<&Transform, With<Player>>,
) {
    camera.translation.x = player.translation.x;
    camera.translation.y = player.translation.y;
    camera.translation.z = 100.0;
}
