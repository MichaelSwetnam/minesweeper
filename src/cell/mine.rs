use bevy::prelude::*;
use crate::cell::{CELL_BORDER_PATH, Cell, CellBehavior};

fn update_border(sprite: &mut Sprite, visibility: &mut Visibility, asset_server: &AssetServer, has_border: bool) {
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

/// Mine cells are those which are mines in minesweeper. When revealed, they explode.
/// They are flaggable.
#[derive(Component, Clone)]
#[require(Cell)]
pub struct Mine;
impl CellBehavior for Mine {
    fn size() -> u32 { 16 }
    fn has_border() -> bool { true }

    fn update_content(&self, _sprite: &mut Sprite, visibility: &mut Visibility, _asset_server: &AssetServer) {
        *visibility = Visibility::Hidden;
    }
    
    fn update_border(&self, sprite: &mut Sprite, visibility: &mut Visibility, asset_server: &AssetServer) {
        update_border(sprite, visibility, asset_server, true);
    }
}

