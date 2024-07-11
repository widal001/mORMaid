use core::fmt;

#[derive(PartialEq, Debug)]
pub enum Cardinality {
    ZeroOrOne,
    ExactlyOne,
    ZeroOrMore,
    OneOrMore,
}
enum Direction {
    Left,
    Right,
}

impl Cardinality {
    // Combines Cardinality and Direction to format the relationship join symbol
    #[allow(clippy::match_same_arms)]
    fn fmt_with_direction(&self, dir: Direction) -> String {
        match (self, dir) {
            // formatting left_cardinality
            (Cardinality::ZeroOrOne, Direction::Left) => "|o".to_string(),
            (Cardinality::ExactlyOne, Direction::Left) => "||".to_string(),
            (Cardinality::ZeroOrMore, Direction::Left) => "}o".to_string(),
            (Cardinality::OneOrMore, Direction::Left) => "}|".to_string(),
            // formatting right_cardinality
            (Cardinality::ZeroOrOne, Direction::Right) => "o|".to_string(),
            (Cardinality::ExactlyOne, Direction::Right) => "||".to_string(),
            (Cardinality::ZeroOrMore, Direction::Right) => "o{".to_string(),
            (Cardinality::OneOrMore, Direction::Right) => "|{".to_string(),
        }
    }
}

impl std::fmt::Display for Cardinality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.fmt_with_direction(Direction::Right))
    }
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
#[must_use]
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
    /// To create a non-identifying relationship, use [`Relationship::as_non_identifying()`].
    /// To add a label, use [`Relationship::with_label()`].
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
        self
    }

    /// Add a label to the relationship.
    pub fn with_label(mut self, label: &str) -> Self {
        self.label = Some(label.to_string());
        self
    }
}

impl fmt::Display for Relationship {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // format the left and right ends based on their cardinality
        let left_str = format!(
            "{} {}",
            self.left_id.as_str(),
            self.left_cardinality.fmt_with_direction(Direction::Left)
        );
        let right_str = format!(
            "{} {}",
            self.right_cardinality.fmt_with_direction(Direction::Right),
            self.right_id.as_str()
        );
        // format the relationship as a solid or dashed line
        let mut relationship_str = if self.is_identifying {
            format!("{left_str}--{right_str}") // solid line
        } else {
            format!("{left_str}..{right_str}") // dashed line
        };
        // append the label if the relationship has one
        if let Some(label) = self.label.as_deref() {
            relationship_str += &format!(" : \"{label}\"");
        }
        write!(f, "{relationship_str}")
    }
}

// =========================
// EntityId tests
// =========================
#[cfg(test)]
mod tests {

    use super::super::*;
    use super::*;

    const ALBUM_ID: &str = "ALBUM";
    const SONG_ID: &str = "SONG";

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

    #[test]
    fn test_display_identifying_without_a_label() {
        // arrange
        let relationship = Relationship::new(
            ALBUM_ID,
            SONG_ID,
            Cardinality::ZeroOrOne,
            Cardinality::ZeroOrMore,
        );
        let wanted = "ALBUM |o--o{ SONG";
        // act
        let got = relationship.to_string();
        // assert
        assert_eq!(got, wanted);
    }

    #[test]
    fn test_display_non_identifying_with_a_label() {
        // arrange
        let label = "includes";
        let relationship = Relationship::new(
            ALBUM_ID,
            SONG_ID,
            Cardinality::ExactlyOne,
            Cardinality::OneOrMore,
        )
        .as_non_identifying()
        .with_label(label);
        let wanted = "ALBUM ||..|{ SONG : \"includes\"";
        // act
        let got = relationship.to_string();
        // assert
        assert_eq!(got, wanted);
    }
}
