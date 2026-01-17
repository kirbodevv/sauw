use std::f32::consts::TAU;

use bevy::prelude::*;
use bevy_firefly::data::FireflyConfig;

use crate::game::world::camera::MainCamera;

const DAY_CYCLE_SECONDS: f32 = 10.0 * 60.0;
const DAY_TIME_SPEED: f32 = 1.0 / DAY_CYCLE_SECONDS;

#[derive(Resource)]
pub struct DayTime {
    pub time: f32,
}

fn update_day_time(time: Res<Time>, mut day: ResMut<DayTime>) {
    day.time = (day.time + time.delta_secs() * DAY_TIME_SPEED) % 1.0;
}

fn lerp_color(a: Color, b: Color, t: f32) -> Color {
    let t = t.clamp(0.0, 1.0);

    let [ar, ag, ab, aa] = a.to_srgba().to_f32_array();
    let [br, bg, bb, ba] = b.to_srgba().to_f32_array();

    Color::srgb(ar + (br - ar) * t, ag + (bg - ag) * t, ab + (bb - ab) * t)
        .with_alpha(aa + (ba - aa) * t)
}

fn update_ambient_light(
    day: Res<DayTime>,
    mut config: Single<&mut FireflyConfig, With<MainCamera>>,
) {
    let t = day.time;

    let daylight = (t * TAU).sin().max(0.0);

    config.ambient_brightness = daylight * 0.7 + 0.25;

    let night = Color::srgb(0.08, 0.2, 0.3);
    let sunrise = Color::srgb(1.0, 0.6, 0.3);
    let day = Color::srgb(1.0, 0.98, 0.95);

    let color = if daylight < 0.3 {
        lerp_color(night, sunrise, daylight / 0.3)
    } else {
        lerp_color(sunrise, day, (daylight - 0.3) / 0.7)
    };

    config.ambient_color = color;
}

pub struct TimePlugin;

impl Plugin for TimePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DayTime { time: 0.25 })
            .add_systems(Update, (update_day_time, update_ambient_light).chain());
    }
}
