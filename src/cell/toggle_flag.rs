use bevy::prelude::*;
use crate::cell::*;

pub fn get_cursor_position(
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

pub fn world_to_cell(pos: Vec2, grid: &Grid) -> (i32, i32) {
    let cell = grid.cell_size() as f32;
    (
        (pos.x / cell).round() as i32,
        (pos.y / cell).round() as i32
    )
}


pub fn toggle_flag(
    grid: Res<Grid>,
    input: Res<ButtonInput<MouseButton>>,
    mut commands: Commands,

    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    children_q: Query<&Children>,
    mut cells: Query<(Entity, Option<&Air>, Option<&Wall>, Option<&Flagged>)>,
    mut content_sprites: Query<&mut Visibility, With<CellContent>>,
) {
    if !input.just_pressed(MouseButton::Right) {
        return;
    }

    let Some(world_pos) = get_cursor_position(windows, camera_q) else { return };
    let (cx, cy) = world_to_cell(world_pos, &grid);
    let Some(cell_entity) = grid.get(cx, cy) else { return };

    let (entity, air, wall, flagged) = match cells.get_mut(cell_entity) {
        Ok(cell) => cell,
        Err(_) => return, // entity has no Cell component
    };

    // Wall cannot be flagged.
    if wall.is_some() { return }
    // Revealed cells cannot be flagged.
    if let Some(Air { revealed, .. }) = air && *revealed { return }

    // Add / remove the flagged component
    if flagged.is_some() {
        commands.entity(entity).remove::<Flagged>();
    } else {
        commands.entity(entity).insert(Flagged);
    }

    // Get the children of the entity
    let children = match children_q.get(cell_entity) {
        Ok(children) => children,
        Err(_) => return, // entity has no children
    };

    // Toggle visibility of the child sprites
    for child in children.iter() {
        if let Ok(mut visibility) = content_sprites.get_mut(child) {
            *visibility = if !flagged.is_some() {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        }
    }
}