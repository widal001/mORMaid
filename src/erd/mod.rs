use std::collections::HashMap;

pub mod entity;
pub mod relationship;

// ==================================================================
// EntityId struct and implementation
// ==================================================================

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct EntityId(String);

impl EntityId {
    pub fn new(s: String) -> Self {
        EntityId(s)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for EntityId {
    fn from(s: &str) -> Self {
        EntityId(s.to_string())
    }
}
// ================================================================
// ERD struct and implementation
// ================================================================
pub struct ERD {
    pub title: Option<String>,
    pub entities: HashMap<EntityId, entity::Entity>,
    pub relationships: Vec<relationship::Relationship>,
}
impl ERD {
    pub fn new() -> Self {
        ERD {
            title: None,
            entities: HashMap::new(),
            relationships: Vec::new(),
        }
    }
}

// ========================================
// Implement ERD methods to manage entities
// ========================================
impl ERD {
    /// Add an entity to `ERD.entities`, keyed by the entity's id.
    pub fn add_entity(&mut self, entity: entity::Entity) {
        let id = EntityId::from(entity.id.as_str());
        self.entities.insert(id, entity);
    }

    /// Add an entity to the ERD on creation by chaining with [ERD::new()].
    pub fn with_entity(mut self, entity: entity::Entity) -> Self {
        self.add_entity(entity);
        self
    }

    /// Try to find an entity in the ERD using its id.
    pub fn get_entity_by_id(&self, id: &EntityId) -> Option<&entity::Entity> {
        self.entities.get(id)
    }

    /// If a entity doesn't exist in the ERD, create and insert it.
    pub fn create_entity_if_missing(&mut self, id: &EntityId) {
        if self.get_entity_by_id(id).is_none() {
            self.add_entity(entity::Entity::new(id.as_str()));
        }
    }
}

// =============================================
// Implement ERD methods to manage relationships
// =============================================
impl ERD {
    /// Add a relationship to `ERD.relationships`.
    ///
    /// This method also creates and adds the entities referenced in the relationship
    /// if they don't already exist in `ERD.entities`.
    pub fn add_relationship(&mut self, relationship: relationship::Relationship) {
        // Ensure that both the left and right entities exist in the ERD
        self.create_entity_if_missing(&relationship.left_id);
        self.create_entity_if_missing(&relationship.right_id);
        // Then add the relationship to the ERD
        self.relationships.push(relationship);
    }

    /// Add a relationship to the ERD on creation by chaining with [ERD::new()].
    pub fn with_relationship(mut self, relationship: relationship::Relationship) -> Self {
        self.add_relationship(relationship);
        self
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    const ALBUM_ID: &str = "ALBUM";
    const SONG_ID: &str = "SONG";

    // =========================
    // EntityId tests
    // =========================
    mod entity_id_tests {

        use super::*;

        #[test]
        fn entity_ids_with_same_string_are_equal() {
            // act
            let first_id = EntityId::from(ALBUM_ID);
            let second_id = EntityId::from(ALBUM_ID);
            // assert
            assert_eq!(first_id, second_id);
        }
    }

    mod erd_tests {

        use super::*;

        #[test]
        fn add_entity_after_creating_erd() {
            // arrange
            let mut erd = ERD::new();
            // act
            erd.add_entity(entity::Entity::new(ALBUM_ID));
            erd.add_entity(entity::Entity::new(SONG_ID));
            // assert
            assert_eq!(erd.entities.len(), 2);
            let product = erd.get_entity_by_id(&EntityId::from(ALBUM_ID));
            let tag = erd.get_entity_by_id(&EntityId::from(SONG_ID));
            assert!(product.is_some());
            assert!(tag.is_some());
        }

        #[test]
        fn create_erd_with_entities() {
            // act
            let erd = ERD::new()
                .with_entity(entity::Entity::new(ALBUM_ID))
                .with_entity(entity::Entity::new(SONG_ID));
            // assert
            assert_eq!(erd.entities.len(), 2);
            let album = erd.get_entity_by_id(&EntityId::from(ALBUM_ID));
            let song = erd.get_entity_by_id(&EntityId::from(SONG_ID));
            assert!(album.is_some());
            assert!(song.is_some());
        }

        #[test]
        fn add_relationship_for_existing_entities() {
            // arrange
            let mut erd = ERD::new()
                .with_entity(entity::Entity::new(ALBUM_ID))
                .with_entity(entity::Entity::new(SONG_ID));
            let entity_count_old = erd.entities.len();
            assert_eq!(entity_count_old, 2);
            // act
            erd.add_relationship(relationship::Relationship::new(
                ALBUM_ID,
                SONG_ID,
                relationship::Cardinality::ExactlyOne,
                relationship::Cardinality::OneOrMore,
            ));
            // assert
            assert_eq!(erd.relationships.len(), 1);
            let entity_count_new = erd.entities.len();
            assert_eq!(entity_count_new, entity_count_old)
        }

        #[test]
        // Entities referenced in a relationship should be added if they don't exist
        fn add_relationship_for_missing_entities() {
            // arrange
            let mut erd = ERD::new();
            let entity_count_old = erd.entities.len();
            assert_eq!(entity_count_old, 0);
            // act
            erd.add_relationship(relationship::Relationship::new(
                ALBUM_ID,
                SONG_ID,
                relationship::Cardinality::ExactlyOne,
                relationship::Cardinality::OneOrMore,
            ));
            // assert
            assert_eq!(erd.relationships.len(), 1);
            let entity_count_new = erd.entities.len();
            assert_eq!(entity_count_new, entity_count_old + 2);
        }

        #[test]
        fn create_erd_with_relationship() {
            // act
            let erd = ERD::new().with_relationship(relationship::Relationship::new(
                ALBUM_ID,
                SONG_ID,
                relationship::Cardinality::ExactlyOne,
                relationship::Cardinality::OneOrMore,
            ));
            // assert
            assert_eq!(erd.relationships.len(), 1);
            assert_eq!(erd.entities.len(), 2);
        }
    }
}
