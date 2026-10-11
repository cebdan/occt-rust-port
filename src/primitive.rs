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

    pub fn distance_to(self, other: Self) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
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

    pub fn is_valid(&self, builder: &PrimitiveBuilder) -> bool {
        if self.faces.is_empty() {
            return false;
        }
        self.faces.iter().all(|face_id| {
            builder.face(*face_id)
                .map(|f| f.is_closed())
                .unwrap_or(false)
        })
    }
}

pub struct IdGenerator {
    vertex_counter: u64,
    edge_counter: u64,
    face_counter: u64,
}

impl IdGenerator {
    pub fn new() -> Self {
        Self {
            vertex_counter: 10000,
            edge_counter: 20000,
            face_counter: 30000,
        }
    }

    pub fn next_vertex_id(&mut self) -> VertexId {
        let id = self.vertex_counter;
        self.vertex_counter += 1;
        VertexId(id)
    }

    pub fn next_edge_id(&mut self) -> EdgeId {
        let id = self.edge_counter;
        self.edge_counter += 1;
        EdgeId(id)
    }

    pub fn next_face_id(&mut self) -> FaceId {
        let id = self.face_counter;
        self.face_counter += 1;
        FaceId(id)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PrimitiveBuilder {
    pub vertices: HashMap<VertexId, Vertex>,
    pub edges: HashMap<EdgeId, Edge>,
    pub faces: HashMap<FaceId, Face>,
    pub solids: HashMap<SolidId, Solid>,
    id_gen: IdGenerator,
}

impl PrimitiveBuilder {
    pub fn new() -> Self {
        Self {
            vertices: HashMap::new(),
            edges: HashMap::new(),
            faces: HashMap::new(),
            solids: HashMap::new(),
            id_gen: IdGenerator::new(),
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

    pub fn vertex(&self, id: VertexId) -> Option<&Vertex> {
        self.vertices.get(&id)
    }

    pub fn face(&self, id: FaceId) -> Option<&Face> {
        self.faces.get(&id)
    }

    pub fn solid(&self, id: SolidId) -> Option<&Solid> {
        self.solids.get(&id)
    }

    pub fn box_solid(&mut self, id: u64, min: Point3, max: Point3) -> Solid {
        let v0 = Vertex::new(self.id_gen.next_vertex_id().0, min);
        let v1 = Vertex::new(self.id_gen.next_vertex_id().0, Point3::new(max.x, min.y, min.z));
        let v2 = Vertex::new(self.id_gen.next_vertex_id().0, Point3::new(max.x, max.y, min.z));
        let v3 = Vertex::new(self.id_gen.next_vertex_id().0, Point3::new(min.x, max.y, min.z));
        let v4 = Vertex::new(self.id_gen.next_vertex_id().0, Point3::new(min.x, min.y, max.z));
        let v5 = Vertex::new(self.id_gen.next_vertex_id().0, Point3::new(max.x, min.y, max.z));
        let v6 = Vertex::new(self.id_gen.next_vertex_id().0, max);
        let v7 = Vertex::new(self.id_gen.next_vertex_id().0, Point3::new(min.x, max.y, max.z));

        for v in [v0.clone(), v1.clone(), v2.clone(), v3.clone(), v4.clone(), v5.clone(), v6.clone(), v7.clone()].iter() {
            self.add_vertex(v.clone());
        }

        let e0 = Edge::new(self.id_gen.next_edge_id().0, v0.id, v1.id);
        let e1 = Edge::new(self.id_gen.next_edge_id().0, v1.id, v2.id);
        let e2 = Edge::new(self.id_gen.next_edge_id().0, v2.id, v3.id);
        let e3 = Edge::new(self.id_gen.next_edge_id().0, v3.id, v0.id);
        let e4 = Edge::new(self.id_gen.next_edge_id().0, v4.id, v5.id);
        let e5 = Edge::new(self.id_gen.next_edge_id().0, v5.id, v6.id);
        let e6 = Edge::new(self.id_gen.next_edge_id().0, v6.id, v7.id);
        let e7 = Edge::new(self.id_gen.next_edge_id().0, v7.id, v4.id);
        let e8 = Edge::new(self.id_gen.next_edge_id().0, v0.id, v4.id);
        let e9 = Edge::new(self.id_gen.next_edge_id().0, v1.id, v5.id);
        let e10 = Edge::new(self.id_gen.next_edge_id().0, v2.id, v6.id);
        let e11 = Edge::new(self.id_gen.next_edge_id().0, v3.id, v7.id);

        for e in [e0.clone(), e1.clone(), e2.clone(), e3.clone(), e4.clone(), e5.clone(), e6.clone(), e7.clone(), e8.clone(), e9.clone(), e10.clone(), e11.clone()].iter() {
            self.add_edge(e.clone());
        }

        let f0 = Face::new(self.id_gen.next_face_id().0, vec![v0.id, v1.id, v2.id, v3.id], vec![e0.id, e1.id, e2.id, e3.id]);
        let f1 = Face::new(self.id_gen.next_face_id().0, vec![v4.id, v5.id, v6.id, v7.id], vec![e4.id, e5.id, e6.id, e7.id]);
        let f2 = Face::new(self.id_gen.next_face_id().0, vec![v0.id, v1.id, v5.id, v4.id], vec![e0.id, e9.id, e4.id, e8.id]);
        let f3 = Face::new(self.id_gen.next_face_id().0, vec![v1.id, v2.id, v6.id, v5.id], vec![e1.id, e10.id, e5.id, e9.id]);
        let f4 = Face::new(self.id_gen.next_face_id().0, vec![v2.id, v3.id, v7.id, v6.id], vec![e2.id, e11.id, e6.id, e10.id]);
        let f5 = Face::new(self.id_gen.next_face_id().0, vec![v3.id, v0.id, v4.id, v7.id], vec![e3.id, e8.id, e7.id, e11.id]);

        for f in [f0.clone(), f1.clone(), f2.clone(), f3.clone(), f4.clone(), f5.clone()].iter() {
            self.add_face(f.clone());
        }

        let solid = Solid::new(id, vec![f0.id, f1.id, f2.id, f3.id, f4.id, f5.id]);
        self.add_solid(solid.clone());
        solid
    }

    pub fn cylinder_solid(&mut self, id: u64, radius: f64, height: f64, segments: u32) -> Solid {
        let segments = segments.max(3) as usize;
        let mut face_ids = Vec::new();
        let mut bottom_verts = Vec::new();
        let mut top_verts = Vec::new();

        for i in 0..segments {
            let angle = (i as f64 / segments as f64) * std::f64::consts::TAU;
            let x = angle.cos() * radius;
            let y = angle.sin() * radius;
            let v_bottom = Vertex::new(self.id_gen.next_vertex_id().0, Point3::new(x, y, -height / 2.0));
            let v_top = Vertex::new(self.id_gen.next_vertex_id().0, Point3::new(x, y, height / 2.0));
            self.add_vertex(v_bottom.clone());
            self.add_vertex(v_top.clone());
            bottom_verts.push(v_bottom.id);
            top_verts.push(v_top.id);
        }

        for i in 0..segments {
            let j = (i + 1) % segments;
            let e_bottom = Edge::new(self.id_gen.next_edge_id().0, bottom_verts[i], bottom_verts[j]);
            let e_top = Edge::new(self.id_gen.next_edge_id().0, top_verts[i], top_verts[j]);
            let e_side_a = Edge::new(self.id_gen.next_edge_id().0, bottom_verts[i], top_verts[i]);
            let e_side_b = Edge::new(self.id_gen.next_edge_id().0, bottom_verts[j], top_verts[j]);
            self.add_edge(e_bottom.clone());
            self.add_edge(e_top.clone());
            self.add_edge(e_side_a.clone());
            self.add_edge(e_side_b.clone());

            let face = Face::new(
                self.id_gen.next_face_id().0,
                vec![bottom_verts[i], bottom_verts[j], top_verts[j], top_verts[i]],
                vec![e_bottom.id, e_side_b.id, e_top.id, e_side_a.id],
            );
            self.add_face(face.clone());
            face_ids.push(face.id);
        }

        let solid = Solid::new(id, face_ids);
        self.add_solid(solid.clone());
        solid
    }

    pub fn sphere_solid(&mut self, id: u64, radius: f64, latitude_segments: u32, longitude_segments: u32) -> Solid {
        let lat = latitude_segments.max(2) as usize;
        let lon = longitude_segments.max(3) as usize;
        let mut face_ids = Vec::new();
        let mut vertices = Vec::new();

        for i in 0..=lat {
            let theta = (i as f64 / lat as f64) * std::f64::consts::PI;
            for j in 0..lon {
                let phi = (j as f64 / lon as f64) * std::f64::consts::TAU;
                let x = radius * theta.sin() * phi.cos();
                let y = radius * theta.sin() * phi.sin();
                let z = radius * theta.cos();
                let v = Vertex::new(self.id_gen.next_vertex_id().0, Point3::new(x, y, z));
                self.add_vertex(v.clone());
                vertices.push(v.id);
            }
        }

        for i in 0..lat {
            for j in 0..lon {
                let base = i * lon + j;
                let next_lon = (j + 1) % lon;
                let next_lat = (i + 1) * lon;

                let v0 = vertices[base];
                let v1 = vertices[base + next_lon];
                let v2 = vertices[next_lat + next_lon];
                let v3 = vertices[next_lat + j];

                let e0 = Edge::new(self.id_gen.next_edge_id().0, v0, v1);
                let e1 = Edge::new(self.id_gen.next_edge_id().0, v1, v2);
                let e2 = Edge::new(self.id_gen.next_edge_id().0, v2, v3);
                let e3 = Edge::new(self.id_gen.next_edge_id().0, v3, v0);

                self.add_edge(e0.clone());
                self.add_edge(e1.clone());
                self.add_edge(e2.clone());
                self.add_edge(e3.clone());

                let face = Face::new(
                    self.id_gen.next_face_id().0,
                    vec![v0, v1, v2, v3],
                    vec![e0.id, e1.id, e2.id, e3.id],
                );
                self.add_face(face.clone());
                face_ids.push(face.id);
            }
        }

        let solid = Solid::new(id, face_ids);
        self.add_solid(solid.clone());
        solid
    }
}

#[cfg(test)]
mod tests {
    use super::{Point3, PrimitiveBuilder};

    #[test]
    fn primitive_builder_creates_box_solid() {
        let mut builder = PrimitiveBuilder::new();
        let solid = builder.box_solid(1, Point3::new(0.0, 0.0, 0.0), Point3::new(2.0, 3.0, 4.0));
        assert_eq!(solid.face_count(), 6);
        assert!(solid.is_valid(&builder));
    }

    #[test]
    fn primitive_builder_creates_cylinder_solid() {
        let mut builder = PrimitiveBuilder::new();
        let solid = builder.cylinder_solid(2, 1.0, 2.0, 8);
        assert_eq!(solid.face_count(), 8);
        assert!(solid.is_valid(&builder));
    }

    #[test]
    fn primitive_builder_creates_sphere_solid() {
        let mut builder = PrimitiveBuilder::new();
        let solid = builder.sphere_solid(3, 1.0, 4, 8);
        assert!(solid.face_count() > 0);
        assert!(solid.is_valid(&builder));
    }

    #[test]
    fn id_generator_produces_unique_ids() {
        let mut builder = PrimitiveBuilder::new();
        let v1 = builder.id_gen.next_vertex_id();
        let v2 = builder.id_gen.next_vertex_id();
        assert_ne!(v1, v2);
    }

    #[test]
    fn solid_validation_checks_face_closure() {
        let mut builder = PrimitiveBuilder::new();
        let solid = builder.box_solid(10, Point3::origin(), Point3::new(1.0, 1.0, 1.0));
        assert!(solid.is_valid(&builder));
    }
}
