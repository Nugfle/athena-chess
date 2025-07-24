use crate::engine::attack_tables::AttackTables;
use crate::engine::game::{Game, Move};

pub trait EvaluationEngine {
    fn get_best_move(game: &Game, at: &'static AttackTables) -> Move;
}
