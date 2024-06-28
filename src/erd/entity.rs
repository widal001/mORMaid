// ==================================================================
// Entity struct and implementation
// ==================================================================
pub struct Entity {
    /// The id for the entity in the ERD.
    ///
    /// The id is used to specify relationships between entities. It can
    /// only consist of letters and numbers and will be cast to uppercase.
    /// If no alias is passed, this id will also be the name displayed in
    /// the rendered version of the ERD.
    pub id: String,
    /// The alias is the name displayed in the rendered version of the ERD.
    ///
    /// Unlike id, the alias can be multiple words separated by spaces.
    pub alias: Option<String>,
    /// The alias is the name displayed in the rendered version of the ERD.
    ///
    /// For many ERDs this is the table name. Unlike id, the alias can be
    /// multiple words separated by spaces.
    pub attributes: Vec<Attribute>, // Use Vec for a dynamic array
}
impl Entity {
    /// Create a new Entity with a given id
    pub fn new(id: &str) -> Self {
        Entity {
            id: id.to_string(),
            alias: None,
            attributes: Vec::new(), // Initialize with None
        }
    }

    /// Chain with Entity::new() to create an entity with an alias
    pub fn with_alias(mut self, alias: &str) -> Self {
        self.alias = Some(alias.to_string());
        self
    }

    /// Add an attribute to an entity
    pub fn add_attribute(mut self, attribute: Attribute) -> Self {
        self.attributes.push(attribute);
        self
    }
}

// ==================================================================
// Attribute struct and implementation
// ==================================================================
#[derive(Debug)]
pub struct Attribute {
    pub attr_type: String,
    pub name: String,
    pub constraints: KeyConstraints,
    pub comment: Option<String>,
}
impl Attribute {
    pub fn new(attr_type: &str, name: &str) -> Self {
        Attribute {
            attr_type: attr_type.to_string(),
            name: name.to_string(),
            constraints: KeyConstraints::default(),
            comment: None,
        }
    }

    pub fn with_comment(mut self, comment: &str) -> Self {
        self.comment = Some(comment.to_string());
        self
    }

    pub fn as_primary_key(mut self) -> Self {
        self.constraints.is_primary = true;
        self
    }

    pub fn as_foreign_key(mut self) -> Self {
        self.constraints.is_foreign = true;
        self
    }

    pub fn as_unique(mut self) -> Self {
        self.constraints.is_unique = true;
        self
    }
}

// ==================================================================
// KeyConstraints struct and implementation
// ==================================================================
#[derive(Debug)]
pub struct KeyConstraints {
    pub is_primary: bool,
    pub is_foreign: bool,
    pub is_unique: bool,
}
impl KeyConstraints {
    fn default() -> Self {
        KeyConstraints {
            is_primary: false,
            is_foreign: false,
            is_unique: false,
        }
    }
}

// ==================================================================
// Entity tests
// ==================================================================

#[cfg(test)]
mod tests {
    use super::*;

    const ENTITY_ID: &str = "PRODUCT";

    // =========================
    // Entity tests
    // =========================
    mod entity_tests {
        use super::*;
        #[test]
        fn test_create_without_alias_or_attributes() {
            // act
            let entity = Entity::new(ENTITY_ID);
            // assert
            assert_eq!(entity.id, ENTITY_ID); // id is set correctly
            assert_eq!(entity.alias, None); // alias is unset
            assert_eq!(entity.attributes.len(), 0); // entity has no attributes
        }

        #[test]
        fn test_create_with_alias() {
            // arrange
            let alias = "product_table";
            // act
            let entity = Entity::new(ENTITY_ID).with_alias(alias);
            // assert
            assert_eq!(entity.alias, Some(alias.to_string()));
        }

        #[test]
        fn test_add_attribute() {
            // arrange
            let alias = "product_table";
            let attr_type = "string";
            let attr_name = "product_id";
            // act
            let entity = Entity::new(ENTITY_ID)
                .with_alias(alias)
                .add_attribute(Attribute::new(attr_type, attr_name));
            // assert
            assert_eq!(entity.attributes.len(), 1); // entity has one attribute
            assert_eq!(entity.attributes[0].name, attr_name); // attr name matches
        }
    }

    // =========================
    // Attribute tests
    // =========================
    mod attribute_tests {
        use super::*;
        #[test]
        fn test_create_without_alias_or_attributes() {
            // arrange
            let attr_type: &str = "string";
            let attr_name: &str = "product_id";
            // act
            let attr = Attribute::new(attr_type, attr_name);
            // assert
            assert_eq!(attr.attr_type, attr_type);
            assert_eq!(attr.name, attr_name);
        }
    }
}
