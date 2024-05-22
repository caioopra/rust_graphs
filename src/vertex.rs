use std::cell::RefCell;
use std::rc::Rc;

pub type VertexPtr = Rc<RefCell<Vertex>>;

#[derive(Debug)]
pub struct Vertex {
    pub index: u32,
    pub label: String,
    pub neighbors: Vec<VertexPtr>,
    pub degree: u32,
}

impl Vertex {
    /// Returns a Reference Counting pointer to a RefCell to a vertex (Rc<RefCell<Vertex>>)
    pub fn new(index: u32, label: String) -> VertexPtr {
        Rc::new(RefCell::new(Vertex {
            index,
            label,
            neighbors: Vec::new(),
            degree: 0,
        }))
    }

    pub fn add_neighbor(self_rc: VertexPtr, vertex: VertexPtr) {
        if !self_rc.borrow().neighbors.contains(&vertex) {
            self_rc.borrow_mut().add_as_neighbor(vertex.clone());

            if !vertex.borrow().neighbors.contains(&self_rc) {
                vertex.borrow_mut().add_as_neighbor(self_rc.clone());
            }
        }
    }

    fn add_as_neighbor(&mut self, vertex: VertexPtr) {
        self.neighbors.push(vertex);
        self.degree += 1;
    }
}

impl PartialEq for Vertex {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vertex_creation() {
        let v = Vertex::new(0, String::from("Test"));

        assert_eq!(v.borrow().index, 0);
        assert_eq!(v.borrow().label, String::from("Test"));
        assert_eq!(v.borrow().degree, 0);
    }

    #[test]
    fn adding_neigbor() {
        let u = Vertex::new(0, String::from("TestU"));
        let v = Vertex::new(1, String::from("TestV"));

        Vertex::add_neighbor(u.clone(), v.clone());

        assert_eq!(u.borrow().degree, 1);
        assert_eq!(u.borrow().neighbors.contains(&v), true);

        assert_eq!(v.borrow().degree, 1);
        assert_eq!(v.borrow().neighbors.contains(&u), true);
    }
}
