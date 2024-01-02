use bevy::{app::App, DefaultPlugins};
use bevy_flycam::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            PlayerPlugin,
        ))
        .run();
}