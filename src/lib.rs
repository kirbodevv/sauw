pub mod constants;
pub mod game;
pub mod icon;
pub mod platform {
    pub mod safe_zone;
}

use crate::game::GamePlugin;
use bevy::prelude::*;

pub fn run_game() {
    let mut app = App::new();
    app.add_plugins(GamePlugin);

    #[cfg(any(target_os = "windows", target_os = "linux"))]
    {
        use crate::icon::AppIconPlugin;
        app.add_plugins(AppIconPlugin::new("assets/icon/icon_128.png"));
    }

    app.run();
}

#[cfg(target_os = "android")]
#[bevy_main]
fn main() {
    run_game();
}
