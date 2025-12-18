use bevy::{platform::collections::HashMap, prelude::*};

const CHUNK_WIDTH: usize = 16;
const CHUNK_HEIGHT: usize = 16;
pub struct Chunk {
    cells: [Option<Entity>; CHUNK_WIDTH * CHUNK_HEIGHT]
}
impl Chunk {
    fn index(&self, x: u32, y: u32) -> usize {
        (y as usize * CHUNK_WIDTH) + x as usize
    }

    pub fn new() -> Self {
        Self { cells: [None; CHUNK_WIDTH * CHUNK_HEIGHT] }
    }

    pub fn get(&self, x: u32, y: u32) -> Option<Entity> {
        self.cells[self.index(x, y)]
    }

    pub fn insert(&mut self, x: u32, y: u32, entity: Entity) {
        self.cells[self.index(x, y)] = Some(entity);
    }

    #[allow(dead_code)]
    pub fn delete(&mut self, x: u32, y: u32) {
        self.cells[self.index(x, y)] = None;
    }
}

#[derive(Resource)]
pub struct Grid {
    /// Width in cells
    width: u32,
    /// Height in cells
    height: u32,
    // Determines the size of a full cell. Currently cell_size + gap is the actual cell size.
    cell_size: u32,
    chunks: HashMap<(i32, i32), Chunk>
}
impl Grid {

    /** Getters */
    pub fn width(&self) -> u32 { self.width }
    pub fn height(&self) -> u32 { self.height }
    pub fn cell_size(&self) -> u32 { self.cell_size }
    
    pub fn get(&self, x: i32, y: i32) -> Option<Entity> {
        let cx = x / CHUNK_WIDTH as i32;
        let cy = y / CHUNK_HEIGHT as i32;

        if !self.chunks.contains_key(&(cx, cy)) { panic!("Attempted to reference a chunk which does not exist.") };
        
        // Local (x, y) within the chunk.
        let lx = x.rem_euclid(CHUNK_WIDTH as i32) as u32;
        let ly = y.rem_euclid(CHUNK_HEIGHT as i32) as u32;
        
        self.chunks.get(&(cx, cy)).unwrap().get(lx, ly)
    }

    pub fn insert(&mut self, x: i32, y: i32, entity: Entity) {
        let cx = x / CHUNK_WIDTH as i32;
        let cy = y / CHUNK_HEIGHT as i32;

        if !self.chunks.contains_key(&(cx, cy)) { 
            self.chunks.insert((cx, cy), Chunk::new());
        }

        // Local (x, y) within the chunk.
        let lx = x.rem_euclid(CHUNK_WIDTH as i32) as u32;
        let ly = y.rem_euclid(CHUNK_HEIGHT as i32) as u32;

        let chunk = self.chunks.get_mut(&(cx, cy)).unwrap();
        chunk.insert(lx, ly, entity);
    }

    pub fn pos_from_world(&self, pos: Vec2) -> Vec2 {
        Vec2::new(pos.x / self.cell_size() as f32, pos.y / self.cell_size() as f32)
    }
    pub fn cell_from_world(&self, pos: Vec2) -> IVec2 {
        let pos = self.pos_from_world(pos).round();
        IVec2::new(pos.x as i32, pos.y as i32)
    }
    
}
impl Default for Grid {
    fn default() -> Self {
        let width = 16;
        let height = 16;

         Self {
            width,
            height,
            cell_size: 20,
            chunks: HashMap::new()
        }
    }
}