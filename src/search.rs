use std::cmp::Ordering;
use std::fmt;
use std::fmt::Display;
use std::sync::atomic::AtomicBool;
use std::thread;

use chess::{Board, ChessMove, MoveGen};
use log::debug;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::sync::Arc;
use std::sync::Mutex;

use transposition_table::{Flag, TransTable, TransTableEntry};

use crate::search::utils::dump_top_moves;

mod evaluate;
pub(crate) mod transposition_table;
pub(crate) mod utils;

const HELPER_THREADS: i32 = 3;

#[derive(Debug, Copy, Clone)]
pub struct MoveEval {
    chess_move: ChessMove,
    eval: f32,
}

impl PartialOrd for MoveEval {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for MoveEval {}

impl PartialEq for MoveEval {
    fn eq(&self, other: &Self) -> bool {
        self.chess_move == other.chess_move && self.eval == other.eval
    }
}

impl Ord for MoveEval {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.eval > other.eval {
            Ordering::Greater
        } else if self.eval < other.eval {
            Ordering::Less
        } else {
            Ordering::Equal
        }
    }
}

impl Display for MoveEval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.chess_move, self.eval)
    }
}

// Uses iterative deepening technique and transposition tables to optimize faster search
pub fn iterative_deepening_search(
    board: Board,
    target_depth: i32,
    tt_raw: Option<Arc<Mutex<TransTable>>>,
) -> ChessMove {
    // start with depth 4
    let mut depth = 2;
    let mut initial_guess = 0.0;
    let mut tt: Arc<Mutex<TransTable>> =
        Arc::new(Mutex::new(transposition_table::TransTable::new()));
    let mut best_move: Option<MoveEval> = None;
    let mut rng = thread_rng();
    let stop_now = Arc::new(AtomicBool::new(false));
    // let mut handles = vec![];

    if let Some(external_table) = tt_raw {
        tt = external_table;
    }

    let possible_moves: Vec<ChessMove> = MoveGen::new_legal(&board).collect();

    while depth < target_depth + 1 {
        // the best moves from the last iteration are searched first to improve alpha-beta pruning performance
        debug!(
            "Evaluating {} positions with depth {}",
            possible_moves.len(),
            depth
        );

        for _ in 0..HELPER_THREADS {
            // Shuffle the possible moves for better performance
            let mut thread_local_move_list = possible_moves.clone();
            thread_local_move_list.shuffle(&mut rng);
            let thread_local_tt = tt.clone();
            let thread_local_stop_now = stop_now.clone();

            thread::spawn(move || {
                negamax(
                    board,
                    target_depth,
                    target_depth,
                    -f32::INFINITY,
                    f32::INFINITY,
                    thread_local_tt,
                    thread_local_stop_now,
                );
            });
        }

        // Need to rethink... this may result in two copies of the transposition table at once
        let search_result = mtdf(
            board,
            depth,
            initial_guess,
            &possible_moves,
            tt.clone(),
            stop_now.clone(),
        )
        .expect("Got empty response from MTDF");
        best_move = Some(search_result[0]);

        if search_result[0].eval > 175.0 {
            return search_result[0].chess_move;
        }
        // Best move from last depth is the first guess for current depth.
        // keep inital guesses at 0, since using different guesses misleads the engine
        initial_guess = best_move.unwrap().eval;

        depth += 1;
    }

    stop_now.store(true, std::sync::atomic::Ordering::Release);

    best_move.unwrap().chess_move
}

fn mtdf(
    board: Board,
    depth: i32,
    first_guess: f32,
    possible_moves: &[ChessMove],
    tt: Arc<Mutex<TransTable>>,
    stop_now: Arc<AtomicBool>,
) -> Option<Vec<MoveEval>> {
    // Not named alpha and beta for clarity's sake
    let mut guess = first_guess;
    let mut upperbound = f32::INFINITY;
    let mut lowerbound = -f32::INFINITY;
    let mut best_moves: Option<Vec<MoveEval>> = None;

    debug!("First guess: {}", first_guess);

    loop {
        let beta = if guess == lowerbound {
            guess + 1.0
        } else {
            guess
        };

        let results = negamax_root(
            board,
            beta - 1.0,
            beta,
            depth,
            possible_moves,
            tt.clone(),
            stop_now.clone(),
        );
        dump_top_moves(&results);

        guess = results[0].eval;
        best_moves = Some(results);

        if guess > 170.0 {
            return best_moves;
        }

        if guess < beta {
            upperbound = guess;
        } else {
            lowerbound = guess;
        }

        debug!(
            "\nUpperbound: {}\nLowerbound: {}\nBeta: {}\nDepth: {}",
            upperbound, lowerbound, beta, depth
        );

        if lowerbound >= upperbound || stop_now.load(std::sync::atomic::Ordering::Relaxed) {
            break;
        }
    }

    best_moves
}

