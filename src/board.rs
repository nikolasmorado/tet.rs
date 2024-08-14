use std::time::{Duration, Instant};

use crate::mino::{
    GhostType, PieceData, Tetromino, TetrominoType, LARGE_MINO_KICK_TABLE, SMALL_MINO_KICK_TABLE,
};
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
    pub col_buffer: Vec<Vec<bool>>,
    active_tetromino: Option<Tetromino>,
    x: i32,
    y: i32,
    upcoming: Vec<TetrominoType>,
    held_piece: Option<TetrominoType>,
    held: bool,
    gravity_timer: Instant,
    gravity_interval: Duration,
    lock_delay_timer: Option<Instant>,
    lock_delay_interval: Duration,
    lock_delay_max: Duration,
    lock_delay_cur: Duration,
}

impl Board {
    pub fn new(dims: (usize, usize)) -> Self {
        let tiles = vec![vec![Status::Empty; dims.0]; dims.1 + 10];
        let col_buffer = vec![vec![false; dims.0]; dims.1 + 10];

        let mut upcoming = gen_bag();
        upcoming.extend(gen_bag());

        // let at = Tetromino::new(upcoming.remove(0));

        Board {
            width: dims.0,
            height: dims.1 + 10,
            tiles,
            col_buffer,
            active_tetromino: None,
            x: (dims.0 / 2 - 2) as i32,
            y: 10,
            upcoming,
            held_piece: None,
            held: false,
            gravity_timer: Instant::now(),
            gravity_interval: Duration::from_millis(1000),
            lock_delay_timer: None,
            lock_delay_interval: Duration::from_millis(500),
            lock_delay_max: Duration::from_millis(5000),
            lock_delay_cur: Duration::from_millis(500),
        }
    }

