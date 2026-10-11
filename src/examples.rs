use crate::primitive::{Point3, PrimitiveBuilder, Solid};
use crate::validation::Validator;
use crate::api::SolidBuilder;

/// Example 1: Create and validate basic primitives
pub fn example_basic_primitives() {
    println!("\n=== Example 1: Basic Primitives ===");
    
    let builder = SolidBuilder::new()
        .with_box(1, Point3::origin(), Point3::new(2.0, 2.0, 2.0))
        .with_cylinder(2, 1.0, 3.0, 12)
        .with_sphere(3, 1.5, 6, 12)
        .build();
    
    let solids = builder.all_solids();
    println!("Created {} solids", solids.len());
    
    for solid in solids.iter() {
        match Validator::validate_solid(solid, &builder) {
            Ok(()) => println!("  ✓ Solid {} is valid (faces: {})", solid.id.0, solid.face_count()),
            Err(e) => println!("  ✗ Solid {} validation failed: {}", solid.id.0, e.message),
        }
    }
}

/// Example 2: Boolean operations and manifold checking
pub fn example_boolean_operations() {
    println!("\n=== Example 2: Boolean Operations ===");
    
    let mut builder = PrimitiveBuilder::new();
    
    let box_a = builder.box_solid(1, Point3::origin(), Point3::new(2.0, 2.0, 2.0));
    let box_b = builder.box_solid(2, Point3::new(1.0, 1.0, 1.0), Point3::new(3.0, 3.0, 3.0));
    
    println!("Box A: {} faces", box_a.face_count());
    println!("Box B: {} faces", box_b.face_count());
    
    // Test Union
    let union_result = builder.boolean(box_a.id, box_b.id, crate::boolean::BooleanOperation::Union);
    println!("Union: valid={}, result_count={}", union_result.is_valid, union_result.solids.len());
    
    // Test Intersection
    let intersection_result = builder.boolean(box_a.id, box_b.id, crate::boolean::BooleanOperation::Intersection);
    println!("Intersection: valid={}, result_count={}", intersection_result.is_valid, intersection_result.solids.len());
    
    // Test Difference
    let difference_result = builder.boolean(box_a.id, box_b.id, crate::boolean::BooleanOperation::Difference);
    println!("Difference: valid={}, result_count={}", difference_result.is_valid, difference_result.solids.len());
}

/// Example 3: Validation across multiple solids
pub fn example_validation_suite() {
    println!("\n=== Example 3: Validation Suite ===");
    
    let mut builder = PrimitiveBuilder::new();
    
    // Create multiple solids
    for i in 0..3 {
        let offset = i as f64 * 0.5;
        builder.box_solid(
            i as u64,
            Point3::new(offset, offset, offset),
            Point3::new(2.0 + offset, 2.0 + offset, 2.0 + offset),
        );
    }
    
    // Validate all solids
    let results = crate::api::SolidOperations::validate_all(&builder);
    println!("Validated {} solids:", results.len());
    
    for (solid_id, validation) in results.iter() {
        match validation {
            Ok(()) => println!("  ✓ Solid {} passed validation", solid_id.0),
            Err(e) => println!("  ✗ Solid {} failed: {}", solid_id.0, e),
        }
    }
}

/// Example 4: Benchmark operations
pub fn example_benchmark() {
    println!("\n=== Example 4: Benchmark Operations ===");
    
    let result = crate::benchmark::Benchmarker::benchmark_box_creation(100);
    println!("Operation: {}", result.operation);
    println!("Duration: {:.3} ms", result.duration_ms);
    println!("Per-solid: {:.4} ms", result.duration_ms / result.solid_count as f64);
    
    let result = crate::benchmark::Benchmarker::benchmark_boolean_operation(50);
    println!("\nOperation: {}", result.operation);
    println!("Duration: {:.3} ms", result.duration_ms);
    println!("Per-operation: {:.4} ms", result.duration_ms / (result.solid_count / 2) as f64);
}

/// Run all examples
pub fn run_all_examples() {
    println!("\n╔════════════════════════════════════════╗");
    println!("║   OCCT Rust Port - Example Suite      ║");
    println!("╚════════════════════════════════════════╝");
    
    example_basic_primitives();
    example_boolean_operations();
    example_validation_suite();
    example_benchmark();
    
    println!("\n╔════════════════════════════════════════╗");
    println!("║   Examples Complete                   ║");
    println!("╚════════════════════════════════════════╝\n");
}
