use bevy::ecs::component::Component;

#[derive(Component)]
pub struct BlockEntity;

#[derive(Component)]
pub struct BlockPos {
    pub x: u8,
    pub y: u8,
    pub layer: u8,
}
