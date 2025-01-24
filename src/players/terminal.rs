use std::io;

use crate::{
    board::{Board, Position},
    pieces::{Color, Move},
};

use super::Player;

// TerminalPlayer asks stdin for which moves to make.
pub struct TerminalPlayer;

impl TerminalPlayer {
    fn read_position(&self, prompt: &str) -> Position {
        loop {
            println!("{prompt}");
            let mut input = String::new();
            let _ = io::stdin().read_line(&mut input);

            if input.len() < 3 {
                continue;
            }

            let rank = input.remove(1) as u8;
            let file = input.remove(0) as u8;

            let rank = match rank {
                b'1' => 0,
                b'2' => 8,
                b'3' => 16,
                b'4' => 24,
                b'5' => 32,
                b'6' => 40,
                b'7' => 48,
                b'8' => 56,
                _ => continue,
            };

            let file = match file {
                b'a' => 0,
                b'b' => 1,
                b'c' => 2,
                b'd' => 3,
                b'e' => 4,
                b'f' => 5,
                b'g' => 6,
                b'h' => 7,
                _ => continue,
            };

            return ((rank + file) as usize).into();
        }
    }
}

impl Player for TerminalPlayer {
    fn make_move(&self, board: &Board, color: Color) -> Move {
        loop {
            let from = self.read_position("What piece to move?");

            if !board.is_occupied_by(from, Some(color), None) {
                println!("That is not one of your pieces.");
                continue;
            }

            if let Some(moves) = board.get_moves(&from) {
                if moves.is_empty() {
                    println!("That piece has no moves.");
                    continue;
                }
                for mv in moves.iter() {
                    println!("{}", mv);
                }
                let to = self.read_position("Where to move the piece?");

                match moves.iter().find(|mv| mv.to == to) {
                    Some(mv) => return *mv,
                    None => continue,
                }
            }
        }
    }
}
