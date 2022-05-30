use std::fmt::Debug;
use std::ops::Range;

struct Node<T> {
    parent_index: Option<usize>,
    self_index: usize,
    children_index_range: Range<usize>,
    pub data: T,
}

pub struct Tree<T> {
    nodes: Vec<Node<T>>,
}

impl<T: Default> Tree<T> {
    pub fn new() -> Self {
        Self {
            nodes: vec![Node {
                parent_index: None,
                self_index: 0,
                children_index_range: Range { start: 0, end: 0 },
                data: T::default(),
            }],
        }
    }

    pub fn root_index(&mut self) -> usize {
        0
    }

    pub fn set_data(&mut self, node_index: usize, data: T) {
        self.nodes[node_index].data = data;
    }

    pub fn get_data(&self, node_index: usize) -> &T {
        &self.nodes[node_index].data
    }

    pub fn range(&self, node_index: usize) -> Range<usize> {
        self.nodes[node_index].children_index_range.clone()
    }

    pub fn add_children(&mut self, node_index: usize, children_data: Vec<T>) {
        debug_assert!(
            self.nodes[node_index].children_index_range.is_empty(),
            "can't add children to a node which already has some"
        );

        self.nodes[node_index].children_index_range = Range {
            start: self.nodes.len(),
            end: self.nodes.len() + children_data.len(),
        };

        let mut self_index = self.nodes.len();
        for child_data in children_data {
            self.nodes.push(Node {
                parent_index: Some(self.nodes[node_index].self_index),
                self_index,
                children_index_range: Range { start: 0, end: 0 },
                data: T::default(),
            });
            self_index += 1;
        }
    }
}

impl<T: Default + Debug> Debug for Tree<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = add_children_strings::<T>(String::new(), self, 0, 0);
        write!(f, "{}", s)
    }
}

fn add_children_strings<T: Default + Debug>(
    mut s: String,
    tree: &Tree<T>,
    index: usize,
    depth: usize,
) -> String {
    s.push_str("·".repeat(depth).as_str());
    s.push_str(tree.nodes[index].self_index.to_string().as_str());
    s.push_str(": ");
    s.push_str(format!("{:?}", tree.get_data(index)).as_str());
    s.push('\n');

    for child_index in tree.range(index) {
        s = add_children_strings::<T>(s, tree, child_index, depth + 1);
    }

    s
}
