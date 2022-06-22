mod search;
mod tests;
use chess::{self, BoardStatus, ChessMove};
use chess::{Board, Color};
use log::debug;
use std::io;
use search::transposition_table;
#[macro_use]
extern crate lazy_static;

use std::str::FromStr;


fn main() {
    env_logger::init();
    //player_play();
    testing();
}

fn player_play() {
    let mut board = Board::default();
    let mut tt = &mut transposition_table::TransTable::new();

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

        let engine_move = search::iterative_deepening_search(board, color_to_move, 7, Some(tt));
        board = board.make_move_new(engine_move);
        println!("Engine move: {}", engine_move);

        if board.status() == BoardStatus::Checkmate || board.status() == BoardStatus::Stalemate {
            break;
        }
    }
}

fn testing() {
    let color_to_move = Color::Black;
    let board = Board::from_str("rnbqkb1r/pppp1ppp/5n2/4P3/5p2/2N5/PPPP2PP/R1BQKBNR b KQkq - 0 4")
        .expect("Invalid FEN");
    let best_move = search::iterative_deepening_search(board, color_to_move, 7, None);
    debug!("Test");

    println!("Top Engine Move: {}", best_move);
}
