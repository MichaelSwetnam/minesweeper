use bevy::{platform::collections::HashMap, prelude::*};
use std::sync::LazyLock;

use crate::env::{EnvVariable, acquire_num};

static CHUNK_WIDTH: LazyLock<usize> = LazyLock::new(|| acquire_num(EnvVariable::CHUNK_WIDTH));
static CHUNK_HEIGHT: LazyLock<usize> = LazyLock::new(|| acquire_num(EnvVariable::CHUNK_HEIGHT));

pub struct Chunk {
    // cells: [Option<Entity>; CHUNK_WIDTH * CHUNK_HEIGHT]
    cells: Vec<Option<Entity>>
}
impl Chunk {
    fn index(&self, x: u32, y: u32) -> usize {
        (y as usize * *CHUNK_WIDTH) + x as usize
    }

    pub fn new() -> Self {
        Self { cells: vec![None; *CHUNK_WIDTH * *CHUNK_HEIGHT] }
    }

    pub fn get(&self, x: u32, y: u32) -> Option<Entity> {
        self.cells[self.index(x, y)]
    }

    pub fn insert(&mut self, x: u32, y: u32, entity: Entity) {
        let i = self.index(x, y);
        self.cells[i] = Some(entity);
    }

    #[allow(dead_code)]
    pub fn delete(&mut self, x: u32, y: u32) {
        let i = self.index(x, y);
        self.cells[i] = None;
    }
}

#[derive(Resource)]
pub struct Grid {
    /// Width in cells
    width: u32,
    /// Height in cells
    height: u32,
    // The size of a full cell. The scaling is included in this number.
    cell_size: u32,
    // How much every cell is scaled up from it's original texture.
    scale: f32,
    chunks: HashMap<(i32, i32), Chunk>
}
impl Grid {
    /** Getters */
    pub fn width(&self) -> u32 { self.width }
    pub fn height(&self) -> u32 { self.height }
    pub fn cell_size(&self) -> u32 { self.cell_size }
    pub fn scale(&self) -> f32 { self.scale }

    pub fn get(&self, x: i32, y: i32) -> Option<Entity> {
        let cx = x / *CHUNK_WIDTH as i32;
        let cy = y / *CHUNK_HEIGHT as i32;

        if !self.chunks.contains_key(&(cx, cy)) { panic!("Attempted to reference a chunk which does not exist.") };
        
        // Local (x, y) within the chunk.
        let lx = x.rem_euclid(*CHUNK_WIDTH as i32) as u32;
        let ly = y.rem_euclid(*CHUNK_HEIGHT as i32) as u32;
        
        self.chunks.get(&(cx, cy)).unwrap().get(lx, ly)
    }

    pub fn insert(&mut self, x: i32, y: i32, entity: Entity) {
        let cx = x / *CHUNK_WIDTH as i32;
        let cy = y / *CHUNK_HEIGHT as i32;

        if !self.chunks.contains_key(&(cx, cy)) { 
            self.chunks.insert((cx, cy), Chunk::new());
        }

        // Local (x, y) within the chunk.
        let lx = x.rem_euclid(*CHUNK_WIDTH as i32) as u32;
        let ly = y.rem_euclid(*CHUNK_HEIGHT as i32) as u32;

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

const CELL_SIZE: LazyLock<u32> = LazyLock::new(|| acquire_num(EnvVariable::CELL_SIZE));
const CELL_SCALE: LazyLock<f32> = LazyLock::new(|| acquire_num(EnvVariable::CELL_SCALE));
impl Default for Grid {
    fn default() -> Self {
        let width = 16;
        let height = 16;

         Self {
            width,
            height,
            cell_size: (*CELL_SIZE as f32 * *CELL_SCALE).floor() as u32,
            scale: *CELL_SCALE,
            chunks: HashMap::new()
        }
    }
}