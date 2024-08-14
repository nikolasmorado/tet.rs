pub mod board;
pub mod constants;
pub mod mino;
pub mod util;

use constants::{HEIGHT, WIDTH};
use crossterm::event::{
    self, KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags,
};
use crossterm::terminal::enable_raw_mode;
use crossterm::{cursor, execute, queue, style::Print, terminal};
use std::io::{stdout, Write};
use std::thread;
use std::time::Duration;

use board::{Board, Status};

use crate::util::{get_ghost_color, get_tile_color};

fn main() {
    let mut stdout = stdout();
    enable_raw_mode().unwrap();

    execute!(
        stdout,
        PushKeyboardEnhancementFlags(
            KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
                | KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES
        )
    )
    .unwrap();

    let mut board = Board::new((WIDTH, HEIGHT));
    board.new_tetromino();

    let mut frame_count = 0;
    let start = std::time::Instant::now();
    let mut fps;

    loop {
        // Drawing loop
        frame_count += 1;
        let duration = start.elapsed().as_secs_f64();
        fps = frame_count as f64 / duration;

        let (cols, rows) = crossterm::terminal::size().unwrap();

        let bx_px = (board.width * 2) + 2;
        let by_px = (board.height + 1 - 10) + 1; 

        let sx = (cols - bx_px as u16) / 2;
        let sy = (rows - by_px as u16) / 2 ;


        execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();
        execute!(stdout, cursor::MoveTo(sx, sy)).unwrap();
        execute!(stdout, cursor::Hide).unwrap();


        board.update();
        // board.draw_at(false);

        for y in 10..board.height + 1 {
            execute!(stdout, cursor::MoveTo(sx, sy + y as u16 - 10 )).unwrap();
            for x in 0..board.width + 2 {
                if x == 0 && y == board.height {
                    // print!("└");
                    print!("▀");
                } else if x == board.width + 1 && y == board.height {
                    // print!("┘");
                    print!("▀");
                } else if x == 0 || x == board.width + 1 {
                    // print!("|");
                    print!("█");
                } else if y == board.height {
                    // print!("--");
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

        // Input loop

        if event::poll(Duration::from_millis(10)).unwrap() {
            if let event::Event::Key(event) = event::read().unwrap() {
                match event.code {
                    event::KeyCode::Char('c')
                        if event.modifiers.contains(event::KeyModifiers::CONTROL) =>
                    {
                        execute!(stdout, PopKeyboardEnhancementFlags).unwrap();
                        terminal::disable_raw_mode().unwrap();
                        break;
                    }
                    event::KeyCode::Char(' ') => {
                        board.hard_drop();
                    }
                    event::KeyCode::Right => {
                        board.move_tetromino((1, 0));
                    }
                    event::KeyCode::Left => {
                        board.move_tetromino((-1, 0));
                    }
                    event::KeyCode::Down => {
                        // board.move_tetromino((0, 1));
                        board.soft_harddrop();
                    }
                    event::KeyCode::Up => {
                        board.rotate_c();
                    }
                    event::KeyCode::Modifier(event::ModifierKeyCode::LeftControl) => {
                        board.rotate_cc();
                    }
                    event::KeyCode::Modifier(event::ModifierKeyCode::LeftShift) => {
                        board.hold_piece();
                    }
                    _ => (),
                }
            }
        }

        // Input loop

        thread::sleep(Duration::from_millis(1000 / 120));
    }
}

