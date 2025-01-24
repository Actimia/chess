use std::io;

use crate::{
    board::{Board, Position},
    pieces::{Color, Move},
};

pub trait Player {
    fn make_move(&self, board: &Board, color: Color) -> Move;
}

pub struct TerminalPlayer {}

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
        println!("{}", board);
        println!();
        loop {
            let from = self.read_position("What piece to move?");

            if !board[from].is_occupied_by(color) {
                println!("That is not one of your pieces.");
                continue;
            }

            if let Some(moves) = board.get_moves(from) {
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

pub struct RandomPlayer {}

impl Player for RandomPlayer {
    fn make_move(&self, board: &Board, color: Color) -> Move {
        let pieces = board.get_pieces(color);

        let moves: Vec<Move> = pieces
            .iter()
            .flat_map(|(pos, _)| board.get_moves(pos).unwrap())
            .collect();

        let randomish_index = (31 * moves.len() + 17) % moves.len();
        return moves[randomish_index];
    }
}

pub struct Game<W, B>
where
    W: Player,
    B: Player,
{
    board: Board,
    // previous_states: Vec<Board>,
    white: W,
    black: B,
}

impl<White: Player, Black: Player> Game<White, Black> {
    pub fn new(white: White, black: Black) -> Game<White, Black> {
        Game {
            board: Board::new(),
            // previous_states: Vec::new(),
            white,
            black,
        }
    }

    pub fn start(&mut self) {
        loop {
            let white_move = self.white.make_move(&self.board, Color::White);
            self.board = self.board.apply(&white_move);
            // check for gameover

            let black_move = self.black.make_move(&self.board, Color::Black);
            self.board = self.board.apply(&black_move);
            // check for gameover
        }
    }
}
