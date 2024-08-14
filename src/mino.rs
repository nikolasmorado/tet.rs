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
pub enum PieceData {
    Small([[[bool; 3]; 3]; 4]),
    Medium([[[bool; 4]; 3]; 4]),
    Large([[[bool; 4]; 4]; 4]),
}

#[derive(Copy, Clone, PartialEq)]
pub enum GhostType {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

#[derive(Clone, PartialEq)]
pub struct Tetromino {
    pub tr_type: TetrominoType,
    pub piece_data: PieceData,
    pub orientation: usize,
}

impl Tetromino {
    pub fn new(mino: TetrominoType) -> Self {
        match mino {
            TetrominoType::I => Tetromino {
                tr_type: TetrominoType::I,
                piece_data: PieceData::Large(I_MINO_DATA),
                orientation: 0,
            },
            TetrominoType::O => Tetromino {
                tr_type: TetrominoType::O,
                piece_data: PieceData::Medium(O_MINO_DATA),
                orientation: 0,
            },
            TetrominoType::T => Tetromino {
                tr_type: TetrominoType::T,
                piece_data: PieceData::Small(T_MINO_DATA),
                orientation: 0,
            },
            TetrominoType::S => Tetromino {
                tr_type: TetrominoType::S,
                piece_data: PieceData::Small(S_MINO_DATA),
                orientation: 0,
            },
            TetrominoType::Z => Tetromino {
                tr_type: TetrominoType::Z,
                piece_data: PieceData::Small(Z_MINO_DATA),
                orientation: 0,
            },
            TetrominoType::J => Tetromino {
                tr_type: TetrominoType::J,
                piece_data: PieceData::Small(J_MINO_DATA),
                orientation: 0,
            },
            TetrominoType::L => Tetromino {
                tr_type: TetrominoType::L,
                piece_data: PieceData::Small(L_MINO_DATA),
                orientation: 0,
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

pub const I_MINO_DATA: [[[bool; 4]; 4]; 4] = [
    [
        [false, false, false, false],
        [true, true, true, true],
        [false, false, false, false],
        [false, false, false, false],
    ],
    [
        [false, false, true, false],
        [false, false, true, false],
        [false, false, true, false],
        [false, false, true, false],
    ],
    [
        [false, false, false, false],
        [false, false, false, false],
        [true, true, true, true],
        [false, false, false, false],
    ],
    [
        [false, true, false, false],
        [false, true, false, false],
        [false, true, false, false],
        [false, true, false, false],
    ],
];

pub const J_MINO_DATA: [[[bool; 3]; 3]; 4] = [
    [
        [true, false, false],
        [true, true, true],
        [false, false, false],
    ],
    [
        [false, true, true],
        [false, true, false],
        [false, true, false],
    ],
    [
        [false, false, false],
        [true, true, true],
        [false, false, true],
    ],
    [
        [false, true, false],
        [false, true, false],
        [true, true, false],
    ],
];

pub const L_MINO_DATA: [[[bool; 3]; 3]; 4] = [
    [
        [false, false, true],
        [true, true, true],
        [false, false, false],
    ],
    [
        [false, true, false],
        [false, true, false],
        [false, true, true],
    ],
    [
        [false, false, false],
        [true, true, true],
        [true, false, false],
    ],
    [
        [true, true, false],
        [false, true, false],
        [false, true, false],
    ],
];

pub const O_MINO_DATA: [[[bool; 4]; 3]; 4] = [
    [
        [false, true, true, false],
        [false, true, true, false],
        [false, false, false, false],
    ],
    [
        [false, true, true, false],
        [false, true, true, false],
        [false, false, false, false],
    ],
    [
        [false, true, true, false],
        [false, true, true, false],
        [false, false, false, false],
    ],
    [
        [false, true, true, false],
        [false, true, true, false],
        [false, false, false, false],
    ],
];

pub const S_MINO_DATA: [[[bool; 3]; 3]; 4] = [
    [
        [false, true, true],
        [true, true, false],
        [false, false, false],
    ],
    [
        [false, true, false],
        [false, true, true],
        [false, false, true],
    ],
    [
        [false, false, false],
        [false, true, true],
        [true, true, false],
    ],
    [
        [true, false, false],
        [true, true, false],
        [false, true, false],
    ],
];

pub const T_MINO_DATA: [[[bool; 3]; 3]; 4] = [
    [
        [false, true, false],
        [true, true, true],
        [false, false, false],
    ],
    [
        [false, true, false],
        [false, true, true],
        [false, true, false],
    ],
    [
        [false, false, false],
        [true, true, true],
        [false, true, false],
    ],
    [
        [false, true, false],
        [true, true, false],
        [false, true, false],
    ],
];

pub const Z_MINO_DATA: [[[bool; 3]; 3]; 4] = [
    [
        [true, true, false],
        [false, true, true],
        [false, false, false],
    ],
    [
        [false, false, true],
        [false, true, true],
        [false, true, false],
    ],
    [
        [false, false, false],
        [true, true, false],
        [false, true, true],
    ],
    [
        [false, true, false],
        [true, true, false],
        [true, false, false],
    ],
];

// https://tetris.wiki/Super_Rotation_System
// We need to invert the y values though
//
// 0->R   ( 0, 0) 	(-1, 0) 	(-1,+1) 	( 0,-2) 	(-1,-2)
// R->0 	( 0, 0) 	(+1, 0) 	(+1,-1) 	( 0,+2) 	(+1,+2)
// R->2 	( 0, 0) 	(+1, 0) 	(+1,-1) 	( 0,+2) 	(+1,+2)
// 2->R 	( 0, 0) 	(-1, 0) 	(-1,+1) 	( 0,-2) 	(-1,-2)
// 2->L 	( 0, 0) 	(+1, 0) 	(+1,+1) 	( 0,-2) 	(+1,-2)
// L->2 	( 0, 0) 	(-1, 0) 	(-1,-1) 	( 0,+2) 	(-1,+2)
// L->0 	( 0, 0) 	(-1, 0) 	(-1,-1) 	( 0,+2) 	(-1,+2)
// 0->L 	( 0, 0) 	(+1, 0) 	(+1,+1) 	( 0,-2) 	(+1,-2)

pub const SMALL_MINO_KICK_TABLE: [[(i8, i8); 5]; 8] = [
    [(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)],
    [(0, 0), (1, 0), (1, 1), (0, 2), (1, -2)],
    [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
    [(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)],
    [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
    [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
    [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
    [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
];

// 0->R 	( 0, 0) 	(-2, 0) 	(+1, 0) 	(-2,-1) 	(+1,+2)
// R->0 	( 0, 0) 	(+2, 0) 	(-1, 0) 	(+2,+1) 	(-1,-2)
// R->2 	( 0, 0) 	(-1, 0) 	(+2, 0) 	(-1,+2) 	(+2,-1)
// 2->R 	( 0, 0) 	(+1, 0) 	(-2, 0) 	(+1,-2) 	(-2,+1)
// 2->L 	( 0, 0) 	(+2, 0) 	(-1, 0) 	(+2,+1) 	(-1,-2)
// L->2 	( 0, 0) 	(-2, 0) 	(+1, 0) 	(-2,-1) 	(+1,+2)
// L->0 	( 0, 0) 	(+1, 0) 	(-2, 0) 	(+1,-2) 	(-2,+1)
// 0->L 	( 0, 0) 	(-1, 0) 	(+2, 0) 	(-1,+2) 	(+2,-1)

pub const LARGE_MINO_KICK_TABLE: [[(i8, i8); 5]; 8] = [
    [(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)],
    [(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)],
    [(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)],
    [(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)],
    [(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)],
    [(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)],
    [(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)],
    [(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)],
];
