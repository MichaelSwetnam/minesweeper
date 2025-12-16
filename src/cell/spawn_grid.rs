use rand::Rng;

use crate::{Position, cell::{Air, Cell, CellBorder, CellContent, Mine, Wall}, grid::Grid};
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
        (y as usize * grid_settings.width as usize) + x as usize
    };

    // Flattened width by height 2d array
    let mut grid: Vec<CellType> = vec![Air(0); (grid_settings.width * grid_settings.height) as usize];
    let mut r = rand::rng();

    // Insert bombs
    for x in 0..grid_settings.width {
        for y in 0..grid_settings.height {
            if r.random::<f32>() < (grid_settings.mine_chance / 100.0) {
                grid[idx(x, y)] = Mine;
            }
        }
    }

    // Calculate number of surrounding bombs.
    for x in 0..grid_settings.width {
        for y in 0..grid_settings.height {
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

                    if nx < grid_settings.width && ny < grid_settings.height {
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
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let border_texture = asset_server.load("cell_border.png");
    let flag_texture = asset_server.load("flag.png");

    let layout = TextureAtlasLayout::from_grid(UVec2::new(16, 16), 1, 1, None, None);
    let layout_handle: Handle<TextureAtlasLayout> = layouts.add(layout);

    let grid_x = grid.width;
    let grid_y = grid.height;
    let step = grid.cell_size as f32 + grid.gap as f32;

    // Centering offsets
    let offset_x = (grid_x as f32 * step) / 2.0;
    let offset_y = (grid_y as f32 * step) / 2.0;

    let grid_cells = generate_grid(&grid);
    for (index, cell) in grid_cells.iter().enumerate() {
        let x = index as u32 % grid.width;
        let y = index as u32 / grid.width;

        let transform_x = x as f32 * step - offset_x;
        let transform_y = y as f32 * step - offset_y;

        // let entity = commands.spawn((
        //     match cell {
        //         CellType::Mine => (
        //             Position::new(x, y),
        //             Cell,
        //             Mine
        //         ),
        //         CellType::Wall => (
        //             Position::new(x, y),
        //             Cell,
        //             Wall
        //         ),

        //         CellType::Air(_) => todo!(),
        //     },
        //     Transform::from_translation(Vec3::new(transform_x, transform_y, 0.0)),
        //     Visibility::Visible,
        // ))
        let entity = 
            match cell {
                CellType::Mine => {
                    commands.spawn((
                        Position::new(x, y),
                        Cell,
                        Mine,
                        Transform::from_translation(Vec3::new(transform_x, transform_y, 0.0)),
                        Visibility::Visible,
                    ))
                },
                CellType::Wall => {
                    commands.spawn((
                        Position::new(x, y),
                        Cell,
                        Wall,
                        Transform::from_translation(Vec3::new(transform_x, transform_y, 0.0)),
                        Visibility::Visible,
                    ))
                },
                CellType::Air(neighbors) => {
                    commands.spawn((
                        Position::new(x, y),
                        Cell,
                        Air { neighbor_mines: *neighbors, revealed: false },
                        Transform::from_translation(Vec3::new(transform_x, transform_y, 0.0)),
                        Visibility::Visible,
                    ))
                }
            }
            .with_children(|parent| {
                parent.spawn((
                    Sprite {
                        image: border_texture.clone(),
                        texture_atlas: Some(layout_handle.clone().into()),
                        ..Default::default()
                    },
                    Transform::default(),
                    Visibility::Visible,
                    CellBorder,
                ));

                parent.spawn((
                    Sprite {
                        image: flag_texture.clone(),
                        texture_atlas: Some(layout_handle.clone().into()),
                        color: Color::linear_rgb(1.0, 0.0, 0.0),
                        ..Default::default()
                    },
                    Transform::default(),
                    Visibility::Hidden,
                    CellContent,
                ));
            })
            .id();

        grid.insert(x, y, entity);
    }
}
