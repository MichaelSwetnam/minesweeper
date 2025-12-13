mod camera;
mod cell;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())    
        )
        .add_plugins(
            (camera::CameraPlugin, cell::CellPlugin)
        )
        .insert_resource(ClearColor(Color::BLACK))
        .run();
}