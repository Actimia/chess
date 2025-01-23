mod board;
mod pieces;

fn main() {
    let board = board::Board::new();
    println!("{}", board);
    for mv in board.get_moves(b"e2").unwrap() {
        println!("{}", mv);
    }
}
