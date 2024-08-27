pub mod board;
pub mod constants;
pub mod input;
pub mod mino;
pub mod util;

use constants::{HEIGHT, WIDTH};
use crossterm::event::{KeyEventKind, KeyModifiers};
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::enable_raw_mode;
use crossterm::{cursor, execute, style::Print, terminal};
use std::collections::HashMap;
use std::io::stdout;
use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::time::{Duration, Instant};

use board::{Board, RotationDirection, Status};

use crate::util::{get_ghost_color, get_tile_color};

fn input_thread(tx: std::sync::mpsc::Sender<Event>) {
    loop {
        if event::poll(Duration::from_millis(10)).unwrap() {
            if let Ok(key_event) = event::read() {
                tx.send(key_event).unwrap();
            }
        }
    }
}

fn main() {
    let mut stdout = stdout();
    enable_raw_mode().unwrap();

    execute!(
        stdout,
        crossterm::event::PushKeyboardEnhancementFlags(
            crossterm::event::KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
                | crossterm::event::KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES
                | crossterm::event::KeyboardEnhancementFlags::REPORT_EVENT_TYPES
        )
    )
    .unwrap();

    let mut board = Board::new((WIDTH, HEIGHT));
    board.new_tetromino();

    let mut frame_count = 0;
    let start = std::time::Instant::now();
    execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();

    let mut pressed_keys: HashMap<KeyCode, bool> = HashMap::new();
    let mut last_drop_time = Instant::now();
    let mut last_move_time = Instant::now();
    let mut last_rotate_time = Instant::now();

    let (input_tx, input_rx): (std::sync::mpsc::Sender<Event>, Receiver<Event>) = mpsc::channel();

    thread::spawn(move || input_thread(input_tx));

    loop {
        // Drawing loop
        frame_count += 1;
        let duration = start.elapsed().as_secs_f64();

        let fps = frame_count as f64 / duration;
        let pps = board.pieces_placed as f64 / duration;
        let lc = board.lines_cleared;

        let (cols, rows) = crossterm::terminal::size().unwrap();

        let bx_px = (board.width * 2) + 2;
        let by_px = (board.height + 1 - 10) + 1;

        let sx = (cols - bx_px as u16) / 2;
        let sy = (rows - by_px as u16) / 2;

        execute!(stdout, cursor::MoveTo(sx, sy)).unwrap();
        execute!(stdout, cursor::Hide).unwrap();

        board.update();

        for y in 10..board.height + 1 {
            execute!(stdout, cursor::MoveTo(sx, sy + y as u16 - 10)).unwrap();
            for x in 0..board.width + 2 {
                if x == 0 && y == board.height || x == board.width + 1 && y == board.height {
                    print!("▀");
                } else if x == 0 || x == board.width + 1 {
                    print!("█");
                } else if y == board.height {
                    print!("▀▀");
                } else {
                    match board.tiles[y][x - 1] {
                        Status::Empty => print!("  "),
                        Status::FillType(mino) => print!("{}", get_tile_color(mino)),
                        Status::FillGhost(mino) => print!("{}", get_ghost_color(mino)),
                    }
                }
            }
            // println!();
            execute!(stdout, Print("\r\n")).unwrap();
        }

        execute!(
            stdout,
            cursor::MoveTo(sx, sy + board.height as u16 - 10 + 1)
        )
        .unwrap();
        print!("     FPS:   {:.2}   ", fps);
        execute!(
            stdout,
            cursor::MoveTo(sx, sy + board.height as u16 - 10 + 2)
        )
        .unwrap();
        print!("     PPS:   {:.2}   ", pps);
        execute!(
            stdout,
            cursor::MoveTo(sx, sy + board.height as u16 - 10 + 3)
        )
        .unwrap();
        print!("     LC:    {:.0}   ", lc);

        for y in 0..20 {
            execute!(stdout, cursor::MoveTo(sx + bx_px as u16 + 4, sy as u16 + y)).unwrap();
            for x in 0..4 {
                match board.upcoming_tiles[y as usize][x] {
                    Status::Empty => print!("  "),
                    Status::FillType(mino) => print!("{}", get_tile_color(mino)),
                    Status::FillGhost(mino) => print!("{}", get_ghost_color(mino)),
                }
            }
        }

        for y in 0..4 {
            execute!(stdout, cursor::MoveTo(sx - 8, sy as u16 + y)).unwrap();
            for x in 0..4 {
                match board.held_tiles[y as usize][x] {
                    Status::Empty => print!("  "),
                    Status::FillType(mino) => print!("{}", get_tile_color(mino)),
                    Status::FillGhost(mino) => print!("{}", get_ghost_color(mino)),
                }
            }
        }

        while let Ok(event) = input_rx.try_recv() {
            if let Event::Key(key_event) = event {
                match key_event.code {
                    KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
                        execute!(stdout, crossterm::event::PopKeyboardEnhancementFlags).unwrap();
                        terminal::disable_raw_mode().unwrap();
                        return;
                    }
                    KeyCode::Char(' ') if key_event.kind == KeyEventKind::Press => {
                        if last_drop_time.elapsed() >= Duration::from_millis(10) {
                            board.hard_drop();
                            last_drop_time = Instant::now();
                        }
                    }
                    KeyCode::Right if key_event.kind == KeyEventKind::Press => {
                        if last_move_time.elapsed() >= Duration::from_millis(10) {
                            board.move_tetromino((1, 0));
                            last_move_time = Instant::now();
                        }
                    }
                    KeyCode::Left if key_event.kind == KeyEventKind::Press => {
                        if last_move_time.elapsed() >= Duration::from_millis(10) {
                            board.move_tetromino((-1, 0));
                            last_move_time = Instant::now();
                        }
                    }
                    KeyCode::Up if key_event.kind == KeyEventKind::Press => {
                        if last_rotate_time.elapsed() >= Duration::from_millis(10) {
                            board.rotate(RotationDirection::Clockwise);
                            last_rotate_time = Instant::now();
                        }
                    }
                    KeyCode::Modifier(event::ModifierKeyCode::LeftControl) if key_event.kind == KeyEventKind::Press => {
                        if last_rotate_time.elapsed() >= Duration::from_millis(10) {
                            board.rotate(RotationDirection::CounterClockwise);
                            last_rotate_time = Instant::now();
                        }
                    }
                    KeyCode::Down if key_event.kind == KeyEventKind::Press => {
                        board.soft_harddrop();
                    }
                    KeyCode::Modifier(event::ModifierKeyCode::LeftShift) if key_event.kind == KeyEventKind::Press => {
                        board.hold_piece();
                    }
                    _ => (),
                }
            }
        }

        thread::sleep(Duration::from_millis(1000 / 120));
    }
}
