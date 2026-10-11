use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VertexId(pub u64);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EdgeId(pub u64);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FaceId(pub u64);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ShellId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn origin() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Vertex {
    pub id: VertexId,
    pub point: Point3,
}

impl Vertex {
    pub fn new(id: u64, point: Point3) -> Self {
        Self { id: VertexId(id), point }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Edge {
    pub id: EdgeId,
    pub start: VertexId,
    pub end: VertexId,
}

impl Edge {
    pub fn new(id: u64, start: VertexId, end: VertexId) -> Self {
        Self { id: EdgeId(id), start, end }
    }

    pub fn contains_vertex(&self, vertex: VertexId) -> bool {
        self.start == vertex || self.end == vertex
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Face {
    pub id: FaceId,
    pub vertices: Vec<VertexId>,
    pub edges: Vec<EdgeId>,
}

impl Face {
    pub fn new(id: u64, vertices: Vec<VertexId>, edges: Vec<EdgeId>) -> Self {
        Self { id: FaceId(id), vertices, edges }
    }

    pub fn is_closed(&self) -> bool {
        self.vertices.len() >= 3 && self.vertices.len() == self.edges.len()
    }

    pub fn has_vertex(&self, vertex: VertexId) -> bool {
        self.vertices.contains(&vertex)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Shell {
    pub id: ShellId,
    pub faces: Vec<FaceId>,
}

impl Shell {
    pub fn new(id: u64, faces: Vec<FaceId>) -> Self {
        Self { id: ShellId(id), faces }
    }

    pub fn unique_faces(&self) -> HashSet<FaceId> {
        self.faces.iter().cloned().collect()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Topology {
    vertices: HashMap<VertexId, Vertex>,
    edges: HashMap<EdgeId, Edge>,
    faces: HashMap<FaceId, Face>,
    shells: HashMap<ShellId, Shell>,
}

impl Topology {
    pub fn new() -> Self {
        Self {
            vertices: HashMap::new(),
            edges: HashMap::new(),
            faces: HashMap::new(),
            shells: HashMap::new(),
        }
    }

    pub fn add_vertex(&mut self, vertex: Vertex) {
        self.vertices.insert(vertex.id, vertex);
    }

    pub fn add_edge(&mut self, edge: Edge) {
        self.edges.insert(edge.id, edge);
    }

    pub fn add_face(&mut self, face: Face) {
        self.faces.insert(face.id, face);
    }

    pub fn add_shell(&mut self, shell: Shell) {
        self.shells.insert(shell.id, shell);
    }

    pub fn vertex(&self, id: VertexId) -> Option<&Vertex> {
        self.vertices.get(&id)
    }

    pub fn edge(&self, id: EdgeId) -> Option<&Edge> {
        self.edges.get(&id)
    }

    pub fn face(&self, id: FaceId) -> Option<&Face> {
        self.faces.get(&id)
    }

    pub fn shell(&self, id: ShellId) -> Option<&Shell> {
        self.shells.get(&id)
    }
}

#[cfg(test)]
mod tests {
    use super::{Edge, Face, Point3, Shell, Topology, Vertex};

    #[test]
    fn edge_contains_vertex() {
        let edge = Edge::new(1, super::VertexId(10), super::VertexId(11));
        assert!(edge.contains_vertex(super::VertexId(10)));
        assert!(!edge.contains_vertex(super::VertexId(99)));
    }

    #[test]
    fn face_requires_at_least_three_vertices() {
        let face = Face::new(1, vec![super::VertexId(1), super::VertexId(2)], vec![super::EdgeId(10), super::EdgeId(11)]);
        assert!(!face.is_closed());
    }

    #[test]
    fn face_with_matching_counts_is_closed() {
        let face = Face::new(
            2,
            vec![super::VertexId(1), super::VertexId(2), super::VertexId(3)],
            vec![super::EdgeId(10), super::EdgeId(11), super::EdgeId(12)],
        );
        assert!(face.is_closed());
    }

    #[test]
    fn topology_can_store_entities() {
        let mut topo = Topology::new();
        let v = Vertex::new(1, Point3::new(0.0, 0.0, 0.0));
        topo.add_vertex(v.clone());
        assert_eq!(topo.vertex(super::VertexId(1)).unwrap().id, v.id);
    }

    #[test]
    fn shell_unique_faces_are_deduplicated() {
        let shell = Shell::new(1, vec![super::FaceId(1), super::FaceId(1), super::FaceId(2)]);
        assert_eq!(shell.unique_faces().len(), 2);
    }
}
