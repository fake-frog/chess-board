#![allow(unused)]

use crate::game::{Color, Piece};
use game::Board;

pub mod game {

    #[derive(Clone, Copy, Debug)]
    pub enum Color {
        White,
        Black,
    }
    // bool to check if it has moved
    // important for castling and enpasaunt
    #[derive(Clone, Copy, Debug)]
    pub enum Piece {
        Pawn(Color, bool),
        Rook(Color, bool),
        Knight(Color),
        Bishop(Color),
        Queen(Color),
        King(Color, bool),
        None,
    }

    pub fn get_index_from_sqr(str_loc: &str) -> usize {
        let file = str_loc.chars().nth(0).unwrap_or('a');
        let file = file.to_ascii_lowercase() as u8 - b'a';
        let file = file.clamp(0, 7);

        let rank = str_loc.chars().nth(1).unwrap_or('1');
        let rank = rank.to_digit(10).unwrap_or(0) as u8;
        let rank = rank.clamp(1, 8) - 1;

        (rank * 8 + file) as usize
    }

    // This is effectivly just renaming the function above
    // Mostly for learning I guess?
    #[macro_export]
    macro_rules! sloc {
        ($s:expr) => {
            crate::game::get_index_from_sqr($s)
        };
    }

    fn print_square(loc: usize, r: usize, s: &str) {
        if r % 2 == 0 {
            if loc % 2 == 0 {
                print!("{s} ");
            } else {
                print!("\x1b[100m{s} \x1b[0m");
            }
        } else {
            if loc % 2 == 0 {
                print!("\x1b[100m{s} \x1b[0m");
            } else {
                print!("{s} ");
            }
        }
    }

    pub struct Board {
        squares: [Piece; 64],
    }

    impl Board {
        pub fn new() -> Self {
            let mut new_board = Self {
                squares: [Piece::None; 64],
            };

            new_board.set_peice_at(Piece::Rook(Color::White, false), sloc!("a1"));
            new_board.set_peice_at(Piece::Knight(Color::White), sloc!("b1"));
            new_board.set_peice_at(Piece::Bishop(Color::White), sloc!("c1"));
            new_board.set_peice_at(Piece::Queen(Color::White), sloc!("d1"));
            new_board.set_peice_at(Piece::King(Color::White, false), sloc!("e1"));
            new_board.set_peice_at(Piece::Bishop(Color::White), sloc!("f1"));
            new_board.set_peice_at(Piece::Knight(Color::White), sloc!("g1"));
            new_board.set_peice_at(Piece::Rook(Color::White, false), sloc!("h1"));

            for i in 8..=15 {
                new_board.set_peice_at(Piece::Pawn(Color::White, false), i);
            }

            new_board.set_peice_at(Piece::Rook(Color::Black, false), sloc!("a8"));
            new_board.set_peice_at(Piece::Knight(Color::Black), sloc!("b8"));
            new_board.set_peice_at(Piece::Bishop(Color::Black), sloc!("c8"));
            new_board.set_peice_at(Piece::Queen(Color::Black), sloc!("d8"));
            new_board.set_peice_at(Piece::King(Color::Black, false), sloc!("e8"));
            new_board.set_peice_at(Piece::Bishop(Color::Black), sloc!("f8"));
            new_board.set_peice_at(Piece::Knight(Color::Black), sloc!("g8"));
            new_board.set_peice_at(Piece::Rook(Color::Black, false), sloc!("h8"));

            for i in 48..=55 {
                new_board.set_peice_at(Piece::Pawn(Color::Black, false), i);
            }

            new_board
        }

        pub fn get_peice_at(&self, loc: usize) -> &Piece {
            &self.squares[loc]
        }

        pub fn set_peice_at(&mut self, piece: Piece, loc: usize) {
            self.squares[loc] = piece;
        }

        pub fn move_square(&mut self, loc1: usize, loc2: usize) {
            let piece = self.get_peice_at(loc1);
            self.set_peice_at(*piece, loc2);
            self.set_peice_at(Piece::None, loc1);
        }

        pub fn print(&self, player_color: Color, show_label: bool) {
            /*
            (these colors are reversed to look better on dark screens)
                   black   white
                     □       ■
            King   ♔ 2654  ♚ 265A
            Queen  ♕ 2655  ♛ 265B
            Rook   ♖ 2656  ♜ 265C
            Bishop ♗ 2657  ♝ 265D
            Knight ♘ 2658  ♞ 265E
            Pawn   ♙ 2659  ♟ 265F
             */

            println!("\x1b[33m┌──────────────────┐\x1b[0m");
            for r in (0..8) {
                let r = match player_color {
                    Color::White => 7 - r,
                    Color::Black => r,
                };

                if show_label {
                    print!("\x1b[33m{} \x1b[0m", r + 1);
                } else {
                    print!("\x1b[33m│ \x1b[0m");
                }

                for f in (0..8) {
                    let f = match player_color {
                        Color::White => f,
                        Color::Black => 7 - f,
                    };
                    let loc = r * 8 + f;
                    let piece = self.get_peice_at(loc);
                    // TODO: maybe we build a string here instead of all of these print statements
                    match piece {
                        Piece::Pawn(color, has_moved) => match color {
                            Color::White => print_square(loc, r, "♟"),
                            Color::Black => print_square(loc, r, "♙"),
                        },
                        Piece::Rook(color, has_moved) => match color {
                            Color::White => print_square(loc, r, "♜"),
                            Color::Black => print_square(loc, r, "♖"),
                        },
                        Piece::Knight(color) => match color {
                            Color::White => print_square(loc, r, "♞"),
                            Color::Black => print_square(loc, r, "♘"),
                        },
                        Piece::Bishop(color) => match color {
                            Color::White => print_square(loc, r, "♝"),
                            Color::Black => print_square(loc, r, "♗"),
                        },
                        Piece::Queen(color) => match color {
                            Color::White => print_square(loc, r, "♛"),
                            Color::Black => print_square(loc, r, "♕"),
                        },
                        Piece::King(color, has_moved) => match color {
                            Color::White => print_square(loc, r, "♚"),
                            Color::Black => print_square(loc, r, "♔"),
                        },
                        // be cool if we could change the background color here
                        Piece::None => {
                            print_square(loc, r, " ");
                        }
                    }
                }
                println!("\x1b[33m │\x1b[0m");
            }
            if show_label {
                match player_color {
                    Color::White => println!("\x1b[33m└ a b c d e f g h ─┘\x1b[0m"),
                    Color::Black => println!("\x1b[33m└ h g f e d c b a ─┘\x1b[0m"),
                }
            } else {
                println!("\x1b[33m└──────────────────┘\x1b[0m");
            }
        }
    }
}

fn main() {
    let mut board = Board::new();
    board.move_square(sloc!("e2"), sloc!("e4"));
    board.print(Color::White, true);
}
