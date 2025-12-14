mod spawn_grid;
mod reveal_cells;

use bevy::prelude::*;

pub struct CellPlugin;
impl Plugin for CellPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(CellGridResource::new())
            .add_message::<reveal_cells::RevealCellMessage>()
            .add_systems(Startup, spawn_grid::spawn_grid)
            .add_systems(Update, 
                (toggle_flag, reveal_cells::handle_reveal_click, reveal_cells::reveal_cell)
            );
    }
}

#[derive(Resource)]
pub struct CellGridResource {
    pub width: u32,
    pub height: u32,
    pub gap: u32,
    pub cell_size: u32,
    cells: Vec<Option<Entity>>
}
impl CellGridResource {
    fn new() -> Self {
        let width = 10;
        let height = 8;

        Self {
            width,
            height,
            cell_size: 16,
            gap: 4,
            cells: vec![None; (width * height) as usize] 
        }
    }
    
    fn index(&self, x: u32, y: u32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }

    fn cell_entity(&self, x: u32, y: u32) -> Option<Entity> {
        self.cells[self.index(x, y)]
    }

    fn insert(&mut self, x: u32, y: u32, entity: Entity) {
        let index = self.index(x, y);
        self.cells[index] = Some(entity);
    }
}

#[derive(Component)]
pub struct Cell {
    pub x: u32,
    pub y: u32,
    pub has_mine: bool,
    pub revealed: bool,
    pub flagged: bool,
    // How many mines this cell has as neighbors
    pub neighbor_mines: u8
}

#[derive(Component)]
struct CellContent;

#[derive(Component)]
struct CellBorder;


#[derive(Debug, Clone)]
enum CellType {
    Bomb,
    Safe(u8)
}

fn get_cursor_position(
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
) -> Option<Vec2> {
    let window = windows.single().unwrap();
    let (camera, camera_transform) = camera_q.single().unwrap();

    window.cursor_position().and_then(|cursor_pos| {
        match camera.viewport_to_world_2d(camera_transform, cursor_pos) {
            Ok(x) => Some(x),
            Err(_) => None
        }
    })
}

fn world_to_cell(pos: Vec2, grid: &CellGridResource) -> Option<(u32, u32)> {
    let cell = grid.cell_size as f32;
    let gap = grid.gap as f32;
    let step = cell + gap;

    // Centers of cells are at x*step - offset_x, y*step - offset_y
    let offset_x = (grid.width as f32) * step * 0.5;
    let offset_y = (grid.height as f32) * step * 0.5;

    // Find the nearest cell center by rounding to the closest step index
    let ix = ((pos.x + offset_x) / step).round() as i32;
    let iy = ((pos.y + offset_y) / step).round() as i32;

    // Bounds check
    if ix < 0 || ix >= grid.width as i32 || iy < 0 || iy >= grid.height as i32 {
        return None;
    }

    // Compute the center of the chosen cell
    let cx = (ix as f32) * step - offset_x;
    let cy = (iy as f32) * step - offset_y;

    // Check if the click is within the 16x16 sprite (cell area), not in the gap
    let half = cell * 0.5;
    if (pos.x - cx).abs() <= half && (pos.y - cy).abs() <= half {
        Some((ix as u32, iy as u32))
    } else {
        None
    }
}

fn toggle_flag(
    grid: Res<CellGridResource>,
    input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,

    mut cells: Query<&mut Cell>,
    mut content_sprites: Query<&mut Visibility, With<CellContent>>,
    children_q: Query<&Children>,
) {
    if !input.just_pressed(MouseButton::Right) {
        return;
    }

    let Some(world_pos) = get_cursor_position(windows, camera_q) else { return };
    let Some((cx, cy)) = world_to_cell(world_pos, &grid) else { return };
    let Some(cell_entity) = grid.cell_entity(cx, cy) else { return };

    let mut cell = match cells.get_mut(cell_entity) {
        Ok(cell) => cell,
        Err(_) => return, // entity has no Cell component
    };

    // If a cell is revealed, it can't be flagged.
    if cell.revealed { return }

    cell.flagged = !cell.flagged;

    // Get the children of the entity
    let children = match children_q.get(cell_entity) {
        Ok(children) => children,
        Err(_) => return, // entity has no children
    };

    // Toggle visibility of the child sprites
    for child in children.iter() {
        if let Ok(mut visibility) = content_sprites.get_mut(child) {
            *visibility = if cell.flagged {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        }
    }
}