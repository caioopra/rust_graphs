use std::borrow::BorrowMut;
use std::collections::HashMap;

use crate::utils::file_processor::FileProcessor;

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
    pub fn insert_edge(&mut self, u_index: u32, v_index: u32, weight: f32) {
        if !self.vertex_exist(u_index) || !self.vertex_exist(v_index) {
            return;
        }

        let u = self.vertices.get(&u_index).unwrap();
        let v = self.vertices.get(&v_index).unwrap();

        let edge = Edge::new(Rc::clone(u), Rc::clone(v), weight);

        self.edges.insert((u_index, v_index), edge);
    }

    pub fn vertex_exist(&self, id: u32) -> bool {
        self.vertices.contains_key(&id)
    }

    pub fn edge_exist(&self, u_index: u32, v_index: u32) -> bool {
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
    pub fn label(&self, index: u32) -> Result<String, String> {
        match self.vertices.get(&index) {
            Some(vertex) => Ok(vertex.borrow().label.clone()),
            None => Err(format!("Vertex with index {} doesn't exist", index)),
        }
    }

    /// Returns the degree of a vertex given it's index (-1 if doesn't exist)
    pub fn degree(&self, index: u32) -> Result<u32, i32> {
        if self.vertex_exist(index) {
            return Ok(self.vertices.get(&index).unwrap().borrow().degree);
        }

        Err(-1)
    }

    /// Returns the weight of a given index; u and v are interchangeable for this method
    pub fn weight(&self, u_index: u32, v_index: u32) -> Result<f32, i32> {
        if self.edge_exist(u_index, v_index) {
            return Ok(self.edges.get(&(u_index, v_index)).unwrap().weight());
        }

        if self.edge_exist(v_index, u_index) {
            return Ok(self.edges.get(&(v_index, u_index)).unwrap().weight());
        }

        Err(-1)
    }

    /// returns the amount of vertices in the graph
    pub fn vertices_amount(&self) -> u32 {
        self.vertices.len() as u32
    }

    /// returns the amount of edges in the graph
    pub fn edges_amount(&self) -> u32 {
        self.edges.len() as u32
    }

    /// Loads the graph stored in a file (.txt or .net)
    /// Graphs should be represented in the file in the shape:
    /// * Vertices: start a line with `*vertices <amount of vertices>`; the following lines should
    /// represent vertices by `<index> <label>`
    /// * Edges   : start a line with `*arcs` or `*edges`; the edges will be represented by
    /// `<u_index> <v_index> <weight (float)>`
    ///
    /// # Arguments
    ///
    /// * filename: full path starting from the root of the project to the file
    ///
    /// # Returns
    /// Result with the Graph if file was correctly parsed, else a &str with the error message. 
    ///
    /// # Example:
    /// ```Rust
    /// let graph = Graph::read_from_file("full/path/to/data.net");
    /// ```
    pub fn read_from_file(filename: &str) -> Result<Graph, &str> {
        let result = FileProcessor::read(filename, false);

        if result.is_err() {
            return Err(result.unwrap_err());
        }

        let mut edges = false;
        let mut graph = Graph::new();

        for row in result.unwrap().content().lines() {
            if row == "" || row.contains("*vertices") {
                continue;
            } else if row.contains("*edges") || row.contains("*arcs") {
                edges = true;
            } else if !edges {
                let parts: Vec<&str> = row.split_whitespace().collect();
                if parts.len() >= 2 {
                    let index = parts[0];
                    let label = parts[1..].join(" ");

                    graph.insert_vertex(index.parse().unwrap(), label);
                }
            } else {
                let parts: Vec<&str> = row.split_whitespace().collect();

                let u_index: u32 = parts[0].parse().unwrap();
                let v_index: u32 = parts[1].parse().unwrap();
                let weight: f32 = parts[2].parse().unwrap();

                graph.insert_edge(u_index, v_index, weight);
            }
        }

        Ok(graph)
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
        assert_eq!(g.label(0).unwrap(), String::from("Test0"));

        assert_eq!(g.vertices.get(&1).unwrap().borrow().index, 1);
        assert_eq!(g.label(1).unwrap(), String::from("Test1"));
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

        g.insert_edge(0, 1, 1.0);

        assert_eq!(g.edges.len(), 1);

        assert_eq!(g.vertices.get(&0).unwrap().borrow().degree, 1);
        assert_eq!(g.vertices.get(&1).unwrap().borrow().degree, 1);
    }

    #[test]
    fn reading_graph_from_file() {
        let graph = Graph::read_from_file("data/test.net");
        let graph = graph.unwrap();

        assert_eq!(graph.vertices_amount(), 8);
        assert_eq!(graph.edges_amount(), 11);
    }
}
