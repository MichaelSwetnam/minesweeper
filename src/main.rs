mod camera;
mod cell;
mod grid;

use bevy::prelude::*;

use crate::grid::Grid;

#[derive(Component, Default)]
#[allow(dead_code)]
struct Position { x: u32, y: u32 }
impl Position {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

fn main() {
    let grid = Grid::default();
    // grid.mine_chance = 25.0;

    App::new()
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())    
        )
        .add_plugins(
            (camera::CameraPlugin, cell::CellPlugin)
        )
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(grid)
        .run();
}