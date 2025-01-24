use play::{Game, ManualStep, PrintBoard, PrintMoves, RandomPlayer, TerminalPlayer};

mod board;
mod pieces;
mod play;

fn main() {
    //let white = PrintBoard::wrap(TerminalPlayer);
    let white = PrintBoard::wrap(PrintMoves::wrap(RandomPlayer));
    let black = PrintMoves::wrap(RandomPlayer);
    let mut game = Game::new(white, black);
    game.start();
}
