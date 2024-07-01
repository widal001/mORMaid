#[derive(PartialEq, Debug)]
pub enum Cardinality {
    ZeroOrOne,
    ExactlyOne,
    OneOrMore,
    ZeroOrMore,
}

/// Represents relationships between entities in an ERD.
///
/// # Example
///
/// ```
/// # use crate::mormaid::erd::relationship;
///
/// let album_to_song = relationship::Relationship::new(
///     "ALBUM",
///     "SONG",
///     relationship::Cardinality::ExactlyOne,
///     relationship::Cardinality::ZeroOrMore,
/// )
///     .as_non_identifying()
///     .with_label("has");
/// ```
pub struct Relationship {
    // The id
    pub left_id: super::EntityId,
    pub right_id: super::EntityId,
    pub left_cardinality: Cardinality,
    pub right_cardinality: Cardinality,
    pub is_identifying: bool,
    pub label: Option<String>,
}

impl Relationship {
    /// Create a new relationship.
    ///
    /// # Note
    /// Relationships are identifying by default, meaning they are
    /// represented with a solid slide. They also have no label by default.
    /// To create a non-identifying relationship, use [Relationship::as_non_identifying].
    /// To add a label, use [Relationship::with_label].
    pub fn new(
        left_id: &str,
        right_id: &str,
        left_cardinality: Cardinality,
        right_cardinality: Cardinality,
    ) -> Self {
        Relationship {
            left_id: super::EntityId::from(left_id),
            right_id: super::EntityId::from(right_id),
            left_cardinality,
            right_cardinality,
            is_identifying: true,
            label: None,
        }
    }

    /// Make the relationship non-identifying, represented by a dashed line.
    ///
    /// For more information about the difference between identifying and
    /// non-identifying relationships, see:
    /// [the mermaid documentation](https://mermaid.js.org/syntax/entityRelationshipDiagram.html#identification)
    pub fn as_non_identifying(mut self) -> Self {
        self.is_identifying = false;
        return self;
    }

    /// Add a label to the relationship.
    pub fn with_label(mut self, label: &str) -> Self {
        self.label = Some(label.to_string());
        return self;
    }
}

// =========================
// EntityId tests
// =========================
#[cfg(test)]
mod tests {

    use super::super::*;
    use super::*;

    const ALBUM_ID: &str = "PRODUCT";
    const SONG_ID: &str = "PRODUCT";

    #[test]
    fn test_that_entity_ids_with_same_string_are_equal() {
        // act
        let relationship = Relationship::new(
            ALBUM_ID,
            SONG_ID,
            Cardinality::ExactlyOne,
            Cardinality::OneOrMore,
        );
        // assert
        assert_eq!(relationship.left_id, EntityId::from(ALBUM_ID));
        assert_eq!(relationship.right_id, EntityId::from(SONG_ID));
        assert_eq!(relationship.left_cardinality, Cardinality::ExactlyOne);
        assert_eq!(relationship.right_cardinality, Cardinality::OneOrMore);
    }
}
