use std::time::{Duration, Instant};

pub enum Direction {
    Left,
    Right,
}

pub enum InputEvent {
    MoveLeft,
    MoveRight,
    MoveDown,
    RotateCW,
    RotateCCW,
    SoftDrop,
    HardDrop,
    Hold,
    Quit,
}

pub struct Input {
    das: Duration,
    das_timer: Option<Instant>,
    arr: Duration,
    autoshift: bool,
    direction: Option<Direction>,
}

impl Input {
    pub fn new(das: Duration, arr: Duration) -> Self {
        Self { 
            das, 
            das_timer: None,
            arr, 
            autoshift: false,
            direction: None,
        }
    }

    pub fn start_das_timing(&mut self, direction: Direction) {
        self.das_timer = Some(Instant::now());
        self.direction = Some(direction);
    }

    pub fn reset_das(&mut self) {
        self.das_timer = None;
        self.direction = None;
    }
}
