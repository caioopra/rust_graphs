use std::cell::RefCell;
use std::rc::Rc;

use crate::vertex::Vertex;

#[derive(Debug)]
pub struct Edge {
    u: Rc<RefCell<Vertex>>,
    v: Rc<RefCell<Vertex>>,
}

impl Edge {
    pub fn new(u: Rc<RefCell<Vertex>>, v: Rc<RefCell<Vertex>>) -> Self {
        Edge { u, v }
    }
}
