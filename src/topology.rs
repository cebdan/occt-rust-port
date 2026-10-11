use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VertexId(pub u64);

#[derive(Debug, Clone, PartialEq)]
pub struct Vertex {
    pub id: VertexId,
    pub point: crate::maths::Point3,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EdgeId(pub u64);

#[derive(Debug, Clone, PartialEq)]
pub struct Edge {
    pub id: EdgeId,
    pub start: VertexId,
    pub end: VertexId,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FaceId(pub u64);

#[derive(Debug, Clone, PartialEq)]
pub struct Face {
    pub id: FaceId,
    pub vertices: Vec<VertexId>,
    pub edges: Vec<EdgeId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ShellId(pub u64);

#[derive(Debug, Clone, PartialEq)]
pub struct Shell {
    pub id: ShellId,
    pub faces: Vec<FaceId>,
}

impl Vertex {
    pub fn new(id: u64, point: crate::maths::Point3) -> Self {
        Self { id: VertexId(id), point }
    }
}

impl Edge {
    pub fn new(id: u64, start: VertexId, end: VertexId) -> Self {
        Self { id: EdgeId(id), start, end }
    }

    pub fn contains_vertex(&self, vertex: &VertexId) -> bool {
        self.start == *vertex || self.end == *vertex
    }
}

impl Face {
    pub fn new(id: u64, vertices: Vec<VertexId>, edges: Vec<EdgeId>) -> Self {
        Self { id: FaceId(id), vertices, edges }
    }

    pub fn is_closed(&self) -> bool {
        self.vertices.len() >= 3 && self.vertices.len() == self.edges.len()
    }
}

impl Shell {
    pub fn new(id: u64, faces: Vec<FaceId>) -> Self {
        Self { id: ShellId(id), faces }
    }

    pub fn unique_faces(&self) -> HashSet<FaceId> {
        self.faces.iter().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::{Edge, EdgeId, Face, FaceId, Shell, ShellId, Vertex, VertexId};
    use crate::maths::Point3;

    #[test]
    fn edge_contains_vertex() {
        let a = VertexId(1);
        let b = VertexId(2);
        let edge = Edge::new(10, a, b);
        assert!(edge.contains_vertex(&a));
        assert!(!edge.contains_vertex(&VertexId(99)));
    }

    #[test]
    fn face_is_closed_when_counts_match() {
        let face = Face::new(1, vec![VertexId(1), VertexId(2), VertexId(3)], vec![EdgeId(10), EdgeId(11), EdgeId(12)]);
        assert!(face.is_closed());
    }

    #[test]
    fn shell_unique_faces_removes_duplicates() {
        let shell = Shell::new(1, vec![FaceId(1), FaceId(1), FaceId(2)]);
        assert_eq!(shell.unique_faces().len(), 2);
    }

    #[test]
    fn vertex_can_be_instantiated() {
        let p = Point3::new(1.0, 2.0, 3.0);
        let v = Vertex::new(7, p);
        assert_eq!(v.id, VertexId(7));
        assert_eq!(v.point, p);
    }
}
