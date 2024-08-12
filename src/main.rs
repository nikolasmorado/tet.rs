pub const WIDTH: usize = 10;
pub const HEIGHT: usize = 20;

pub const RED_TILE: &str = "\x1b[31m██\x1b[0m";
pub const GREEN_TILE: &str = "\x1b[32m██\x1b[0m";
pub const YELLOW_TILE: &str = "\x1b[33m██\x1b[0m";
pub const BLUE_TILE: &str = "\x1b[34m██\x1b[0m";
pub const CYAN_TILE: &str = "\x1b[36m██\x1b[0m";
pub const ORANGE_TILE: &str = "\x1b[33m██\x1b[0m";
pub const MAGENTA_TILE: &str = "\x1b[35m██\x1b[0m";

type Shape = [[bool; 4]; 4];

#[derive(Copy, Clone)]
pub enum Status {
    Empty,
    FillType(TetronimoType),
}

pub struct Board {
    width: usize,
    height: usize,
    tiles: Vec<Vec<Status>>,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        let tiles = vec![vec![Status::Empty; width]; height];

        Board {
            width,
            height,
            tiles,
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

#[derive(Copy, Clone)]
pub enum TetronimoType {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

pub struct Tetronimo {
    shape: Shape,
    tr_type: TetronimoType,
}

impl Tetronimo {
    pub fn new(mino: TetronimoType) -> Self {
        match mino {
            TetronimoType::I => Tetronimo {
                shape: I_MINO,
                tr_type: TetronimoType::I,
            },
            TetronimoType::O => Tetronimo {
                shape: O_MINO,
                tr_type: TetronimoType::O,
            },
            TetronimoType::T => Tetronimo {
                shape: T_MINO,
                tr_type: TetronimoType::T,
            },
            TetronimoType::S => Tetronimo {
                shape: S_MINO,
                tr_type: TetronimoType::S,
            },
            TetronimoType::Z => Tetronimo {
                shape: Z_MINO,
                tr_type: TetronimoType::Z,
            },
            TetronimoType::J => Tetronimo {
                shape: J_MINO,
                tr_type: TetronimoType::J,
            },
            TetronimoType::L => Tetronimo {
                shape: L_MINO,
                tr_type: TetronimoType::L,
            },
        }
    }

    pub fn draw(&self) {
        let color = get_tile_color(self.tr_type);

        for (y, row) in self.shape.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if cell {
                    // Move the cursor to the correct position and print the block
                    print!("\x1b[{};{}H{}", y, x * 2 + 1, color);
                }
            }
        }
    }
}

fn get_tile_color(mino: TetronimoType) -> &'static str {
    match mino {
        TetronimoType::I => CYAN_TILE,
        TetronimoType::O => YELLOW_TILE,
        TetronimoType::T => MAGENTA_TILE,
        TetronimoType::S => GREEN_TILE,
        TetronimoType::Z => RED_TILE,
        TetronimoType::J => BLUE_TILE,
        TetronimoType::L => ORANGE_TILE,
    }
}

fn main() {
    let board = Board::new(WIDTH, HEIGHT);

    loop {
        println!("\x1b[2J\x1b[H\x1b[?25l");
        println!("\x1b[H");

        for y in 0..board.height + 1 {
            for x in 0..board.width + 2 {
                if x == 0 && y == board.height {
                    print!("└");
                } else if x == board.width + 1 && y == board.height {
                    print!("┘");
                } else if x == 0 || x == board.width + 1 {
                    print!("|");
                } else if y == board.height {
                    print!("--");
                } else {
                    match board.tiles[y][x - 1] {
                        Status::Empty => print!("  "),
                        Status::FillType(mino) => print!("{}", get_tile_color(mino)),
                    }
                }
            }
            println!();
        }

        std::thread::sleep(std::time::Duration::from_millis(1000 / 60));
    }
}

