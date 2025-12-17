use rand::Rng;

use crate::{cell::cell_factory::CellFactory, grid::Grid};
use bevy::prelude::*;

#[derive(Debug, Clone)]
enum CellType {
    Air(u8),
    Mine,
    Wall
}

/**
 * Returns a flattened X by Y 2d-vector.
 */
fn generate_grid(grid_settings: &Grid) -> Vec<CellType> {
    use CellType::*;

    // Helper fx
    let idx = |x: u32, y: u32| -> usize {
        (y as usize * grid_settings.width() as usize) + x as usize
    };

    // Flattened width by height 2d array
    let mut grid: Vec<CellType> = vec![Air(0); (grid_settings.width() * grid_settings.height()) as usize];
    let mut r = rand::rng();

    // Insert bombs
    for x in 0..grid_settings.width() {
        for y in 0..grid_settings.height() {
            if x == 0 || y == 0 || x == grid_settings.width() - 1 || y == grid_settings.height() - 1 {
                grid[idx(x, y)] = Wall;
            } else if r.random::<f32>() < (grid_settings.mine_chance / 100.0) {
                grid[idx(x, y)] = Mine;
            } else if r.random::<f32>() < (grid_settings.wall_chance / 100.0) {
                grid[idx(x, y)] = Wall;
            }
        }
    }

    // Calculate number of surrounding bombs.
    for x in 0..grid_settings.width() {
        for y in 0..grid_settings.height() {
            // Select only air elements
            let Air(_) = grid[idx(x, y)] else { continue };
        
            let mut neighbors = 0;

            // Check neighbors
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }

                    let Some(nx) = x.checked_add_signed(dx) else { continue };
                    let Some(ny) = y.checked_add_signed(dy) else { continue };

                    if nx < grid_settings.width() && ny < grid_settings.height() {
                        if matches!(grid[idx(nx, ny)], Mine) {
                            neighbors += 1;
                        }
                    }
                }
            }

            grid[idx(x, y)] = Air(neighbors);
        }
    }

    return grid;
}

pub fn spawn_grid(
    asset_server: Res<AssetServer>,
    mut grid: ResMut<Grid>,
    mut commands: Commands,
) {
    let grid_cells = generate_grid(&grid);
    for (index, cell) in grid_cells.iter().enumerate() {
        let x = index as u32 % grid.width();
        let y = index as u32 / grid.width();

        match cell {
            CellType::Air(n) => CellFactory::spawn_air(&mut commands, &mut grid, &asset_server, x, y, *n),
            CellType::Mine =>        CellFactory::spawn_mine(&mut commands, &mut grid, &asset_server, x, y),
            CellType::Wall =>        CellFactory::spawn_wall(&mut commands, &mut grid, &asset_server, x, y),
        };
    }
}
