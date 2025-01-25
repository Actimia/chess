use std::{fmt::Display, ops::Neg};

use rand::Rng;

use crate::{
    board::{Board, Position},
    pieces::{Color, Move, Piece, PieceType},
};

use super::Player;

pub struct EnginePlayer;

impl Player for EnginePlayer {
    fn make_move(&self, board: &Board) -> Move {
        let (eval, best_move) = EnginePlayer::evaluate(board);

        println!("Eval: {}", eval);
        best_move
    }
}

impl EnginePlayer {
    pub fn evaluate(board: &Board) -> (Evaluation, Move) {
        let (white, black) = board.count_pieces();
        let depth = if white + black < 5 {
            4
        } else if white + black < 10 {
            4
        } else {
            4
        };
        let color = board.current_turn();
        let (eval_board, eval) = negamax_search(board, depth, color);

        (
            eval,
            eval_board
                .last_move
                .expect("There will always be a last move"),
        )
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord)]
pub enum Evaluation {
    Win(usize),
    Eval(i32),
    Loss(usize),
}

impl Evaluation {
    fn increment_depth(&self) -> Evaluation {
        match self {
            Evaluation::Win(depth) => Evaluation::Win(depth + 1),
            Evaluation::Eval(eval) => Evaluation::Eval(*eval),
            Evaluation::Loss(depth) => Evaluation::Loss(depth + 1),
        }
    }
}

impl Display for Evaluation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Evaluation::Win(ply) => write!(f, "W{}", ply),
            Evaluation::Eval(eval) => write!(f, "{:.2}", eval),
            Evaluation::Loss(ply) => write!(f, "L{}", ply),
        }
    }
}

impl PartialOrd for Evaluation {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            // wins in fewer moves are better
            (Evaluation::Win(left), Evaluation::Win(right)) => right.partial_cmp(left),
            (Evaluation::Win(_), _) => Some(std::cmp::Ordering::Greater),
            (_, Evaluation::Win(_)) => Some(std::cmp::Ordering::Less),
            (Evaluation::Eval(left), Evaluation::Eval(right)) => left.partial_cmp(right),
            (Evaluation::Eval(_), _) => Some(std::cmp::Ordering::Greater),
            (_, Evaluation::Eval(_)) => Some(std::cmp::Ordering::Less),
            // losses in more moves are better
            (Evaluation::Loss(left), Evaluation::Loss(right)) => left.partial_cmp(right),
        }
    }
}

impl Neg for Evaluation {
    type Output = Evaluation;

    fn neg(self) -> Self::Output {
        match self {
            Evaluation::Win(moves) => Evaluation::Loss(moves),
            Evaluation::Eval(eval) => Evaluation::Eval(-eval),
            Evaluation::Loss(moves) => Evaluation::Win(moves),
        }
    }
}

trait SearchNode: Sized + Copy {
    fn get_next_states(&self) -> Vec<Self>
    where
        Self: Sized + Copy;

    fn evaluate(&self) -> Evaluation;
}

impl SearchNode for Board {
    fn get_next_states(&self) -> Vec<Board> {
        let pieces = self.get_pieces(self.current_turn());
        // our king has been taken, game is over
        if let None = pieces.iter().find(|(_, p)| p.typ == PieceType::King) {
            return Vec::new();
        }
        let mut moves: Vec<Move> = pieces
            .iter()
            .flat_map(|(pos, _)| self.get_moves(pos))
            .flatten()
            .collect();

        // will search through moves that are likely to be decisive (captures etc) first
        moves.sort();

        moves.iter().map(|mv| self.apply(&mv)).collect()
    }

