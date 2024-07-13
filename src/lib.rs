#![warn(clippy::pedantic)]
#![warn(clippy::style)]
#![warn(clippy::perf)]
#![warn(clippy::cargo)]
pub mod erd;

#[cfg(test)]
mod tests {
    use crate::erd::{Attribute, Cardinality, Entity, Relationship, ERD};

    #[test]
    fn it_works() {
        // create Entities with attributes and other details
        let album_table = Entity::new("PRODUCT")
            .with_alias("product")
            .with_attribute(Attribute::new("int", "albumId").as_primary_key())
            .with_attribute(Attribute::new("str", "title"));

        let song_table = Entity::new("SONG")
            .with_alias("product")
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

        let diagram = ERD::new()
            .with_entity(album_table)
            .with_entity(song_table)
            .with_relationship(album_songs)
            .with_relationship(album_artists);
        println!("{diagram}");
    }
}
