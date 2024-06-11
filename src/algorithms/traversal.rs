use crate::graph::Graph;

use std::collections::{HashMap, VecDeque};

impl Graph {
    /// This function assume that the indexes of the vertices start at 1
    pub fn dfs(&self, start: u32) -> Vec<u32> {
        // preparing the necessary data structures
        let mut _visited: HashMap<u32, bool> = HashMap::new();
        let mut _distances: HashMap<u32, i32> = HashMap::new();
        let mut _queue: VecDeque<u32> = VecDeque::from([start]);

        for i in 1..self.vertices_amount() {
            _visited.insert(i, false);
            _distances.insert(i, -1);
        }
        _visited.insert(start, true);
        _distances.insert(start, 0);

        while !_queue.is_empty() {
            let vertex: u32 = _queue.pop_back().unwrap();

            for neighbor in self.vertices().get(&vertex).unwrap().borrow().neighbors() {
                let index: u32 = neighbor.borrow().index;
                if !_visited.get(&index).unwrap() {
                    _visited.insert(index, true);
                    _distances.insert(index, *_distances.get(&index).unwrap() + 1);
                    _queue.push_back(index);
                }
            }
        }

        return Vec::new();
    }
}
