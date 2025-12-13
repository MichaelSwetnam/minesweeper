use bevy::prelude::*;

#[derive(Resource)]
pub struct CellGridResource {
    width: u32,
    height: u32,
    cell_size: u32,
    gap: u32
}

pub struct CellPlugin;
impl Plugin for CellPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(CellGridResource {
                width: 10,
                height: 8,
                cell_size: 16,
                gap: 4
            })
            .add_systems(Startup, spawn_grid)
            .add_systems(Update, toggle_flag);
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

fn spawn_grid(
    asset_server: Res<AssetServer>,
    grid: Res<CellGridResource>,
    mut commands: Commands,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let border_texture = asset_server.load("cell_border.png");
    let flag_texture = asset_server.load("flag.png");

    let layout = TextureAtlasLayout::from_grid(UVec2::new(16, 16), 1, 1, None, None);
    let layout_handle: Handle<TextureAtlasLayout> = layouts.add(layout);

    let grid_x = grid.width;
    let grid_y = grid.height;
    let step = grid.cell_size as f32 + grid.gap as f32;

    // Centering offsets
    let offset_x = (grid_x as f32 * step) / 2.0;
    let offset_y = (grid_y as f32 * step) / 2.0;

    for y in 0..grid_y {
        for x in 0..grid_x {
            let transform_x = x as f32 * step - offset_x;
            let transform_y = y as f32 * step - offset_y;

            // 👇 Parent gets the grid position
            commands.spawn((
                Cell {
                    x,
                    y,
                    has_mine: false,
                    revealed: false,
                    flagged: false,
                    neighbor_mines: 0,
                },
                Transform::from_translation(Vec3::new(transform_x, transform_y, 0.0)),
                Visibility::Visible,
            ))
            .with_children(|parent| {
                // 👇 Children just sit at parent origin
                parent.spawn((
                    Sprite {
                        image: border_texture.clone(),
                        texture_atlas: Some(layout_handle.clone().into()),
                        ..Default::default()
                    },
                    Transform::default(),
                    Visibility::Visible,
                    CellBorder,
                ));

                parent.spawn((
                    Sprite {
                        image: flag_texture.clone(),
                        texture_atlas: Some(layout_handle.clone().into()),
                        ..Default::default()
                    },
                    Transform::default(),
                    Visibility::Hidden,
                    CellContent,
                ));
            });
        }
    }
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

    mut cells: Query<(&mut Cell, Option<&Children>)>,
    mut content_sprites: Query<&mut Visibility, With<CellContent>>,
) {
    if !input.just_pressed(MouseButton::Right) {
        return;
    }

    let Some(world_pos) = get_cursor_position(windows, camera_q) else { return; };
    let Some((cx, cy)) = world_to_cell(world_pos, &grid) else { return; };

    for (mut cell, children) in &mut cells {
        if cell.x != cx || cell.y != cy { continue; }
        let Some(children) = children else { continue; };

        cell.flagged = !cell.flagged;

        // Get content sprite
        for child in children.iter() {
            let Ok(mut visibility) = content_sprites.get_mut(child) else { continue; };

            *visibility = if cell.flagged {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        }
    }
}