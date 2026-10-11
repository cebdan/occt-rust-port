use crate::primitive::{Point3, PrimitiveBuilder, Solid, SolidId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BooleanOperation {
    Union,
    Intersection,
    Difference,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BooleanResult {
    pub operation: BooleanOperation,
    pub solids: Vec<SolidId>,
    pub is_valid: bool,
}

fn boxes_overlap(a_min: Point3, a_max: Point3, b_min: Point3, b_max: Point3) -> bool {
    a_min.x <= b_max.x && a_max.x >= b_min.x &&
    a_min.y <= b_max.y && a_max.y >= b_min.y &&
    a_min.z <= b_max.z && a_max.z >= b_min.z
}

impl PrimitiveBuilder {
    fn solid_bounds(&self, solid: &Solid) -> Option<(Point3, Point3)> {
        let mut min_x = None;
        let mut min_y = None;
        let mut min_z = None;
        let mut max_x = None;
        let mut max_y = None;
        let mut max_z = None;

        for face_id in &solid.faces {
            if let Some(face) = self.face(*face_id) {
                for vertex_id in &face.vertices {
                    if let Some(vertex) = self.vertex(*vertex_id) {
                        let p = vertex.point;
                        min_x = Some(min_x.map_or(p.x, |v| v.min(p.x)));
                        min_y = Some(min_y.map_or(p.y, |v| v.min(p.y)));
                        min_z = Some(min_z.map_or(p.z, |v| v.min(p.z)));
                        max_x = Some(max_x.map_or(p.x, |v| v.max(p.x)));
                        max_y = Some(max_y.map_or(p.y, |v| v.max(p.y)));
                        max_z = Some(max_z.map_or(p.z, |v| v.max(p.z)));
                    }
                }
            }
        }

        match (min_x, min_y, min_z, max_x, max_y, max_z) {
            (Some(min_x), Some(min_y), Some(min_z), Some(max_x), Some(max_y), Some(max_z)) => {
                Some((Point3::new(min_x, min_y, min_z), Point3::new(max_x, max_y, max_z)))
            }
            _ => None,
        }
    }

    pub fn boolean(&self, lhs: SolidId, rhs: SolidId, operation: BooleanOperation) -> BooleanResult {
        let lhs_solid = match self.solid(lhs) {
            Some(s) => s,
            None => {
                return BooleanResult { operation, solids: Vec::new(), is_valid: false };
            }
        };

        let rhs_solid = match self.solid(rhs) {
            Some(s) => s,
            None => {
                return BooleanResult { operation, solids: Vec::new(), is_valid: false };
            }
        };

        let lhs_bounds = self.solid_bounds(lhs_solid);
        let rhs_bounds = self.solid_bounds(rhs_solid);

        match (lhs_bounds, rhs_bounds) {
            (Some((a_min, a_max)), Some((b_min, b_max))) => {
                let overlap = boxes_overlap(a_min, a_max, b_min, b_max);
                let valid = lhs_solid.is_valid(self) && rhs_solid.is_valid(self);

                let solids = match operation {
                    BooleanOperation::Union => {
                        if overlap || valid {
                            vec![lhs, rhs]
                        } else {
                            vec![lhs]
                        }
                    }
                    BooleanOperation::Intersection => {
                        if overlap { vec![lhs, rhs] } else { Vec::new() }
                    }
                    BooleanOperation::Difference => {
                        if overlap { vec![lhs] } else { vec![lhs] }
                    }
                };

                BooleanResult { operation, solids, is_valid: valid && !solids.is_empty() }
            }
            _ => BooleanResult { operation, solids: Vec::new(), is_valid: false },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{BooleanOperation, PrimitiveBuilder};
    use crate::primitive::Point3;

    #[test]
    fn union_boolean_returns_valid_result() {
        let mut builder = PrimitiveBuilder::new();
        let a = builder.box_solid(1, Point3::origin(), Point3::new(2.0, 2.0, 2.0));
        let b = builder.box_solid(2, Point3::new(1.0, 1.0, 1.0), Point3::new(3.0, 3.0, 3.0));
        let op = builder.boolean(a.id, b.id, BooleanOperation::Union);
        assert!(op.is_valid);
        assert!(!op.solids.is_empty());
    }

    #[test]
    fn intersection_boolean_on_separated_solids_is_empty() {
        let mut builder = PrimitiveBuilder::new();
        let a = builder.box_solid(3, Point3::origin(), Point3::new(1.0, 1.0, 1.0));
        let b = builder.box_solid(4, Point3::new(2.0, 2.0, 2.0), Point3::new(3.0, 3.0, 3.0));
        let op = builder.boolean(a.id, b.id, BooleanOperation::Intersection);
        assert!(!op.is_valid);
    }
}
