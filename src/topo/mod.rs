pub mod geom;

pub trait Shape {
    fn id(&self) -> u64;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EmptyShape {
    id: u64,
}

impl EmptyShape {
    pub fn new(id: u64) -> Self {
        Self { id }
    }
}

impl Shape for EmptyShape {
    fn id(&self) -> u64 {
        self.id
    }
}
