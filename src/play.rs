use std::fmt::Display;

use crate::{
    board::Board,
    pieces::{Color, Move, PieceType},
    players::Player,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum GameResult {
    WhiteWin,
    DrawByRepetition,
    DrawBy50MoveRule,
    BlackWin,
}

impl Display for GameResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameResult::WhiteWin => write!(f, "White won")?,
            GameResult::DrawByRepetition => write!(f, "Draw by repetition")?,
            GameResult::DrawBy50MoveRule => write!(f, "Draw by 50-move rule")?,
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
    previous_states: Vec<String>,
    white: W,
    black: B,
}

impl<White: Player, Black: Player> Game<White, Black> {
    pub fn new(
        fen: Option<String>,
        white: White,
        black: Black,
    ) -> anyhow::Result<Game<White, Black>> {
        Ok(Game {
            board: Board::new(fen)?,
            previous_states: Vec::new(),
            white,
            black,
        })
    }

    fn get_next_move(&self) -> Move {
        match self.board.current_turn() {
            Color::White => self.white.make_move(&self.board),
            Color::Black => self.black.make_move(&self.board),
        }
    }

    fn is_gameover(&self) -> Option<GameResult> {
        let fen_pieces = self.board.get_fen_pieces();
        if self
            .previous_states
            .iter()
            .filter(|prev| **prev == fen_pieces)
            .count()
            >= 2
        {
            // if we have seen the current gamestate twice before, it is a draw
            Some(GameResult::DrawByRepetition)
        } else if self.board.ply - self.board.last_pawn_move >= 50 {
            // 50 moves since last pawn move, it is a draw
            Some(GameResult::DrawBy50MoveRule)
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
            // For the purposes of determining a draw, we could clear this
            // if we see a pawn move or capture
            self.previous_states.push(self.board.get_fen_pieces());
            self.board = self.board.apply(&mv);

            if let Some(result) = self.is_gameover() {
                println!("{}", self.board);
                println!(
                    "Game over: {} after {} moves",
                    result,
                    1 + self.board.ply / 2
                );
                break;
            }
        }
    }
}
