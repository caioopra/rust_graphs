use std::cell::RefCell;
use std::rc::Rc;

use crate::vertex::{Vertex, VertexPtr};

#[derive(Debug)]
pub struct Edge {
    u: VertexPtr,
    v: VertexPtr,
    weight: i32,
}

impl Edge {
    pub fn new(u: VertexPtr, v: VertexPtr) -> Self {
        Vertex::add_neighbor(u.clone(), v.clone());

        Edge { u, v, weight: 0 }
    }

    pub fn weight(&self) -> i32 {
        self.weight
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn edge_creation() {
        let u = Vertex::new(0, "U".to_string());
        let v = Vertex::new(1, "V".to_string());

        let edge = Edge::new(u.clone(), v.clone());

        assert_eq!(edge.u.borrow().index, u.borrow().index);
        assert_eq!(edge.v.borrow().index, v.borrow().index);
    }
}
