use strum_macros::AsRefStr;

#[derive(AsRefStr)]
#[allow(non_camel_case_types)]
pub enum EnvVariable {
    CHUNK_WIDTH,
    CHUNK_HEIGHT,
    CELL_SIZE,
    CELL_SCALE,
    PLAYER_SPEED
}