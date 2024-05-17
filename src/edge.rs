use crate::vertex::Vertex;

#[derive(Debug)]
pub struct Edge<'a> {
    u: &'a Vertex,
    v: &'a Vertex,
}

impl<'a> Edge<'a> {
    pub fn new(u: &'a Vertex, v: &'a Vertex) -> Self {
        Edge { u, v }
    }
}

