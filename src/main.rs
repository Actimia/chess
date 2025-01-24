use play::{Game, RandomPlayer, TerminalPlayer};

mod board;
mod pieces;
mod play;

fn main() {
    let white = TerminalPlayer {};
    let black = RandomPlayer {};
    let mut game = Game::new(white, black);
    game.start();
}