pub fn negamax_root(
    board: Board,
    mut alpha: f32,
    beta: f32,
    max_depth: i32,
    moves: &[ChessMove],
    tt: Arc<Mutex<TransTable>>,
    stop_now: Arc<AtomicBool>,
) -> Vec<MoveEval> {
    // Returns moves in best to worst order
    let mut scores: Vec<MoveEval> = vec![];
    let mut value = -f32::INFINITY;

    for (i, possible_move) in moves.iter().enumerate() {
        debug!("Evaluating {}/{} moves", i + 1, moves.len());

        let new_board = board.make_move_new(*possible_move);

        // Check if it's a terminal node
        if new_board.status() == chess::BoardStatus::Checkmate {
            // Return 10000 and +/- for how close to checkmate it is
            let score = MoveEval {
                chess_move: *possible_move,
                eval: 10000.0,
            };
            scores.push(score);
            break;
        } else if new_board.status() == chess::BoardStatus::Stalemate {
            let score = MoveEval {
                chess_move: *possible_move,
                eval: -1000.0,
            };
            scores.push(score);
        } else {
            value = -negamax(
                new_board,
                max_depth,
                max_depth - 1,
                -beta,
                -alpha,
                tt.clone(),
                stop_now.clone(),
            );

            let score = MoveEval {
                chess_move: *possible_move,
                eval: value,
            };

            scores.push(score);

            alpha = f32::max(alpha, value);

            if alpha >= beta {
                break;
            }
        }
    }

    // Sort from best to worst
    scores.sort_by(|a, b| b.cmp(a));

    scores
}

fn negamax(
    board: chess::Board,
    max_depth: i32,
    depth_left: i32,
    mut alpha: f32,
    mut beta: f32,
    tt: Arc<Mutex<TransTable>>,
    stop_now: Arc<AtomicBool>,
) -> f32 {
    let alpha_original = alpha;

    let tt_entry = tt.lock().unwrap();
    let tt_entry_unwrapped = tt_entry.tt.get(&board.get_hash());

    match tt_entry_unwrapped {
        Some(entry) => {
            if entry.depth >= depth_left {
                if entry.flag == Flag::Exact {
                    return entry.eval;
                } else if entry.flag == Flag::Lowerbound {
                    alpha = f32::max(alpha, entry.eval);
                } else if entry.flag == Flag::Upperbound {
                    beta = f32::min(beta, entry.eval);
                }

                if alpha >= beta {
                    return entry.eval;
                }
            }
        }
        None => {}
    }

    // Unlock asap
    drop(tt_entry);

    // Check for checkmate first before transposition tables
    let current_board_status = board.status();
    if current_board_status == chess::BoardStatus::Checkmate {
        // Return 1000 and +/- for how close to checkmate it is
        if board.side_to_move() == chess::Color::White {
            return (10000 - (max_depth - depth_left)) as f32;
        } else {
            return (-10000 + (max_depth - depth_left)) as f32;
        }
    }

    // Negamax algorithm requires that evaluations be returned relative to the side being evaluated
    if depth_left == 0 {
        if board.side_to_move() == chess::Color::White {
            return evaluate::evaluate(board);
        } else {
            return -evaluate::evaluate(board);
        }
    }

    // Eventually use algorithm to sort them by potential to save time
    let possible_moves = MoveGen::new_legal(&board);
    // Use fail soft variation
    let mut value = -f32::INFINITY;

    if current_board_status == chess::BoardStatus::Stalemate {
        // Avoid stalemate at all costs but at less cost than checkmate
        if board.side_to_move() == chess::Color::White {
            value = (-1000 + (max_depth - depth_left)) as f32;
        } else {
            value = (1000 - (max_depth - depth_left)) as f32;
        }
    }

    for possible_move in possible_moves {
        if stop_now.load(std::sync::atomic::Ordering::Relaxed) {
            break;
        }

        // Check if it's a terminal node
        value = f32::max(
            value,
            -negamax(
                board.make_move_new(possible_move),
                max_depth,
                depth_left - 1,
                -beta,
                -alpha,
                tt.clone(),
                stop_now.clone(),
            ),
        );

        alpha = f32::max(alpha, value);

        if alpha >= beta {
            break;
        }
    }

    let flag: Flag;
    if value <= alpha_original {
        flag = Flag::Upperbound;
    } else if value >= beta {
        flag = Flag::Lowerbound;
    } else {
        flag = Flag::Exact;
    }

    let tt_entry = TransTableEntry {
        depth: depth_left,
        flag,
        eval: value,
    };

    tt.lock().unwrap().add_entry(board, tt_entry);

    value
}
