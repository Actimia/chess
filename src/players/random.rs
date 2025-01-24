use rand::Rng;

use crate::{
    board::Board,
    pieces::{Color, Move},
};

use super::Player;

// RandomPlayer makes a random legal move
pub struct RandomPlayer;

impl Player for RandomPlayer {
    fn make_move(&self, board: &Board, color: Color) -> Move {
        let pieces = board.get_pieces(color);
        let moves: Vec<Move> = pieces
            .iter()
            .flat_map(|(pos, _)| board.get_moves(pos))
            .flatten()
            .collect();

        let random_index = rand::thread_rng().gen_range(0..moves.len());
        return moves[random_index];
    }
}
