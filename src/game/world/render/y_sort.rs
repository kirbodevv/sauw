use bevy::prelude::*;

#[derive(Component)]
pub struct YSort {
    pub z: f32,
}

const Y_SORT_BASE: f32 = 10.0;

pub fn y_sort_z(layer: f32, world_y: f32) -> f32 {
    Y_SORT_BASE + layer - 1.0 / (1.0 + 2.0_f32.powf(-0.01 * world_y))
}

pub fn apply_y_sort(mut query: Query<(&mut Transform, &GlobalTransform, &YSort)>) {
    for (mut transform, global_transform, y_sort) in &mut query {
        transform.translation.z = y_sort_z(y_sort.z, global_transform.translation().y);
    }
}
