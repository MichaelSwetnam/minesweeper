mod camera;
mod cell;
mod grid;
mod player;

use bevy::prelude::*;

use crate::grid::Grid;

fn main() {
    let grid = Grid::default();
    // grid.mine_chance = 25.0;

    App::new()
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())    
        )
        .add_plugins(
            (camera::CameraPlugin, cell::CellPlugin, player::PlayerPlugin)
        )
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(grid)
        .run();
}