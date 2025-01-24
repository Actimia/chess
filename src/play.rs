use std::{fmt::Display, io};

use rand::Rng;

use crate::{
    board::{Board, Position},
    pieces::{Color, Move, PieceType},
};

pub trait Player {
    fn make_move(&self, board: &Board, color: Color) -> Move;
}

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

// RandomPlayer makes a random legal move
pub struct RandomPlayer;

impl Player for RandomPlayer {
    fn make_move(&self, board: &Board, color: Color) -> Move {
        let pieces = board.get_pieces(color);
        let moves: Vec<Move> = pieces
            .iter()
            .flat_map(|(pos, _)| board.get_moves(pos))
            .flatten()
            .collect();

        let random_index = rand::thread_rng().gen_range(0..moves.len());
        return moves[random_index];
    }
}

pub struct PrintBoard<P: Player> {
    player: P,
}

impl<P: Player> Player for PrintBoard<P> {
    fn make_move(&self, board: &Board, color: Color) -> Move {
        println!("{}", board);
        println!();
        self.player.make_move(board, color)
    }
}

impl<P: Player> PrintBoard<P> {
    pub fn wrap(player: P) -> Self {
        Self { player }
    }
}

pub struct PrintMoves<P: Player> {
    player: P,
}

impl<P: Player> Player for PrintMoves<P> {
    fn make_move(&self, board: &Board, color: Color) -> Move {
        let mv = self.player.make_move(board, color);
        println!("{}", mv);
        mv
    }
}

impl<P: Player> PrintMoves<P> {
    pub fn wrap(player: P) -> Self {
        Self { player }
    }
}

pub struct ManualStep<P: Player> {
    player: P,
}

impl<P: Player> Player for ManualStep<P> {
    fn make_move(&self, board: &Board, color: Color) -> Move {
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input);
        self.player.make_move(board, color)
    }
}

impl<P: Player> ManualStep<P> {
    pub fn wrap(player: P) -> Self {
        Self { player }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum GameResult {
    WhiteWin,
    Draw,
    BlackWin,
}

impl Display for GameResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameResult::WhiteWin => write!(f, "White won")?,
            GameResult::Draw => write!(f, "Draw")?,
            GameResult::BlackWin => write!(f, "Black won")?,
        }
        Ok(())
    }
}

pub struct Game<W, B>
where
    W: Player,
    B: Player,
{
    board: Board,
    previous_states: Vec<Board>,
    white: W,
    black: B,
}

impl<White: Player, Black: Player> Game<White, Black> {
    pub fn new(white: White, black: Black) -> Game<White, Black> {
        println!("{}", std::mem::size_of::<Board>());
        Game {
            board: Board::new(),
            previous_states: Vec::new(),
            white,
            black,
        }
    }

    fn get_next_move(&self) -> Move {
        match self.board.get_turn() {
            Color::White => self.white.make_move(&self.board, Color::White),
            Color::Black => self.black.make_move(&self.board, Color::Black),
        }
    }

    fn is_gameover(&self) -> Option<GameResult> {
        if self
            .previous_states
            .iter()
            .filter(|prev| **prev == self.board)
            .count()
            >= 2
        {
            // if we have seen the current gamestate twice before, it is a draw
            Some(GameResult::Draw)
        } else if self.board.ply - self.board.last_pawn_move >= 50 {
            // 50 moves since last pawn move, it is a draw
            Some(GameResult::Draw)
        } else {
            let black = self.board.get_pieces(Color::Black);
            if let None = black.iter().find(|(_, piece)| piece.typ == PieceType::King) {
                // The black king has been captured, white wins
                return Some(GameResult::WhiteWin);
            }
            let white = self.board.get_pieces(Color::White);
            if let None = white.iter().find(|(_, piece)| piece.typ == PieceType::King) {
                // The white king has been captured, black wins
                return Some(GameResult::BlackWin);
            }

            // the game is still ongoing
            None
        }
    }

    pub fn start(&mut self) {
        loop {
            let mv = self.get_next_move();
            self.previous_states.push(self.board);
            self.board = self.board.apply(&mv);

            if let Some(result) = self.is_gameover() {
                println!("{}", self.board);
                println!("Game over: {} after {} moves", result, self.board.ply / 2);
                break;
            }
        }
    }
}
