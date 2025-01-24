mod util;
pub use util::*;

mod terminal;
pub use terminal::*;

mod random;
pub use random::*;

mod eval;
pub use eval::*;

use crate::{
    board::Board,
    pieces::{Color, Move},
};

pub trait Player {
    fn make_move(&self, board: &Board, color: Color) -> Move;
}
