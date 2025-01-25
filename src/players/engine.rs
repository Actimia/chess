use std::{f64::INFINITY, fmt::Display, ops::Neg};

use rand::Rng;

use crate::{
    board::{Board, Position},
    pieces::{Color, Move, Piece, PieceType},
};

use super::Player;

pub struct EnginePlayer;

impl Player for EnginePlayer {
    fn make_move(&self, board: &Board, color: Color) -> Move {
        let (white, black) = board.count_pieces();
        let depth = if white + black < 5 {
            7
        } else if white + black < 10 {
            5
        } else {
            4
        };
        let (eval_board, eval) = negamax_search(board, depth, color);

        println!("Eval: {}", eval);
        eval_board
            .last_move
            .expect("There will always be a last move")
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Evaluation {
    Win(usize),
    Eval(f64),
    Loss(usize),
}

impl Display for Evaluation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Evaluation::Win(moves) => write!(f, "W{}", moves),
            Evaluation::Eval(eval) => write!(f, "{:.2}", eval),
            Evaluation::Loss(moves) => write!(f, "L{}", moves),
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
        let pieces = self.get_pieces(self.get_turn());
        pieces
            .iter()
            .flat_map(|(pos, _)| self.get_moves(pos))
            .flatten()
            .map(|mv| self.apply(&mv))
            .collect()
    }

    fn evaluate(&self) -> Evaluation {
        let black = self.get_pieces(Color::Black);
        if black
            .iter()
            .find(|(_, p)| p.typ == PieceType::King)
            .is_none()
        {
            return Evaluation::Win(0);
        }
        let white = self.get_pieces(Color::White);
        if white
            .iter()
            .find(|(_, p)| p.typ == PieceType::King)
            .is_none()
        {
            return Evaluation::Loss(0);
        }
        fn sum_piece_values(pieces: Vec<(Position, Piece)>) -> f64 {
            pieces
                .iter()
                .map(|(pos, piece)| match piece.typ {
                    PieceType::King => 100.0, // technically infinite, but this will probably suffice
                    PieceType::Queen => 9.0,
                    PieceType::Rook => 5.0,
                    PieceType::Bishop => 3.0,
                    PieceType::Knight => 2.75,
                    PieceType::Pawn => {
                        // values for white
                        const PAWN_VALUES: [f64; 8] = [0.0, 1.0, 1.05, 1.1, 1.25, 1.6, 2.0, 9.0];
                        let idx = match piece.color {
                            Color::White => pos.rank(),
                            Color::Black => 7 - pos.rank(),
                        };
                        PAWN_VALUES[idx]
                    }
                })
                .sum()
        }
        let black_pieces: f64 = sum_piece_values(black);
        let white_pieces: f64 = sum_piece_values(white);

        let noise: f64 = rand::thread_rng().gen_range(-0.1..=0.1);
        Evaluation::Eval(white_pieces - black_pieces + noise)
    }
}

/*
fn alpha_beta_search<State: SearchNode>(initial: &State, max_depth: usize) -> (State, f64) {
    fn inner<State: SearchNode>(
        state: &State,
        depth: usize,
        mut alpha: f64,
        mut beta: f64,
        maximize: bool,
    ) -> (State, f64) {
        let child_states = state.get_next_states();
        if depth == 0 || child_states.is_empty() {
            let eval = state.evaluate();
            // println!("leaf: {} {} {}", eval, alpha, beta);
            return (*state, eval);
        }

        if maximize {
            let mut best = -INFINITY;
            let mut best_child = None;
            for child in child_states {
                let (_, child_value) = inner(&child, depth - 1, alpha, beta, !maximize);
                if child_value > best {
                    best = child_value;
                    best_child = Some(child);
                }
                if child_value >= beta {
                    break;
                }
                alpha = alpha.max(child_value)
            }
            (best_child.unwrap(), best)
        } else {
            let mut best = INFINITY;
            let mut best_child = None;
            for child in child_states {
                let (_, child_value) = inner(&child, depth - 1, alpha, beta, !maximize);
                if child_value < best {
                    best = child_value;
                    best_child = Some(child);
                }
                if child_value <= alpha {
                    break;
                }
                beta = beta.min(child_value)
            }
            (best_child.unwrap(), best)
        }
    }

    inner(initial, max_depth, -INFINITY, INFINITY, true)
}
*/

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
        let child_nodes = node.get_next_states();
        if depth == 0 || child_nodes.is_empty() {
            let eval = match color {
                Color::Black => -node.evaluate(),
                Color::White => node.evaluate(),
            };
            // println!("leaf: {} {} {}", eval, alpha, beta);
            return (*node, eval);
        }

        let mut best_eval = Evaluation::Loss(0);
        let mut best_child = None;
        for child in child_nodes {
            let (_, child_eval) = inner(&child, depth - 1, -beta, -alpha, !color);
            let child_eval = -child_eval;
            if child_eval > best_eval {
                best_eval = child_eval;
                best_child = Some(child);
            }
            if (alpha < child_eval) {
                alpha = child_eval;
            }
            if alpha >= beta {
                break;
            }
        }
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
