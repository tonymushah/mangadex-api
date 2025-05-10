use crate::RelationshipType;

#[derive(thiserror::Error, Debug)]
pub enum RelationshipConversionError {
    #[error("The input relationship type {input} is incompatible with {inner}")]
    InvalidInputRelationshipType {
        input: RelationshipType,
        inner: RelationshipType,
    },
    #[error("The {0} related attributes is not found")]
    AttributesNotFound(RelationshipType),
}
