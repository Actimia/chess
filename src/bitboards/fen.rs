use nom::{
    branch::alt,
    bytes::tag,
    character::complete::one_of,
    combinator::{map, value},
    IResult, Parser,
};

use crate::pieces::{Color, PieceType};

use super::BitboardGame;

impl BitboardGame {
    /* fn from_fen(fen: &str) -> anyhow::Result<BitboardGame> {
      let mut board = BitboardGame::new();
      Ok(board)
    } */
}

fn parser(fen: &str) -> IResult<&str, &str> {
    //let (pieces, rest) = parse_pieces(fen)?;

    Ok((fen, fen))
}

fn parse_pieces<'a>(fen: &str) -> IResult<&str, &str> {
    let mut index = 0;

    let mut board = BitboardGame::new();

    for row in fen.split("/") {
        let mut text = row;
        loop {
            if let Ok((rest, ws)) = parse_empty(text) {
                index += ws;
                text = rest;
                continue;
            }
            if let Ok((rest, piece)) = parse_piece(text) {
                board.set(index, piece);
                text = rest;
                index += 1;
                continue;
            }
        }
    }

    Ok((fen, fen))
}

fn parse_empty(input: &str) -> IResult<&str, u32> {
    map(one_of("12345678"), |x| x.to_digit(10).unwrap()).parse(input)
    // map(u8(), |x| x as u32).parse(input)
}

fn parse_piece(input: &str) -> IResult<&str, (Color, PieceType)> {
    let black_king = value((Color::Black, PieceType::King), tag("k"));
    let black_queen = value((Color::Black, PieceType::Queen), tag("q"));
    let black_rook = value((Color::Black, PieceType::Rook), tag("r"));
    let black_bishop = value((Color::Black, PieceType::Bishop), tag("b"));
    let black_knight = value((Color::Black, PieceType::Knight), tag("n"));
    let black_pawn = value((Color::Black, PieceType::Pawn), tag("p"));

    let white_king = value((Color::White, PieceType::King), tag("K"));
    let white_queen = value((Color::White, PieceType::Queen), tag("Q"));
    let white_rook = value((Color::White, PieceType::Rook), tag("R"));
    let white_bishop = value((Color::White, PieceType::Bishop), tag("B"));
    let white_knight = value((Color::White, PieceType::Knight), tag("N"));
    let white_pawn = value((Color::White, PieceType::Pawn), tag("P"));

    alt((
        black_king,
        black_queen,
        black_rook,
        black_bishop,
        black_knight,
        black_pawn,
        white_king,
        white_queen,
        white_rook,
        white_bishop,
        white_knight,
        white_pawn,
    ))
    .parse(input)
}