    fn evaluate(&self) -> Evaluation {
        let color = self.current_turn();
        let friendly = self.get_pieces(color);
        if friendly
            .iter()
            .find(|(_, p)| p.typ == PieceType::King)
            .is_none()
        {
            return Evaluation::Loss(0);
        }
        let enemy = self.get_pieces(!color);
        if enemy
            .iter()
            .find(|(_, p)| p.typ == PieceType::King)
            .is_none()
        {
            return Evaluation::Win(0);
        }
        fn sum_piece_values(pieces: Vec<(Position, Piece)>) -> i32 {
            pieces
                .iter()
                .map(|(pos, piece)| match piece.typ {
                    PieceType::King => 10000, // technically infinite, but this will probably suffice
                    PieceType::Queen => 900,
                    PieceType::Rook => 500,
                    PieceType::Bishop => 300,
                    PieceType::Knight => 275,
                    PieceType::Pawn => {
                        // values for white
                        const PAWN_VALUES: [i32; 8] = [0, 100, 105, 110, 125, 160, 200, 900];
                        let idx = match piece.color {
                            Color::White => pos.rank(),
                            Color::Black => 7 - pos.rank(),
                        };
                        PAWN_VALUES[idx]
                    }
                })
                .sum()
        }
        let enemy_pieces: i32 = sum_piece_values(enemy);
        let friendly_pieces: i32 = sum_piece_values(friendly);

        let noise: i32 = rand::thread_rng().gen_range(-10..=10);
        Evaluation::Eval(friendly_pieces - enemy_pieces + noise)
    }
}

fn negamax_search<Node: SearchNode>(
    initial: &Node,
    max_depth: usize,
    color: Color,
) -> (Node, Evaluation) {
    fn inner<Node: SearchNode>(
        node: &Node,
        depth: usize,
        mut alpha: Evaluation,
        beta: Evaluation,
        color: Color, // maximizing player
    ) -> (Node, Evaluation) {
        let indent = " ".repeat(2 * (4 - depth.max(0)));
        let child_nodes = node.get_next_states();
        if depth == 0 || child_nodes.is_empty() {
            let eval = node.evaluate();

            eprintln!(
                "{indent}leaf({:?}): eval = {}, alpha = {}, beta = {}",
                color, eval, alpha, beta
            );
            return (*node, eval);
        }

        eprintln!(
            "{indent}node({:?}): num = {}, alpha = {}, beta = {}",
            color,
            child_nodes.len(),
            alpha,
            beta,
        );
        let mut best_eval = Evaluation::Loss(0);
        let mut best_child = None;
        for child in child_nodes {
            let (_, child_eval) = inner(&child, depth - 1, -beta, -alpha, !color);
            let child_eval = -child_eval;
            eprintln!("{indent}  child: eval = {}", child_eval);
            if child_eval >= best_eval {
                eprintln!("{indent}  best child: {} >= {}", child_eval, best_eval);
                best_eval = child_eval;
                best_child = Some(child);
            }

            alpha = alpha.max(child_eval);
            if child_eval > alpha {
                eprintln!(
                    "{indent}  update alpha: {} > {}, beta = {}",
                    child_eval, alpha, beta
                );
                alpha = child_eval;
            }
            if alpha >= beta {
                eprintln!("{indent}  cutoff: alpha = {}, beta = {}", alpha, beta);
                break;
            }
        }
        let best_eval = best_eval.increment_depth();
        (best_child.unwrap(), best_eval)
    }

    inner(
        initial,
        max_depth,
        Evaluation::Loss(1),
        Evaluation::Win(1),
        color,
    )
}

#[cfg(test)]
mod tests {
    use super::Evaluation;

    #[test]
    fn test_eval_cmp() {
        let w1 = Evaluation::Win(1);
        let w4 = Evaluation::Win(4);

        let better = Evaluation::Eval(53);
        let even = Evaluation::Eval(5);
        let worse = Evaluation::Eval(-50);

        let l4 = Evaluation::Loss(4);
        let l1 = Evaluation::Loss(1);

        assert!(w1 > w4);
        assert!(w1 > better);
        assert!(w1 > l4);

        assert!(better > even);
        assert!(even > worse);

        assert!(even > l4);

        assert!(l4 > l1);
        assert!(w1 >= w1);
    }

    #[test]
    fn test_eval_invert_cmp() {
        let w1 = Evaluation::Win(1);
        let w4 = Evaluation::Win(4);

        let better = Evaluation::Eval(53);
        let l1 = Evaluation::Loss(1);

        assert_eq!(-w1, l1);
        assert!(-w4 > l1);
        assert!(better > -w4);
        assert!(better > -better);
    }
}
