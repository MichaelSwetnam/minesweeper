use bevy::prelude::*;
use crate::cell::*;

#[derive(Message)]
pub struct RevealCellMessage {
    pub x: u32,
    pub y: u32
}

pub fn reveal_cell(
    mut commands: Commands,

    asset_server: Res<AssetServer>,
    grid: Res<CellGridResource>,
    children_q: Query<&Children>,

    mut reader: MessageReader<RevealCellMessage>,
    mut cells: Query<&mut Cell>,
    mut content_sprites: Query<(&mut Visibility, &mut Sprite), With<CellContent>>,
    border_sprites: Query<&CellBorder>
) {
    let textures = [
        asset_server.load("one.png"),
        asset_server.load("two.png"),
        asset_server.load("three.png"),
        asset_server.load("four.png"),
        asset_server.load("five.png"),
        asset_server.load("six.png"),
        asset_server.load("seven.png"),
        asset_server.load("eight.png"),
    ]; 

    let mut queue = Vec::new();

    for RevealCellMessage { x, y } in reader.read() {
        queue.push((*x, *y));
    }  

    let mut i = 0;
    while i < queue.len() {
        let (x, y) = queue[i];
        i += 1;

        let Some(entity) = grid.cell_entity(x, y) else { continue };
        let mut cell = match cells.get_mut(entity) {
            Ok(cell) => cell,
            Err(_) => continue
        };

        // If a cell is revealed, it can't be revealed again.
        if cell.revealed { continue };
        // If a cell is flagged, it can't be revelead.
        if cell.flagged { continue };

        if cell.has_mine {
            println!("This is a mine!");
            continue;
        }
        
        cell.revealed = true;

        // Get the children of the entity
        let children = match children_q.get(entity) {
            Ok(children) => children,
            Err(_) => continue, // entity has no children
        };

        // Toggle visibility of the child sprites
        for child in children.iter() {
            // Content Sprite
            if let Ok((mut visibility, mut sprite)) = content_sprites.get_mut(child) {
                if cell.neighbor_mines == 0 {
                    commands.entity(child).despawn();

                    // Reveal neighbors
                    for dx in -1..=1 {
                        for dy in -1..=1 {
                            if dx == 0 && dy == 0 { continue };
                            let rx = x as i32 + dx;
                            let ry = y as i32 + dy;

                            // Out of bounds
                            if rx < 0 || ry < 0 || rx >= grid.width as i32 || ry >= grid.height as i32 { continue; }

                            // Queue the neighbors
                            queue.push((rx as u32, ry as u32));
                        }
                    }
                } else {
                    *visibility = Visibility::Visible;
                    sprite.image = textures[cell.neighbor_mines as usize - 1].clone();
                }
            }

            // Border Sprite
            if let Ok(_) = border_sprites.get(child) {
                if cell.neighbor_mines == 0 {
                    commands.entity(child).despawn();
                }

            }
        }
    }
}

pub fn handle_reveal_click(
    mut events: MessageWriter<RevealCellMessage>,
    grid: Res<CellGridResource>,
    input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
) {
    if !input.just_pressed(MouseButton::Left) {
        return;
    }

    let Some(world_pos) = get_cursor_position(windows, camera_q) else { return; };
    let Some((cx, cy)) = world_to_cell(world_pos, &grid) else { return; };
    events.write(RevealCellMessage { x: cx, y: cy });
}