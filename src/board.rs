use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

use crate::pieces::{Color, Move, PieceType, Square};

#[derive(Debug, Clone, Copy)]
pub struct Position(usize);

impl From<(usize, usize)> for Position {
    fn from((rank, file): (usize, usize)) -> Self {
        assert!(rank < 8);
        assert!(file < 8);
        Position(rank * 8 + file)
    }
}
impl From<(i32, i32)> for Position {
    fn from((rank, file): (i32, i32)) -> Self {
        assert!(rank >= 0 && rank < 8);
        assert!(file >= 0 && file < 8);
        Position((rank * 8 + file) as usize)
    }
}

impl From<usize> for Position {
    fn from(position: usize) -> Self {
        assert!(position < 64);
        Position(position)
    }
}

impl From<&Position> for Position {
    fn from(position: &Position) -> Self {
        Position(position.0)
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

    pub fn offset(&self, file_offset: i32, rank_offset: i32) -> Option<Position> {
        let rank = self.rank() as i32 + rank_offset;
        let file = self.file() as i32 + file_offset;
        if rank < 0 || rank >= 8 {
            return None;
        }
        if file < 0 || file >= 8 {
            return None;
        }
        Some((rank, file).into())
    }

    pub fn iterate_offset(&self, file_offset: i32, rank_offset: i32) -> Vec<Position> {
        // can be at most 7 steps in any direction
        let x: Vec<Position> = (1..8)
            .into_iter()
            .map(|i| self.offset(i * file_offset, i * rank_offset))
            .flatten()
            .collect();

        vec![]
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const FILES: &[u8; 8] = b"abcdefgh";
        write!(f, "{}{}", FILES[self.file()] as char, self.rank() + 1)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Board {
    squares: [Square; 64],
    pub ply: usize,
}

impl Board {
    pub fn new() -> Self {
        let mut board = Board {
            squares: [Square::Empty; 64],
            ply: 0,
        };

        {
            board[b"a1"] = (Color::White, PieceType::Rook).into();
            board[b"b1"] = (Color::White, PieceType::Knight).into();
            board[b"c1"] = (Color::White, PieceType::Bishop).into();
            board[b"d1"] = (Color::White, PieceType::King).into();
            board[b"e1"] = (Color::White, PieceType::Queen).into();
            board[b"f1"] = (Color::White, PieceType::Bishop).into();
            board[b"g1"] = (Color::White, PieceType::Knight).into();
            board[b"h1"] = (Color::White, PieceType::Rook).into();
            board[b"a2"] = (Color::White, PieceType::Pawn).into();
            board[b"b2"] = (Color::White, PieceType::Pawn).into();
            board[b"c2"] = (Color::White, PieceType::Pawn).into();
            board[b"d2"] = (Color::White, PieceType::Pawn).into();
            board[b"e2"] = (Color::White, PieceType::Pawn).into();
            board[b"f2"] = (Color::White, PieceType::Pawn).into();
            board[b"g2"] = (Color::White, PieceType::Pawn).into();
            board[b"h2"] = (Color::White, PieceType::Pawn).into();

            board[b"a8"] = (Color::Black, PieceType::Rook).into();
            board[b"b8"] = (Color::Black, PieceType::Knight).into();
            board[b"c8"] = (Color::Black, PieceType::Bishop).into();
            board[b"d8"] = (Color::Black, PieceType::King).into();
            board[b"e8"] = (Color::Black, PieceType::Queen).into();
            board[b"f8"] = (Color::Black, PieceType::Bishop).into();
            board[b"g8"] = (Color::Black, PieceType::Knight).into();
            board[b"h8"] = (Color::Black, PieceType::Rook).into();
            board[b"a7"] = (Color::Black, PieceType::Pawn).into();
            board[b"b7"] = (Color::Black, PieceType::Pawn).into();
            board[b"c7"] = (Color::Black, PieceType::Pawn).into();
            board[b"d7"] = (Color::Black, PieceType::Pawn).into();
            board[b"e7"] = (Color::Black, PieceType::Pawn).into();
            board[b"f7"] = (Color::Black, PieceType::Pawn).into();
            board[b"g7"] = (Color::Black, PieceType::Pawn).into();
            board[b"h7"] = (Color::Black, PieceType::Pawn).into();
        }

        board
    }

    pub fn get_turn(&self) -> Color {
        if self.ply % 2 == 0 {
            Color::White
        } else {
            Color::Black
        }
    }

    pub fn get_moves<T>(&self, position: T) -> Option<Vec<Move>>
    where
        T: Into<Position>,
    {
        let pos = position.into();
        self[pos].possible_moves(self, &pos)
    }

    pub fn apply(&self, mv: &Move) -> Board {
        let mut res = self.clone();

        if let Square::Occupied(mut piece) = res[mv.from].clone() {
            piece.most_recent_move = Some(res.ply);
            res[mv.to] = Square::Occupied(piece);
            res[mv.from] = Square::Empty;
            res.ply += 1;
        }

        res
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
        for file in 0..8 {
            write!(f, "  {}", file + 1)?;
        }
        writeln!(f, "")?;
        for rank in (0..8).rev() {
            // print black on top
            const RANKS: &[u8; 8] = b"abcdefgh";
            write!(f, "{}", RANKS[rank] as char)?;
            for file in 0..8 {
                let square = self[(rank, file)];
                write!(f, "{}", square)?
            }
            writeln!(f, "")?
        }
        Ok(())
    }
}
