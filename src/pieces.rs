use std::{fmt::Display, ops::Not};

use crate::board::{Board, Position};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Color {
    White,
    Black,
}

impl Not for Color {
    type Output = Color;

    fn not(self) -> Self::Output {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Piece {
    pub color: Color,
    pub typ: PieceType,
    pub most_recent_move: Option<usize>,
}

const ALL_DIRECTIONS: [(i32, i32); 8] = [
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1),
    (1, 1),
    (1, -1),
    (-1, -1),
    (-1, 1),
];

const DIAGONALS: [(i32, i32); 4] = [(1, 1), (1, -1), (-1, -1), (-1, 1)];
const STRAIGHTS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
const KNIGHT_MOVES: [(i32, i32); 8] = [
    (2, 1),
    (-2, 1),
    (1, 2),
    (1, -2),
    (-2, -1),
    (2, -1),
    (-1, 2),
    (-1, -2),
];

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

    fn moves_pawn(&self, board: &Board, pawn: &Position) -> Vec<Move> {
        let up = match self.color {
            Color::Black => -1,
            Color::White => 1,
        };
        let mut moves = vec![];

        // normal move
        if let Some(mv1) = pawn.offset(0, up) {
            if board[mv1].is_empty() {
                moves.push(mv1);

                // starting move
                if let Some(mv) = pawn.offset(0, 2 * up) {
                    if self.most_recent_move.is_none() && board[mv].is_empty() {
                        moves.push(mv);
                    }
                }
            }
        }

        // captures
        if let Some(mv) = pawn.offset(-1, up) {
            if board[mv].is_occupied_by(!self.color) {
                moves.push(mv);
            }
        }
        if let Some(mv) = pawn.offset(1, up) {
            if board[mv].is_occupied_by(!self.color) {
                moves.push(mv);
            }
        }

        let mut moves: Vec<Move> = moves
            .into_iter()
            .map(|to| {
                let special = if to.rank() == 8 || to.rank() == 1 {
                    // TODO: other promotions
                    Some(SpecialMove::Promotion(PieceType::Queen))
                } else {
                    None
                };

                Move {
                    from: *pawn,
                    to,
                    special,
                }
            })
            .collect();

        // en passant
        let enpassant_rank = match self.color {
            Color::White => 5,
            Color::Black => 3,
        };
        // if we are on the fifth or third rank...
        if pawn.rank() == enpassant_rank {
            // and either square next to us...
            for file_offset in vec![-1, 1] {
                if let Some(pos) = pawn.offset(file_offset, 0) {
                    // is occupied...
                    if let Square::Occupied(piece) = board[pos] {
                        // by a pawn of the opposite color...
                        if piece.color == !self.color && piece.typ == PieceType::Pawn {
                            // who just moved...
                            if piece
                                .most_recent_move
                                .is_some_and(|ply| ply == board.ply - 1)
                            {
                                // and the target square...
                                if let Some(to) = pawn.offset(-1, up) {
                                    // is empty...
                                    if board[to].is_empty() {
                                        // we can capture en passant
                                        let enpassant = Move {
                                            from: *pawn,
                                            to,
                                            special: Some(SpecialMove::EnPassant(pos)),
                                        };
                                        moves.push(enpassant);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        moves
    }

    fn moves_bishop(&self, board: &Board, bishop: &Position) -> Vec<Move> {
        self.slide_helper(board, bishop, DIAGONALS.to_vec())
    }
    fn moves_knight(&self, board: &Board, knight: &Position) -> Vec<Move> {
        KNIGHT_MOVES
            .into_iter()
            .map(|(f, r)| knight.offset(f, r))
            .flatten()
            .filter(|pos| !board[pos].is_occupied_by(self.color))
            .map(|to| Move {
                from: *knight,
                to,
                special: None,
            })
            .collect()
    }
    fn moves_rook(&self, board: &Board, rook: &Position) -> Vec<Move> {
        self.slide_helper(board, rook, STRAIGHTS.to_vec())
    }
    fn moves_queen(&self, board: &Board, queen: &Position) -> Vec<Move> {
        self.slide_helper(board, queen, ALL_DIRECTIONS.to_vec())
    }
    fn moves_king(&self, board: &Board, king: &Position) -> Vec<Move> {
        let mut moves: Vec<Move> = ALL_DIRECTIONS
            .into_iter()
            .map(|(f, r)| king.offset(f, r))
            .flatten()
            .filter(|pos| !board[pos].is_occupied_by(self.color))
            .map(|to| Move {
                from: *king,
                to,
                special: None,
            })
            .collect();

        // castling
        let directions: Vec<i32> = vec![-1, 1];
        if self.most_recent_move.is_none() {
            for dir in directions {
                let mut can_castle: Option<Position> = None; // rooks position
                for pos in king.iterate_offset(dir, 0) {
                    if let Square::Occupied(piece) = board[pos] {
                        if piece.typ == PieceType::Rook && piece.most_recent_move.is_none() {
                            can_castle = Some(pos)
                        } else {
                            break;
                        }
                    }
                }
                if let Some(rook) = can_castle {
                    moves.push(Move {
                        from: *king,
                        to: king.offset(2 * dir, 0).unwrap(),
                        special: Some(SpecialMove::Castling(rook, king.offset(dir, 0).unwrap())),
                    })
                }
            }
        }
        moves
    }

    fn slide_helper(&self, board: &Board, from: &Position, offsets: Vec<(i32, i32)>) -> Vec<Move> {
        let mut moves = Vec::new();
        for (file, rank) in offsets {
            let mut captured = false;
            let new_moves: Vec<Move> = from
                .iterate_offset(file, rank)
                .into_iter()
                .take_while(|pos| {
                    // we need to return false on the iteration AFTER we actually capture
                    if captured {
                        return false;
                    }
                    match board[pos] {
                        Square::Empty => true,
                        Square::Occupied(piece) if piece.color == !self.color => {
                            captured = true;
                            true
                        }
                        _ => false,
                    }
                })
                .map(|to| Move {
                    from: *from,
                    to,
                    special: None,
                })
                .collect();
            moves.extend(new_moves);
        }

        moves
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
            (Color::Black, PieceType::Knight) => "♞",
            (Color::Black, PieceType::Bishop) => "♝",
            (Color::Black, PieceType::Pawn) => "♟",
        };
        write!(f, " {text} ")
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Square {
    Empty,
    Occupied(Piece),
}

impl From<(Color, PieceType)> for Square {
    fn from((color, typ): (Color, PieceType)) -> Self {
        Self::Occupied(Piece {
            color,
            typ,
            most_recent_move: None,
        })
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "   "),
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

    pub fn is_empty(&self) -> bool {
        matches!(self, Square::Empty)
    }
    pub fn is_occupied_by(&self, col: Color) -> bool {
        match self {
            Square::Occupied(piece) if piece.color == col => true,
            _ => false,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum SpecialMove {
    EnPassant(Position),          // position of the pawn that is captured
    Promotion(PieceType),         // type of piece to promote to
    Castling(Position, Position), // start and end position of the rook
}

#[derive(Debug, Copy, Clone)]
pub struct Move {
    pub from: Position,
    pub to: Position,
    pub special: Option<SpecialMove>,
}

impl Move {}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.from, self.to)?;
        Ok(())
    }
}
