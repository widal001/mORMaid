use std::collections::HashMap;
use std::fmt;

pub mod element;
pub mod relationship;
pub mod requirement;

use crate::utils;
pub use element::Element;
pub use relationship::{Relationship, RelationshipType};
pub use requirement::{Requirement, RequirementType, Risk, VerifyMethod};

#[must_use]
#[derive(Default)]
pub struct RequirementDiagram {
    pub requirements: HashMap<String, Requirement>,
    pub elements: HashMap<String, Element>,
    pub relationships: Vec<Relationship>,
}
impl RequirementDiagram {
    pub fn new() -> Self {
        RequirementDiagram {
            requirements: HashMap::new(),
            elements: HashMap::new(),
            relationships: Vec::new(),
        }
    }
}

// ============================================================
// Implement RequirementDiagram methods to manage elements
// ============================================================
impl RequirementDiagram {
    /// Add an element to `RequirementDiagram.elements`, keyed by the element's name.
    pub fn add_element(&mut self, element: Element) {
        self.elements.insert(element.name.to_string(), element);
    }

    /// Add an element to the `RequirementDiagram` on creation by chaining with [`RequirementDiagram::new()`].
    pub fn with_element(mut self, element: Element) -> Self {
        self.add_element(element);
        self
    }

    /// Try to find an element in the `RequirementDiagram` using its name.
    #[must_use]
    pub fn get_element_by_name(&self, name: &str) -> Option<&Element> {
        self.elements.get(name)
    }
}

// implement the Display trait
impl fmt::Display for RequirementDiagram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // initialize the erDiagram
        let mut out_str = "requirementDiagram".to_string();

        // append elements if the diagram has them
        if !self.elements.is_empty() {
            out_str = utils::append_items(out_str, self.elements.values(), "Elements", 4);
        }

        // append requirements if the diagram has them
        if !self.requirements.is_empty() {
            out_str = utils::append_items(out_str, self.requirements.values(), "Requirements", 4);
        }

        // append relationships if the diagram has them
        if !self.relationships.is_empty() {
            out_str = utils::append_items(out_str, &self.relationships, "Relationships", 4);
        }
        write!(f, "{out_str}")
    }
}

// ============================================================
// Implement RequirementDiagram methods to manage requirements
// ============================================================
impl RequirementDiagram {
    /// Add a requirement to `RequirementDiagram.requirements`, keyed by the requirement's name.
    pub fn add_requirement(&mut self, req: Requirement) {
        self.requirements.insert(req.name.to_string(), req);
    }

    /// Add a requirement to the `RequirementDiagram` on creation by chaining with [`RequirementDiagram::new()`].
    pub fn with_requirement(mut self, req: Requirement) -> Self {
        self.add_requirement(req);
        self
    }

    /// Try to find a requirement in the `RequirementDiagram` using its name.
    #[must_use]
    pub fn get_requirement_by_name(&self, name: &str) -> Option<&Requirement> {
        self.requirements.get(name)
    }
}

// ============================================================
// Implement RequirementDiagram methods to manage relationships
// ============================================================
impl RequirementDiagram {
    /// Add a relationship to `RequirementDiagram.relationships`.
    ///
    /// # Panics
    /// This method will panic if a developer tries to insert a relationship
    /// that references an element or requirement not found in the diagram.
    pub fn add_relationship(&mut self, relationship: Relationship) {
        // Ensure that both the source and target exist in the RequirementDiagram
        let src = relationship.source.as_str();
        assert!(
            self.found_in_diagram(src),
            "{src} isn't found in the list of elements or requirements"
        );
        let tgt = relationship.target.as_str();
        assert!(
            self.found_in_diagram(tgt),
            "{tgt} isn't found in the list of elements or requirements"
        );
        // Then add the relationship to the RequirementDiagram
        self.relationships.push(relationship);
    }

    /// Add a relationship to the `RequirementDiagram` on creation by chaining with [`RequirementDiagram::new()`].
    pub fn with_relationship(mut self, relationship: Relationship) -> Self {
        self.add_relationship(relationship);
        self
    }

