use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Vertex {
    pub index: u32,
    pub label: String,
    pub neighbors: Vec<Rc<RefCell<Vertex>>>,
    pub degree: u32,
}

impl Vertex {
    pub fn new(index: u32, label: String) -> Vertex {
        Vertex {
            index,
            label,
            neighbors: Vec::new(),
            degree: 0,
        }
    }

    pub fn add_neighbor(self_rc: Rc<RefCell<Vertex>>, vertex: Rc<RefCell<Vertex>>) {
        if !self_rc.borrow().neighbors.contains(&vertex) {
            self_rc.borrow_mut().add_as_neighbor(vertex.clone());

            if !vertex.borrow().neighbors.contains(&self_rc) {
                vertex.borrow_mut().add_as_neighbor(self_rc.clone());
            }
        }
    }

    fn add_as_neighbor(&mut self, vertex: Rc<RefCell<Vertex>>) {
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

        assert_eq!(v.index, 0);
        assert_eq!(v.label, String::from("Test"));
        assert_eq!(v.degree, 0);
    }

    #[test]
    fn adding_neigbor() {
        let u = Rc::new(RefCell::new(Vertex::new(0, String::from("TestU"))));
        let v = Rc::new(RefCell::new(Vertex::new(1, String::from("TestV"))));

        Vertex::add_neighbor(u.clone(), v.clone());

        assert_eq!(u.borrow().degree, 1);
        assert_eq!(u.borrow().neighbors.contains(&v), true);

        assert_eq!(v.borrow().degree, 1);
        assert_eq!(v.borrow().neighbors.contains(&u), true);
    }
}
