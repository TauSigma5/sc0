use std::collections::VecDeque;

use chess::ChessMove;

const THREADS: usize = 4;

pub fn divide_work(possible_moves: &mut VecDeque<ChessMove>) -> Vec<Vec<ChessMove>> {
    // Try to split them up as evenly as possible
    let mut work_out: Vec<Vec<ChessMove>> = vec![];
    for i in 0..THREADS {
        work_out.push(Vec::new());
    }

    while !possible_moves.is_empty() {
        for i in 0..THREADS {
            if let Some(possible_move) = possible_moves.pop_front() {
                work_out[i].push(possible_move);
            } else {
                break;
            }
        }
    }

    /*
    // Could use improvements in the future, since the first thread may evaluate much faster
    // than the second and third threads due to move ordering improvements.
    for _ in 0..THREADS {
        let mut thread_work = vec![];

        if num_per_thread != 0 {
            for _ in 0..num_per_thread {
                thread_work.push(
                    possible_moves
                        .pop_front()
                        .expect("Error in divide work function."),
                );
            }
        }

        if offset != 0 {
            thread_work.push(
                possible_moves
                    .pop_front()
                    .expect("Error in divide work function."),
            );
            offset -= 1;
        }

        work_out.push(thread_work);
    }

    assert!(possible_moves.len() == 0);
    */

    work_out
}
