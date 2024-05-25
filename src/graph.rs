use std::collections::HashMap;

use std::cell::RefCell;
use std::rc::Rc;

use crate::edge::Edge;
use crate::vertex::{Vertex, VertexPtr};

#[derive(Debug)]
struct Graph {
    vertices: HashMap<u32, VertexPtr>,
    edges: HashMap<(u32, u32), Edge>,
}

impl Graph {
    pub fn new() -> Graph {
        Graph {
            vertices: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    pub fn insert_vertex(&mut self, id: u32, label: String) {
        if self.vertex_exist(id) {
            return;
        }

        let vertex = Vertex::new(id, label);
        self.vertices.insert(id, vertex);
    }

    /// Creates and edge given the two vertices that are on it.
    /// This implementation suport multigraphs, so there can be more than one edge between two
    /// vertices.
    pub fn insert_edge(&mut self, u_index: u32, v_index: u32) {
        if !self.vertex_exist(u_index) || !self.vertex_exist(v_index) {
            return;
        }

        let u = self.vertices.get(&u_index).unwrap();
        let v = self.vertices.get(&v_index).unwrap();

        let edge = Edge::new(Rc::clone(u), Rc::clone(v));

        self.edges.insert((u_index, v_index), edge);
    }

    pub fn vertex_exist(&self, id: u32) -> bool {
        self.vertices.contains_key(&id)
    }

    pub fn edge_exists(&self, u_index: u32, v_index: u32) -> bool {
        self.edges.contains_key(&(u_index, v_index))
    }

    /// Gets the label of a a vertex given it's index
    ///
    /// # Arguments
    ///
    /// * `index` - the index of the Vertex which the label is being retrieved
    ///
    /// # Returns
    ///
    /// A `Result` with `Ok` and the label, or an `Err` saying that the argument vertex doesn't
    /// exist
    pub fn get_label(&self, index: u32) -> Result<String, String> {
        match self.vertices.get(&index) {
            Some(vertex) => Ok(vertex.borrow().label.clone()),
            None => Err(format!("Vertex with index {} doesn't exist", index)),
        }
    }

    /// returns the amount of vertices in the graph
    pub fn vertices_amount(&self) -> u32 {
        self.vertices.len() as u32
    }

    /// returns the amount of edges in the graph
    pub fn edges_amount(&self) -> u32 {
        self.edges.len() as u32
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

        assert_eq!(g.vertices.get(&0).unwrap().borrow().index, 0);
        assert_eq!(g.get_label(0).unwrap(), String::from("Test0"));

        assert_eq!(g.vertices.get(&1).unwrap().borrow().index, 1);
        assert_eq!(g.get_label(1).unwrap(), String::from("Test1"));
    }

    #[test]
    fn vertex_unique_id() {
        let mut g = Graph::new();
        g.insert_vertex(0, String::from("Test0"));

        assert_eq!(g.vertex_exist(0), true);
        assert_eq!(g.vertex_exist(10), false);

        g.insert_vertex(0, "Not a vertex".to_string()); // this vertex shouldn't be inserted

        assert_eq!(g.vertices_amount(), 1);
    }

    #[test]
    fn edge_insertion() {
        let mut g = Graph::new();

        g.insert_vertex(0, String::from("Test0"));
        g.insert_vertex(1, String::from("Test1"));

        g.insert_edge(0, 1);

        assert_eq!(g.edges.len(), 1);

        assert_eq!(g.vertices.get(&0).unwrap().borrow().degree, 1);
        assert_eq!(g.vertices.get(&1).unwrap().borrow().degree, 1);
    }
}
