use bevy::prelude::*;
use crate::cell::{CELL_BORDER_PATH, Cell, CellBehavior};



/// Air cells are those which show information about surrounding mines. Think the 1, 2, 3, ... in typical minesweeper.
/// They are flaggable.
#[derive(Component, Clone)]
#[require(Cell)]
pub struct Air {
    pub(crate) neighbor_mines: u8,
    pub(crate) revealed: bool
}
impl CellBehavior for Air {
    fn size() -> u32 { 16 }
    fn has_border() -> bool { true }
    
    fn update_content(&self, sprite: &mut Sprite, visibility: &mut Visibility, asset_server: &AssetServer) {
        let textures: [Handle<Image>; 8] = [
            asset_server.load("one.png"),
            asset_server.load("two.png"),
            asset_server.load("three.png"),
            asset_server.load("four.png"),
            asset_server.load("five.png"),
            asset_server.load("six.png"),
            asset_server.load("seven.png"),
            asset_server.load("eight.png"),
        ]; 

        // https://lospec.com/palette-list/flatter18#comments
        let texture_colors = [
            Color::linear_rgb(80.0 / 255.0, 96.0 / 255.0, 219.0 / 255.0),
            Color::linear_rgb(21.0 / 255.0, 181.0 / 255.0, 81.0 / 255.0),
            Color::linear_rgb(233.0 / 255.0, 64.0 / 255.0, 51.0 / 255.0),
            Color::linear_rgb(63.0 / 255.0, 63.0 / 255.0, 143.0 / 255.0),
            Color::linear_rgb(187.0 / 255.0, 51.0 / 255.0, 37.0 / 255.0),
            Color::linear_rgb(45.0 / 255.0, 151.0 / 255.0, 170.0 / 255.0),
            Color::linear_rgb(226.0 / 255.0, 181.0 / 255.0, 23.0 / 255.0),
            Color::linear_rgb(177.0 / 255.0, 70.0 / 255.0, 193.0 / 255.0),
        ];

        if !self.revealed {
            *visibility = Visibility::Hidden;
            return;
        }

        match self.neighbor_mines {
            0 => {
                *visibility = Visibility::Hidden;
            },
            1..=8 => {
                *visibility = Visibility::Visible;
                sprite.image = textures[self.neighbor_mines as usize - 1].clone();
                sprite.color = texture_colors[self.neighbor_mines as usize - 1];
            },
            _ => unreachable!()
        }
    }
    
    fn update_border(&self, sprite: &mut Sprite, visibility: &mut Visibility, asset_server: &AssetServer) {        
        if !self.revealed {
            sprite.image = asset_server.load(CELL_BORDER_PATH);
            *visibility = Visibility::Visible;
            return;
        }
        
        match self.neighbor_mines {
            0 => {
                *visibility = Visibility::Hidden;
            },
            1..=8 => {
                sprite.image = asset_server.load(CELL_BORDER_PATH);
                *visibility = Visibility::Visible;
            },
            _ => unreachable!()
        }
    }
}