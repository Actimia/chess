use std::{
    array::IntoIter,
    ops::{Add, AddAssign, Mul, MulAssign},
};

use crate::pieces::{Color, PieceType};

type Piece = (Color, PieceType);

const PIECES: [Piece; 12] = [
    (Color::White, PieceType::King),
    (Color::Black, PieceType::King),
    (Color::White, PieceType::Queen),
    (Color::Black, PieceType::Queen),
    (Color::White, PieceType::Rook),
    (Color::Black, PieceType::Rook),
    (Color::White, PieceType::Knight),
    (Color::Black, PieceType::Knight),
    (Color::White, PieceType::Bishop),
    (Color::Black, PieceType::Bishop),
    (Color::White, PieceType::Pawn),
    (Color::Black, PieceType::Pawn),
];

fn piece_idx((color, typ): Piece) -> usize {
    // lowest bit is color, rest is typ
    let color = match color {
        Color::White => 0,
        Color::Black => 1,
    };

    let piece = match typ {
        PieceType::King => 0,
        PieceType::Queen => 2,
        PieceType::Rook => 4,
        PieceType::Knight => 6,
        PieceType::Bishop => 8,
        PieceType::Pawn => 10,
    };
    color + piece
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct BitboardGame {
    bitboards: [Bitboard; 12],
}

impl BitboardGame {
    pub fn new(/*fen: Into<String> */) -> Self {
        Self {
            bitboards: [Bitboard(0); PIECES.len()],
        }
    }

    fn board_for(&self, piece: Piece) -> &Bitboard {
        &self.bitboards[piece_idx(piece)]
    }

    fn board_for_mut(&mut self, piece: Piece) -> &mut Bitboard {
        &mut self.bitboards[piece_idx(piece)]
    }

    pub fn get(&self, pos: u32) -> Option<Piece> {
        for (board, piece) in self.bitboards.iter().zip(PIECES.iter()) {
            if board.is_set(pos) {
                return Some(*piece);
            }
        }
        None
    }

    pub fn set(&mut self, pos: u32, piece: Piece) -> Option<Piece> {
        let prev = self.get(pos);

        let mask: u64 = 1 << pos;
        *self.board_for_mut(piece) += Bitboard(mask);
        prev
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Bitboard(u64);

impl Bitboard {
    fn is_set(&self, idx: u32) -> bool {
        self.0 & (1 << idx) != 0
    }
}

impl Add for Bitboard {
    type Output = u64;

    fn add(self, rhs: Self) -> Self::Output {
        self.0 | rhs.0
    }
}

impl AddAssign for Bitboard {
    fn add_assign(&mut self, rhs: Self) {
        (*self).0 |= rhs.0
    }
}

impl Mul for Bitboard {
    type Output = u64;

    fn mul(self, rhs: Self) -> Self::Output {
        self.0 | rhs.0
    }
}

impl MulAssign for Bitboard {
    fn mul_assign(&mut self, rhs: Self) {
        (*self).0 |= rhs.0
    }
}

struct BitboardIter {
    bitboard: u64,
}

impl Iterator for BitboardIter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.bitboard == 0 {
            None
        } else {
            let one_index = self.bitboard.trailing_zeros() + 1;
            let mask = 1 << one_index;
            self.bitboard &= mask;
            Some(one_index)
        }
    }
}

impl IntoIterator for Bitboard {
    type Item = u32;

    type IntoIter = BitboardIter;

    fn into_iter(self) -> Self::IntoIter {
        BitboardIter { bitboard: self.0 }
    }
}
