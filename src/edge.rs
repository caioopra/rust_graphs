use crate::vertex::{Vertex, VertexPtr};

#[derive(Debug)]
pub struct Edge {
    u: VertexPtr,
    v: VertexPtr,
    weight: f32,
}

impl Edge {
    pub fn new(u: VertexPtr, v: VertexPtr, weight: f32) -> Self {
        Vertex::add_neighbor(u.clone(), v.clone());

        Edge { u, v, weight }
    }

    pub fn weight(&self) -> f32 {
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

        let edge = Edge::new(u.clone(), v.clone(), 1.0);

        assert_eq!(edge.u.borrow().index, u.borrow().index);
        assert_eq!(edge.v.borrow().index, v.borrow().index);
    }
}
