use std::fmt;
#[derive(Debug, PartialEq)]
pub enum RelationshipType {
    Contains,
    Copies,
    Derives,
    Satisfies,
    Verifies,
    Refines,
    Traces,
}

impl fmt::Display for RelationshipType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let type_str = match self {
            RelationshipType::Contains => "contains",
            RelationshipType::Copies => "copies",
            RelationshipType::Derives => "derives",
            RelationshipType::Satisfies => "satisfies",
            RelationshipType::Verifies => "verifies",
            RelationshipType::Refines => "refines",
            RelationshipType::Traces => "traces",
        };
        write!(f, "{type_str}")
    }
}

#[must_use]
pub struct Relationship {
    pub source: String,
    pub target: String,
    pub kind: RelationshipType,
}

impl Relationship {
    // create a new relationship
    pub fn new(source: &str, target: &str, kind: RelationshipType) -> Self {
        Relationship {
            source: source.to_string(),
            target: target.to_string(),
            kind,
        }
    }
}

impl fmt::Display for Relationship {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out_str = format!(
            "{} - {} -> {}",
            self.source,
            self.kind.to_string(),
            self.target,
        );
        write!(f, "{out_str}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SOURCE: &str = "Foo";
    const TARGET: &str = "Bar";
    const KIND: RelationshipType = RelationshipType::Contains;

    #[test]
    fn create_relationship() {
        // act
        let got = Relationship::new(SOURCE, TARGET, KIND);
        // assert
        assert_eq!(got.source, SOURCE.to_string());
        assert_eq!(got.target, TARGET.to_string());
        assert_eq!(got.kind, KIND);
    }

    #[test]
    fn display_relationship() {
        // arrange
        let wanted = "Foo - contains -> Bar";
        // act
        let got = Relationship::new(SOURCE, TARGET, KIND).to_string();
        // assert
        assert_eq!(got, wanted);
    }
}
