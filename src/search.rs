use std::f64::consts::SQRT_2;

use crate::common::tree::{NodeIndex, Tree};
use crate::movegen::Movegen;
use crate::state::PlayoutData;
use crate::State;

pub trait Search {
    fn search(&mut self);
}

impl Search for State {
    fn search(&mut self) {
        let root_index = self.tree.root_index();

        let selected_index = self.select(root_index);
        let expanded_index = self.expand(selected_index);

        println!("{}", selected_index);
    }
}

impl State {
    fn select(&mut self, node_index: NodeIndex) -> NodeIndex {
        let children = self.tree.children(node_index);
        if children.is_empty() {
            return node_index;
        }

        let parent_move = self.tree.get_data(node_index).mov;
        let parent_total = self.tree.get_data(node_index).total;

        self.make_move(parent_move);

        // determine best child
        let mut best_score = 0.0;
        let mut best_child_index = 0;
        for child_index in children {
            let child_data = self.tree.get_data(child_index);
            let child_wins = child_data.wins;
            let child_total = child_data.total;

            let score = ucb1(child_wins, child_total, parent_total);

            if score > best_score {
                best_score = score;
                best_child_index = child_index;
                if score == f64::MAX {
                    break;
                }
            }
        }

        self.select(best_child_index)
    }

    fn expand(&mut self, node_index: NodeIndex) -> NodeIndex {
        let moves = self.generate_moves();
        if moves.len() == 0 {
            return node_index;
        }

        let playout_data: Vec<PlayoutData> = moves.into_iter().map(PlayoutData::from).collect();
        self.tree.add_children(node_index, playout_data);

        let children_range = self.tree.children(node_index);

        // TODO: make random move, also the random move and child index can be determined immediately after the move generation
        let selected_child = self.rand.range(children_range);//children_range.start;
        self.make_move(self.tree.get_data(selected_child).mov);

        selected_child
    }

    fn simulate() {}

    fn backprop() {}
}

fn ucb1(self_wins: i64, self_total: i64, parent_total: i64) -> f64 {
    if self_total <= 0 {
        return f64::MAX;
    }

    let part1 = (self_wins as f64) / (self_total as f64);
    let part2 = SQRT_2 * ((parent_total as f64).ln() / (self_total as f64)).sqrt();

    part1 + part2
}
