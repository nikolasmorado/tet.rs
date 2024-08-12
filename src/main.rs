use getch_rs::{Getch, Key};
use std::thread;
use std::time::Duration;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub const WIDTH: usize = 10;
pub const HEIGHT: usize = 20;

pub const RED_TILE: &str = "\x1b[31m██\x1b[0m";
pub const GREEN_TILE: &str = "\x1b[32m██\x1b[0m";
pub const YELLOW_TILE: &str = "\x1b[33m██\x1b[0m";
pub const BLUE_TILE: &str = "\x1b[34m██\x1b[0m";
pub const CYAN_TILE: &str = "\x1b[36m██\x1b[0m";
pub const ORANGE_TILE: &str = "\x1b[33m██\x1b[0m";
pub const MAGENTA_TILE: &str = "\x1b[35m██\x1b[0m";

pub const RED_GHOST: &str = "\x1b[31m░░\x1b[0m";
pub const GREEN_GHOST: &str = "\x1b[32m░░\x1b[0m";
pub const YELLOW_GHOST: &str = "\x1b[33m░░\x1b[0m";
pub const BLUE_GHOST: &str = "\x1b[34m░░\x1b[0m";
pub const CYAN_GHOST: &str = "\x1b[36m░░\x1b[0m";
pub const ORANGE_GHOST: &str = "\x1b[33m░░\x1b[0m";
pub const MAGENTA_GHOST: &str = "\x1b[35m░░\x1b[0m";

type Shape = [[bool; 4]; 4];

fn gen_bag() -> Vec<TetrominoType>{
    let mut bag = vec![
        TetrominoType::I,
        TetrominoType::O,
        TetrominoType::T,
        TetrominoType::S,
        TetrominoType::Z,
        TetrominoType::J,
        TetrominoType::L,
    ];

    let mut rng = thread_rng();

    bag.shuffle(&mut rng);
    bag
}

#[derive(Copy, Clone, PartialEq)]
pub enum Status {
    Empty,
    FillType(TetrominoType),
}

pub struct Board {
    width: usize,
    height: usize,
    tiles: Vec<Vec<Status>>,
    active_tetromino: Option<Tetromino>,
    x: i32,
    y: i32,
    upcoming: Vec<TetrominoType>
}

impl Board {
    pub fn new(dims: (usize, usize)) -> Self {
        let tiles = vec![vec![Status::Empty; dims.0]; dims.1];

        let mut upcoming = gen_bag();
        upcoming.extend(gen_bag());

        let at = Tetromino::new(upcoming.remove(0));

        Board {
            width: dims.0,
            height: dims.1,
            tiles,
            active_tetromino: Some(at),
            x: (dims.0 / 2 - 2) as i32,
            y: 0,
            upcoming
        }
    }


    pub fn new_tetromino(&mut self) {
        let at = Tetromino::new(self.upcoming.remove(0));
    
        if self.upcoming.len() < 7 {
            self.upcoming.extend(gen_bag());
        }

        self.active_tetromino = Some(at);
        self.x = (self.width / 2 - 2) as i32;
        self.y = 0;
    }

    pub fn collision_check(&self, offset: (i32, i32)) -> bool {
        for y in 0..4 {
            for x in 0..4 {
                if self.active_tetromino.as_ref().unwrap().shape[y][x] {
                    if self.y as i32 + y as i32 + offset.1 >= self.height as i32
                        || self.x as i32 + x as i32 + offset.0 < 0
                        || self.x as i32 + x as i32 + offset.0 >= self.width as i32
                    {
                        return true;
                    }

                    if self.tiles[(self.y + y as i32 + offset.1) as usize]
                        [(self.x + x as i32 + offset.0) as usize]
                        != Status::Empty
                    {
                        return true;
                    }

                } else {
                    continue;
                }
            }
        }

        false
    }

