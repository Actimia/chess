use play::Game;
use players::{EvalPlayer, PrintBoard, PrintMoves, TerminalPlayer};

mod board;
mod pieces;
mod play;
mod players;

fn main() {
    let white = PrintBoard::wrap(TerminalPlayer);
    //let white = PrintBoard::wrap(PrintMoves::wrap(RandomPlayer));
    // let black = PrintMoves::wrap(RandomPlayer);
    let black = PrintMoves::wrap(EvalPlayer);
    let mut game = Game::new(white, black);
    game.start();
}
