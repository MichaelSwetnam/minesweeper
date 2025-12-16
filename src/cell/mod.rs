mod spawn_grid;
mod reveal_cells;
mod toggle_flag;

use bevy::prelude::*;

use crate::{Position, cell::reveal_cells::RevealCellPlugin, grid::Grid};

pub struct CellPlugin;
impl Plugin for CellPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(RevealCellPlugin)
            .add_systems(Startup, spawn_grid::spawn_grid)
            .add_systems(Update, 
                toggle_flag::toggle_flag
            );
    }
}
pub struct CellFactory;
impl CellFactory {
    pub fn spawn_mine(cmds: &mut Commands, x: u32, y: u32) -> Entity {
        cmds.spawn((
            Position::new(x, y),
            Mine
        ))
        .id()
    }

    pub fn spawn_air(cmds: &mut Commands, x: u32, y: u32) -> Entity {
        cmds.spawn((
            Position::new(x, y),
            Air::default()
        ))
        .id()
    }

    pub fn spawn_wall(cmds: &mut Commands, x: u32, y: u32) -> Entity {
        cmds.spawn((
            Position::new(x, y),
            Wall
        ))
        .id()
    }
}


#[derive(Debug, Clone)]
pub enum CellKind {
    Mine,
    Air,
    Wall
}

/** Cell Components */

#[derive(Component)]
pub struct Mine;

#[derive(Component, Default)]
pub struct Air {
    neighbor_mines: u8,
    revealed: bool
}

#[derive(Component)]
pub struct Wall;

#[derive(Component)]
#[require(Position)]
pub struct Cell;

#[derive(Component)]
pub struct Flagged;

/** Sprite Marker Components */

/// Marks the sprite which controls the contents of the cell
/// This sprite is responsible for displaying the flag, mine, and numbers for the cell.
#[derive(Component)]
struct CellContent;

/// Marks the sprite which controls the borders of the cell
/// The borders are enabled unless the cell is a wall or free space (air) with 0 mine neighbors.
#[derive(Component)]
struct CellBorder;