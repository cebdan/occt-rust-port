use crate::primitive::PrimitiveBuilder;

/// Benchmark thresholds for regression testing
pub struct BenchmarkThresholds {
    pub box_creation_per_solid_ms: f64,
    pub cylinder_creation_per_solid_ms: f64,
    pub sphere_creation_per_solid_ms: f64,
    pub validation_per_solid_ms: f64,
    pub boolean_union_per_op_ms: f64,
}

impl BenchmarkThresholds {
    pub fn default() -> Self {
        Self {
            box_creation_per_solid_ms: 0.5,
            cylinder_creation_per_solid_ms: 1.0,
            sphere_creation_per_solid_ms: 2.0,
            validation_per_solid_ms: 0.2,
            boolean_union_per_op_ms: 0.1,
        }
    }

    pub fn loose() -> Self {
        Self {
            box_creation_per_solid_ms: 1.0,
            cylinder_creation_per_solid_ms: 2.0,
            sphere_creation_per_solid_ms: 4.0,
            validation_per_solid_ms: 0.5,
            boolean_union_per_op_ms: 0.2,
        }
    }
}

/// Regression test suite
pub struct RegressionSuite;

impl RegressionSuite {
    /// Test 1: Box solid creation and integrity
    pub fn test_box_solid_creation() -> Result<String, String> {
        let mut builder = PrimitiveBuilder::new();
        let solid = builder.box_solid(1, crate::primitive::Point3::origin(), crate::primitive::Point3::new(1.0, 1.0, 1.0));
        
        if solid.face_count() != 6 {
            return Err(format!("Expected 6 faces, got {}", solid.face_count()));
        }
        
        match crate::validation::Validator::validate_solid(&solid, &builder) {
            Ok(()) => Ok("Box solid creation: PASS".to_string()),
            Err(e) => Err(format!("Box solid validation failed: {}", e.message)),
        }
    }

    /// Test 2: Cylinder solid creation and integrity
    pub fn test_cylinder_solid_creation() -> Result<String, String> {
        let mut builder = PrimitiveBuilder::new();
        let solid = builder.cylinder_solid(2, 1.0, 2.0, 8);
        
        if solid.face_count() != 8 {
            return Err(format!("Expected 8 faces, got {}", solid.face_count()));
        }
        
        match crate::validation::Validator::validate_solid(&solid, &builder) {
            Ok(()) => Ok("Cylinder solid creation: PASS".to_string()),
            Err(e) => Err(format!("Cylinder solid validation failed: {}", e.message)),
        }
    }

    /// Test 3: Sphere solid creation and integrity
    pub fn test_sphere_solid_creation() -> Result<String, String> {
        let mut builder = PrimitiveBuilder::new();
        let solid = builder.sphere_solid(3, 1.0, 4, 8);
        
        if solid.face_count() == 0 {
            return Err("Sphere solid has no faces".to_string());
        }
        
        match crate::validation::Validator::validate_solid(&solid, &builder) {
            Ok(()) => Ok(format!("Sphere solid creation: PASS (faces: {})", solid.face_count())),
            Err(e) => Err(format!("Sphere solid validation failed: {}", e.message)),
        }
    }

    /// Test 4: Union of overlapping boxes
    pub fn test_boolean_union() -> Result<String, String> {
        let mut builder = PrimitiveBuilder::new();
        let a = builder.box_solid(4, crate::primitive::Point3::origin(), crate::primitive::Point3::new(2.0, 2.0, 2.0));
        let b = builder.box_solid(5, crate::primitive::Point3::new(1.0, 1.0, 1.0), crate::primitive::Point3::new(3.0, 3.0, 3.0));
        
        let result = builder.boolean(a.id, b.id, crate::boolean::BooleanOperation::Union);
        
        if !result.is_valid || result.solids.is_empty() {
            return Err("Boolean union operation failed".to_string());
        }
        
        Ok("Boolean union: PASS".to_string())
    }

    /// Test 5: Intersection of overlapping boxes
    pub fn test_boolean_intersection() -> Result<String, String> {
        let mut builder = PrimitiveBuilder::new();
        let a = builder.box_solid(6, crate::primitive::Point3::origin(), crate::primitive::Point3::new(2.0, 2.0, 2.0));
        let b = builder.box_solid(7, crate::primitive::Point3::new(1.0, 1.0, 1.0), crate::primitive::Point3::new(3.0, 3.0, 3.0));
        
        let result = builder.boolean(a.id, b.id, crate::boolean::BooleanOperation::Intersection);
        
        if !result.is_valid {
            return Err("Boolean intersection operation failed".to_string());
        }
        
        Ok("Boolean intersection: PASS".to_string())
    }

    /// Test 6: Intersection of separated boxes returns empty
    pub fn test_boolean_intersection_empty() -> Result<String, String> {
        let mut builder = PrimitiveBuilder::new();
        let a = builder.box_solid(8, crate::primitive::Point3::origin(), crate::primitive::Point3::new(1.0, 1.0, 1.0));
        let b = builder.box_solid(9, crate::primitive::Point3::new(2.0, 2.0, 2.0), crate::primitive::Point3::new(3.0, 3.0, 3.0));
        
        let result = builder.boolean(a.id, b.id, crate::boolean::BooleanOperation::Intersection);
        
        if result.is_valid {
            return Err("Separated boxes should not have valid intersection".to_string());
        }
        
        Ok("Boolean intersection (empty): PASS".to_string())
    }

    /// Test 7: Solid manifold validation
    pub fn test_manifold_validation() -> Result<String, String> {
        let mut builder = PrimitiveBuilder::new();
        let solid = builder.box_solid(10, crate::primitive::Point3::origin(), crate::primitive::Point3::new(1.0, 1.0, 1.0));
        
        match crate::validation::Validator::validate_solid_manifold(&solid, &builder) {
            Ok(()) => Ok("Manifold validation: PASS".to_string()),
            Err(e) => Err(format!("Manifold validation failed: {}", e.message)),
        }
    }

    /// Run all regression tests
    pub fn run_all() -> Vec<(String, Result<String, String>)> {
        vec![
            ("Box Creation".to_string(), Self::test_box_solid_creation()),
            ("Cylinder Creation".to_string(), Self::test_cylinder_solid_creation()),
            ("Sphere Creation".to_string(), Self::test_sphere_solid_creation()),
            ("Boolean Union".to_string(), Self::test_boolean_union()),
            ("Boolean Intersection".to_string(), Self::test_boolean_intersection()),
            ("Boolean Intersection (Empty)".to_string(), Self::test_boolean_intersection_empty()),
            ("Manifold Validation".to_string(), Self::test_manifold_validation()),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regression_suite_all_tests_pass() {
        let results = RegressionSuite::run_all();
        let failures: Vec<_> = results.iter().filter(|(_, r)| r.is_err()).collect();
        
        if !failures.is_empty() {
            for (name, err) in failures {
                println!("FAILED: {} - {}", name, err.as_ref().unwrap_err());
            }
        }
        
        assert!(failures.is_empty(), "{} test(s) failed", failures.len());
    }

    #[test]
    fn benchmark_thresholds_are_reasonable() {
        let thresholds = BenchmarkThresholds::default();
        assert!(thresholds.box_creation_per_solid_ms > 0.0);
        assert!(thresholds.boolean_union_per_op_ms > 0.0);
    }
}
