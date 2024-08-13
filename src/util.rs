use crate::constants::{
    BLUE_GHOST, BLUE_TILE, CYAN_GHOST, CYAN_TILE, GREEN_GHOST, GREEN_TILE, MAGENTA_GHOST,
    MAGENTA_TILE, ORANGE_GHOST, ORANGE_TILE, RED_GHOST, RED_TILE, YELLOW_GHOST, YELLOW_TILE,
};
use crate::mino::{GhostType, TetrominoType};
use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn gen_bag() -> Vec<TetrominoType> {
    let mut bag = vec![
        TetrominoType::I,
        TetrominoType::O,
        TetrominoType::T,
        TetrominoType::S,
        TetrominoType::Z,
        TetrominoType::J,
        TetrominoType::L,
    ];

    // let mut bag = vec![TetrominoType::I; 7];

    let mut rng = thread_rng();

    bag.shuffle(&mut rng);
    bag
}

pub fn get_tile_color(mino: TetrominoType) -> &'static str {
    match mino {
        TetrominoType::I => CYAN_TILE,
        TetrominoType::O => YELLOW_TILE,
        TetrominoType::T => MAGENTA_TILE,
        TetrominoType::S => GREEN_TILE,
        TetrominoType::Z => RED_TILE,
        TetrominoType::J => BLUE_TILE,
        TetrominoType::L => ORANGE_TILE,
    }
}

pub fn get_ghost_color(mino: GhostType) -> &'static str {
    match mino {
        GhostType::I_G => CYAN_GHOST,
        GhostType::O_G => YELLOW_GHOST,
        GhostType::T_G => MAGENTA_GHOST,
        GhostType::S_G => GREEN_GHOST,
        GhostType::Z_G => RED_GHOST,
        GhostType::J_G => BLUE_GHOST,
        GhostType::L_G => ORANGE_GHOST,
    }
}

pub fn mino_to_ghost(mino: TetrominoType) -> GhostType {
    match mino {
        TetrominoType::I => GhostType::I_G,
        TetrominoType::O => GhostType::O_G,
        TetrominoType::T => GhostType::T_G,
        TetrominoType::S => GhostType::S_G,
        TetrominoType::Z => GhostType::Z_G,
        TetrominoType::J => GhostType::J_G,
        TetrominoType::L => GhostType::L_G,
    }
}
