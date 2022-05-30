use crate::common::tree::Tree;
use crate::state::PlayoutData;
use crate::State;

trait Search {
    fn search(self) -> Tree<PlayoutData>;
}

impl Search for State {
    fn search(mut self) -> Tree<PlayoutData> {
        let mut root = self.tree.root_index();



        self.tree
    }
}

fn select() {}

fn expand() {}

fn simulate() {}

fn backprop() {}
