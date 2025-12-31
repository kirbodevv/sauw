use bevy::ecs::system::NonSendMarker;
use bevy::prelude::*;
use bevy_winit::WINIT_WINDOWS;
use winit::window::Icon;

#[derive(Resource, Default)]
pub struct IconSet(bool);

pub fn set_window_icon_once(_marker: NonSendMarker, mut icon_set: ResMut<IconSet>) {
    if icon_set.0 {
        return;
    }

    WINIT_WINDOWS.with_borrow(|winit_windows| {
        if winit_windows.windows.len() == 0 {
            return;
        }

        let (icon_rgba, icon_width, icon_height) = {
            let image = image::open("assets/icon/icon_128.png")
                .expect("Failed to open icon path")
                .into_rgba8();
            let (width, height) = image.dimensions();
            let rgba = image.into_raw();
            (rgba, width, height)
        };

        let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

        for window in winit_windows.windows.values() {
            window.set_window_icon(Some(icon.clone()));
        }

        icon_set.0 = true;
        info!("Window icon set");
    });
}
