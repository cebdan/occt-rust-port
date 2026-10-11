use crate::primitive::{Face, PrimitiveBuilder, Solid};

#[derive(Debug, Clone, PartialEq)]
pub struct ValidationError {
    pub message: String,
    pub error_type: ValidationErrorType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValidationErrorType {
    InvalidFaceStructure,
    InvalidSolidStructure,
    MissingFaces,
    DuplicateVertices,
    BrokenEdgeChain,
}

impl ValidationError {
    pub fn new(message: &str, error_type: ValidationErrorType) -> Self {
        Self {
            message: message.to_string(),
            error_type,
        }
    }
}

pub struct Validator;

impl Validator {
    pub fn validate_face(face: &Face) -> Result<(), ValidationError> {
        if !face.is_closed() {
            return Err(ValidationError::new(
                "Face is not closed: vertex count does not match edge count",
                ValidationErrorType::InvalidFaceStructure,
            ));
        }

        if face.vertices.len() < 3 {
            return Err(ValidationError::new(
                "Face has fewer than 3 vertices",
                ValidationErrorType::InvalidFaceStructure,
            ));
        }

        if face.edges.len() < 3 {
            return Err(ValidationError::new(
                "Face has fewer than 3 edges",
                ValidationErrorType::InvalidFaceStructure,
            ));
        }

        Ok(())
    }

    pub fn validate_solid(solid: &Solid, builder: &PrimitiveBuilder) -> Result<(), ValidationError> {
        if solid.faces.is_empty() {
            return Err(ValidationError::new(
                "Solid has no faces",
                ValidationErrorType::MissingFaces,
            ));
        }

        for face_id in &solid.faces {
            match builder.face(*face_id) {
                Some(face) => {
                    Self::validate_face(face)?;
                }
                None => {
                    return Err(ValidationError::new(
                        "Solid references non-existent face",
                        ValidationErrorType::InvalidSolidStructure,
                    ));
                }
            }
        }

        Ok(())
    }

    pub fn validate_solid_manifold(solid: &Solid, builder: &PrimitiveBuilder) -> Result<(), ValidationError> {
        Self::validate_solid(solid, builder)?;

        let mut edge_count = std::collections::HashMap::new();

        for face_id in &solid.faces {
            if let Some(face) = builder.face(*face_id) {
                for edge_id in &face.edges {
                    *edge_count.entry(*edge_id).or_insert(0) += 1;
                }
            }
        }

        for (_, count) in edge_count.iter() {
            if *count > 2 {
                return Err(ValidationError::new(
                    "Edge is referenced by more than 2 faces (non-manifold)",
                    ValidationErrorType::BrokenEdgeChain,
                ));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{Validator, ValidationErrorType};
    use crate::primitive::{Face, Point3, PrimitiveBuilder};

    #[test]
    fn validator_rejects_unclosed_face() {
        let face = Face::new(1, vec![], vec![]);
        let result = Validator::validate_face(&face);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().error_type, ValidationErrorType::InvalidFaceStructure);
    }

    #[test]
    fn validator_accepts_closed_face() {
        let face = Face::new(
            2,
            vec![crate::primitive::VertexId(1), crate::primitive::VertexId(2), crate::primitive::VertexId(3)],
            vec![crate::primitive::EdgeId(10), crate::primitive::EdgeId(11), crate::primitive::EdgeId(12)],
        );
        assert!(Validator::validate_face(&face).is_ok());
    }

    #[test]
    fn validator_rejects_empty_solid() {
        let mut builder = PrimitiveBuilder::new();
        let solid = crate::primitive::Solid::new(1, vec![]);
        let result = Validator::validate_solid(&solid, &builder);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().error_type, ValidationErrorType::MissingFaces);
    }

    #[test]
    fn validator_accepts_valid_box_solid() {
        let mut builder = PrimitiveBuilder::new();
        let solid = builder.box_solid(10, Point3::origin(), Point3::new(1.0, 1.0, 1.0));
        assert!(Validator::validate_solid(&solid, &builder).is_ok());
    }

    #[test]
    fn validator_checks_manifold_property() {
        let mut builder = PrimitiveBuilder::new();
        let solid = builder.box_solid(11, Point3::origin(), Point3::new(2.0, 2.0, 2.0));
        assert!(Validator::validate_solid_manifold(&solid, &builder).is_ok());
    }
}
