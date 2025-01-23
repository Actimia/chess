use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

use crate::pieces::{Color, Move, PieceType, Square};

#[derive(Debug, Clone, Copy)]
pub struct Board {
    squares: [Square; 64],
}

#[derive(Debug, Clone, Copy)]
pub struct Position(usize);

impl From<(usize, usize)> for Position {
    fn from((rank, file): (usize, usize)) -> Self {
        assert!(rank < 8);
        assert!(file < 8);
        Position(rank * 8 + file)
    }
}

impl From<usize> for Position {
    fn from(position: usize) -> Self {
        assert!(position < 64);
        Position(position)
    }
}

impl From<&[u8; 2]> for Position {
    fn from([file, rank]: &[u8; 2]) -> Self {
        let rank = match rank {
            b'1' => 0,
            b'2' => 8,
            b'3' => 16,
            b'4' => 24,
            b'5' => 32,
            b'6' => 40,
            b'7' => 48,
            b'8' => 56,
            _ => panic!("Invalid notation"),
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
            _ => panic!("Invalid notation"),
        };

        Position(rank + file)
    }
}

impl Position {
    pub fn rank(&self) -> usize {
        self.0 / 8
    }

    pub fn file(&self) -> usize {
        self.0 % 8
    }
}

impl Board {
    pub fn new() -> Self {
        let mut board = Board {
            squares: [Square::Empty; 64],
        };

        {
            board[b"a1"] = (Color::White, PieceType::Rook).into();
            board[b"a2"] = (Color::White, PieceType::Knight).into();
            board[b"a3"] = (Color::White, PieceType::Bishop).into();
            board[b"a4"] = (Color::White, PieceType::King).into();
            board[b"a5"] = (Color::White, PieceType::Queen).into();
            board[b"a6"] = (Color::White, PieceType::Bishop).into();
            board[b"a7"] = (Color::White, PieceType::Knight).into();
            board[b"a8"] = (Color::White, PieceType::Rook).into();
            board[b"b1"] = (Color::White, PieceType::Pawn).into();
            board[b"b2"] = (Color::White, PieceType::Pawn).into();
            board[b"b3"] = (Color::White, PieceType::Pawn).into();
            board[b"b4"] = (Color::White, PieceType::Pawn).into();
            board[b"b5"] = (Color::White, PieceType::Pawn).into();
            board[b"b6"] = (Color::White, PieceType::Pawn).into();
            board[b"b7"] = (Color::White, PieceType::Pawn).into();
            board[b"b8"] = (Color::White, PieceType::Pawn).into();

            board[b"h1"] = (Color::Black, PieceType::Rook).into();
            board[b"h2"] = (Color::Black, PieceType::Knight).into();
            board[b"h3"] = (Color::Black, PieceType::Bishop).into();
            board[b"h4"] = (Color::Black, PieceType::King).into();
            board[b"h5"] = (Color::Black, PieceType::Queen).into();
            board[b"h6"] = (Color::Black, PieceType::Bishop).into();
            board[b"h7"] = (Color::Black, PieceType::Knight).into();
            board[b"h8"] = (Color::Black, PieceType::Rook).into();
            board[b"g1"] = (Color::Black, PieceType::Pawn).into();
            board[b"g2"] = (Color::Black, PieceType::Pawn).into();
            board[b"g3"] = (Color::Black, PieceType::Pawn).into();
            board[b"g4"] = (Color::Black, PieceType::Pawn).into();
            board[b"g5"] = (Color::Black, PieceType::Pawn).into();
            board[b"g6"] = (Color::Black, PieceType::Pawn).into();
            board[b"g7"] = (Color::Black, PieceType::Pawn).into();
            board[b"g8"] = (Color::Black, PieceType::Pawn).into();
        }

        board
    }

    pub fn get_moves<T>(&self, position: T) -> Option<Vec<Move>>
    where
        T: Into<Position>,
    {
        let pos = position.into();
        self[pos].possible_moves(self, &pos)
    }
}

impl<T> Index<T> for Board
where
    T: Into<Position>,
{
    type Output = Square;

    fn index<'a>(&'a self, position: T) -> &'a Self::Output {
        let position = position.into();
        &self.squares[position.0]
    }
}

impl<T> IndexMut<T> for Board
where
    T: Into<Position>,
{
    fn index_mut<'a>(&'a mut self, position: T) -> &'a mut Self::Output {
        let position = position.into();
        &mut self.squares[position.0]
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for rank in (0..8).rev() {
            // print black on top
            for file in 0..8 {
                let square = self[(rank, file)];
                write!(f, "{}", square)?
            }
            writeln!(f, "")?
        }
        Ok(())
    }
}
