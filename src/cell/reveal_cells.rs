use bevy::prelude::*;
use crate::{cell::{toggle_flag::{get_cursor_position, world_to_cell}, *}, player::Player};

pub struct RevealCellPlugin;
impl Plugin for RevealCellPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_message::<RevealCell>()
            .add_message::<UpdateSprite>()
            .add_systems(Update, (update_sprite, reveal_cell, handle_reveal_click, player_collision_reveals_mine))
        ;
    }
}

#[derive(Message)]
struct RevealCell {
    pub x: u32,
    pub y: u32
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
    mut commands: Commands,
    mut reader: MessageReader<UpdateSprite>,
    asset_server: Res<AssetServer>,
    
    cells: Query<&Air>,
    children: Query<&Children>,
    mut content_sprites: Query<(&mut Visibility, &mut Sprite), With<CellContent>>,
    mut border_sprites: Query<(), With<CellBorder>>
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
    
    for UpdateSprite { entity} in reader.read() {
        // Make sure the entity has the air component (only air cells can be revealed)  
        let Ok(&Air { neighbor_mines, .. }) = cells.get(*entity) else { panic!("UpdateSprite message sent with invalid entity (Did not have the air componnet).") };

        // Get children of the entity
        let Ok(children) = children.get(*entity) else { panic!("UpdateSprite message sent with an invalid cell entity (Had no children).") };
        for &child in children {
            // Is content sprite
            if let Ok((mut visibility, mut sprite)) = content_sprites.get_mut(child) {
                if neighbor_mines == 0 {
                    commands.entity(child).despawn();
                } else {
                    *visibility = Visibility::Visible;
                    sprite.image = textures[neighbor_mines as usize - 1].clone();
                    sprite.color = texture_colors[neighbor_mines as usize - 1];
                }
            }

            // Is border sprite
            if let Ok(_) = border_sprites.get_mut(child) {
                if neighbor_mines == 0 {
                    commands.entity(child).despawn();
                }
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

    mut cells: Query<(Option<&mut Air>, Option<&Mine>, Option<&Wall>, Option<&Flagged>), With<Cell>>,
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
                        queue.push((rx as u32, ry as u32));
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

fn player_collision_reveals_mine(
    players: Query<&Transform, With<Player>>,
    // grid: Res<Grid>,
    // mut writer: MessageWriter<RevealCell>
) {
    let transform = players.single().unwrap();
    println!("Player Position: {}, {}", transform.translation.x, transform.translation.y);
    
    // let Some((x, y)) = world_to_cell(transform.translation.xy(), &grid) else { panic!("Player went out of bounds.") };
    
    // let x = x as i32;
    // let y = y as i32;

    // let scale = transform.scale.xy();
    // println!("{}, {}", scale.x, scale.y);

    // for dx in 0..(scale.y / grid.cell_size as f32) as i32 {
    //     for dy in 0..(scale.x / grid.cell_size as f32) as i32 {

    //         if dx + x >= grid.width as i32 || dx + x < 0 { continue; }
    //         if dy + y >= grid.width as i32 || dy + y < 0 { continue; }


    //         writer.write(RevealCell { x: (dx + x) as u32, y: (dy + y) as u32 });
    //     }
    // }
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
    let Some((cx, cy)) = world_to_cell(world_pos, &grid) else { return; };
    events.write(RevealCell { x: cx, y: cy });
}