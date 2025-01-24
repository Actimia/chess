mod board;
mod pieces;

fn main() {
    let board = board::Board::new();
    println!("{}", board);

    let moves = board.get_moves(b"e2").unwrap();

    println!("{}", moves.len());

    moves.iter().for_each(|mv| {
        println!("{}", mv);
        let board = board.apply(mv);
        println!("{}", board);
    });
}
