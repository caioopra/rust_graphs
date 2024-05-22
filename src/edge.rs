use std::cell::RefCell;
use std::rc::Rc;

use crate::vertex::{Vertex, VertexPtr};

#[derive(Debug)]
pub struct Edge {
    u: VertexPtr,
    v: VertexPtr,
}

impl Edge {
    pub fn new(u: VertexPtr, v: VertexPtr) -> Self {
        Vertex::add_neighbor(u.clone(), v.clone());

        Edge { u, v }
    }
}

// TODO: create test foe Edge creation
