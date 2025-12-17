use bevy::prelude::*;

#[derive(Resource)]
pub struct Grid {
    /// Width in cells
    width: u32,
    /// Height in cells
    height: u32,
    pub gap: u32,
    pub cell_size: u32,
    pub mine_chance: f32, // Percentage
    pub wall_chance: f32,
    cells: Vec<Option<Entity>>
}
impl Grid {
    fn index(&self, x: u32, y: u32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    /** Getters */
    pub fn width(&self) -> u32 { self.width }
    pub fn height(&self) -> u32 { self.height }

    pub fn cell_entity(&self, x: u32, y: u32) -> Option<Entity> {
        self.cells[self.index(x, y)]
    }

    pub fn insert(&mut self, x: u32, y: u32, entity: Entity) {
        let index = self.index(x, y);
        if self.cells[index].is_some() { panic!("A system is attempting to insert an entity into the Grid, but an entitity already exists there!") }
        
        self.cells[index] = Some(entity);
    }
}
impl Default for Grid {
    fn default() -> Self {
        let width = 16;
        let height = 16;

         Self {
            width,
            height,
            cell_size: 16,
            gap: 4,
            mine_chance: 12.3,
            wall_chance: 12.3,
            cells: vec![None; (width * height) as usize] 
        }
    }
}