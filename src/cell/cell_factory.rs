use bevy::prelude::*;
use crate::{cell::{Air, CellBehavior, CellBorder, CellContent, Mine, Wall}, grid::Grid};

fn add_children(cmds: &mut EntityCommands<'_>, asset_server: &Res<AssetServer>) {
    cmds.with_children(|parent| {
        parent.spawn((
            Sprite {
                image: asset_server.load("cell_border.png"),
                ..Default::default()
            },
            Transform::default(),
            Visibility::Visible,
            CellBorder,
        ));

        parent.spawn((
            Sprite {
                image: asset_server.load("flag.png"),
                color: Color::linear_rgb(1.0, 0.0, 0.0),
                ..Default::default()
            },
            Transform::default(),
            Visibility::Hidden,
            CellContent,
        ));
    });
}

pub struct CellFactory;
impl CellFactory {
    pub fn spawn_mine(cmds: &mut Commands, grid: &mut ResMut<Grid>, asset_server: &Res<AssetServer>, x: i32, y: i32) -> Entity {
        let mut ec = cmds.spawn((
            Mine,
            Mine::transform(grid, x, y, 1.0),
            Visibility::Visible
        ));

        add_children(&mut ec, asset_server);
        
        let entity = ec.id();
        grid.insert(x, y, entity);
        return entity;
    }

    pub fn spawn_air(cmds: &mut Commands, grid: &mut ResMut<Grid>, asset_server: &Res<AssetServer>, x: i32, y: i32, neighbor_mines: u8) -> Entity {
        let mut ec = cmds.spawn((
            Air { neighbor_mines, revealed: false },
            Air::transform(grid, x, y, 1.0),
            Visibility::Visible
        ));

        add_children(&mut ec, asset_server);
        
        let entity = ec.id();
        grid.insert(x, y, entity);
        return entity;
    }

    pub fn spawn_wall(cmds: &mut Commands, grid: &mut ResMut<Grid>, asset_server: &Res<AssetServer>, x: i32, y: i32) -> Entity {
        let entity = cmds.spawn((
            Wall,
            Sprite {
                image: asset_server.load("wall.png"),
                ..Default::default()
            },
            Wall::transform(grid, x, y, 1.0),
            Visibility::Visible
        ))
        .id();

        grid.insert(x, y, entity);
        return entity;
    }
}
