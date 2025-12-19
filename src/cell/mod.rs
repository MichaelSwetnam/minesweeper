mod systems;
mod air;
mod mine;
mod wall;
pub use air::Air;
pub use mine::Mine;
pub use wall::Wall;

use bevy::prelude::*;

use crate::{cell::systems::{RevealCellPlugin, spawn_grid, toggle_flag}, grid::Grid};

pub const CELL_BORDER_PATH: &'static str = "cell_border.png";


fn add_children<T : CellBehavior>(obj: &T, cmds: &mut EntityCommands<'_>, asset_server: &AssetServer) {
    // Build default components
    let mut content_sprite = Sprite::default();
    let mut content_visibility = Visibility::default();

    // Get cell specific content + border information
    obj.update_content(&mut content_sprite, &mut content_visibility, asset_server);

    // Spawn border
    if T::has_border() {
        let mut border_sprite = Sprite::default();
        let mut border_visibility = Visibility::default();

        obj.update_border(&mut border_sprite, &mut border_visibility, asset_server);

        cmds.with_children(|parent| {
            parent.spawn((
                border_sprite,
                border_visibility,
                Transform::default(),
                CellBorder,
            ));
        });
    }

    // Spawn content
    cmds.with_children(|parent| {
        parent.spawn((
            content_sprite,
            content_visibility,
            Transform::default(),
            CellContent,
        ));
    });
}

pub(crate) fn update_border(sprite: &mut Sprite, visibility: &mut Visibility, asset_server: &AssetServer, has_border: bool) {
    match has_border {
        true => {
            sprite.image = asset_server.load(CELL_BORDER_PATH);
            *visibility = Visibility::Visible;
        },
        false => {
            *visibility = Visibility::Hidden;
        }
    }

}

pub struct CellPlugin;
impl Plugin for CellPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(RevealCellPlugin)
            .add_systems(Startup, spawn_grid)
            .add_systems(Update, 
                toggle_flag
            );
    }
}

pub trait CellBehavior : Component + Sized + Clone {
    fn size() -> u32;
    fn has_border() -> bool;

    fn spawn(self, cmds: &mut Commands, grid: &mut ResMut<Grid>, asset_server: &Res<AssetServer>, x: i32, y: i32) -> Entity {        
        let mut ec = cmds.spawn((
            self.clone(),
            Self::transform(grid, x, y, 1.0),
            Visibility::Visible
        ));

        add_children(&self, &mut ec, asset_server);
        let entity = ec.id();
        grid.insert(x, y, entity);

        entity
    }

    fn update_content(&self, sprite: &mut Sprite, visibility: &mut Visibility, asset_server: &AssetServer);
    fn update_border(&self, sprite: &mut Sprite, visibility: &mut Visibility, asset_server: &AssetServer);

    fn transform(grid: &Grid, x: i32, y: i32, z: f32) -> Transform
    {
        let cell_size = grid.cell_size();
        let sprite_size = Self::size();
        if sprite_size > cell_size { panic!("Cell::size ({}) exceeds grid cell_size ({})! Must fit inside.", sprite_size, cell_size ) };

        Transform {
            translation: Vec3::new(
                x as f32 * cell_size as f32,
                y as f32 * cell_size as f32,
                z,
            ),
            scale: Vec3::new(grid.scale(), grid.scale(), 1.0),
            ..Default::default()
        }
    }
}



// fn cells_touched_by_transform(
//     transform: &Transform,
//     grid: &Grid,
// ) -> Vec<(i32, i32)> {
//     
// }

/** Cell */
#[derive(Component, Default)]
pub struct Cell;
impl Cell {
    /// Returns all grid cells overlapped by the given transform.
    /// Assumes sprites are centered and each cell is exactly grid.cell_size() in world units.
    fn touched_by(transform: &Transform, grid: &Grid) -> Vec<(i32, i32)> {
        let cell = grid.cell_size() as f32;
        let half_cell = cell * 0.5;

        // Player half extents in world units (scale is the final size)
        let half_w = transform.scale.x * 0.5;
        let half_h = transform.scale.y * 0.5;

        // Player AABB in world space
        let min_x = transform.translation.x - half_w;
        let max_x = transform.translation.x + half_w;
        let min_y = transform.translation.y - half_h;
        let max_y = transform.translation.y + half_h;

        // For cell k centered at k*cell, span is [k*cell - half_cell, k*cell + half_cell].
        // Overlap if: max_x >= k*cell - half_cell AND min_x <= k*cell + half_cell.
        // Solve for k:
        //   k >= (min_x - half_cell)/cell
        //   k <= (max_x + half_cell)/cell
        let min_ix = ((min_x - half_cell) / cell).ceil() as i32;
        let max_ix = ((max_x + half_cell) / cell).floor() as i32;
        let min_iy = ((min_y - half_cell) / cell).ceil() as i32;
        let max_iy = ((max_y + half_cell) / cell).floor() as i32;

        let mut touched = Vec::new();
        if min_ix <= max_ix && min_iy <= max_iy {
            for ix in min_ix..=max_ix {
                for iy in min_iy..=max_iy {
                    touched.push((ix, iy));
                }
            }
        }
        touched
    }
}


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