pub mod erd;

#[cfg(test)]
mod tests {
    use crate::erd::entity;

    #[test]
    fn it_works() {
        let product = entity::Entity::new("PRODUCT");
        assert_eq!(product.id, "PRODUCT");
    }
}
