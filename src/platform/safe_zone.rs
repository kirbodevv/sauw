use bevy::prelude::*;
use jni::{EnvUnowned, objects::JClass};
use once_cell::sync::Lazy;
use std::sync::{
    Mutex,
    atomic::{AtomicBool, Ordering},
};

use crate::game::ui::safe_zone::ChangeSafeZone;

static SAFE_ZONE: Lazy<Mutex<SafeZone>> = Lazy::new(|| Mutex::new(SafeZone::default()));
static DIRTY: AtomicBool = AtomicBool::new(false);

#[derive(Clone, Copy, Default)]
pub struct SafeZone {
    pub top: f32,
    pub bottom: f32,
    pub left: f32,
    pub right: f32,
}

pub struct JNISafeZonePlugin;

fn send_safe_zone(mut messages: MessageWriter<ChangeSafeZone>) {
    let zone = SAFE_ZONE.lock().unwrap();
    if DIRTY.swap(false, Ordering::Relaxed) {
        messages.write(ChangeSafeZone {
            top: zone.top,
            bottom: zone.bottom,
            left: zone.left,
            right: zone.right,
        });
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_cloud_sect_sauw_MainActivity_nativeSetSafeZone<'caller>(
    mut unowned_env: EnvUnowned<'caller>,
    _class: JClass<'caller>,
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
) {
    let _ = unowned_env.with_env(|_| -> Result<(), jni::errors::Error> {
        let mut zone = SAFE_ZONE.lock().unwrap();

        *zone = SafeZone {
            left: left as f32,
            top: top as f32,
            right: right as f32,
            bottom: bottom as f32,
        };

        DIRTY.store(true, Ordering::Relaxed);

        Ok(())
    });
}

impl Plugin for JNISafeZonePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, send_safe_zone);
    }
}
