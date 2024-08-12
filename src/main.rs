pub mod mino;
pub mod board;
pub mod util;
pub mod constants;

use constants::{HEIGHT, WIDTH};
use getch_rs::{Getch, Key};
use std::thread;
use std::time::Duration;

use board::{Board, Status};

use crate::util::get_tile_color;

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
