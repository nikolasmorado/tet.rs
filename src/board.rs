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

pub enum RotationDirection {
    Clockwise,
    CounterClockwise,
}

pub struct Board {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<Vec<Status>>,
    pub pieces_placed: usize,
    pub lines_cleared: usize,
    pub upcoming_tiles: Vec<Vec<Status>>,
    pub held_tiles: Vec<Vec<Status>>,
    col_buffer: Vec<Vec<bool>>,
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

macro_rules! draw_piece {
    ($tiles:expr, $data:expr, $sizex:expr, $sizey:expr, $del:expr, $tr_type:expr, $x:expr, $y:expr, $orientation:expr, $ghost: expr) => {{
        for row in 0..$sizey {
            for col in 0..$sizex {
                if $data[$orientation][row][col] {
                    if $del {
                        $tiles[(row as i32 + $y) as usize][(col as i32 + $x) as usize] =
                            Status::Empty;
                    } else {
                        $tiles[(row as i32 + $y) as usize][(col as i32 + $x) as usize] =
                            match $ghost {
                                true => Status::FillGhost(mino_to_ghost($tr_type)),
                                false => Status::FillType($tr_type),
                            };
                    }
                }
            }
        }
    }};
}

macro_rules! check_col {
    ($self:expr, $data:expr, $sizex:expr, $sizey:expr, $offset:expr) => {
        for y in 0..$sizey {
            for x in 0..$sizex {
                if $data[y][x] {
                    if $self.y as i32 + y as i32 + $offset.1 >= $self.height as i32
                        || $self.x as i32 + x as i32 + $offset.0 < 0
                        || $self.x as i32 + x as i32 + $offset.0 >= $self.width as i32
                    {
                        return true;
                    }

                    if $self.col_buffer[($self.y + y as i32 + $offset.1) as usize]
                        [($self.x + x as i32 + $offset.0) as usize]
                    {
                        return true;
                    }
                }
            }
        }
    };
}

impl Board {
    pub fn new(dims: (usize, usize)) -> Self {
        let tiles = vec![vec![Status::Empty; dims.0]; dims.1 + 10];
        let col_buffer = vec![vec![false; dims.0]; dims.1 + 10];
        let upcoming_tiles = vec![vec![Status::Empty; 4]; 20];
        let held_tiles = vec![vec![Status::Empty; 4]; 4];


        let mut upcoming = gen_bag();
        upcoming.extend(gen_bag());

        Board {
            width: dims.0,
            height: dims.1 + 10,
            tiles,
            col_buffer,
            pieces_placed: 0,
            lines_cleared: 0,
            upcoming_tiles,
            held_tiles,
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


    fn clear_upcoming(&mut self) {
        for y in 0..20 {
            for x in 0..4 {
                self.upcoming_tiles[y][x] = Status::Empty;
            }
        }
    }

    fn clear_held(&mut self) {
        for y in 0..4 {
            for x in 0..4 {
                self.held_tiles[y][x] = Status::Empty;
            }
        }
    }

    fn draw_upcoming(&mut self) {
        for (index, mino) in self.upcoming.iter().enumerate() {
            let at = Tetromino::new(*mino);

            if index > 4 {
                break;
            }

            match at.piece_data {
                PieceData::Small(data) => {
                    draw_piece!(
                        self.upcoming_tiles,
                        data,
                        3,
                        3,
                        false,
                        at.tr_type,
                        0,
                        index as i32 * 4,
                        at.orientation,
                        false
                    );
                }
                PieceData::Medium(data) => {
                    draw_piece!(
                        self.upcoming_tiles,
                        data,
                        4,
                        3,
                        false,
                        at.tr_type,
                        0,
                        index as i32 * 4,
                        at.orientation,
                        false
                    );
                }
                PieceData::Large(data) => {
                    draw_piece!(
                        self.upcoming_tiles,
                        data,
                        4,
                        4,
                        false,
                        at.tr_type,
                        0,
                        index as i32 * 4,
                        at.orientation,
                        false
                    );
                }
            }
        }
    }

    fn draw_held(&mut self) {
        if let Some(mino) = self.held_piece {
            let at = Tetromino::new(mino);

            match at.piece_data {
                PieceData::Small(data) => {
                    draw_piece!(
                        self.held_tiles,
                        data,
                        3,
                        3,
                        false,
                        at.tr_type,
                        0,
                        0,
                        at.orientation,
                        false
                    );
                }
                PieceData::Medium(data) => {
                    draw_piece!(
                        self.held_tiles,
                        data,
                        4,
                        3,
                        false,
                        at.tr_type,
                        0,
                        0,
                        at.orientation,
                        false
                    );
                }
                PieceData::Large(data) => {
                    draw_piece!(
                        self.held_tiles,
                        data,
                        4,
                        4,
                        false,
                        at.tr_type,
                        0,
                        0,
                        at.orientation,
                        false
                    );
                }
            }
        }
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
                self.lines_cleared += 1;
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

        self.clear_upcoming();
        self.draw_upcoming();

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
            self.clear_upcoming();
            self.clear_held();
            self.active_tetromino = Some(Tetromino::new(self.upcoming.remove(0)));
            self.x = (self.width / 2 - 2) as i32;
            self.y = 10;
        } else {
            self.move_tetromino((0, 1));
        }
    }

    pub fn rotate(&mut self, dir: RotationDirection) {
        let original = self.active_tetromino.as_ref().unwrap().clone();
        let mut mino = self.active_tetromino.as_mut().unwrap().clone();

        let table_entry = match dir {
            RotationDirection::Clockwise => match mino.orientation {
                0 => 0,
                1 => 2,
                2 => 4,
                _ => 6,
            },
            RotationDirection::CounterClockwise => match mino.orientation {
                0 => 7,
                1 => 1,
                2 => 3,
                _ => 5,
            },
        };

        mino.orientation = match dir {
            RotationDirection::Clockwise => (mino.orientation + 1) % 4,
            RotationDirection::CounterClockwise => (mino.orientation + 3) % 4,
        };

        let kick_table = match mino.piece_data {
            PieceData::Small(_) => SMALL_MINO_KICK_TABLE,
            PieceData::Medium(_) => SMALL_MINO_KICK_TABLE,
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
            self.lock_delay_cur += self.lock_delay_interval;
        }

        self.draw();
    }

    fn collision_check_buffer(&self, mino: &Tetromino, offset: (i32, i32)) -> bool {
        match mino.piece_data {
            PieceData::Small(data) => check_col!(self, data[mino.orientation], 3, 3, offset),
            PieceData::Medium(data) => check_col!(self, data[mino.orientation], 4, 3, offset),
            PieceData::Large(data) => check_col!(self, data[mino.orientation], 4, 4, offset),
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
                PieceData::Medium(data) => {
                    for row in 0..3 {
                        for col in 0..4 {
                            if data[tetromino.orientation][row][col] {
                                self.col_buffer[(row as i32 + self.y) as usize]
                                    [(col as i32 + self.x) as usize] = true;
                            }
                        }
                    }
                }
                PieceData::Large(data) => {
                    for row in 0..4 {
                        for col in 0..4 {
                            if data[tetromino.orientation][row][col] {
                                self.col_buffer[(row as i32 + self.y) as usize]
                                    [(col as i32 + self.x) as usize] = true;
                            }
                        }
                    }
                }
            }
            self.pieces_placed += 1;
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
        for row in self.y as usize..self.height {
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

        self.clear_held();
        self.clear_upcoming();
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
        self.draw_held();
        self.draw_upcoming();
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
                    PieceData::Small(data) => draw_piece!(
                        self.tiles,
                        data,
                        3,
                        3,
                        del,
                        tetromino.tr_type,
                        self.x,
                        ghost_y,
                        tetromino.orientation,
                        true
                    ),
                    PieceData::Medium(data) => draw_piece!(
                        self.tiles,
                        data,
                        4,
                        3,
                        del,
                        tetromino.tr_type,
                        self.x,
                        ghost_y,
                        tetromino.orientation,
                        true
                    ),
                    PieceData::Large(data) => draw_piece!(
                        self.tiles,
                        data,
                        4,
                        4,
                        del,
                        tetromino.tr_type,
                        self.x,
                        ghost_y,
                        tetromino.orientation,
                        true
                    ),
                }
            }

            match tetromino.piece_data {
                PieceData::Small(data) => {
                    draw_piece!(
                        self.tiles,
                        data,
                        3,
                        3,
                        del,
                        tetromino.tr_type,
                        self.x,
                        self.y,
                        tetromino.orientation,
                        false
                    );
                }
                PieceData::Medium(data) => {
                    draw_piece!(
                        self.tiles,
                        data,
                        4,
                        3,
                        del,
                        tetromino.tr_type,
                        self.x,
                        self.y,
                        tetromino.orientation,
                        false
                    );
                }
                PieceData::Large(data) => {
                    draw_piece!(
                        self.tiles,
                        data,
                        4,
                        4,
                        del,
                        tetromino.tr_type,
                        self.x,
                        self.y,
                        tetromino.orientation,
                        false
                    );
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
                    self.lock_delay_cur = self.lock_delay_interval;
                }

                if let Some(timer) = self.lock_delay_timer {
                    if timer.elapsed() >= self.lock_delay_cur {
                        self.lock_delay_timer = None;
                        self.lock_delay_cur = self.lock_delay_interval;

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
