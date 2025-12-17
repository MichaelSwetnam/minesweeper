use bevy::prelude::*;
use crate::{cell::{Air, Cell, CellBorder, CellContent, Mine, Wall}, grid::Grid};

fn transform(grid: &ResMut<Grid>, x: u32, y: u32) -> (f32, f32) {
    let step = grid.cell_size as f32 + grid.gap as f32;
    let offset_x = (grid.width as f32 * step) / 2.0;
    let offset_y = (grid.height as f32 * step) / 2.0;

    return (
        x as f32 * step - offset_x,
        y as f32 * step - offset_y
    );
}

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
    pub fn spawn_mine(cmds: &mut Commands, grid: &mut ResMut<Grid>, asset_server: &Res<AssetServer>, x: u32, y: u32) -> Entity {
        let (tx, ty) = transform(grid, x, y);

        let mut ec = cmds.spawn((
            Cell,
            Mine,
            Transform::from_translation(Vec3::new(tx, ty, 0.0)),
            Visibility::Visible
        ));

        add_children(&mut ec, asset_server);
        
        let entity = ec.id();
        grid.insert(x, y, entity);
        return entity;
    }

    pub fn spawn_air(cmds: &mut Commands, grid: &mut ResMut<Grid>, asset_server: &Res<AssetServer>, x: u32, y: u32, neighbor_mines: u8) -> Entity {
        let (tx, ty) = transform(&grid, x, y);

        let mut ec = cmds.spawn((
            Cell,
            Air { neighbor_mines, revealed: false },
            Transform::from_translation(Vec3::new(tx, ty, 0.0)),
            Visibility::Visible
        ));

        add_children(&mut ec, asset_server);
        
        let entity = ec.id();
        grid.insert(x, y, entity);
        return entity;
    }

    pub fn spawn_wall(cmds: &mut Commands, grid: &mut ResMut<Grid>, asset_server: &Res<AssetServer>, x: u32, y: u32) -> Entity {
        let (tx, ty) = transform(&grid, x, y);

        let entity = cmds.spawn((
            Cell,
            Wall,
            Sprite {
                image: asset_server.load("wall.png"),
                ..Default::default()
            },
            Transform::from_translation(Vec3::new(tx, ty, 0.0)),
            Visibility::Visible
        ))
        .id();

        grid.insert(x, y, entity);
        return entity;
    }
}
