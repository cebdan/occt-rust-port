use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VertexId(pub u64);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EdgeId(pub u64);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FaceId(pub u64);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SolidId(pub u64);

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
}

#[derive(Debug, Clone, PartialEq)]
pub struct Solid {
    pub id: SolidId,
    pub faces: Vec<FaceId>,
}

impl Solid {
    pub fn new(id: u64, faces: Vec<FaceId>) -> Self {
        Self { id: SolidId(id), faces }
    }

    pub fn face_count(&self) -> usize {
        self.faces.len()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PrimitiveBuilder {
    vertices: HashMap<VertexId, Vertex>,
    edges: HashMap<EdgeId, Edge>,
    faces: HashMap<FaceId, Face>,
    solids: HashMap<SolidId, Solid>,
}

impl PrimitiveBuilder {
    pub fn new() -> Self {
        Self {
            vertices: HashMap::new(),
            edges: HashMap::new(),
            faces: HashMap::new(),
            solids: HashMap::new(),
        }
    }

    pub fn add_vertex(&mut self, v: Vertex) {
        self.vertices.insert(v.id, v);
    }

    pub fn add_edge(&mut self, e: Edge) {
        self.edges.insert(e.id, e);
    }

    pub fn add_face(&mut self, f: Face) {
        self.faces.insert(f.id, f);
    }

    pub fn add_solid(&mut self, s: Solid) {
        self.solids.insert(s.id, s);
    }

    pub fn box_solid(&mut self, id: u64, min: Point3, max: Point3) -> Solid {
        let v0 = Vertex::new(1000 + id * 10 + 0, min);
        let v1 = Vertex::new(1000 + id * 10 + 1, Point3::new(max.x, min.y, min.z));
        let v2 = Vertex::new(1000 + id * 10 + 2, Point3::new(max.x, max.y, min.z));
        let v3 = Vertex::new(1000 + id * 10 + 3, Point3::new(min.x, max.y, min.z));
        let v4 = Vertex::new(1000 + id * 10 + 4, Point3::new(min.x, min.y, max.z));
        let v5 = Vertex::new(1000 + id * 10 + 5, Point3::new(max.x, min.y, max.z));
        let v6 = Vertex::new(1000 + id * 10 + 6, max);
        let v7 = Vertex::new(1000 + id * 10 + 7, Point3::new(min.x, max.y, max.z));

        for v in [v0, v1, v2, v3, v4, v5, v6, v7].iter() {
            self.add_vertex(v.clone());
        }

        let edges = [
            Edge::new(2000 + id * 10 + 0, VertexId(1000 + id * 10 + 0), VertexId(1000 + id * 10 + 1)),
            Edge::new(2000 + id * 10 + 1, VertexId(1000 + id * 10 + 1), VertexId(1000 + id * 10 + 2)),
            Edge::new(2000 + id * 10 + 2, VertexId(1000 + id * 10 + 2), VertexId(1000 + id * 10 + 3)),
            Edge::new(2000 + id * 10 + 3, VertexId(1000 + id * 10 + 3), VertexId(1000 + id * 10 + 0)),
            Edge::new(2000 + id * 10 + 4, VertexId(1000 + id * 10 + 4), VertexId(1000 + id * 10 + 5)),
            Edge::new(2000 + id * 10 + 5, VertexId(1000 + id * 10 + 5), VertexId(1000 + id * 10 + 6)),
            Edge::new(2000 + id * 10 + 6, VertexId(1000 + id * 10 + 6), VertexId(1000 + id * 10 + 7)),
            Edge::new(2000 + id * 10 + 7, VertexId(1000 + id * 10 + 7), VertexId(1000 + id * 10 + 4)),
            Edge::new(2000 + id * 10 + 8, VertexId(1000 + id * 10 + 0), VertexId(1000 + id * 10 + 4)),
            Edge::new(2000 + id * 10 + 9, VertexId(1000 + id * 10 + 1), VertexId(1000 + id * 10 + 5)),
            Edge::new(2000 + id * 10 + 10, VertexId(1000 + id * 10 + 2), VertexId(1000 + id * 10 + 6)),
            Edge::new(2000 + id * 10 + 11, VertexId(1000 + id * 10 + 3), VertexId(1000 + id * 10 + 7)),
        ];

        for e in edges.iter() {
            self.add_edge(e.clone());
        }

        let faces = [
            Face::new(3000 + id * 10 + 0, vec![VertexId(1000 + id * 10 + 0), VertexId(1000 + id * 10 + 1), VertexId(1000 + id * 10 + 2), VertexId(1000 + id * 10 + 3)], vec![EdgeId(2000 + id * 10 + 0), EdgeId(2000 + id * 10 + 1), EdgeId(2000 + id * 10 + 2), EdgeId(2000 + id * 10 + 3)]),
            Face::new(3000 + id * 10 + 1, vec![VertexId(1000 + id * 10 + 4), VertexId(1000 + id * 10 + 5), VertexId(1000 + id * 10 + 6), VertexId(1000 + id * 10 + 7)], vec![EdgeId(2000 + id * 10 + 4), EdgeId(2000 + id * 10 + 5), EdgeId(2000 + id * 10 + 6), EdgeId(2000 + id * 10 + 7)]),
            Face::new(3000 + id * 10 + 2, vec![VertexId(1000 + id * 10 + 0), VertexId(1000 + id * 10 + 1), VertexId(1000 + id * 10 + 5), VertexId(1000 + id * 10 + 4)], vec![EdgeId(2000 + id * 10 + 0), EdgeId(2000 + id * 10 + 9), EdgeId(2000 + id * 10 + 4), EdgeId(2000 + id * 10 + 8)]),
            Face::new(3000 + id * 10 + 3, vec![VertexId(1000 + id * 10 + 1), VertexId(1000 + id * 10 + 2), VertexId(1000 + id * 10 + 6), VertexId(1000 + id * 10 + 5)], vec![EdgeId(2000 + id * 10 + 1), EdgeId(2000 + id * 10 + 10), EdgeId(2000 + id * 10 + 5), EdgeId(2000 + id * 10 + 9)]),
            Face::new(3000 + id * 10 + 4, vec![VertexId(1000 + id * 10 + 2), VertexId(1000 + id * 10 + 3), VertexId(1000 + id * 10 + 7), VertexId(1000 + id * 10 + 6)], vec![EdgeId(2000 + id * 10 + 2), EdgeId(2000 + id * 10 + 11), EdgeId(2000 + id * 10 + 6), EdgeId(2000 + id * 10 + 10)]),
            Face::new(3000 + id * 10 + 5, vec![VertexId(1000 + id * 10 + 3), VertexId(1000 + id * 10 + 0), VertexId(1000 + id * 10 + 4), VertexId(1000 + id * 10 + 7)], vec![EdgeId(2000 + id * 10 + 3), EdgeId(2000 + id * 10 + 8), EdgeId(2000 + id * 10 + 7), EdgeId(2000 + id * 10 + 11)]),
        ];

        for face in faces.iter() {
            self.add_face(face.clone());
        }

        let solid = Solid::new(id, faces.iter().map(|f| f.id).collect());
        self.add_solid(solid.clone());
        solid
    }
}

#[cfg(test)]
mod tests {
    use super::{Point3, PrimitiveBuilder, VertexId};

    #[test]
    fn primitive_builder_creates_box_solid() {
        let mut builder = PrimitiveBuilder::new();
        let solid = builder.box_solid(1, Point3::new(0.0, 0.0, 0.0), Point3::new(2.0, 3.0, 4.0));
        assert_eq!(solid.face_count(), 6);
    }

    #[test]
    fn primitive_builder_box_has_expected_vertices() {
        let mut builder = PrimitiveBuilder::new();
        builder.box_solid(2, Point3::origin(), Point3::new(1.0, 2.0, 3.0));
        assert!(builder.vertices.contains_key(&VertexId(1000 + 2 * 10 + 0)));
        assert!(builder.vertices.contains_key(&VertexId(1000 + 2 * 10 + 7)));
    }
}
