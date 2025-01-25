use play::Game;
use players::{EvalPlayer, ManualStep, PrintBoard, PrintMoves, RandomPlayer, TerminalPlayer};

mod board;
mod pieces;
mod play;
mod players;

fn main() {
    let white = TerminalPlayer;
    //let white = RandomPlayer;
    //let white = EvalPlayer;
    //let white = PrintMoves::wrap(white);
    let white = PrintBoard::wrap(white);
    // let white = ManualStep::wrap(white);

    let black = RandomPlayer;
    //let black = EvalPlayer;
    let black = PrintMoves::wrap(black);
    //let black = PrintBoard::wrap(black);
    //let black = ManualStep::wrap(black);

    let mut game = Game::new(white, black);
    game.start();
}
