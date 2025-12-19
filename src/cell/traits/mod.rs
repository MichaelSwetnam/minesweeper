use bevy::prelude::*;

use crate::{cell::{CellBorder, CellContent}, grid::Grid};


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
