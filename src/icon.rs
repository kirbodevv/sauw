use bevy::ecs::system::NonSendMarker;
use bevy::prelude::*;
use bevy::winit::WINIT_WINDOWS;
use winit::window::Icon;

pub struct AppIconPlugin(String);

impl AppIconPlugin {
    pub fn new(icon_path: &str) -> Self {
        Self(icon_path.to_string())
    }
}

impl Plugin for AppIconPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WindowIconPath(self.0.clone()))
            .add_systems(Startup, set_window_icon);
    }
}

#[derive(Resource)]
struct WindowIconPath(String);

fn set_window_icon(mut commands: Commands, _marker: NonSendMarker, icon_path: Res<WindowIconPath>) {
    WINIT_WINDOWS.with_borrow(|winit_windows| {
        if winit_windows.windows.len() == 0 {
            return;
        }

        let (icon_rgba, icon_width, icon_height) = {
            let image = image::open(&icon_path.0)
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

        commands.remove_resource::<WindowIconPath>();
        info!("Window icon set");
    });
}
