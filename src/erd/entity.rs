// ==================================================================
// Entity struct and implementation
// ==================================================================

use std::fmt;

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

impl fmt::Display for Entity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // format entity id
        let mut entity_str = format!("{}", self.id);
        // format the alias if it exists
        if let Some(alias) = self.alias.as_deref() {
            entity_str += &format!("[\"{}\"]", alias);
        }
        write!(f, "{}", entity_str)
    }
}

// ==================================================================
// Attribute struct and implementation
// ==================================================================
pub struct Attribute {
    pub attr_type: String,
    pub name: String,
    pub key: KeyConstraints,
    pub comment: Option<String>,
}
impl Attribute {
    pub fn new(attr_type: &str, name: &str) -> Self {
        Attribute {
            attr_type: attr_type.to_string(),
            name: name.to_string(),
            key: KeyConstraints::default(),
            comment: None,
        }
    }

    //
    pub fn with_comment(mut self, comment: &str) -> Self {
        self.comment = Some(comment.to_string());
        self
    }

    pub fn as_primary_key(mut self) -> Self {
        self.key.is_primary = true;
        self
    }

    pub fn as_foreign_key(mut self) -> Self {
        self.key.is_foreign = true;
        self
    }

    pub fn as_unique(mut self) -> Self {
        self.key.is_unique = true;
        self
    }

    pub fn has_constraints(&self) -> bool {
        self.key.is_primary || self.key.is_foreign || self.key.is_unique
    }
}

impl fmt::Display for Attribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // format the required fields
        let mut attr_str = format!("{} {}", self.attr_type, self.name);
        // optionally add key constraints
        if self.has_constraints() {
            attr_str += &format!(" {}", self.key);
        }
        // optionally add comment
        if let Some(comment) = self.comment.as_deref() {
            attr_str += &format!(" \"{}\"", comment);
        }
        write!(f, "{}", attr_str)
    }
}

// ==================================================================
// KeyConstraints struct and implementation
// ==================================================================

#[derive(Debug)]
enum ConstraintCombo {
    None,
    PrimaryKey,
    ForeignKey,
    UniqueKey,
    PrimaryForeignKey,
    PrimaryUniqueKey,
    ForeignUniqueKey,
    PrimaryForeignUniqueKey,
}

impl ConstraintCombo {
    fn from_bools(is_primary: bool, is_foreign: bool, is_unique: bool) -> Self {
        match (is_primary, is_foreign, is_unique) {
            (false, false, false) => ConstraintCombo::None,
            (true, false, false) => ConstraintCombo::PrimaryKey,
            (false, true, false) => ConstraintCombo::ForeignKey,
            (false, false, true) => ConstraintCombo::UniqueKey,
            (true, true, false) => ConstraintCombo::PrimaryForeignKey,
            (true, false, true) => ConstraintCombo::PrimaryUniqueKey,
            (false, true, true) => ConstraintCombo::ForeignUniqueKey,
            (true, true, true) => ConstraintCombo::PrimaryForeignUniqueKey,
        }
    }
}

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

    fn to_combo(&self) -> ConstraintCombo {
        ConstraintCombo::from_bools(self.is_primary, self.is_foreign, self.is_unique)
    }
}

impl fmt::Display for KeyConstraints {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // format the required fields
        let key_str = match self.to_combo() {
            ConstraintCombo::None => "",
            ConstraintCombo::PrimaryKey => "PK",
            ConstraintCombo::ForeignKey => "FK",
            ConstraintCombo::UniqueKey => "UK",
            ConstraintCombo::PrimaryForeignKey => "PK, FK",
            ConstraintCombo::PrimaryUniqueKey => "PK, UK",
            ConstraintCombo::ForeignUniqueKey => "FK, UK",
            ConstraintCombo::PrimaryForeignUniqueKey => "PK, FK, UK",
        };
        // optionally add key constraints
        write!(f, "{}", key_str)
    }
}

// ==================================================================
// Entity tests
// ==================================================================

#[cfg(test)]
mod tests {
    use super::*;

