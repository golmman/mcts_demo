use nanorand::Rng;
use std::f64::consts::SQRT_2;
use std::ops::Range;

use crate::common::tree::{NodeIndex, Tree};
use crate::eval::{Eval, Evaluation, Score, SCORE_WIN};
use crate::movegen::{Move, Movegen};
use crate::state::PlayoutData;
use crate::State;

pub trait Search {
    fn search(&mut self);
}

#[derive(Eq, PartialEq)]
struct ScoredMove {
    score: Score,
    mov: Move,
}

//impl Ord for ScoredMove {
//    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//        todo!()
//    }
//}

impl Search for State {
    fn search(&mut self) {
        let root_index = self.tree.root_index();

        for i in 0..30000 {
            let selected_index = self.select(root_index);
            let expanded_index = self.expand(selected_index);
            let evaluation = self.simulate();

            //println!("moves:    {:?}", self.moves);

            self.backprop(expanded_index, evaluation);

            //println!("selected: {:?}", self.tree.get_data(selected_index));
            //println!("expanded: {:?}", self.tree.get_data(expanded_index));
            //println!("eval:     {:?}", evaluation);
            //println!("moves:    {:?}", self.moves);
            //println!("tree:");
            //println!("{:?}", self.tree);
        }

        //let mut i = 0;
        //for child_index in self.tree.children(root_index) {
        //    println!(
        //        "{:>3}   {:>5}   {:.2}",
        //        i,
        //        self.tree.get_data(child_index).total / SCORE_WIN,
        //        self.tree.get_data(child_index).get_win_percentage()
        //    );
        //    i += 1;
        //}
        //println!("{}", self.tree.get_data(root_index).get_win_percentage());

        let mut win_percentages: [(f32, Move); 225] = [(f32::NAN, Move(0)); 225];

        for child_index in self.tree.children(root_index) {
            let i = usize::from(self.tree.get_data(child_index).mov);
            let value = self.tree.get_data(child_index).get_win_percentage();
            win_percentages[i].0 = value;
        }
        for i in 0..225 {
            if i % 15 == 0 {
                println!();
            }

            if win_percentages[i].0.is_nan() {
                print!("  NAN ");
            } else {
                print!(" {:.3}", win_percentages[i].0);
            }
        }
        println!();

        //win_percentages.sort();
    }
}

impl State {
    fn select(&mut self, node_index: NodeIndex) -> NodeIndex {
        let children = self.tree.children(node_index);
        if children.is_empty() {
            return node_index;
        }

        let parent_total = self.tree.get_data(node_index).total;

        // determine best child
        let mut best_score = 0.0;
        let mut best_child_indices = Vec::<NodeIndex>::new();
        for child_index in children {
            let child_data = self.tree.get_data(child_index);
            let child_wins = child_data.wins;
            let child_total = child_data.total;

            let score = ucb1(child_wins, child_total, parent_total);

            if score == best_score {
                best_child_indices.push(child_index);
            }

            if score > best_score {
                best_score = score;
                best_child_indices = vec![child_index];
            }
        }

        let best_child_index = best_child_indices[self.rng(Range {
            start: 0,
            end: best_child_indices.len() - 1,
        })];

        self.make_move(self.tree.get_data(best_child_index).mov);
        self.select(best_child_index)
    }

    fn expand(&mut self, node_index: NodeIndex) -> NodeIndex {
        // TODO: double eval in simulate??
        if self.evaluate() != Evaluation::Draw {
            return node_index;
        }

        let moves = self.generate_moves();
        if moves.len() == 0 {
            return node_index;
        }

        let playout_data: Vec<PlayoutData> = moves.into_iter().map(PlayoutData::from).collect();
        self.tree.add_children(node_index, playout_data);

        let children_range = self.tree.children(node_index);

        // TODO: make random move, also the random move and child index can be determined immediately after the move generation
        let selected_child = self.rng(children_range); //children_range.start;
        self.make_move(self.tree.get_data(selected_child).mov);

        selected_child
    }

    fn simulate(&mut self) -> Evaluation {
        loop {
            let evaluation = self.evaluate();
            if evaluation != Evaluation::Draw {
                return evaluation;
            }

            let moves = self.generate_moves();
            if moves.len() == 0 {
                return Evaluation::Draw;
            }

            let random_move = moves[self.rng(Range {
                start: 0,
                end: moves.len(),
            })];

            self.make_move(random_move);
        }
    }

    fn backprop(&mut self, node_index: NodeIndex, evaluation: Evaluation) {
        let last_move_in_tree = self.tree.get_data(node_index).mov;

        while self.moves.len() > 0 {
            let back_move = self.moves[self.moves.len() - 1];
            if back_move == last_move_in_tree {
                break;
            }
            self.unmake_move();
        }

        self.update_playout_data(node_index, evaluation);
    }

    fn update_playout_data(&mut self, node_index: NodeIndex, evaluation: Evaluation) {
        //if self.is_player1_turn() {
        //    match evaluation {
        //        Evaluation::WinPlayer1 => self.tree.get_data_mut(node_index).add_win(),
        //        Evaluation::WinPlayer2 => self.tree.get_data_mut(node_index).add_defeat(),
        //        Evaluation::Draw => self.tree.get_data_mut(node_index).add_draw(),
        //    }
        //} else {
        //    match evaluation {
        //        Evaluation::WinPlayer1 => self.tree.get_data_mut(node_index).add_defeat(),
        //        Evaluation::WinPlayer2 => self.tree.get_data_mut(node_index).add_win(),
        //        Evaluation::Draw => self.tree.get_data_mut(node_index).add_draw(),
        //    }
        //}
        if self.is_player1_turn() {
            match evaluation {
                Evaluation::WinPlayer1 => self.tree.get_data_mut(node_index).add_defeat(),
                Evaluation::WinPlayer2 => self.tree.get_data_mut(node_index).add_win(),
                Evaluation::Draw => self.tree.get_data_mut(node_index).add_draw(),
            }
        } else {
            match evaluation {
                Evaluation::WinPlayer1 => self.tree.get_data_mut(node_index).add_win(),
                Evaluation::WinPlayer2 => self.tree.get_data_mut(node_index).add_defeat(),
                Evaluation::Draw => self.tree.get_data_mut(node_index).add_draw(),
            }
        }

        if let Some(parent_index) = self.tree.parent(node_index) {
            self.unmake_move();
            self.update_playout_data(parent_index, evaluation);
        }
    }

    fn rng(&mut self, r: Range<usize>) -> usize {
        //self.rand.range(r)
        self.rand2.generate_range(r)
    }
}

fn ucb1(self_wins: i64, self_total: i64, parent_total: i64) -> f64 {
    //const C: f64 = SQRT_2;
    const C: f64 = 4.0;
    //const C: f64 = 100.0;

    if self_total <= 0 {
        return f64::MAX;
    }

    let exploitation = (self_wins as f64) / (self_total as f64);
    let exploration = C * ((parent_total as f64).ln() / (self_total as f64)).sqrt();

    exploitation + exploration
}
