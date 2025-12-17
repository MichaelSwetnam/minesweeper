mod spawn_grid;
mod reveal_cells;
mod toggle_flag;
mod cell_factory;

use bevy::prelude::*;

use crate::{cell::reveal_cells::RevealCellPlugin, grid::Grid};

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

/** Cell Components */

// trait Cell {

// }

/// Mine cells are those which are mines in minesweeper. When revealed, they explode.
/// They are flaggable.
#[derive(Component)]
// #[require(Cell)]
pub struct Mine;

/// Air cells are those which show information about surrounding mines. Think the 1, 2, 3, ... in typical minesweeper.
/// They are flaggable.
#[derive(Component, Default)]
// #[require(Cell)]

pub struct Air {
    neighbor_mines: u8,
    revealed: bool
}

#[derive(Component)]
// #[require(Cell)]
pub struct Wall;

/// Marks cells which are "Cells." These are essentially blocks on the grid.
// #[derive(Component, Default)]
// pub struct Cell;

/** Other Components */

/// Marks cells which are currently flagged. Should be applicable only to Mine and Air cell types.
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