use crate::tic_tac_toe::{TicTacToe, Action, Mark};
use cache_macro::cache;
use lru_cache::LruCache;

pub fn best_action(depth: i32, game: TicTacToe) -> Action {
    let legal_moves = game.get_legal_moves(&Mark::Bot);
    let mut best_score = std::i32::MIN;
    if legal_moves.is_empty() {
        panic!("No legal move left");
    }
    let mut best_move = Action::new(0, 0, Mark::Bot);

    for action in legal_moves {
        let score = minimax(depth, game.clone(), &action, &Mark::Bot);
        if score > best_score {
            best_move = action;
            best_score = score;
        }
    }

    return best_move;
}

#[cache(LruCache : LruCache::new(1000))]
fn minimax(depth: i32, game: TicTacToe, action: &Action, mark: &Mark) -> i32 {
    let mut theoretic_game = game.clone();
    theoretic_game.make_move(action).unwrap();
    let score = get_reward(theoretic_game.assess_position(theoretic_game.winning_len()));
    let new_mark = match mark {
        Mark::Bot => Mark::Player,
        Mark::Player => Mark::Bot
    };
    let legal_moves = theoretic_game.get_legal_moves(&new_mark);
    let mut best_score = std::i32::MIN;
    let mut worst_score = std::i32::MAX;
    if depth == 0 || score != 0 || legal_moves.len() == 0 {
        return score;
    }
    for legal_move in legal_moves {
        let score = minimax(depth - 1, theoretic_game.clone(), &legal_move, &new_mark);
        worst_score = std::cmp::min(score, worst_score);
        best_score = std::cmp::max(score, best_score);
    }
    let result = match new_mark {
        Mark::Bot => best_score,
        Mark::Player => worst_score
    };
    return result;
}

fn get_reward(score: (i32, i32)) -> i32 {
    return score.1 - score.0;
}