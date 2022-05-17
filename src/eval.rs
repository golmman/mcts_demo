pub mod evaluate;

#[cfg(test)]
mod evaluate_test;

trait Eval {
    fn evaluate(&self) -> f32;
}
