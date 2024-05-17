use std::collections::HashMap;

use crate::edge::Edge;
use crate::vertex::Vertex;

#[derive(Debug)]
struct Graph<'a> {
    vertices: HashMap<u32, Vertex>,
    edges: HashMap<(u32, u32), Edge<'a>>,
}

impl<'a> Graph<'a> {
    fn new() -> Graph<'a> {
        Graph {
            vertices: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    // TODO: add index verification (must be unique)
    fn insert_vertex(&mut self, id: u32, label: String) {
        let vertex = Vertex::new(id, label);
        self.vertices.insert(id, vertex);
    }

    /// Creates and edge given the two vertices that are on it
    // TODO: add verification if vertices exist and if edge already does
    fn insert_edge(&'a mut self, u_index: u32, v_index: u32) {
        let u = self.vertices.get(&u_index).unwrap();
        let v = self.vertices.get(&v_index).unwrap();

        let edge = Edge::new(u, v);

        self.edges.insert((u_index, v_index), edge);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn graph_creation() {
        let g = Graph::new();

        assert_eq!(g.vertices.len(), 0);
        assert_eq!(g.edges.len(), 0);
    }

    #[test]
    fn vertex_insertion() {
        let mut g = Graph::new();
        g.insert_vertex(0, String::from("Test0"));
        g.insert_vertex(1, String::from("Test1"));

        assert_eq!(g.vertices.len(), 2);

        assert_eq!(g.vertices.get(&0).unwrap().index, 0);
        assert_eq!(g.vertices.get(&0).unwrap().label, String::from("Test0"));
        assert_eq!(g.vertices.get(&1).unwrap().index, 1);
        assert_eq!(g.vertices.get(&1).unwrap().label, String::from("Test1"));
    }

    #[test]
    fn edge_insertion() {
        let mut g = Graph::new();

        g.insert_vertex(0, String::from("Test0"));
        g.insert_vertex(1, String::from("Test1"));

        g.insert_edge(0, 1);

        assert_eq!(g.edges.len(), 1);
    }
}
