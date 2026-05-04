use crate::game::GameState;
use crate::game::PieceType;

pub fn quiescence(
    game_state: &mut GameState,
    mut alpha: i32,
    mut beta: i32,
    maximizing: bool,
) -> i32 {
    let stand_pat = game_state.evaluate();

    if maximizing {
        if stand_pat >= beta {
            return stand_pat; // Beta cutoff
        }
        if stand_pat > alpha {
            alpha = stand_pat;
        }

        let mut best = stand_pat;
        for (from, to) in game_state.get_capture_move_list() {
            game_state.make_move(from, to);
            let score = quiescence(game_state, alpha, beta, false);
            game_state.undo_move();

            if score > best {
                best = score;
            }
            if score > alpha {
                alpha = score;
            }
            if alpha >= beta {
                break; // Beta cutoff
            }
        }
        best
    } else {
        if stand_pat <= alpha {
            return stand_pat; // Alpha cutoff
        }
        if stand_pat < beta {
            beta = stand_pat;
        }

        let mut best = stand_pat;
        for (from, to) in game_state.get_capture_move_list() {
            game_state.make_move(from, to);
            let score = quiescence(game_state, alpha, beta, true);
            game_state.undo_move();

            if score < best {
                best = score;
            }
            if score < beta {
                beta = score;
            }
            if alpha >= beta {
                break; // Alpha cutoff
            }
        }
        best
    }
}

pub fn search(
    game_state: &mut GameState,
    depth: usize,
    mut alpha: i32,
    mut beta: i32,
    maximizing: bool,
) -> i32 {
    if depth == 0 {
        return quiescence(game_state, alpha, beta, maximizing);
    }

    let moves = game_state.get_legal_move_list();
    if moves.is_empty() {
        return if game_state.white_is_checked || game_state.black_is_checked {
            if maximizing {
                -(30_000 + depth as i32)
            } else {
                30_000 + depth as i32
            }
        } else {
            0
        };
    }

    if maximizing {
        let mut best_score = i32::MIN + 1;
        for (from, to) in moves {
            game_state.make_move(from, to);
            let score = search(game_state, depth - 1, alpha, beta, false);
            game_state.undo_move();

            if score > best_score {
                best_score = score;
            }
            if score > alpha {
                alpha = score;
            }
            if alpha >= beta {
                break; // Beta cutoff — minimizer won't allow this
            }
        }
        best_score
    } else {
        let mut best_score = i32::MAX;
        for (from, to) in moves {
            game_state.make_move(from, to);
            let score = search(game_state, depth - 1, alpha, beta, true);
            game_state.undo_move();

            if score < best_score {
                best_score = score;
            }
            if score < beta {
                beta = score;
            }
            if alpha >= beta {
                break; // Alpha cutoff — maximizer won't allow this
            }
        }
        best_score
    }
}

pub fn get_best_move(game_state: &mut GameState, depth: usize) -> Option<(u64, u64)> {
    let maximizing = game_state.white_to_move;
    let mut alpha = i32::MIN + 1;
    let mut beta = i32::MAX;
    let mut best_score = if maximizing { i32::MIN + 1 } else { i32::MAX };
    let mut best_move = None;

    for (from, to) in game_state.get_legal_move_list() {
        game_state.make_move(from, to);
        let score = search(game_state, depth - 1, alpha, beta, !maximizing);
        game_state.undo_move();

        if maximizing && score > best_score || !maximizing && score < best_score {
            best_score = score;
            best_move = Some((from, to));
        }
        // Narrow the window at the root too
        if maximizing && score > alpha {
            alpha = score;
        } else if !maximizing && score < beta {
            beta = score;
        }
    }

    best_move
}
