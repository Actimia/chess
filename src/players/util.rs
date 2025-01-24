use std::io;

use crate::{
    board::Board,
    pieces::{Color, Move},
};

use super::Player;

pub struct PrintBoard<P: Player> {
    player: P,
}

impl<P: Player> Player for PrintBoard<P> {
    fn make_move(&self, board: &Board, color: Color) -> Move {
        println!("{}", board);
        println!();
        self.player.make_move(board, color)
    }
}

impl<P: Player> PrintBoard<P> {
    pub fn wrap(player: P) -> Self {
        Self { player }
    }
}

// ---

pub struct PrintMoves<P: Player> {
    player: P,
}

impl<P: Player> Player for PrintMoves<P> {
    fn make_move(&self, board: &Board, color: Color) -> Move {
        let mv = self.player.make_move(board, color);
        println!("{}", mv);
        mv
    }
}

impl<P: Player> PrintMoves<P> {
    pub fn wrap(player: P) -> Self {
        Self { player }
    }
}

// ---

pub struct ManualStep<P: Player> {
    player: P,
}

impl<P: Player> Player for ManualStep<P> {
    fn make_move(&self, board: &Board, color: Color) -> Move {
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input);
        self.player.make_move(board, color)
    }
}

impl<P: Player> ManualStep<P> {
    pub fn wrap(player: P) -> Self {
        Self { player }
    }
}
