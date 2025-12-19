use bevy::prelude::*;
use crate::cell::{Cell, CellBehavior};

#[derive(Component, Clone)]
#[require(Cell)]
pub struct Wall;
impl CellBehavior for Wall {
    fn size() -> u32 { 20 }
    fn has_border() -> bool { false }

    fn update_content(&self, sprite: &mut Sprite, visibility: &mut Visibility, asset_server: &AssetServer) {
        sprite.image = asset_server.load("wall.png");
        *visibility = Visibility::Visible;
    }
    
    fn update_border(&self, _a: &mut Sprite, _b: &mut Visibility, _c: &AssetServer) {
        panic!()
    }
}