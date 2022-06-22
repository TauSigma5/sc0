use std::collections::VecDeque;

use chess::ChessMove;

const THREADS: usize = 8;

pub fn divide_work(possible_moves: &mut VecDeque<ChessMove>) -> Vec<Vec<ChessMove>> {
    // Try to split them up as evenly as possible
    let num_per_thread = possible_moves.len() / THREADS;
    let mut offset = possible_moves.len() - num_per_thread * THREADS;
    let mut work_out: Vec<Vec<ChessMove>> = vec![];

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

    work_out
}
