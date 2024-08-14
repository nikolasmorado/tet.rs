use std::time::{Duration, Instant};

use crate::mino::{GhostType, Tetromino, TetrominoType, PieceData};
use crate::util::{gen_bag, mino_to_ghost};

#[derive(Copy, Clone, PartialEq)]
pub enum Status {
    Empty,
    FillType(TetrominoType),
    FillGhost(GhostType),
}

pub struct Board {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Vec<Status>>,
    active_tetromino: Option<Tetromino>,
    x: i32,
    y: i32,
    upcoming: Vec<TetrominoType>,
    held_piece: Option<TetrominoType>,
    held: bool,
    gravity_timer: Instant,
    gravity_interval: Duration,
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
            held_piece: None,
            held: false,
            gravity_timer: Instant::now(),
            gravity_interval: Duration::from_millis(1000),
        }
    }

    fn check_loss(&mut self) -> bool {
        if let Some(ref mino) = self.active_tetromino {
            if self.y == 0 && self.collision_check(mino, (0, 0)) {
                return true;
            }
        }
        false
    }

    fn clear_lines(&mut self) {
        let mut lines = Vec::new();

        for y in 0..self.height {
            let mut full = true;
            for x in 0..self.width {
                if self.tiles[y][x] == Status::Empty {
                    full = false;
                    break;
                }
            }

            if full {
                lines.push(y);
            }
        }

        for line in lines {
            for y in (4..=line).rev() {
                for x in 0..self.width {
                    self.tiles[y][x] = self.tiles[y - 1][x];
                }
            }
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

        if self.check_loss() {
            self.tiles = vec![vec![Status::Empty; self.width]; self.height];
            self.upcoming = gen_bag();
            self.upcoming.extend(gen_bag());
            self.held_piece = None;
            self.held = false;
            self.active_tetromino = Some(Tetromino::new(self.upcoming.remove(0)));
        }
    }

    pub fn rotate_cc(&mut self) {
        let original = self.active_tetromino.as_ref().unwrap().clone();
        let mut mino = self.active_tetromino.as_mut().unwrap().clone();

        mino.orientation = (mino.orientation + 3) % 4;

        self.clear();
        self.active_tetromino = Some(mino.clone());
        if self.collision_check(&mino, (0, 0)) {
            self.active_tetromino = Some(original);
        }
        self.draw();
    }

    pub fn rotate_c(&mut self) {
        let original = self.active_tetromino.as_ref().unwrap().clone();
        let mut mino = self.active_tetromino.as_mut().unwrap().clone();

        mino.orientation = (mino.orientation + 1) % 4;

        self.clear();
        self.active_tetromino = Some(mino.clone());
        if self.collision_check(&mino, (0, 0)) {
            self.active_tetromino = Some(original);
        }
        self.draw();
    }

    fn collision_check(&self, mino: &Tetromino, offset: (i32, i32)) -> bool {

        match mino.piece_data {

            PieceData::Small(data) => {
               for y in 0..3 {
                   for x in 0..3 {
                        if data[mino.orientation][y][x] {
                            if self.y as i32 + y as i32 + offset.1 >= self.height as i32
                                || self.x as i32 + x as i32 + offset.0 < 0
                                || self.x as i32 + x as i32 + offset.0 >= self.width as i32
                            {
                                return true;
                            }

                            match self.tiles[(self.y + y as i32 + offset.1) as usize]
                                [(self.x + x as i32 + offset.0) as usize]
                            {
                                Status::Empty | Status::FillGhost(_) => continue,
                                _ => return true,
                            }
                        } else {
                            continue;
                        }
                        
                   }
               }
            }
            PieceData::Large(data) => {
                for y in 0..5 {
                    for x in 0..5 {
                        if data[mino.orientation][y][x] {
                            if self.y as i32 + y as i32 + offset.1 >= self.height as i32
                                || self.x as i32 + x as i32 + offset.0 < 0
                                || self.x as i32 + x as i32 + offset.0 >= self.width as i32
                            {
                                return true;
                            }

                            match self.tiles[(self.y + y as i32 + offset.1) as usize]
                                [(self.x + x as i32 + offset.0) as usize]
                            {
                                Status::Empty | Status::FillGhost(_) => continue,
                                _ => return true,
                            }
                        } else {
                            continue;
                        }
                    }
                }
            }
        }

        false
    }

    pub fn soft_harddrop(&mut self) {
        if let Some(ref mut mino) = self.active_tetromino.clone() {
            self.clear();

            loop {
                if self.collision_check(mino, (0, 1)) {
                    break;
                }
                self.y += 1;
            }
            self.draw();
        }
    }

    pub fn hard_drop(&mut self) {
        if let Some(ref mut mino) = self.active_tetromino.clone() {
            self.clear();

            loop {
                if self.collision_check(mino, (0, 1)) {
                    break;
                }
                self.y += 1;
            }

            self.held = false;
            self.draw();
            self.clear_lines();
            self.new_tetromino();
        }
    }

    fn draw(&mut self) {
        self.draw_at(false);
    }

    fn clear(&mut self) {
        self.draw_at(true);
    }

    pub fn move_tetromino(&mut self, offset: (i32, i32)) {
        if let Some(ref mut mino) = self.active_tetromino.clone() {
            self.clear();
            if !self.collision_check(mino, offset) {
                self.x = self.x + offset.0;
                self.y = self.y + offset.1;
            }
            self.draw();
        }
    }

    pub fn clear_all_ghosts(&mut self) {
        for row in 0..self.height {
            for col in 0..self.width {
                match self.tiles[row][col] {
                    Status::FillGhost(_) => self.tiles[row][col] = Status::Empty,
                    _ => continue,
                }
            }
        }
    }

    pub fn hold_piece(&mut self) {
        if self.held {
            return;
        }

        self.held = true;

        let mut held = self.held_piece;
        let mut at = self.active_tetromino.clone().unwrap();

        self.clear();

        if let Some(h) = held {
            held = Some(at.tr_type);
            at = Tetromino::new(h);
        } else {
            held = Some(at.tr_type);
            at = Tetromino::new(self.upcoming.remove(0));
        }

        self.held_piece = held;
        self.active_tetromino = Some(at);
        self.x = (self.width / 2 - 2) as i32;
        self.y = 0;
        self.draw();
    }

    pub fn draw_at(&mut self, del: bool) {
        if del {
            self.clear_all_ghosts();
        }
        if let Some(tetromino) = &self.active_tetromino {
            if !del {
                let ghost_mino = tetromino.clone();
                let mut ghost_y = self.y;

                while !self.collision_check(&ghost_mino, (0, 1 + ghost_y - self.y)) {
                    ghost_y += 1;
                }

                match ghost_mino.piece_data {
                    PieceData::Small(data) => {
                        for row in 0..3 {
                            for col in 0..3 {
                                if data[ghost_mino.orientation][row][col] {
                                    if del {
                                        self.tiles[(row as i32 + ghost_y) as usize]
                                            [(col as i32 + self.x) as usize] = Status::Empty
                                    } else {
                                        self.tiles[(row as i32 + ghost_y) as usize]
                                            [(col as i32 + self.x) as usize] =
                                            Status::FillGhost(mino_to_ghost(tetromino.tr_type))
                                    }
                                }
                            }
                        }
                    }
                    PieceData::Large(data) => {
                        for row in 0..5 {
                            for col in 0..5 {
                                if data[ghost_mino.orientation][row][col] {
                                    if del {
                                        self.tiles[(row as i32 + ghost_y) as usize]
                                            [(col as i32 + self.x) as usize] = Status::Empty
                                    } else {
                                        self.tiles[(row as i32 + ghost_y) as usize]
                                            [(col as i32 + self.x) as usize] =
                                            Status::FillGhost(mino_to_ghost(tetromino.tr_type))
                                    }
                                }
                            }
                        }
                    }
                }
            }

            match tetromino.piece_data {
                PieceData::Small(data) => {
                    for row in 0..3 {
                        for col in 0..3 {
                            if data[tetromino.orientation][row][col] {
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
                PieceData::Large(data) => {
                    for row in 0..5 {
                        for col in 0..5 {
                            if data[tetromino.orientation][row][col] {
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
    }

    pub fn apply_gravity(&mut self) {
        if self.gravity_timer.elapsed() >= self.gravity_interval {
            self.gravity_timer = Instant::now();

            self.clear();
            if !self.collision_check(self.active_tetromino.as_ref().unwrap(), (0, 1)) {
                self.y += 1;
            } else {
                self.held = false;
                self.draw();
                self.clear_lines();
                self.new_tetromino();
            }
            self.draw();
        }
    }
}
