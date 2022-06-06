use crate::common::point2d::Point2D;
use crate::common::rand::Random;
use crate::common::tree::{NodeIndex, Tree};
use crate::common::{BOARD_SIZE, BOARD_WIDTH};
use crate::eval::{Score, SCORE_DEFEAT, SCORE_DRAW, SCORE_WIN};
use crate::movegen::Move;
use std::fmt::Debug;

#[derive(Clone, Copy, PartialEq)]
pub enum PieceType {
    Player1 = 0,
    Player2 = 1,
}

#[derive(Default, Debug)]
pub struct PlayoutData {
    pub mov: Move,
    pub wins: Score,
    pub total: Score,
}

impl PlayoutData {
    pub fn get_win_percentage(&self) -> f32 {
        if self.total == 0 {
            return 0.0;
        }

        self.wins as f32 / self.total as f32
    }

    pub fn add_win(&mut self) {
        self.wins += SCORE_WIN;
        self.total += SCORE_WIN;
    }

    pub fn add_defeat(&mut self) {
        self.wins += SCORE_DEFEAT;
        self.total += SCORE_WIN;
    }

    pub fn add_draw(&mut self) {
        self.wins += SCORE_DRAW;
        self.total += SCORE_WIN;
    }
}

impl Debug for Tree<PlayoutData> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = add_children_strings(String::new(), self, 0, 0);
        write!(f, "{}", s)
    }
}

fn add_children_strings(
    mut s: String,
    tree: &Tree<PlayoutData>,
    index: NodeIndex,
    depth: usize,
) -> String {
    if tree.get_data(index).total > 0 {
        s.push_str("·".repeat(depth).as_str());
        s.push_str(tree.nodes[index].self_index.to_string().as_str());
        s.push_str(": ");
        s.push_str(format!("{:?}", tree.get_data(index)).as_str());
        s.push('\n');
    }

    for child_index in tree.children(index) {
        s = add_children_strings(s, tree, child_index, depth + 1);
    }

    s
}

impl From<Move> for PlayoutData {
    fn from(mov: Move) -> Self {
        Self {
            mov,
            wins: 0,
            total: 0,
        }
    }
}

pub type Board = [Option<PieceType>; BOARD_SIZE];

pub struct State {
    pub board: Board,
    pub moves: Vec<Move>,
    pub rand: Random,
    pub rand2: nanorand::WyRand,
    pub tree: Tree<PlayoutData>,
}

impl State {
    pub fn new() -> Self {
        Self {
            board: [None; BOARD_SIZE],
            moves: Vec::new(),
            rand: Random::new(13424),
            rand2: nanorand::WyRand::new(),
            tree: Tree::new(),
        }
    }

    pub fn is_player1_turn(&self) -> bool {
        if self.moves.len() % 2 == 0 {
            return true;
        }

        return false;
    }

    pub fn get_piece_at(&self, p: &Point2D<i8>) -> Option<&PieceType> {
        self.board[BOARD_WIDTH * p.get_y() as usize + p.get_x() as usize].as_ref()
    }

    pub fn make_move(&mut self, mov: Move) {
        if self.moves.len() % 2 == 0 {
            self.board[usize::from(mov)] = Some(PieceType::Player1);
        } else {
            self.board[usize::from(mov)] = Some(PieceType::Player2);
        }

        self.moves.push(mov);
    }

    pub fn unmake_move(&mut self) -> Option<Move> {
        let popped_move = self.moves.pop();
        if let Some(mov) = popped_move {
            self.board[usize::from(mov)] = None;
        }
        popped_move
    }
}
