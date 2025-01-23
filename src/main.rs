use std::{fmt::Display, ops::{Index, IndexMut}};

#[derive(Debug, Clone, Copy)]
enum ChessError {
    InvalidSquare,
}

#[derive(Debug, Clone, Copy)]
enum Color {
    White,
    Black
}

#[derive(Debug, Clone, Copy)]
enum PieceType {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King
}


#[derive(Debug, Clone, Copy)]
enum Square {
    Empty,
    Piece(Color, PieceType)
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, " "),
            Self::Piece(col, piece) => {
                let text = match (col, piece) {
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
    }
}

#[derive(Debug, Clone, Copy)]
struct Board {
    squares: [Square; 64]
}

impl Board {
    pub fn new() -> Self {
        let mut board= Board {
            squares: [Square::Empty; 64]
        };

        {
            board.set(b"a1", Square::Piece(Color::White, PieceType::Rook));
            board.set(b"a2", Square::Piece(Color::White, PieceType::Knight));
            board.set(b"a3", Square::Piece(Color::White, PieceType::Bishop));
            board.set(b"a4", Square::Piece(Color::White, PieceType::King));
            board.set(b"a5", Square::Piece(Color::White, PieceType::Queen));
            board.set(b"a6", Square::Piece(Color::White, PieceType::Bishop));
            board.set(b"a7", Square::Piece(Color::White, PieceType::Knight));
            board.set(b"a8", Square::Piece(Color::White, PieceType::Rook));
            board.set(b"b1", Square::Piece(Color::White, PieceType::Pawn));
            board.set(b"b2", Square::Piece(Color::White, PieceType::Pawn));
            board.set(b"b3", Square::Piece(Color::White, PieceType::Pawn));
            board.set(b"b4", Square::Piece(Color::White, PieceType::Pawn));
            board.set(b"b5", Square::Piece(Color::White, PieceType::Pawn));
            board.set(b"b6", Square::Piece(Color::White, PieceType::Pawn));
            board.set(b"b7", Square::Piece(Color::White, PieceType::Pawn));
            board.set(b"b8", Square::Piece(Color::White, PieceType::Pawn));

            board.set(b"h1", Square::Piece(Color::Black, PieceType::Rook));
            board.set(b"h2", Square::Piece(Color::Black, PieceType::Knight));
            board.set(b"h3", Square::Piece(Color::Black, PieceType::Bishop));
            board.set(b"h4", Square::Piece(Color::Black, PieceType::King));
            board.set(b"h5", Square::Piece(Color::Black, PieceType::Queen));
            board.set(b"h6", Square::Piece(Color::Black, PieceType::Bishop));
            board.set(b"h7", Square::Piece(Color::Black, PieceType::Knight));
            board.set(b"h8", Square::Piece(Color::Black, PieceType::Rook));
            board.set(b"g1", Square::Piece(Color::Black, PieceType::Pawn));
            board.set(b"g2", Square::Piece(Color::Black, PieceType::Pawn));
            board.set(b"g3", Square::Piece(Color::Black, PieceType::Pawn));
            board.set(b"g4", Square::Piece(Color::Black, PieceType::Pawn));
            board.set(b"g5", Square::Piece(Color::Black, PieceType::Pawn));
            board.set(b"g6", Square::Piece(Color::Black, PieceType::Pawn));
            board.set(b"g7", Square::Piece(Color::Black, PieceType::Pawn));
            board.set(b"g8", Square::Piece(Color::Black, PieceType::Pawn));
        }

        board
    }

    pub fn set(&mut self, position: &[u8; 2], square: Square) -> Square {
        let prev = self[position];
        self[position] = square;
        prev
    }

    fn square(square: &[u8; 2]) -> usize {
        let [file, rank,] = square;
        let file = match file {
            b'a' => 0,
            b'b' => 8,
            b'c' => 16,
            b'd' => 24,
            b'e' => 32,
            b'f' => 40,
            b'g' => 48,
            b'h' => 56,
            _ => panic!("Invalid notation")
        };

        let rank = match rank {
            b'1' => 0,
            b'2' => 1,
            b'3' => 2,
            b'4' => 3,
            b'5' => 4,
            b'6' => 5,
            b'7' => 6,
            b'8' => 7,
            _ => panic!("Invalid notation")
        };

        file + rank
    }
}

impl Index<(usize, usize)> for Board {
    type Output = Square;

    fn index<'a> (&'a self, (file, rank): (usize, usize)) -> &'a Self::Output {
        let idx = rank * 8 + file;
        &self.squares[idx]
    }
}


impl IndexMut<(usize, usize)> for Board {
    fn index_mut<'a> (&'a mut self, (file, rank): (usize, usize)) -> &'a mut Self::Output {
        let idx = rank * 8 + file;
        &mut self.squares[idx]
    }
}

impl Index<&[u8; 2]> for Board {
    type Output = Square;

    fn index<'a> (&'a self, sq: &[u8; 2]) -> &'a Self::Output {
        let idx = Self::square(sq);
        &self.squares[idx]
    }
}

impl IndexMut<&[u8; 2]> for Board {

    fn index_mut<'a> (&'a mut self, sq: &[u8; 2]) -> &'a mut Self::Output {
        let idx = Self::square(sq);
        &mut self.squares[idx]
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        for i in (0..8).rev() { // print black on top
            for j in 0..8 {
                let square = self[(i, j)];
                write!(f, "{}", square)?
            }
            writeln!(f, "")?
        }
        Ok(())
    }
}


fn main() {
    let board = Board::new();
    println!("{}", board)
}
