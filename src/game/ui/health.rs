use bevy::prelude::*;

use crate::game::{
    GameState, ImageAssets,
    player::{Player, health::Health},
};

#[derive(Component)]
struct Heart {
    index: usize,
}

#[derive(Component)]
struct HeartsContainer;

#[derive(Message)]
pub struct SpawnPlayerHearts {
    pub count: usize,
}

#[derive(Message)]
struct HealthChanged {
    entity: Entity,
    #[allow(dead_code)]
    health_change: u8,
}

fn check_health_changed(
    mut q_healths: Query<(Entity, &mut Health)>,
    mut ev_health_changed: MessageWriter<HealthChanged>,
) {
    for (entity, mut health) in &mut q_healths {
        if health.health != health.old_health {
            let health_change = health.health.abs_diff(health.old_health);
            ev_health_changed.write(HealthChanged {
                entity,
                health_change,
            });
            health.old_health = health.health;
        }
    }
}

fn spawn_heart(commands: &mut Commands, assets: &Res<ImageAssets>, index: usize) -> Entity {
    commands
        .spawn((
            Heart { index },
            ImageNode {
                image: assets.ui_heart_full.clone(),
                ..default()
            },
            Node {
                width: Val::Percent(2.0),
                margin: UiRect {
                    right: Val::Percent(0.25),
                    ..default()
                },
                ..default()
            },
        ))
        .id()
}

fn spawn_hearts(
    mut commands: Commands,
    assets: Res<ImageAssets>,
    mut ev_spawn_player_hearts: MessageReader<SpawnPlayerHearts>,
) {
    for ev in ev_spawn_player_hearts.read() {
        let root = commands
            .spawn((
                HeartsContainer,
                Node {
                    width: Val::Percent(90.0),
                    top: Val::Percent(3.5),
                    left: Val::Percent(2.5),
                    ..default()
                },
            ))
            .id();

        for i in 0..ev.count {
            let heart_entity = spawn_heart(&mut commands, &assets, i);
            commands.entity(root).add_children(&[heart_entity]);
        }
    }
}

fn despawn_hearts(
    mut commands: Commands,
    q_hearts_containers: Query<Entity, With<HeartsContainer>>,
) {
    for entity in &q_hearts_containers {
        commands.entity(entity).despawn();
    }
}

fn update_player_hearts(
    assets: Res<ImageAssets>,
    q_player: Query<&Health, With<Player>>,
    mut q_hearts: Query<(&mut ImageNode, &Heart)>,
    mut ev_health_changed: MessageReader<HealthChanged>,
) {
    for ev in ev_health_changed.read() {
        let player_health = match q_player.get(ev.entity) {
            Ok(p) => p,
            Err(_) => continue,
        };

        let threshold_index = player_health.health as usize;
        for (mut ui_image, heart) in &mut q_hearts {
            if heart.index >= threshold_index {
                ui_image.image = assets.ui_heart_empty.clone();
            } else {
                ui_image.image = assets.ui_heart_full.clone();
            }
        }
    }
}

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (check_health_changed, spawn_hearts, update_player_hearts)
                .run_if(in_state(GameState::Gaming)),
        )
        .add_message::<SpawnPlayerHearts>()
        .add_message::<HealthChanged>()
        .add_systems(OnEnter(GameState::GameOver), despawn_hearts);
    }
}
