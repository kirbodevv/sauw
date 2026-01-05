use bevy::ecs::schedule::SystemSet;

#[derive(SystemSet, Debug, Clone, Hash, Eq, PartialEq)]

pub enum StartupSet {
    Assets,
    World,
    Actors,
}
