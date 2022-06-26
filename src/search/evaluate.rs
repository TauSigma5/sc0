//! The super weird evaluation function. It counts up pieces for each side, then compare number of possible moves.
//! It weights the bishop slightly more heavily than
//! the knight, which is generally true for almost all cases. It used to be 3.5, but I figured
//! that the possible moves decrease from the loss of a bishop may compensate for that. Each additional move would add 0.1
//! The randomness is added so that moves with the same eval can be chosen randomly.

use chess::{Color, MoveGen};
use log::debug;
use rand::prelude::SmallRng;

// This  implements Piece Square Tables (PSQT) for each piece type. The
// PSQT's are written from White's point of view, as if looking at a chess
// diagram, with A1 on the lower left corner.
// Taken from https://github.com/mvanthoor/rustic/blob/master/src/evaluation/psqt.rs

type Psqt = [i32; 64];
const PIECE_TABLE_ARRAY: [Psqt; 6] = [PAWN_MG, KNIGHT_MG, BISHOP_MG, ROOK_MG, QUEEN_MG, KING_MG];

#[rustfmt::skip]
const KING_MG: Psqt = [
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -30,-40,-40,-50,-50,-40,-40,-30,
    -20,-30,-30,-40,-40,-30,-30,-20,
    -10,-20,-20,-20,-20,-20,-20,-10,
     20, 20,  0,  0,  0,  0, 20, 20,
     20, 30, 10,  0,  0, 10, 30, 20
];

#[rustfmt::skip]
const KING_MG_ENDGAME: Psqt = [
    -50,-40,-30,-20,-20,-30,-40,-50,
    -30,-20,-10,  0,  0,-10,-20,-30,
    -30,-10, 20, 30, 30, 20,-10,-30,
    -30,-10, 30, 40, 40, 30,-10,-30,
    -30,-10, 30, 40, 40, 30,-10,-30,
    -30,-10, 20, 30, 30, 20,-10,-30,
    -30,-30,  0,  0,  0,  0,-30,-30,
    -50,-30,-30,-30,-30,-30,-30,-50
];

#[rustfmt::skip]
const QUEEN_MG: Psqt = [
    -20,-10,-10, -5, -5,-10,-10,-20,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -10,  0,  5,  5,  5,  5,  0,-10,
     -5,  0,  5,  5,  5,  5,  0, -5,
      0,  0,  5,  5,  5,  5,  0, -5,
    -10,  5,  5,  5,  5,  5,  0,-10,
    -10,  0,  5,  0,  0,  0,  0,-10,
    -20,-10,-10, -5, -5,-10,-10,-20
];

#[rustfmt::skip]
const ROOK_MG: Psqt = [
    0,  0,  0,  0,  0,  0,  0,  0,
    5, 10, 10, 10, 10, 10, 10,  5,
   -5,  0,  0,  0,  0,  0,  0, -5,
   -5,  0,  0,  0,  0,  0,  0, -5,
   -5,  0,  0,  0,  0,  0,  0, -5,
   -5,  0,  0,  0,  0,  0,  0, -5,
   -5,  0,  0,  0,  0,  0,  0, -5,
    0,  0,  0,  5,  5,  0,  0,  0
];

#[rustfmt::skip]
const BISHOP_MG: Psqt = [
    -20,-10,-10,-10,-10,-10,-10,-20,
    -10,  0,  0,  0,  0,  0,  0,-10,
    -10,  0,  5, 10, 10,  5,  0,-10,
    -10,  5,  5, 10, 10,  5,  5,-10,
    -10,  0, 10, 10, 10, 10,  0,-10,
    -10, 10, 10, 10, 10, 10, 10,-10,
    -10,  5,  0,  0,  0,  0,  5,-10,
    -20,-10,-10,-10,-10,-10,-10,-20,
];

#[rustfmt::skip]
const KNIGHT_MG: Psqt = [
    -50,-40,-30,-30,-30,-30,-40,-50,
    -40,-20,  0,  0,  0,  0,-20,-40,
    -30,  0, 10, 15, 15, 10,  0,-30,
    -30,  5, 15, 20, 20, 15,  5,-30,
    -30,  0, 15, 20, 20, 15,  0,-30,
    -30,  5, 10, 15, 15, 10,  5,-30,
    -40,-20,  0,  5,  5,  0,-20,-40,
    -50,-40,-30,-30,-30,-30,-40,-50,
];

#[rustfmt::skip]
const PAWN_MG: Psqt = [
     0,  0,  0,  0,  0,  0,  0,  0,
    50, 50, 50, 50, 50, 50, 50, 50,
    10, 10, 20, 30, 30, 20, 10, 10,
     5,  5, 10, 25, 25, 10,  5,  5,
     0,  0,  0, 20, 20,  0,  0,  0,
     5, -5,-10,  0,  0,-10, -5,  5,
     5, 10, 10,-20,-20, 10, 10,  5,
     0,  0,  0,  0,  0,  0,  0,  0
];