    // Check if a given element or requirement exists with the name provided
    fn found_in_diagram(&self, name: &str) -> bool {
        self.elements.contains_key(name) || self.requirements.contains_key(name)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    const ELEMENT_NAME: &str = "foo";
    const ELEMENT_KIND: &str = "brief";
    const REQ_ID: &str = "1.1.1";
    const REQ_NAME: &str = "milestone";
    const REQ_KIND: RequirementType = RequirementType::Default;

    mod creation_tests {

        use super::*;

        #[test]
        fn create_empty_diagram() {
            // act
            let got = RequirementDiagram::new();
            // assert
            assert!(got.requirements.is_empty());
            assert!(got.elements.is_empty());
            assert!(got.relationships.is_empty());
        }

        #[test]
        fn add_element_to_existing_diagram() {
            // arrange
            let mut diagram = RequirementDiagram::new();
            // act
            diagram.add_element(Element::new(ELEMENT_NAME, ELEMENT_KIND));
            // assert
            let element = diagram
                .get_element_by_name(ELEMENT_NAME)
                .expect("Expected element but got None");
            assert_eq!(element.name, ELEMENT_NAME);
        }

        #[test]
        fn create_diagram_with_element() {
            // act
            let diagram =
                RequirementDiagram::new().with_element(Element::new(ELEMENT_NAME, ELEMENT_KIND));
            // assert
            let element = diagram
                .get_element_by_name(ELEMENT_NAME)
                .expect("Expected element but got None");
            assert_eq!(element.name, ELEMENT_NAME);
        }

        #[test]
        fn add_requirement_to_existing_diagram() {
            // arrange
            let mut diagram = RequirementDiagram::new();
            // act
            diagram.add_requirement(Requirement::new(REQ_KIND, REQ_NAME, REQ_ID));
            // assert
            let requirement = diagram
                .get_requirement_by_name(REQ_NAME)
                .expect("Expected requirement but got None");
            assert_eq!(requirement.name, REQ_NAME);
        }

        #[test]
        fn create_diagram_with_requirement() {
            // act
            let diagram = RequirementDiagram::new()
                .with_requirement(Requirement::new(REQ_KIND, REQ_NAME, REQ_ID));
            // assert
            let requirement = diagram
                .get_requirement_by_name(REQ_NAME)
                .expect("Expected requirement but got None");
            assert_eq!(requirement.name, REQ_NAME);
        }

        #[test]
        fn add_valid_relationship_to_existing_diagram() {
            // arrange
            let mut diagram = RequirementDiagram::new()
                .with_element(Element::new(ELEMENT_NAME, ELEMENT_KIND))
                .with_requirement(Requirement::new(REQ_KIND, REQ_NAME, REQ_ID));
            // act
            println!("{:?}", diagram.elements.keys());
            diagram.add_relationship(Relationship::new(
                ELEMENT_NAME,
                REQ_NAME,
                RelationshipType::Satisfies,
            ));
            // assert
            assert_eq!(diagram.relationships.len(), 1);
        }

        #[test]
        #[should_panic = "Fake isn't found in the list of elements or requirements"]
        fn add_invalid_relationship_should_panic() {
            // arrange
            let mut diagram = RequirementDiagram::new();
            // act
            diagram.add_relationship(Relationship::new(
                "Fake",
                "bar",
                RelationshipType::Satisfies,
            ));
        }
    }

    mod display_tests {
        use super::*;

        #[test]
        fn display_empty_diagram() {
            // arrange
            let wanted = "requirementDiagram";
            // act
            let got = RequirementDiagram::new().to_string();
            // assert
            assert_eq!(got, wanted);
        }

        #[test]
        fn display_diagram_with_element_and_requirement() {
            // arrange
            let wanted = concat!(
                "requirementDiagram\n",
                "    %% Elements start\n",
                "    element foo {\n",
                "        type: \"brief\"\n",
                "    }\n",
                "    %% Elements end\n",
                "    %% Requirements start\n",
                "    requirement milestone {\n",
                "        id: 1.1.1\n",
                "        risk: Low\n",
                "    }\n",
                "    %% Requirements end",
            );
            // act
            let got = RequirementDiagram::new()
                .with_element(Element::new(ELEMENT_NAME, ELEMENT_KIND))
                .with_requirement(Requirement::new(REQ_KIND, REQ_NAME, REQ_ID).with_risk(Risk::Low))
                .to_string();
            // assert
            assert_eq!(got, wanted, "\n\nGot:\n{got}\n\nWanted:\n{wanted}");
        }

        #[test]
        fn display_diagram_with_all_components() {
            // arrange
            let wanted = concat!(
                "requirementDiagram\n",
                "    %% Elements start\n",
                "    element foo {\n",
                "        type: \"brief\"\n",
                "    }\n",
                "    %% Elements end\n",
                "    %% Requirements start\n",
                "    requirement milestone {\n",
                "        id: 1.1.1\n",
                "        risk: Low\n",
                "    }\n",
                "    %% Requirements end\n",
                "    %% Relationships start\n",
                "    foo - copies -> milestone\n",
                "    %% Relationships end",
            );
            // act
            let got = RequirementDiagram::new()
                .with_element(Element::new(ELEMENT_NAME, ELEMENT_KIND))
                .with_requirement(Requirement::new(REQ_KIND, REQ_NAME, REQ_ID).with_risk(Risk::Low))
                .with_relationship(Relationship::new(
                    ELEMENT_NAME,
                    REQ_NAME,
                    RelationshipType::Copies,
                ))
                .to_string();
            // assert
            assert_eq!(got, wanted, "\n\nGot:\n{got}\n\nWanted:\n{wanted}");
        }
    }
}
