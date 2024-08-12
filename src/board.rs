use crate::mino::{Tetromino, TetrominoType};
use crate::util::gen_bag;

#[derive(Copy, Clone, PartialEq)]
pub enum Status {
    Empty,
    FillType(TetrominoType),
}

pub struct Board {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Vec<Status>>,
    active_tetromino: Option<Tetromino>,
    x: i32,
    y: i32,
    upcoming: Vec<TetrominoType>,
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
            upcoming,
        }
    }

    fn new_tetromino(&mut self) {
        let at = Tetromino::new(self.upcoming.remove(0));

        if self.upcoming.len() < 7 {
            self.upcoming.extend(gen_bag());
        }

        self.active_tetromino = Some(at);
        self.x = (self.width / 2 - 2) as i32;
        self.y = 0;
    }

    fn collision_check(&self, offset: (i32, i32)) -> bool {
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

    fn draw(&mut self) {
        self.draw_at(false);
    }

    fn clear(&mut self) {
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

