mod search;
mod tests;
use chess::{self, BoardStatus, ChessMove};
use chess::{Board, Color};
use log::debug;
use std::io;
use std::sync::{Arc, Mutex};
use search::transposition_table;
#[macro_use]
extern crate lazy_static;

use std::str::FromStr;


fn main() {
    env_logger::init();
    self_play();   
}

fn self_play() {
    let mut board = Board::default();
    let mut tt_white = Arc::new(Mutex::new(transposition_table::TransTable::new()));
    let mut tt_black = Arc::new(Mutex::new(transposition_table::TransTable::new()));

    loop {
        let color_to_move = Color::White;

        let engine_move = search::iterative_deepening_search(board, color_to_move, 7, Some(tt_white.clone()));
        board = board.make_move_new(engine_move);
        println!("Engine White Move: {}", engine_move);

        if board.status() == BoardStatus::Checkmate || board.status() == BoardStatus::Stalemate {
            break;
        }

        let color_to_move = Color::Black;

        let engine_move = search::iterative_deepening_search(board, color_to_move, 7, Some(tt_black.clone()));
        board = board.make_move_new(engine_move);
        println!("Engine Black Move: {}", engine_move);

        if board.status() == BoardStatus::Checkmate || board.status() == BoardStatus::Stalemate {
            break;
        }
    }
}

fn player_play() {
    let mut board = Board::default();
    let mut tt = Arc::new(Mutex::new(transposition_table::TransTable::new()));

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

        let engine_move = search::iterative_deepening_search(board, color_to_move, 7, Some(tt.clone()));
        board = board.make_move_new(engine_move);
        println!("Engine move: {}", engine_move);

        if board.status() == BoardStatus::Checkmate || board.status() == BoardStatus::Stalemate {
            break;
        }
    }
}

fn testing() {
    let color_to_move = Color::Black;
    let board = Board::from_str("2R2rk1/4pppp/8/8/8/8/6K1/2R5 w - - 0 1")
        .expect("Invalid FEN");
    let best_move = search::iterative_deepening_search(board, color_to_move, 7, None);
    debug!("Test");

    println!("Top Engine Move: {}", best_move);
}
