mod spawn_grid;
mod reveal_cells;
mod toggle_flag;
mod cell_factory;

use std::ops::Deref;

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


const STANDARD_SCALE: Vec3 = Vec3::new(1.0, 1.0, 1.0);
pub trait CellBehavior {
    fn size() -> u32;

    fn transform<'a, T>(grid: &T, x: u32, y: u32, z: f32) -> Transform
    where T : Deref<Target = Grid>
    {
        let cell_size = grid.cell_size();
        let sprite_size = Self::size();

        if sprite_size > cell_size { panic!("Cell::size ({}) exceeds grid cell_size ({})! Must fit inside.", sprite_size, cell_size ) };

        let gap = cell_size - sprite_size;
        let offset_x = (grid.width() * cell_size) as f32 / 2.0;
        let offset_y = (grid.height() * cell_size) as f32 / 2.0;

        Transform {
            translation: Vec3::new(
                x as f32 * cell_size as f32 - offset_x + gap as f32 / 2.0,
                y as f32 * cell_size as f32 - offset_y + gap as f32 / 2.0,
                z,
            ),
            scale: STANDARD_SCALE,
            ..Default::default()
        }
    }
}

/** Cell */
#[derive(Component, Default)]
pub struct Cell;
impl Cell {
    /// Example method to prove the concept.
    fn size<T : CellBehavior>() -> u32 {
        T::size()
    }
}

/** Types of Cells */

/// Mine cells are those which are mines in minesweeper. When revealed, they explode.
/// They are flaggable.
#[derive(Component)]
#[require(Cell)]
pub struct Mine;
impl CellBehavior for Mine {
    fn size() -> u32 { 16 }
}

/// Air cells are those which show information about surrounding mines. Think the 1, 2, 3, ... in typical minesweeper.
/// They are flaggable.
#[derive(Component, Default)]
#[require(Cell)]
pub struct Air {
    neighbor_mines: u8,
    revealed: bool
}
impl CellBehavior for Air {
    fn size() -> u32 { 16 }
}

#[derive(Component)]
#[require(Cell)]
pub struct Wall;
impl CellBehavior for Wall {
    fn size() -> u32 { 16 }
}

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