    fn check_loss(&mut self) -> bool {
        if let Some(ref mino) = self.active_tetromino {
            if self.y == 10 && self.collision_check_buffer(mino, (0, 0)) {
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
                    self.col_buffer[y][x] = self.col_buffer[y - 1][x];
                }
            }
        }
    }

    pub fn new_tetromino(&mut self) {
        let at = Tetromino::new(self.upcoming.remove(0));

        if self.upcoming.len() < 7 {
            self.upcoming.extend(gen_bag());
        }

        self.x = (self.width / 2 - 2) as i32;
        self.y = 10;
        self.active_tetromino = Some(at);

        if self.check_loss() {
            self.tiles = vec![vec![Status::Empty; self.width]; self.height];
            self.col_buffer = vec![vec![false; self.width]; self.height];
            self.upcoming = gen_bag();
            self.upcoming.extend(gen_bag());
            self.held_piece = None;
            self.held = false;
            self.active_tetromino = Some(Tetromino::new(self.upcoming.remove(0)));
            self.x = (self.width / 2 - 2) as i32;
            self.y = 10;
        } else {
            self.move_tetromino((0, 1));
        }
    }

    pub fn rotate_cc(&mut self) {
        let original = self.active_tetromino.as_ref().unwrap().clone();
        let mut mino = self.active_tetromino.as_mut().unwrap().clone();

        let table_entry = match mino.orientation {
            0 => 7,
            1 => 1,
            2 => 3,
            _ => 5,
        };
        mino.orientation = (mino.orientation + 3) % 4;

        let kick_table = match mino.piece_data {
            PieceData::Small(_) => SMALL_MINO_KICK_TABLE,
            PieceData::Large(_) => LARGE_MINO_KICK_TABLE,
        };

        self.clear();

        let mut pass = false;

        for &(x, y) in &kick_table[table_entry][..5] {
            if !self.collision_check_buffer(&mino, (x as i32, y as i32)) {
                self.x += x as i32;
                self.y += y as i32;
                self.active_tetromino = Some(mino.clone());
                pass = true;
                if mino.tr_type == TetrominoType::T {
                    if x != 0 && y != 0 {
                        self.draw();
                    }
                }

                break;
            }
        }

        if !pass {
            self.active_tetromino = Some(original);
        }

        if self.lock_delay_cur < self.lock_delay_max {
            self.lock_delay_cur += Duration::from_millis(500);
        }

        self.draw();
    }

    pub fn rotate_c(&mut self) {
        let original = self.active_tetromino.as_ref().unwrap().clone();
        let mut mino = self.active_tetromino.as_mut().unwrap().clone();

        let table_entry = match mino.orientation {
            0 => 0,
            1 => 2,
            2 => 4,
            _ => 6,
        };

        mino.orientation = (mino.orientation + 1) % 4;

        let kick_table = match mino.piece_data {
            PieceData::Small(_) => SMALL_MINO_KICK_TABLE,
            PieceData::Large(_) => LARGE_MINO_KICK_TABLE,
        };

        self.clear();

        let mut pass = false;

        for &(x, y) in &kick_table[table_entry][..5] {
            if !self.collision_check_buffer(&mino, (x as i32, y as i32)) {
                self.x += x as i32;
                self.y += y as i32;
                self.active_tetromino = Some(mino);
                pass = true;

                break;
            }
        }

        if !pass {
            self.active_tetromino = Some(original);
        }

        if self.lock_delay_cur < self.lock_delay_max {
            self.lock_delay_cur += Duration::from_millis(500);
        }

        self.draw();
    }

    fn collision_check_buffer(&self, mino: &Tetromino, offset: (i32, i32)) -> bool {
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

                            if self.col_buffer[(self.y + y as i32 + offset.1) as usize]
                                [(self.x + x as i32 + offset.0) as usize]
                            {
                                return true;
                            }
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

                            if self.col_buffer[(self.y + y as i32 + offset.1) as usize]
                                [(self.x + x as i32 + offset.0) as usize]
                            {
                                return true;
                            }
                        }
                    }
                }
            }
        }

        false
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
                                println!(
                                    "BOUNDS s({},{}), p({},{}),  +({},{}) => ({},{})",
                                    self.x,
                                    self.y,
                                    x,
                                    y,
                                    offset.0,
                                    offset.1,
                                    self.x as i32 + x as i32 + offset.0,
                                    self.y as i32 + y as i32 + offset.1
                                );
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
                if self.collision_check_buffer(mino, (0, 1)) {
                    break;
                }
                self.y += 1;
            }
            self.draw();
        }
    }

    fn lock_piece(&mut self) {
        if let Some(tetromino) = &self.active_tetromino {
            match tetromino.piece_data {
                PieceData::Small(data) => {
                    for row in 0..3 {
                        for col in 0..3 {
                            if data[tetromino.orientation][row][col] {
                                self.col_buffer[(row as i32 + self.y) as usize]
                                    [(col as i32 + self.x) as usize] = true;
                            }
                        }
                    }
                }
                PieceData::Large(data) => {
                    for row in 0..5 {
                        for col in 0..5 {
                            if data[tetromino.orientation][row][col] {
                                self.col_buffer[(row as i32 + self.y) as usize]
                                    [(col as i32 + self.x) as usize] = true;
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn hard_drop(&mut self) {
        if let Some(ref mut mino) = self.active_tetromino.clone() {
            self.clear();

            loop {
                if self.collision_check_buffer(mino, (0, 1)) {
                    break;
                }
                self.y += 1;
            }

            self.held = false;
            self.lock_piece();
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
            if !self.collision_check_buffer(mino, offset) {
                self.x = self.x + offset.0;
                self.y = self.y + offset.1;
                if self.lock_delay_cur < self.lock_delay_max {
                    self.lock_delay_cur += Duration::from_millis(500);
                }
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
        self.y = 10;
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

                while !self.collision_check_buffer(&ghost_mino, (0, 1 + ghost_y - self.y)) {
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

    fn apply_gravity(&mut self) {
        if self.gravity_timer.elapsed() >= self.gravity_interval {
            self.gravity_timer = Instant::now();

            self.clear();

            if !self.collision_check_buffer(self.active_tetromino.as_ref().unwrap(), (0, 1)) {
                self.y += 1;
                self.lock_delay_timer = None;
                self.lock_delay_cur = Duration::from_millis(500);
            }

            self.draw();
        }
    }

    fn handle_lock_delay(&mut self) {
        if let Some(mino) = self.active_tetromino.as_ref() {
            if self.collision_check_buffer(&mino, (0, 1)) {
                if self.lock_delay_timer.is_none() {
                    self.lock_delay_timer = Some(Instant::now());
                    self.lock_delay_cur = Duration::from_millis(500);
                }

                if let Some(timer) = self.lock_delay_timer {
                    if timer.elapsed() >= self.lock_delay_cur {
                        self.lock_delay_timer = None;
                        self.lock_delay_cur = Duration::from_millis(500);

                        self.held = false;
                        self.lock_piece();
                        self.draw();
                        self.clear_lines();
                        self.new_tetromino();
                    }
                }
            }
        }
    }

    pub fn update(&mut self) {
        self.apply_gravity();
        self.handle_lock_delay();
    }
}