#[rustfmt::skip]
pub const FLIP: [usize; 128] = [
    56, 57, 58, 59, 60, 61, 62, 63,
    48, 49, 50, 51, 52, 53, 54, 55,
    40, 41, 42, 43, 44, 45, 46, 47,
    32, 33, 34, 35, 36, 37, 38, 39,
    24, 25, 26, 27, 28, 29, 30, 31,
    16, 17, 18, 19, 20, 21, 22, 23,
     8,  9, 10, 11, 12, 13, 14, 15,
     0,  1,  2,  3,  4,  5,  6,  7,
     0,  1,  2,  3,  4,  5,  6,  7,
     8,  9, 10, 11, 12, 13, 14, 15,
    16, 17, 18, 19, 20, 21, 22, 23,
    24, 25, 26, 27, 28, 29, 30, 31,
    32, 33, 34, 35, 36, 37, 38, 39,
    40, 41, 42, 43, 44, 45, 46, 47,
    48, 49, 50, 51, 52, 53, 54, 55,
    56, 57, 58, 59, 60, 61, 62, 63,
];

#[inline(always)]
pub fn evaluate(board: chess::Board) -> f32 {
    // In the order white, black
    let mut color_eval: [f32; 2] = [0.0, 0.0];
    let mut num_of_queens_white = 0;
    let mut num_of_other_minors_white = 0;
    let mut num_of_queens_black = 0;
    let mut num_of_other_minors_black = 0;

    // In the order of pawn, knight, bishop, root, queen, king
    let piece_values: [f32; 6] = [100.0, 320.0, 330.0, 500.0, 900.0, 0.0];

    for color in chess::ALL_COLORS {
        let color_bitboard = board.color_combined(color);
        let mut color_specific_eval: f32 = 0.0;

        for (i, piece) in chess::ALL_PIECES.iter().enumerate() {
            if color == Color::White {
                if piece == &chess::Piece::Queen {
                    num_of_queens_white += 1
                } else if piece == &chess::Piece::Bishop
                    || piece == &chess::Piece::Knight
                    || piece == &chess::Piece::Rook
                {
                    num_of_other_minors_white += 1
                }
            } else {
                if piece == &chess::Piece::Queen {
                    num_of_queens_black += 1
                } else if piece == &chess::Piece::Bishop
                    || piece == &chess::Piece::Knight
                    || piece == &chess::Piece::Rook
                {
                    num_of_other_minors_black += 1
                }
            }

            let piece_bitboard = board.pieces(*piece);
            // Looks for pieces of that type of that color
            let num_of_pieces_of_type = piece_bitboard & color_bitboard;
            color_specific_eval += num_of_pieces_of_type.popcnt() as f32 * piece_values[i];

            if piece == &chess::Piece::King
                && is_endgame(
                    num_of_queens_white,
                    num_of_queens_black,
                    num_of_other_minors_white,
                    num_of_other_minors_black,
                )
            {
                let mut piece_int = num_of_pieces_of_type.0;
                for _ in 0..piece_int.count_ones() {
                    color_specific_eval += KING_MG_ENDGAME
                        [FLIP[64 * color.to_index() + piece_int.leading_zeros() as usize]]
                        as f32;
                    piece_int ^= 1 << piece_int.trailing_zeros();
                }
            } else {
                let mut piece_int = num_of_pieces_of_type.0;
                for _ in 0..piece_int.count_ones() {
                    color_specific_eval += PIECE_TABLE_ARRAY[i]
                        [FLIP[64 * color.to_index() + piece_int.leading_zeros() as usize]]
                        as f32;
                    piece_int ^= 1 << piece_int.trailing_zeros();
                }
            }
        }
        if color == Color::Black {
            color_eval[1] += color_specific_eval;
        } else {
            color_eval[0] += color_specific_eval;
        }
    }

    color_eval[0] as f32 / 100.0 - color_eval[1] as f32 / 100.0
}

fn is_endgame(white_queen: i32, black_queen: i32, white_minor: i32, black_minor: i32) -> bool {
    if white_queen == 0 && black_queen == 0 && white_minor <= 2 && black_minor <= 2 {
        return true;
    } else if white_queen == 1 && black_queen == 0 && white_minor == 0 && black_minor == 0 {
        return true;
    } else if white_queen == 0 && black_queen == 1 && white_minor == 0 && black_minor == 0 {
        return true;
    } else {
        return false;
    }
}
