use super::attack_tables::AttackTables;
use super::{Game, Move};

pub trait EvaluationEngine {
    fn get_best_move(game: &Game, at: &'static AttackTables) -> Move;
}