    pub fn hard_drop(&mut self) {
        self.clear();

        loop {
            if self.collision_check((0, 1)) {
                break;
            }
            self.y += 1;
        }
        self.draw();
        self.new_tetromino();
    }

    pub fn draw(&mut self) {
        self.draw_at(false);
    }

    pub fn clear(&mut self) {
        self.draw_at(true);
    }

    pub fn move_tetromino(&mut self, offset: (i32, i32)) {
        self.clear();
        if !self.collision_check(offset) {
            self.x = self.x + offset.0;
            self.y = self.y + offset.1;
        }
        self.draw();
    }

    pub fn draw_at(&mut self, del: bool) {
        if let Some(tetromino) = &self.active_tetromino {
            for row in 0..4 {
                for col in 0..4 {
                    if tetromino.shape[row][col] {
                        if del {
                            self.tiles[(row as i32 + self.y) as usize]
                                [(col as i32 + self.x) as usize] = Status::Empty
                        } else {
                            self.tiles[(row as i32 + self.y) as usize]
                                [(col as i32 + self.x) as usize] =
                                Status::FillType(tetromino.tr_type)
                        }
                    }
                }
            }
        }
    }
}

pub const I_MINO: [[bool; 4]; 4] = [
    [true, true, true, true],
    [false, false, false, false],
    [false, false, false, false],
    [false, false, false, false],
];

pub const O_MINO: [[bool; 4]; 4] = [
    [false, true, true, false],
    [false, true, true, false],
    [false, false, false, false],
    [false, false, false, false],
];

pub const T_MINO: [[bool; 4]; 4] = [
    [false, true, false, false],
    [true, true, true, false],
    [false, false, false, false],
    [false, false, false, false],
];

pub const S_MINO: [[bool; 4]; 4] = [
    [false, true, true, false],
    [true, true, false, false],
    [false, false, false, false],
    [false, false, false, false],
];

pub const Z_MINO: [[bool; 4]; 4] = [
    [true, true, false, false],
    [false, true, true, false],
    [false, false, false, false],
    [false, false, false, false],
];

pub const J_MINO: [[bool; 4]; 4] = [
    [true, false, false, false],
    [true, true, true, false],
    [false, false, false, false],
    [false, false, false, false],
];

pub const L_MINO: [[bool; 4]; 4] = [
    [false, false, true, false],
    [true, true, true, false],
    [false, false, false, false],
    [false, false, false, false],
];

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

#[derive(Clone, PartialEq)]
pub struct Tetromino {
    shape: Shape,
    tr_type: TetrominoType,
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

fn get_tile_color(mino: TetrominoType) -> &'static str {
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

fn get_ghost_color(mino: TetrominoType) -> &'static str {
    match mino {
        TetrominoType::I => CYAN_GHOST,
        TetrominoType::O => YELLOW_GHOST,
        TetrominoType::T => MAGENTA_GHOST,
        TetrominoType::S => GREEN_GHOST,
        TetrominoType::Z => RED_GHOST,
        TetrominoType::J => BLUE_GHOST,
        TetrominoType::L => ORANGE_GHOST,
    }
}

fn main() {
    let mut board = Board::new((WIDTH, HEIGHT));
    let g = Getch::new();

    loop {
        // Drawing loop
        println!("\x1b[2J\x1b[H\x1b[?25l");
        println!("\x1b[H");

        board.draw_at(false);

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

        // Input loop

        match g.getch() {
            Ok(Key::Ctrl('c')) => break,
            Ok(Key::Char(' ')) => {
                board.hard_drop();
            }
            Ok(Key::Right) => {
                board.move_tetromino((1, 0));
            }
            Ok(Key::Left) => {
                board.move_tetromino((-1, 0));
            }
            Ok(Key::Down) => {
                board.move_tetromino((0, 1));
            }
            Ok(_) => (),
            Err(_) => break,
        }

        thread::sleep(Duration::from_millis(1000 / 60));
    }
}
