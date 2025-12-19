use bevy::prelude::*;
use crate::{cell::{Air, Cell, CellBehavior, CellBorder, CellContent, Flagged, Mine, Wall, systems::get_cursor_position}, grid::Grid, player::Player};

pub struct RevealCellPlugin;
impl Plugin for RevealCellPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_message::<RevealCell>()
            .add_message::<UpdateSprite>()
            .add_systems(Update, (update_sprite, reveal_cell, handle_reveal_click, player_reveals_cells))
        ;
    }
}

#[derive(Message)]
struct RevealCell {
    pub x: i32,
    pub y: i32
}

#[derive(Message)]
struct UpdateSprite {
    pub entity: Entity
}


/// Reads messages from UpdateSprite.
/// Updates the visual of that cell to match the revealed version.
/// Ie: will display the correct number for neighboring mines.
/// If there are 0 neighboring mines, the border and content sprites are despawned.
fn update_sprite(
    mut reader: MessageReader<UpdateSprite>,
    asset_server: Res<AssetServer>,
    
    cells: Query<&Air>,
    children: Query<&Children>,
    mut content_sprites: Query<(&mut Visibility, &mut Sprite), (With<CellContent>, Without<CellBorder>)>,
    mut border_sprites: Query<(&mut Visibility, &mut Sprite), (With<CellBorder>, Without<CellContent>)>
) { 
    for UpdateSprite { entity} in reader.read() {
        // Make sure the entity has the air component (only air cells can be revealed)  
        let Ok(air) = cells.get(*entity) else { panic!("UpdateSprite message sent with invalid entity (Did not have the air componnet).") };

        // Get children of the entity
        let Ok(children) = children.get(*entity) else { panic!("UpdateSprite message sent with an invalid cell entity (Had no children).") };
        for &child in children {
            // Is content sprite
            if let Ok((mut visibility, mut sprite)) = content_sprites.get_mut(child) {
                air.update_content(&mut sprite, &mut visibility, &asset_server);
            }

            // Is border sprite
            if let Ok((mut visibility, mut sprite)) = border_sprites.get_mut(child) {
                air.update_border(&mut sprite, &mut visibility, &asset_server);
            }
        }
    }
}

/// Reads messages from RevealCell. 
/// If the cell is an air cell, reveals that cell. If the cell has 0 neighbors, reveals all neighboring cells.
/// Sends the UpdateSprite message, which will update the visual look of every revealed cell.
fn reveal_cell(
    grid: Res<Grid>,
    mut reader: MessageReader<RevealCell>,
    mut writer: MessageWriter<UpdateSprite>,

    mut cells: Query<(Option<&mut Air>, Option<&Mine>, Option<&Wall>, Option<&Flagged>)>,
) {

    let mut queue = Vec::new();

    for RevealCell { x, y } in reader.read() {
        queue.push((*x, *y));
    }  

    let mut i = 0;
    while i < queue.len() {
        let (x, y) = queue[i];
        i += 1;

        let Some(entity) = grid.get(x, y) else { continue };
        let (air, mine, wall, flagged) = match cells.get_mut(entity) {
            Ok(cell) => cell,
            Err(_) => continue
        };

        // A flagged cell can't be revealed || A wall cell can't be revealed
        if flagged.is_some() || wall.is_some() {
            continue;
        }

        // Handle air cell
        if let Some(mut air) = air {
            if air.revealed { continue }; // Cannot reveal a cell twice.

            air.revealed = true;
            writer.write(UpdateSprite { entity });

            if air.neighbor_mines == 0 {
                // Reveal neighhbors
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        if dx == 0 && dy == 0 { continue };
                        let rx = x as i32 + dx;
                        let ry = y as i32 + dy;

                        // Out of bounds
                        if rx < 0 || ry < 0 || rx >= grid.width() as i32 || ry >= grid.height() as i32 { continue; }

                        // Queue the neighbors
                        queue.push((rx, ry));
                    }
                }
            }

            continue;
        }

        // Handle mine cell
        if mine.is_some() {
            println!("You revealed a mine - game over.");
        }        
    }
}

fn handle_reveal_click(
    mut events: MessageWriter<RevealCell>,
    grid: Res<Grid>,
    input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
) {
    if !input.just_pressed(MouseButton::Left) {
        return;
    }

    let Some(world_pos) = get_cursor_position(windows, camera_q) else { return; };
    let block_pos = grid.cell_from_world(world_pos);
    events.write(RevealCell { x: block_pos.x, y: block_pos.y });
}

fn player_reveals_cells(
    players: Query<&Transform, With<Player>>,
    grid: Res<Grid>,
    mut writer: MessageWriter<RevealCell>
) {
    let player = players.single().unwrap();
    for (x, y) in Cell::touched_by(player, &grid) {
        writer.write(RevealCell { x, y });
    }
}