use board::Board;
use play::Game;
use players::{EnginePlayer, PrintBoard, TerminalPlayer};

mod board;
mod pieces;
mod play;
mod players;

const STARTING_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

fn main() -> anyhow::Result<()> {
    play()
    //eval()
}

fn eval() -> anyhow::Result<()> {
    //let fen = "7Q/p1pbkppp/1p2pq2/3p4/2PP4/2P2N2/P3PPPP/R3KB1R b KQ - 0 11";
    // let fen = "7k/8/8/8/8/2K3Q1/5Q2/8 w - - 0 1"; //M1 for white
    let fen = "7k/8/8/8/8/6q1/5q2/1K6 w - - 0 1"; // M1 for black

    let board = Board::from_fen(fen)?;

    println!("{board}");
    let (eval, best_move) = EnginePlayer::evaluate(&board);
    println!("Eval: {} ({})", eval, best_move);

    Ok(())
}

fn play() -> anyhow::Result<()> {
    let white = TerminalPlayer;
    // let white = RandomPlayer;
    // let white = EnginePlayer;
    // let white = PrintMoves::wrap(white);
    let white = PrintBoard::wrap(white);
    // let white = ManualStep::wrap(white);

    // let black = RandomPlayer;
    let black = EnginePlayer;
    // let black = PrintMoves::wrap(black);
    let black = PrintBoard::wrap(black);
    // let black = ManualStep::wrap(black);

    let mut game = Game::from_fen(
        //let mut game = Game::new(
        // STARTING_POSITION,
        "7k/8/8/8/8/6q1/5q2/1K6 w - - 0 1", // m1
        //"7Q/p1pbkppp/1p2pq2/3p4/2PP4/2P2N2/P3PPPP/R3KB1R b KQ - 0 11",
        //"rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2",
        white,
        black,
    )?;
    game.start();
    Ok(())
}