    const ENTITY_ID: &str = "ALBUM";
    const ALIAS: &str = "album_table";
    const ATTR_NAME: &str = "title";
    const ATTR_TYPE: &str = "string";

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
            // act
            let entity = Entity::new(ENTITY_ID).with_alias(ALIAS);
            // assert
            assert_eq!(entity.alias, Some(ALIAS.to_string()));
        }

        #[test]
        fn test_add_attribute() {
            // act
            let entity = Entity::new(ENTITY_ID).add_attribute(Attribute::new(ATTR_TYPE, ATTR_NAME));
            // assert
            assert_eq!(entity.attributes.len(), 1); // entity has one attribute
            assert_eq!(entity.attributes[0].name, ATTR_NAME); // attr name matches
        }

        #[test]
        fn test_display_without_attributes_or_alias() {
            // arrange
            let entity = Entity::new(ENTITY_ID);
            let wanted = ENTITY_ID;
            // act
            let got = entity.to_string();
            // assert
            assert_eq!(got, wanted);
        }

        #[test]
        fn test_display_with_alias() {
            // arrange
            let entity = Entity::new(ENTITY_ID).with_alias(ALIAS);
            let wanted = format!("ALBUM[\"album_table\"]");
            // act
            let got = entity.to_string();
            // assert
            assert_eq!(got, wanted);
        }
    }

    // =========================
    // Attribute tests
    // =========================
    mod attribute_tests {
        use super::*;
        #[test]
        fn test_create_without_comment_or_key_constraints() {
            // act
            let attr = Attribute::new(ATTR_TYPE, ATTR_NAME);
            // assert
            assert_eq!(attr.attr_type, ATTR_TYPE);
            assert_eq!(attr.name, ATTR_NAME);
            assert_eq!(attr.key.is_primary, false);
            assert_eq!(attr.key.is_foreign, false);
            assert_eq!(attr.key.is_unique, false);
        }

        #[test]
        fn test_create_as_primary_key() {
            // act
            let attr = Attribute::new(ATTR_TYPE, ATTR_NAME).as_primary_key();
            // assert
            assert_eq!(attr.key.is_primary, true);
        }

        #[test]
        fn test_create_as_foreign_key() {
            // act
            let attr = Attribute::new(ATTR_TYPE, ATTR_NAME).as_foreign_key();
            // assert
            assert_eq!(attr.key.is_foreign, true);
        }

        #[test]
        fn test_create_as_unique() {
            // act
            let attr = Attribute::new(ATTR_TYPE, ATTR_NAME).as_unique();
            // assert
            assert_eq!(attr.key.is_unique, true);
        }

        #[test]
        fn test_create_with_comment() {
            // arrange
            let comment = "Unique identifier for the product";
            // act
            let attr = Attribute::new(ATTR_TYPE, ATTR_NAME).with_comment(comment);
            // assert
            assert_eq!(attr.comment, Some(comment.to_string()));
        }

        #[test]
        fn test_display_without_key_or_comment() {
            // arrange
            let wanted = format!("{ATTR_TYPE} {ATTR_NAME}");
            let attr = Attribute::new(ATTR_TYPE, ATTR_NAME);
            // act
            let got = attr.to_string();
            // assert
            assert_eq!(got, wanted)
        }

        #[test]
        fn test_display_with_one_key_constraint() {
            // arrange
            let wanted = format!("{ATTR_TYPE} {ATTR_NAME} PK");
            let attr = Attribute::new(ATTR_TYPE, ATTR_NAME).as_primary_key();
            // act
            let got = attr.to_string();
            // assert
            assert_eq!(got, wanted)
        }

        #[test]
        fn test_display_with_multiple_key_constraints() {
            // arrange
            let wanted = format!("{ATTR_TYPE} {ATTR_NAME} PK, FK, UK");
            let attr = Attribute::new(ATTR_TYPE, ATTR_NAME)
                .as_primary_key()
                .as_foreign_key()
                .as_unique();
            // act
            let got = attr.to_string();
            // assert
            assert_eq!(got, wanted)
        }

        #[test]
        fn test_display_with_comment() {
            // arrange
            let comment = "comment about album";
            let wanted = format!("{ATTR_TYPE} {ATTR_NAME} \"{comment}\"");
            let attr = Attribute::new(ATTR_TYPE, ATTR_NAME).with_comment(comment);
            // act
            let got = attr.to_string();
            // assert
            assert_eq!(got, wanted)
        }
    }
}
