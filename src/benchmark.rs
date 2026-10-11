use crate::primitive::{Point3, PrimitiveBuilder, Solid};
use crate::validation::{ValidationError, Validator};
use std::time::Instant;

pub struct BenchmarkResult {
    pub operation: String,
    pub duration_ms: f64,
    pub solid_count: usize,
}

pub struct Benchmarker;

impl Benchmarker {
    pub fn benchmark_box_creation(count: usize) -> BenchmarkResult {
        let start = Instant::now();
        let mut builder = PrimitiveBuilder::new();

        for i in 0..count {
            let offset = i as f64 * 0.1;
            builder.box_solid(
                i as u64,
                Point3::new(offset, offset, offset),
                Point3::new(1.0 + offset, 1.0 + offset, 1.0 + offset),
            );
        }

        let duration = start.elapsed();
        BenchmarkResult {
            operation: format!("Create {} box solids", count),
            duration_ms: duration.as_secs_f64() * 1000.0,
            solid_count: count,
        }
    }

    pub fn benchmark_validation(builder: &PrimitiveBuilder, solid: &Solid) -> BenchmarkResult {
        let start = Instant::now();
        let _ = Validator::validate_solid(solid, builder);
        let duration = start.elapsed();

        BenchmarkResult {
            operation: "Validate solid".to_string(),
            duration_ms: duration.as_secs_f64() * 1000.0,
            solid_count: 1,
        }
    }

    pub fn benchmark_boolean_operation(count: usize) -> BenchmarkResult {
        let start = Instant::now();
        let mut builder = PrimitiveBuilder::new();

        for i in 0..count {
            let offset = i as f64 * 0.05;
            let a = builder.box_solid(
                i as u64 * 2,
                Point3::new(offset, offset, offset),
                Point3::new(1.0 + offset, 1.0 + offset, 1.0 + offset),
            );
            let b = builder.box_solid(
                i as u64 * 2 + 1,
                Point3::new(offset + 0.5, offset + 0.5, offset + 0.5),
                Point3::new(1.5 + offset, 1.5 + offset, 1.5 + offset),
            );
            let _ = builder.boolean(a.id, b.id, crate::boolean::BooleanOperation::Union);
        }

        let duration = start.elapsed();
        BenchmarkResult {
            operation: format!("Perform {} union operations", count),
            duration_ms: duration.as_secs_f64() * 1000.0,
            solid_count: count * 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Benchmarker;

    #[test]
    fn benchmark_box_creation_completes() {
        let result = Benchmarker::benchmark_box_creation(10);
        assert!(result.duration_ms >= 0.0);
        assert_eq!(result.solid_count, 10);
    }

    #[test]
    fn benchmark_boolean_completes() {
        let result = Benchmarker::benchmark_boolean_operation(5);
        assert!(result.duration_ms >= 0.0);
        assert_eq!(result.solid_count, 10);
    }
}
