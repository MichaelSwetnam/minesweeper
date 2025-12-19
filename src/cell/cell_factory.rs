use bevy::prelude::*;
use crate::{cell::{Air, CellBehavior, CellBorder, CellContent, Mine, Wall}, grid::Grid};

pub struct CellFactory;
impl CellFactory {
    pub fn spawn_mine(cmds: &mut Commands, grid: &mut ResMut<Grid>, asset_server: &Res<AssetServer>, x: i32, y: i32) -> Entity {
        Mine.spawn(cmds, grid, asset_server, x, y)
        // let mut ec = cmds.spawn((
        //     Mine,
        //     Mine::transform(grid, x, y, 1.0),
        //     Visibility::Visible
        // ));

        // add_children(&mut ec, asset_server);
        
        // let entity = ec.id();
        // grid.insert(x, y, entity);
        // return entity;
    }

    pub fn spawn_air(cmds: &mut Commands, grid: &mut ResMut<Grid>, asset_server: &Res<AssetServer>, x: i32, y: i32, neighbor_mines: u8) -> Entity {
        Air {neighbor_mines, revealed: false}.spawn(cmds, grid, asset_server, x, y)
        // let mut ec = cmds.spawn((
        //     Air { neighbor_mines, revealed: false },
        //     Air::transform(grid, x, y, 1.0),
        //     Visibility::Visible
        // ));

        // add_children(&mut ec, asset_server);
        
        // let entity = ec.id();
        // grid.insert(x, y, entity);
        // return entity;
    }

    pub fn spawn_wall(cmds: &mut Commands, grid: &mut ResMut<Grid>, asset_server: &Res<AssetServer>, x: i32, y: i32) -> Entity {
        Wall.spawn(cmds, grid, asset_server, x, y)
        // let entity = cmds.spawn((
        //     Wall,
        //     Sprite {
        //         image: asset_server.load("wall.png"),
        //         ..Default::default()
        //     },
        //     Wall::transform(grid, x, y, 1.0),
        //     Visibility::Visible
        // ))
        // .id();

        // grid.insert(x, y, entity);
        // return entity;
    }
}
