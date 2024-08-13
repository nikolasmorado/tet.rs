pub type Shape = [[bool; 4]; 4];

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum TetrominoType {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

#[derive(Copy, Clone, PartialEq)]
pub enum GhostType {
    I_G,
    O_G,
    T_G,
    S_G,
    Z_G,
    J_G,
    L_G,
}

#[derive(Clone, PartialEq)]
pub struct Tetromino {
    pub shape: Shape,
    pub tr_type: TetrominoType,
}

impl Tetromino {
    pub fn new(mino: TetrominoType) -> Self {
        match mino {
            TetrominoType::I => Tetromino {
                shape: I_MINO,
                tr_type: TetrominoType::I,
            },
            TetrominoType::O => Tetromino {
                shape: O_MINO,
                tr_type: TetrominoType::O,
            },
            TetrominoType::T => Tetromino {
                shape: T_MINO,
                tr_type: TetrominoType::T,
            },
            TetrominoType::S => Tetromino {
                shape: S_MINO,
                tr_type: TetrominoType::S,
            },
            TetrominoType::Z => Tetromino {
                shape: Z_MINO,
                tr_type: TetrominoType::Z,
            },
            TetrominoType::J => Tetromino {
                shape: J_MINO,
                tr_type: TetrominoType::J,
            },
            TetrominoType::L => Tetromino {
                shape: L_MINO,
                tr_type: TetrominoType::L,
            },
        }
    }
}

pub const I_MINO: [[bool; 4]; 4] = [
    [false, false, false, false],
    [true, true, true, true],
    [false, false, false, false],
    [false, false, false, false],
];

pub const O_MINO: [[bool; 4]; 4] = [
    [false, false, false, false],
    [false, true, true, false],
    [false, true, true, false],
    [false, false, false, false],
];

pub const T_MINO: [[bool; 4]; 4] = [
    [false, false, false, false],
    [false, true, false, false],
    [true, true, true, false],
    [false, false, false, false],
];

pub const S_MINO: [[bool; 4]; 4] = [
    [false, false, false, false],
    [false, true, true, false],
    [true, true, false, false],
    [false, false, false, false],
];

pub const Z_MINO: [[bool; 4]; 4] = [
    [false, false, false, false],
    [true, true, false, false],
    [false, true, true, false],
    [false, false, false, false],
];

pub const J_MINO: [[bool; 4]; 4] = [
    [false, false, false, false],
    [true, false, false, false],
    [true, true, true, false],
    [false, false, false, false],
];

pub const L_MINO: [[bool; 4]; 4] = [
    [false, false, false, false],
    [false, false, true, false],
    [true, true, true, false],
    [false, false, false, false],
];

