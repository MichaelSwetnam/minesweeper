use bevy::prelude::*;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_player)
            .add_systems(Update, move_player)
        ;
    }
}

#[derive(Component)]
#[require(Transform)]
pub struct Player;
impl Player {
    fn new() -> impl Bundle {
        (
            Player,
            Sprite {
                color: Color::WHITE,
                ..Default::default()
            },
            Transform::with_scale(Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)), Vec3::new(8.0, 16.0, 8.0)),
            Visibility::Visible
        )
    }
}

fn spawn_player(
    mut commands: Commands
) {
    commands.spawn(Player::new());
}


fn move_player(
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    mut players: Query<&mut Transform, (With<Player>, Without<Camera>)>
) {
    let mut player_transform = players.single_mut().unwrap();
    let mut camera_transform = query.single_mut().unwrap();

    // Movement speed in world units per second
    let base_speed = 40.0; // Pixels

    // Speed relative to zoom level
    let speed = base_speed * time.delta_secs();

    if keyboard.pressed(KeyCode::ArrowLeft) || keyboard.pressed(KeyCode::KeyA) {
        camera_transform.translation.x -= speed;
        player_transform.translation.x -= speed;
    }
    if keyboard.pressed(KeyCode::ArrowRight) || keyboard.pressed(KeyCode::KeyD) {
        camera_transform.translation.x += speed;
        player_transform.translation.x += speed;
    }
    if keyboard.pressed(KeyCode::ArrowUp) || keyboard.pressed(KeyCode::KeyW) {
        camera_transform.translation.y += speed;
        player_transform.translation.y += speed;
    }
    if keyboard.pressed(KeyCode::ArrowDown) || keyboard.pressed(KeyCode::KeyS) {
        camera_transform.translation.y -= speed;
        player_transform.translation.y -= speed;
    }
}