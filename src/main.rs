use bevy::prelude::*;

mod ui;

use ui::camera::CameraPlugin;
use ui::spawn_topology::SpawnTopologyPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.6, 0.6, 0.9)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 750.0,
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraPlugin)
        .add_plugins(SpawnTopologyPlugin)
        .run();
}
