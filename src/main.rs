mod search;
mod tests;
use chess::{self, BoardStatus, ChessMove, MoveGen};
use chess::{Board, Color};
use log::debug;
use search::transposition_table;
use std::io;
use std::sync::{Arc, Mutex};

use std::str::FromStr;

fn main() {
    env_logger::init();
    // self_play();
    testing();
    // player_play();
}

#[allow(dead_code)]
fn self_play() {
    let mut board = Board::default();
    let tt_white = Arc::new(Mutex::new(transposition_table::TransTable::new()));
    let tt_black = Arc::new(Mutex::new(transposition_table::TransTable::new()));

    loop {
        let color_to_move = Color::White;

        let engine_move =
            search::iterative_deepening_search(board, color_to_move, 7, Some(tt_white.clone()));
        board = board.make_move_new(engine_move);
        println!("Engine White Move: {}", engine_move);

        if board.status() == BoardStatus::Checkmate || board.status() == BoardStatus::Stalemate {
            break;
        }

        let color_to_move = Color::Black;

        let engine_move =
            search::iterative_deepening_search(board, color_to_move, 7, Some(tt_black.clone()));
        board = board.make_move_new(engine_move);
        println!("Engine Black Move: {}", engine_move);

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

        let color_to_move = Color::Black;

        let engine_move =
            search::iterative_deepening_search(board, color_to_move, 8, Some(tt.clone()));
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
    let board = Board::from_str("kbK5/pp6/1P6/8/8/8/8/R7 w - -")
        .expect("Invalid FEN");
    let moves = MoveGen::new_legal(&board).collect();
    let tt = Arc::new(Mutex::new(transposition_table::TransTable::new()));

    let best_move = search::negamax_root(board, 198.0, 197.0, 6, moves, tt);

    search::utils::dump_top_moves(&best_move);
    println!("Top Engine Move: {}", best_move[0]);
    */

    // Tests the response of engine after e5 from vienna gambit accepted.
   // Makes sure it doesn't miss mate in ones
   let color_to_move = Color::White;
   let board = Board::from_str("kbK5/pp6/1P6/8/8/8/8/R7 w - -").expect("Invalid FEN");
   let best_move = search::iterative_deepening_search(board, color_to_move, 6, None);
   println!("Top engine move: {}", best_move);
}
