mod search;
mod tests;
use chess::{self, BoardStatus, ChessMove};
use chess::{Board, Color};
use search::transposition_table;
use std::io;
use std::sync::{Arc, Mutex};

use std::str::FromStr;

fn main() {
    env_logger::init();
    // self_play();
    // testing();
    player_play();
}

#[allow(dead_code)]
fn self_play() {
    let mut board = Board::default();
    let tt_white = Arc::new(Mutex::new(transposition_table::TransTable::new()));
    let tt_black = Arc::new(Mutex::new(transposition_table::TransTable::new()));

    loop {
        let engine_move =
            search::iterative_deepening_search(board, 7, Some(tt_white.clone()));
        board = board.make_move_new(engine_move);
        println!("{}", engine_move);

        if board.status() == BoardStatus::Checkmate || board.status() == BoardStatus::Stalemate {
            break;
        }

        let engine_move =
            search::iterative_deepening_search(board, 7, Some(tt_black.clone()));
        board = board.make_move_new(engine_move);
        println!("{}", engine_move);

        if board.status() == BoardStatus::Checkmate || board.status() == BoardStatus::Stalemate {
            break;
        }
    }
}

#[allow(dead_code)]
fn player_play() {
    let mut board = Board::default();
    let tt = Arc::new(Mutex::new(transposition_table::TransTable::new()));

    loop {
        let mut buffer = String::new();
        let stdin = io::stdin(); // We get `Stdin` here.
        stdin.read_line(&mut buffer).unwrap();
        let player_move = ChessMove::from_str(buffer.trim()).unwrap();
        board = board.make_move_new(player_move);

        if board.status() == BoardStatus::Checkmate || board.status() == BoardStatus::Stalemate {
            break;
        }

        let engine_move =
            search::iterative_deepening_search(board,7, Some(tt.clone()));
        board = board.make_move_new(engine_move);
        println!("Engine move: {}", engine_move);

        if board.status() == BoardStatus::Checkmate || board.status() == BoardStatus::Stalemate {
            break;
        }
    }
}

#[allow(dead_code)]
fn testing() {
    /*
    let board = Board::from_str("4k2r/1R3R2/p3p1pp/4b3/1BnNr3/8/P1P5/5K2 w - - 1 0")
        .expect("Invalid FEN");
    let moves = MoveGen::new_legal(&board).collect();
    let tt = Arc::new(Mutex::new(transposition_table::TransTable::new()));

    let best_move = search::negamax_root(board, -f32::INFINITY, f32::INFINITY, 8, moves, tt);

    search::utils::dump_top_moves(&best_move);
    println!("Top Engine Move: {}", best_move[0]);
    */

    // Tests the response of engine after e5 from vienna gambit accepted.
    let board = Board::from_str("rnbqkb1r/pppp1ppp/5n2/4P3/5p2/2N5/PPPP2PP/R1BQKBNR b KQkq - 0 4")
        .expect("Invalid FEN");
    let best_move = search::iterative_deepening_search(board, 7, None);
    // Two possible variations, both are correct
    assert!(best_move.to_string() == "f6g8" || best_move.to_string() == "d8e7");
}
