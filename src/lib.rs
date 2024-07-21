#![warn(clippy::pedantic)]
#![warn(clippy::style)]
#![warn(clippy::perf)]
#![warn(clippy::cargo)]

pub mod erd;
pub mod req;
mod utils;

#[cfg(test)]
mod tests {
    use crate::erd::{Attribute, Cardinality, Entity, Relationship, ERD};
    use crate::req;

    #[test]
    fn create_erd() {
        // create entities with attributes and other details
        let album_table = Entity::new("ALBUM")
            .with_alias("album")
            .with_attribute(Attribute::new("int", "albumId").as_primary_key())
            .with_attribute(Attribute::new("str", "title"));

        // such as foreign key constraints and comments
        let song_table = Entity::new("SONG")
            .with_alias("song")
            .with_attribute(Attribute::new("int", "songId").as_primary_key())
            .with_attribute(Attribute::new("int", "albumId").as_foreign_key())
            .with_attribute(Attribute::new("int", "title"))
            .with_attribute(
                Attribute::new("int", "plays")
                    .with_comment("Number of times the song has been played"),
            );

        // create a relationship between previously created entities
        let album_songs = Relationship::new(
            album_table.id.as_str(),
            song_table.id.as_str(),
            Cardinality::ExactlyOne,
            Cardinality::OneOrMore,
        )
        .with_label("includes");

        // create a relationship between an existing entity and a new entity
        // that will be automatically created on insertion
        let album_artists = Relationship::new(
            album_table.id.as_str(),
            "ARTIST",
            Cardinality::OneOrMore,
            Cardinality::OneOrMore,
        )
        .as_non_identifying();

        // create the diagram and insert the entities and relationships
        // note that the "ARTIST" entity will be created with the relationship
        let diagram = ERD::new()
            .with_entity(album_table)
            .with_entity(song_table)
            .with_relationship(album_songs)
            .with_relationship(album_artists);
        println!("{diagram}");
    }

    #[test]
    fn create_requirement_diagram() {
        // create a constant value for verification method
        const METHOD: req::VerifyMethod = req::VerifyMethod::Test;
        // create elements and requirements
        let search = req::Element::new("search", "release").with_docref("releases/0.1.1/search");
        let requirements = vec![
            req::Requirement::new(req::RequirementType::Functional, "feature_1", "1.1.1")
                .with_risk(req::Risk::High)
                .with_text("Test feature 1")
                .with_verify_method(METHOD),
            req::Requirement::new(req::RequirementType::Functional, "feature_2", "1.1.2")
                .with_risk(req::Risk::Medium)
                .with_text("Test feature 2")
                .with_verify_method(METHOD),
            req::Requirement::new(req::RequirementType::Performance, "speed", "1.1.3")
                .with_risk(req::Risk::Low)
                .with_text("Test performance")
                .with_verify_method(METHOD),
        ];
        // create the diagram with the search element
        let mut diagram = req::RequirementDiagram::new().with_element(search);
        // add the requirements to the diagram and relate each to the search element
        for req in requirements {
            let relation =
                req::Relationship::new("search", &req.name, req::RelationshipType::Satisfies);
            diagram.add_requirement(req);
            diagram.add_relationship(relation);
        }
        // export to mermaid syntax
        println!("{diagram}");
    }
}
