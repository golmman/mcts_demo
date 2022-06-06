pub mod evaluate;

#[cfg(test)]
mod evaluate_test;

// TODO: do we really need that distinction
/// integer used to count the number of wins and draws in the playout tree
pub type Score = i64;

/// integer used to indicate which player won or if the position is drawn / not decided yet
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Evaluation {
    WinPlayer1,
    WinPlayer2,
    Draw,
}

pub const SCORE_WIN: Score = 100;
pub const SCORE_DRAW: Score = SCORE_WIN / 2;
pub const SCORE_DEFEAT: Score = 0;

pub trait Eval {
    fn evaluate(&self) -> Evaluation;
}
