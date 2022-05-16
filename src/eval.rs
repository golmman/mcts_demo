use crate::movegen::Move;

pub mod evaluate;
pub mod move2d;

#[cfg(test)]
mod evaluate_test;

trait Eval {
    fn evaluate(&self, last_move: Move) -> f32;
}
