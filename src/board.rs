use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

use anyhow::{bail, Context};

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

pub type Squares = [Option<Piece>; 64];

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Board {
    pub squares: Squares,
    pub ply: usize,
    pub last_pawn_move: usize,
    pub last_move: Option<Move>,
}

impl Board {
    pub fn new(fen: Option<String>) -> anyhow::Result<Self> {
        const STARTING_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let fen = fen.unwrap_or(STARTING_POSITION.into());

        let parts: Vec<&str> = fen.split(" ").collect();

        if parts.len() != 6 {
            bail!("Not correct amount of pieces");
        }

        fn sq(color: Color, typ: PieceType) -> Option<Piece> {
            Some(Piece {
                typ,
                color,
                most_recent_move: None,
            })
        }

        let active = match parts[1] {
            "w" => 0,
            "b" => 1,
            _ => bail!("invalid active field"),
        };
        let halfmoves: usize = parts[4].parse().context("could not parse half-moves")?;
        let fullmoves: usize = parts[5].parse().context("could not parse full-moves")?;
        let ply = (fullmoves - 1) * 2 + active;
        let mut board = Board {
            squares: [None; 64],
            ply,
            last_pawn_move: ply - halfmoves,
            last_move: None,
        };

        let pieces = parts[0];
        for (rank, rank_fen) in pieces.split("/").enumerate() {
            let mut file: usize = 0;

            let letters: Vec<&str> = rank_fen.split("").collect();
            for ch in letters {
                if let Ok(offset) = ch.parse::<usize>() {
                    file += offset
                } else {
                    let piece = match ch {
                        "" => None,
                        "P" => sq(Color::White, PieceType::Pawn),
                        "N" => sq(Color::White, PieceType::Knight),
                        "B" => sq(Color::White, PieceType::Bishop),
                        "R" => sq(Color::White, PieceType::Rook),
                        "Q" => sq(Color::White, PieceType::Queen),
                        "K" => sq(Color::White, PieceType::King),
                        "p" => sq(Color::Black, PieceType::Pawn),
                        "n" => sq(Color::Black, PieceType::Knight),
                        "b" => sq(Color::Black, PieceType::Bishop),
                        "r" => sq(Color::Black, PieceType::Rook),
                        "q" => sq(Color::Black, PieceType::Queen),
                        "k" => sq(Color::Black, PieceType::King),
                        _ => bail!("unknown piece"),
                    };
                    if piece.is_some() {
                        board[(7 - rank, file)] = piece;
                        file += 1;
                    }
                }
            }
        }

        // todo: implement castling and enpassant
        let _castling = parts[2];
        let _enpassant = parts[3];

        Ok(board)
    }

    pub fn current_turn(&self) -> Color {
        if self.ply % 2 == 0 {
            Color::White
        } else {
            Color::Black
        }
    }

    pub fn get_moves(&self, position: &Position) -> Option<Vec<Move>> {
        self[position].map(|p| p.get_moves(&self, position))
    }

    pub fn count_pieces(&self) -> (u8, u8) {
        let mut white: u8 = 0;
        let mut black: u8 = 0;
        for sq in self.squares {
            match sq {
                Some(p) => match p.color {
                    Color::White => white += 1,
                    Color::Black => black += 1,
                },
                None => {}
            }
        }
        (white, black)
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
        res.last_move = Some(*mv);

        if let Some(mut piece) = res[mv.from].clone() {
            piece.most_recent_move = Some(res.ply);
            res[mv.to] = Some(piece);
            res[mv.from] = None;

            if piece.typ == PieceType::Pawn {
                // Update the 50-move counter with the current ply
                res.last_pawn_move = self.ply
            }

            if let Some(special) = mv.special {
                match special {
                    SpecialMove::EnPassant(pos) => res[pos] = None,
                    SpecialMove::Promotion(new_typ) => {
                        piece.typ = new_typ;
                        res[mv.to] = Some(piece); // second assign is needed
                    }
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
        write!(f, " ")?;
        for file in 0..8 {
            write!(f, " {}", FILES[file] as char)?;
        }
        writeln!(f, "")?;
        for rank in (0..8).rev() {
            // print black on top
            write!(f, "{}", rank + 1)?;
            for file in 0..8 {
                let pos: Position = (rank, file).into();
                let square = self[pos];
                let prefix = match self.last_move {
                    Some(mv) if mv.to == pos => ">",
                    Some(mv) if mv.from == pos => ">",
                    _ => " ",
                };
                match square {
                    Some(piece) => write!(f, "{}{}", prefix, piece)?,
                    None => write!(f, "{} ", prefix)?,
                }
            }
            writeln!(f, "")?
        }
        Ok(())
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_fen_1() {
        let fen = "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2";
        let board = Board::new(Some(fen.into())).unwrap();
        assert!(board[b"f3"].is_some());
        assert_eq!(board.ply, 3);
    }

    #[test]
    fn test_fen_2() {
        let fen = "7Q/p1pbkppp/1p2pq2/3p4/2PP4/2P2N2/P3PPPP/R3KB1R b KQ - 0 11";
        let board = Board::new(Some(fen.into())).unwrap();
        assert!(board[b"h8"].is_some());
        assert_eq!(board.ply, 21);
    }
}
