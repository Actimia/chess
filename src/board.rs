use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

use crate::pieces::{Color, Move, Piece, PieceType, SpecialMove};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
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
        (1..8)
            .into_iter()
            .map(|i| self.offset(i * file_offset, i * rank_offset))
            .flatten()
            .collect()
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const FILES: &[u8; 8] = b"abcdefgh";
        write!(f, "{}{}", FILES[self.file()] as char, self.rank() + 1)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Board {
    squares: [Option<Piece>; 64],
    pub ply: usize,
    pub last_pawn_move: usize,
}

impl Board {
    pub fn new() -> Self {
        let mut board = Board {
            squares: [None; 64],
            ply: 0,
            last_pawn_move: 0,
        };

        fn sq(color: Color, typ: PieceType) -> Option<Piece> {
            Some(Piece {
                typ,
                color,
                most_recent_move: None,
            })
        }

        {
            board[b"a1"] = sq(Color::White, PieceType::Rook);
            board[b"b1"] = sq(Color::White, PieceType::Knight);
            board[b"c1"] = sq(Color::White, PieceType::Bishop);
            board[b"d1"] = sq(Color::White, PieceType::King);
            board[b"e1"] = sq(Color::White, PieceType::Queen);
            board[b"f1"] = sq(Color::White, PieceType::Bishop);
            board[b"g1"] = sq(Color::White, PieceType::Knight);
            board[b"h1"] = sq(Color::White, PieceType::Rook);
            board[b"a2"] = sq(Color::White, PieceType::Pawn);
            board[b"b2"] = sq(Color::White, PieceType::Pawn);
            board[b"c2"] = sq(Color::White, PieceType::Pawn);
            board[b"d2"] = sq(Color::White, PieceType::Pawn);
            board[b"e2"] = sq(Color::White, PieceType::Pawn);
            board[b"f2"] = sq(Color::White, PieceType::Pawn);
            board[b"g2"] = sq(Color::White, PieceType::Pawn);
            board[b"h2"] = sq(Color::White, PieceType::Pawn);

            board[b"a8"] = sq(Color::Black, PieceType::Rook);
            board[b"b8"] = sq(Color::Black, PieceType::Knight);
            board[b"c8"] = sq(Color::Black, PieceType::Bishop);
            board[b"d8"] = sq(Color::Black, PieceType::King);
            board[b"e8"] = sq(Color::Black, PieceType::Queen);
            board[b"f8"] = sq(Color::Black, PieceType::Bishop);
            board[b"g8"] = sq(Color::Black, PieceType::Knight);
            board[b"h8"] = sq(Color::Black, PieceType::Rook);
            board[b"a7"] = sq(Color::Black, PieceType::Pawn);
            board[b"b7"] = sq(Color::Black, PieceType::Pawn);
            board[b"c7"] = sq(Color::Black, PieceType::Pawn);
            board[b"d7"] = sq(Color::Black, PieceType::Pawn);
            board[b"e7"] = sq(Color::Black, PieceType::Pawn);
            board[b"f7"] = sq(Color::Black, PieceType::Pawn);
            board[b"g7"] = sq(Color::Black, PieceType::Pawn);
            board[b"h7"] = sq(Color::Black, PieceType::Pawn);
        }

        board
    }

    /* pub fn from_pgn(pgn: &str) -> Self {
        todo!()
    } */

    pub fn get_turn(&self) -> Color {
        if self.ply % 2 == 0 {
            Color::White
        } else {
            Color::Black
        }
    }

    pub fn get_moves(&self, position: &Position) -> Option<Vec<Move>> {
        self[position].map(|p| p.get_moves(&self, position))
    }

    pub fn is_occupied_by(
        &self,
        pos: Position,
        color: Option<Color>,
        typ: Option<PieceType>,
    ) -> bool {
        self[pos]
            .filter(|p| match color {
                Some(color) => p.color == color,
                None => true,
            })
            .filter(|p| match typ {
                Some(typ) => p.typ == typ,
                None => true,
            })
            .is_some()
    }

    pub fn get_pieces(&self, color: Color) -> Vec<(Position, Piece)> {
        self.squares
            .into_iter()
            .enumerate()
            .filter_map(|(pos, sq)| {
                if let Some(piece) = sq {
                    if piece.color == color {
                        Some((Position(pos), piece))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect()
    }

    pub fn apply(&self, mv: &Move) -> Board {
        let mut res = self.clone();

        if let Some(mut piece) = res[mv.from].clone() {
            piece.most_recent_move = Some(res.ply);
            res[mv.to] = Some(piece);
            res[mv.from] = None;

            if piece.typ == PieceType::Pawn {
                // Update the 50-move counter with the current ply
                res.last_pawn_move = self.ply
            }

            if let Some(special) = mv.special {
                println!("Special: {:?}, {}", special, mv);
                match special {
                    SpecialMove::EnPassant(pos) => res[pos] = None,
                    SpecialMove::Promotion(typ) => piece.typ = typ,
                    SpecialMove::Castling(rook_from, rook_to) => {
                        if let Some(mut rook) = res[rook_from].clone() {
                            rook.most_recent_move = Some(res.ply);
                            res[rook_to] = Some(rook);
                            res[rook_from] = None
                        }
                    }
                }
            }
        }

        res.ply += 1;

        res
    }
}

impl<T> Index<T> for Board
where
    T: Into<Position>,
{
    type Output = Option<Piece>;

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
        const FILES: &[u8; 8] = b"abcdefgh";
        for file in 0..8 {
            write!(f, "  {}", FILES[file] as char)?;
        }
        writeln!(f, "")?;
        for rank in (0..8).rev() {
            // print black on top
            write!(f, "{}", rank + 1)?;
            for file in 0..8 {
                let square = self[(rank, file)];
                match square {
                    Some(piece) => write!(f, "{}", piece)?,
                    None => write!(f, "   ")?,
                }
            }
            writeln!(f, "")?
        }
        Ok(())
    }
}
