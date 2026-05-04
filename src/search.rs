use crate::game::GameState;
use crate::game::PieceType;

fn piece_value(piece: PieceType) -> i32 {
    match piece {
        PieceType::Pawn => 100,
        PieceType::Knight => 300,
        PieceType::Bishop => 300,
        PieceType::Rook => 500,
        PieceType::Queen => 900,
        PieceType::King => 10_000,
    }
}

fn capture_score(game_state: &GameState, from: u64, to: u64) -> i32 {
    let attacker = game_state.get_piece_type_at(from);
    let victim = game_state.get_piece_type_at(to);

    match (attacker, victim) {
        (Some(a), Some(v)) => piece_value(v) * 10 - piece_value(a),
        _ => 0,
    }
}

fn score_move(game_state: &GameState, from: u64, to: u64) -> i32 {
    if game_state.is_capture(to) {
        capture_score(game_state, from, to)
    } else {
        0
    }
}

pub fn quiescence(
    game_state: &mut GameState,
    mut alpha: i32,
    mut beta: i32,
    maximizing: bool,
) -> i32 {
    let stand_pat = game_state.evaluate();

    if maximizing {
        if stand_pat >= beta {
            return stand_pat;
        }
        if stand_pat > alpha {
            alpha = stand_pat;
        }

        let mut best = stand_pat;

        let mut moves = game_state.get_capture_move_list();
        moves.sort_by_key(|&(from, to)| -capture_score(game_state, from, to));

        for (from, to) in moves {
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
                break;
            }
        }
        best
    } else {
        if stand_pat <= alpha {
            return stand_pat;
        }
        if stand_pat < beta {
            beta = stand_pat;
        }

        let mut best = stand_pat;

        let mut moves = game_state.get_capture_move_list();
        moves.sort_by_key(|&(from, to)| -capture_score(game_state, from, to));

        for (from, to) in moves {
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
                break;
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

    let mut moves = game_state.get_legal_move_list();

    // Sort moves for better pruning
    moves.sort_by_key(|&(from, to)| -score_move(game_state, from, to));

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
                break;
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
                break;
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

    let mut moves = game_state.get_legal_move_list();
    moves.sort_by_key(|&(from, to)| -score_move(game_state, from, to));

    for (from, to) in moves {
        game_state.make_move(from, to);
        let score = search(game_state, depth - 1, alpha, beta, !maximizing);
        game_state.undo_move();

        if maximizing && score > best_score || !maximizing && score < best_score {
            best_score = score;
            best_move = Some((from, to));
        }

        if maximizing && score > alpha {
            alpha = score;
        } else if !maximizing && score < beta {
            beta = score;
        }
    }

    best_move
}

