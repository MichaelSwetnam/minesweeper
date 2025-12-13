use bevy::prelude::*;
use bevy::input::mouse::MouseWheel;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_grid)
        .add_systems(Update, scroll_events)
        .run();
}

#[derive(Component)]
struct Camera;

fn spawn_camera(
    mut commands: Commands
) {
    commands.spawn((Camera2d, Camera));
}

fn spawn_grid(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("cell.png");

    let layout = TextureAtlasLayout::from_grid(UVec2::new(16, 16), 1, 1, None, None);
    let layout_handle: Handle<TextureAtlasLayout> = layouts.add(layout);

    let grid_x = 10;
    let grid_y = 8;

    let cell_size = 16.0;
    let gap = 4.0;
    let step = cell_size + gap;

    // Centering offsets
    let offset_x = (grid_x as f32 * step) / 2.0;
    let offset_y = (grid_y as f32 * step) / 2.0;

    // Spawn cells: AtlasSprite + texture handle + layout handle + transform
    for y in 0..grid_y {
        for x in 0..grid_x {
            commands.spawn((
                Sprite {
                    // tell Bevy this sprite uses an atlas
                    image: texture.clone(),
                    texture_atlas: Some(layout_handle.clone().into()),
                    ..Default::default()
                },
                Transform::from_translation(Vec3::new(
                    x as f32 * step - offset_x,
                    y as f32 * step - offset_y,
                    0.0,
                )),
                Visibility::Visible,
            ));
        }
    }
}


fn scroll_events(
    mut evr_scroll: MessageReader<MouseWheel>,
) {
    use bevy::input::mouse::MouseScrollUnit;
    for ev in evr_scroll.read() {
        match ev.unit {
            MouseScrollUnit::Line => {
                println!("Scroll (line units): vertical: {}, horizontal: {}", ev.y, ev.x);
            }
            MouseScrollUnit::Pixel => {
                println!("Scroll (pixel units): vertical: {}, horizontal: {}", ev.y, ev.x);
            }
        }
    }
}