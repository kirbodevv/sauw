use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct Health {
    pub health: u8,
    pub old_health: u8,
    pub max_health: u8,
}

impl Health {
    pub fn new(max_health: u8) -> Self {
        Self {
            health: max_health,
            old_health: max_health,
            max_health,
        }
    }

    pub fn damage(&mut self, amount: u8) {
        self.health = self.health.saturating_sub(amount);
    }

    pub fn heal(&mut self, amount: u8) {
        self.health = self.health.saturating_add(amount);
        if self.health > self.max_health {
            self.health = self.max_health;
        }
    }
}
