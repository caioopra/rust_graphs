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

    pub fn neighbors(&self) -> &Vec<VertexPtr> {
        &self.neighbors
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
    fn adding_neighbor() {
        let u = Vertex::new(0, String::from("TestU"));
        let v = Vertex::new(1, String::from("TestV"));

        Vertex::add_neighbor(u.clone(), v.clone());

        assert_eq!(u.borrow().degree, 1);
        assert_eq!(u.borrow().neighbors.contains(&v), true);

        assert_eq!(v.borrow().degree, 1);
        assert_eq!(v.borrow().neighbors.contains(&u), true);
    }

    #[test]
    fn adding_same_neighbor_twice() {
        let u = Vertex::new(0, String::from("TestU"));
        let v = Vertex::new(1, String::from("TestV"));

        Vertex::add_neighbor(u.clone(), v.clone());
        Vertex::add_neighbor(u.clone(), v.clone());
        Vertex::add_neighbor(v.clone(), u.clone());

        assert_eq!(u.borrow().degree, 1);
        assert_eq!(u.borrow().neighbors.len(), 1);

        assert_eq!(v.borrow().degree, 1);
        assert_eq!(v.borrow().neighbors.len(), 1);
    }

    #[test]
    fn correct_amount_of_neighbors() {
        let a = Vertex::new(0, String::from("Test a"));
        let b = Vertex::new(1, String::from("Test b"));
        let c = Vertex::new(2, String::from("Test c"));
        let d = Vertex::new(3, String::from("Test d"));
        let e = Vertex::new(4, String::from("Test e"));

        Vertex::add_neighbor(a.clone(), b.clone());
        Vertex::add_neighbor(a.clone(), c.clone());

        Vertex::add_neighbor(b.clone(), c.clone());

        Vertex::add_neighbor(c.clone(), d.clone());

        Vertex::add_neighbor(d.clone(), e.clone());
        Vertex::add_neighbor(d.clone(), a.clone());
        Vertex::add_neighbor(d.clone(), c.clone());

        assert_eq!(a.borrow().degree, 3);
        assert_eq!(b.borrow().degree, 2);
        assert_eq!(c.borrow().degree, 3);
        assert_eq!(d.borrow().degree, 3);
        assert_eq!(e.borrow().degree, 1);
    }
}
