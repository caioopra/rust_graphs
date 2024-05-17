#[derive(Debug)]
pub struct Vertex {
    pub index: u32,
    pub label: String,
}

impl Vertex {
    pub fn new(index: u32, label: String) -> Vertex {
        Vertex { index, label }
    }
}
