use bevy::prelude::*;
use std::time::Duration;

#[derive(Resource)]
pub struct TargetScale {
    start_scale: f32,
    target_scale: f32,
    target_time: Timer,
}

impl TargetScale {
    fn set_scale(&mut self, scale: f32) {
        self.start_scale = self.current_scale();
        self.target_scale = scale;
        self.target_time.reset();
    }

    fn current_scale(&self) -> f32 {
        let completion = self.target_time.fraction();
        let t = ease_in_expo(completion);
        self.start_scale.lerp(self.target_scale, t)
    }

    fn tick(&mut self, delta: Duration) -> &Self {
        self.target_time.tick(delta);
        self
    }

    fn already_completed(&self) -> bool {
        self.target_time.is_finished() && !self.target_time.just_finished()
    }
}

fn apply_scaling(
    time: Res<Time>,
    mut target_scale: ResMut<TargetScale>,
    mut ui_scale: ResMut<UiScale>,
) {
    if target_scale.tick(time.delta()).already_completed() {
        return;
    }

    ui_scale.0 = target_scale.current_scale();
}

fn ease_in_expo(x: f32) -> f32 {
    if x == 0. {
        0.
    } else {
        ops::powf(2.0f32, 5. * x - 5.)
    }
}

fn change_scaling(mut ui_scale: ResMut<TargetScale>, window: Single<&mut Window>) {
    let base_width = 1280.0;
    let scale = window.width() / base_width;

    ui_scale.set_scale(scale);
}

const SCALE_TIME: u64 = 400;

pub struct ScalePlugin;

impl Plugin for ScalePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TargetScale {
            start_scale: 1.0,
            target_scale: 1.0,
            target_time: Timer::new(Duration::from_millis(SCALE_TIME), TimerMode::Once),
        })
        .add_systems(Update, (change_scaling, apply_scaling).chain());
    }
}
