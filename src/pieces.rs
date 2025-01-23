use std::fmt::Display;

use crate::board::{Board, Position};

#[derive(Debug, Copy, Clone)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Copy, Clone)]
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
            PieceType::Bishop => self.moves_pawn(board, position),
            PieceType::Knight => self.moves_pawn(board, position),
            PieceType::Rook => self.moves_pawn(board, position),
            PieceType::Queen => self.moves_pawn(board, position),
            PieceType::King => self.moves_pawn(board, position),
        }
    }

    fn moves_pawn(&self, board: &Board, position: &Position) -> Vec<Move> {
        let up = 8;
        Vec::new()
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
    EnPassant,
    Promotion(PieceType),
    Castling,
}
pub struct Move {
    from: Square,
    to: Position,
    special: Option<SpecialMove>,
}

impl Move {
    pub fn get_notation() -> String {
        // impl Display instead?
        return "".into();
    }
}
