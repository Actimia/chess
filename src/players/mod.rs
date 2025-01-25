mod util;
pub use util::*;

mod terminal;
pub use terminal::*;

mod random;
pub use random::*;

mod engine;
pub use engine::*;

use crate::{board::Board, pieces::Move};

pub trait Player {
    fn make_move(&self, board: &Board) -> Move;
}
