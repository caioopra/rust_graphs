use crate::graph::Graph;

use std::collections::{HashMap, VecDeque};

impl Graph {
    /// This function assume that the indexes of the vertices start at 1
    pub fn dfs(&self, start: u32) -> HashMap<u32, Vec<u32>> {
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

        println!("{:?}, _visited", _visited);
        println!("{:?}, _distances", _visited);

        while !_queue.is_empty() {
            let vertex: u32 = _queue.pop_back().unwrap();
            for neighbor in self.vertices().get(&vertex).unwrap().borrow().neighbors() {
                let index: u32 = neighbor.borrow().index;
                println!("asd {:?}, at {:?}", _visited.get(&index), index);
                if !_visited.get(&index).unwrap() {
                    _visited.insert(index, true);
                    _distances.insert(index, *_distances.get(&index).unwrap() + 1);
                    _queue.push_back(index);
                }
            }
        }

        println!("{:?}", _distances);

        return self.create_final_dict(_distances);
    }

    fn create_final_dict(&self, distances: HashMap<u32, i32>) -> HashMap<u32, Vec<u32>> {
        // { id: Vec<U32>}
        let result: HashMap<u32, Vec<u32>> = HashMap::new();

        // for (key, value) in distances.into_iter() {
        //      
        // }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dfs() {
        let graph = Graph::read_from_file("data/test.net");
        let graph = graph.unwrap();

        let res = graph.dfs(1);
        
        assert_eq!(1, 1);
    }
}
