use std::fmt::Display;

use crate::board::{Board, Position};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PieceType {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Copy, Clone)]
pub struct Piece {
    pub color: Color,
    pub typ: PieceType,
    pub has_moved: bool,
}

impl Piece {
    pub fn get_moves(&self, board: &Board, position: &Position) -> Vec<Move> {
        match self.typ {
            PieceType::Pawn => self.moves_pawn(board, position),
            PieceType::Bishop => self.moves_bishop(board, position),
            PieceType::Knight => self.moves_knight(board, position),
            PieceType::Rook => self.moves_rook(board, position),
            PieceType::Queen => self.moves_queen(board, position),
            PieceType::King => self.moves_king(board, position),
        }
    }

    fn moves_pawn(&self, board: &Board, position: &Position) -> Vec<Move> {
        let up = match self.color {
            Color::Black => -1,
            Color::White => 1,
        };
        let mut moves = vec![(up, 0)];
        if !self.has_moved {
            moves.push((2 * up, 0));
        }

        moves
            .into_iter()
            .map(|offset| Move::from_offset(*position, offset))
            .collect()
    }
    fn moves_bishop(&self, board: &Board, position: &Position) -> Vec<Move> {
        Vec::new()
    }
    fn moves_knight(&self, board: &Board, position: &Position) -> Vec<Move> {
        Vec::new()
    }
    fn moves_rook(&self, board: &Board, position: &Position) -> Vec<Move> {
        Vec::new()
    }
    fn moves_queen(&self, board: &Board, position: &Position) -> Vec<Move> {
        Vec::new()
    }
    fn moves_king(&self, board: &Board, position: &Position) -> Vec<Move> {
        Vec::new()
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match (self.color, self.typ) {
            (Color::White, PieceType::King) => "♔",
            (Color::White, PieceType::Queen) => "♕",
            (Color::White, PieceType::Rook) => "♖",
            (Color::White, PieceType::Bishop) => "♗",
            (Color::White, PieceType::Knight) => "♘",
            (Color::White, PieceType::Pawn) => "♙",
            (Color::Black, PieceType::King) => "♚",
            (Color::Black, PieceType::Queen) => "♛",
            (Color::Black, PieceType::Rook) => "♜",
            (Color::Black, PieceType::Knight) => "♝",
            (Color::Black, PieceType::Bishop) => "♞",
            (Color::Black, PieceType::Pawn) => "♟",
        };
        write!(f, " {text} ")
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Square {
    Empty,
    Occupied(Piece),
}

impl From<(Color, PieceType)> for Square {
    fn from((color, typ): (Color, PieceType)) -> Self {
        Self::Occupied(Piece {
            color,
            typ,
            has_moved: false,
        })
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, " "),
            Self::Occupied(piece) => write!(f, "{}", piece),
        }
    }
}

impl Square {
    pub fn possible_moves(&self, board: &Board, position: &Position) -> Option<Vec<Move>> {
        match self {
            Self::Empty => None,
            Self::Occupied(piece) => Some(piece.get_moves(board, position)),
        }
    }
}

pub enum SpecialMove {
    EnPassant, // possibly not needed
    Promotion(PieceType),
    Castling,
}
pub struct Move {
    from: Position,
    to: Position,
    special: Option<SpecialMove>,
}

impl Move {
    pub fn from_offset(from: Position, (rank_offset, file_offset): (i32, i32)) -> Move {
        let to: Position = (
            from.rank() as i32 + rank_offset,
            from.file() as i32 + file_offset,
        )
            .into();
        Move {
            from,
            to,
            special: None,
        }
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.from, self.to)?;
        Ok(())
    }
}
