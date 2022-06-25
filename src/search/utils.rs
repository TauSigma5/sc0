use std::sync::atomic::{self, AtomicU32};

use atomic::Ordering;
use chess::{Board, ChessMove, Color, MoveGen};
use log::debug;

use super::MoveEval;

#[allow(dead_code)]
pub fn flip_color(input_color: Color) -> Color {
    if input_color == Color::White {
        Color::Black
    } else {
        Color::White
    }
}

#[allow(dead_code)]
pub fn dump_top_moves(moves: &Vec<MoveEval>) {
    let mut output: Vec<String> = vec![];

    for chess_move in moves {
        debug!("{}", chess_move);
    }

    
}

// Stolen shamelessly from https://github.com/rust-lang/rust/issues/72353 because
// there is no native atomic f64 support
#[derive(Debug)]
pub struct AtomicF32 {
    storage: AtomicU32,
}
#[allow(dead_code)]
impl AtomicF32 {
    pub fn new(value: f32) -> Self {
        let as_u32 = value.to_bits();
        Self {
            storage: AtomicU32::new(as_u32),
        }
    }
    pub fn store(&self, value: f32, ordering: Ordering) {
        let as_u32 = value.to_bits();
        self.storage.store(as_u32, ordering)
    }
    pub fn load(&self, ordering: Ordering) -> f32 {
        let as_u32 = self.storage.load(ordering);
        f32::from_bits(as_u32)
    }
}
impl Clone for AtomicF32 {
    fn clone(&self) -> AtomicF32 {
        Self::new(self.load(Ordering::Relaxed))
    }
}

#[allow(dead_code)]
pub fn get_quiet_moves(board: &Board, color: Color) -> Vec<ChessMove> {
    // Return all quiet moves for a color
    let mut moves = MoveGen::new_legal(board);
    moves.set_iterator_mask(*board.color_combined(flip_color(color)));

    moves.collect()
}

#[allow(dead_code)]
pub fn next_guess(alpha: f32, beta: f32, subtrees_count: i32) -> f32 {
    alpha + (beta - alpha) * (subtrees_count - 1) as f32 / subtrees_count as f32
}
