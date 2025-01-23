mod board;
mod pieces;

fn main() {
    let board = board::Board::new();
    println!("{}", board)
}
