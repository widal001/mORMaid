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
///     String::from("ALBUM"),
///     relationship::Cardinality::ExactlyOne,
///     String::from("SONG"),
///     relationship::Cardinality::ZeroOrMore,
/// )
///     .as_non_identifying()
///     .with_label("has");
/// ```
pub struct Relationship {
    // The id
    pub left_id: String,
    pub right_id: String,
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
        left_id: String,
        left_cardinality: Cardinality,
        right_id: String,
        right_cardinality: Cardinality,
    ) -> Self {
        Relationship {
            left_id,
            right_id,
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
