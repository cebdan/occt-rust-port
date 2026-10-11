pub mod maths;
pub mod topology;
pub mod primitive;
pub mod boolean;
pub mod validation;
pub mod benchmark;
pub mod api;
pub mod examples;
pub mod regression;

pub use maths::*;
pub use topology::*;
pub use primitive::*;
pub use boolean::*;
pub use validation::*;
pub use benchmark::*;
pub use api::*;
pub use regression::*;

/// Prelude module for convenient importing
pub mod prelude {
    pub use crate::primitive::{Point3, PrimitiveBuilder, Solid, SolidId};
    pub use crate::api::{SolidBuilder, SolidOperations};
    pub use crate::validation::{Validator, ValidationError};
    pub use crate::boolean::BooleanOperation;
    pub use crate::regression::{RegressionSuite, BenchmarkThresholds};
}
