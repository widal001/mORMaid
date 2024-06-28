pub mod entity;
pub mod relationship;

pub struct ERD {
    pub title: Option<String>,
}

impl ERD {
    pub fn new() -> Self {
        ERD { title: None }
    }
}
