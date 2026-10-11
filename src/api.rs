use crate::primitive::{Point3, PrimitiveBuilder, Solid, SolidId};
use crate::boolean::BooleanOperation;
use crate::validation::Validator;

pub struct SolidBuilder {
    builder: PrimitiveBuilder,
}

impl SolidBuilder {
    pub fn new() -> Self {
        Self {
            builder: PrimitiveBuilder::new(),
        }
    }

    pub fn with_box(mut self, id: u64, min: Point3, max: Point3) -> Self {
        self.builder.box_solid(id, min, max);
        self
    }

    pub fn with_cylinder(mut self, id: u64, radius: f64, height: f64, segments: u32) -> Self {
        self.builder.cylinder_solid(id, radius, height, segments);
        self
    }

    pub fn with_sphere(mut self, id: u64, radius: f64, lat: u32, lon: u32) -> Self {
        self.builder.sphere_solid(id, radius, lat, lon);
        self
    }

    pub fn build(self) -> PrimitiveBuilder {
        self.builder
    }
}

pub struct SolidOperations;

impl SolidOperations {
    pub fn union(builder: &PrimitiveBuilder, lhs: SolidId, rhs: SolidId) -> Result<Vec<SolidId>, String> {
        let result = builder.boolean(lhs, rhs, BooleanOperation::Union);
        if result.is_valid {
            Ok(result.solids)
        } else {
            Err("Union operation failed: invalid result".to_string())
        }
    }

    pub fn intersection(builder: &PrimitiveBuilder, lhs: SolidId, rhs: SolidId) -> Result<Vec<SolidId>, String> {
        let result = builder.boolean(lhs, rhs, BooleanOperation::Intersection);
        if result.is_valid {
            Ok(result.solids)
        } else {
            Err("Intersection resulted in empty set".to_string())
        }
    }

    pub fn difference(builder: &PrimitiveBuilder, lhs: SolidId, rhs: SolidId) -> Result<Vec<SolidId>, String> {
        let result = builder.boolean(lhs, rhs, BooleanOperation::Difference);
        if result.is_valid {
            Ok(result.solids)
        } else {
            Err("Difference operation failed".to_string())
        }
    }

    pub fn validate_all(builder: &PrimitiveBuilder) -> Vec<(SolidId, Result<(), String>)> {
        builder
            .all_solids()
            .iter()
            .map(|solid| {
                let validation = Validator::validate_solid(solid, builder).map_err(|e| e.message);
                (solid.id, validation)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitive::Point3;

    #[test]
    fn solid_builder_fluent_api_works() {
        let builder = SolidBuilder::new()
            .with_box(1, Point3::origin(), Point3::new(1.0, 1.0, 1.0))
            .with_cylinder(2, 0.5, 2.0, 8)
            .build();

        assert_eq!(builder.all_solids().len(), 2);
    }

    #[test]
    fn solid_operations_union_succeeds_on_overlapping() {
        let mut builder = PrimitiveBuilder::new();
        let a = builder.box_solid(1, Point3::origin(), Point3::new(2.0, 2.0, 2.0));
        let b = builder.box_solid(2, Point3::new(1.0, 1.0, 1.0), Point3::new(3.0, 3.0, 3.0));
        let result = SolidOperations::union(&builder, a.id, b.id);
        assert!(result.is_ok());
    }

    #[test]
    fn solid_operations_intersection_fails_on_separated() {
        let mut builder = PrimitiveBuilder::new();
        let a = builder.box_solid(3, Point3::origin(), Point3::new(1.0, 1.0, 1.0));
        let b = builder.box_solid(4, Point3::new(2.0, 2.0, 2.0), Point3::new(3.0, 3.0, 3.0));
        let result = SolidOperations::intersection(&builder, a.id, b.id);
        assert!(result.is_err());
    }
}
