pub mod models;

#[cfg(test)]
mod tests {
    use super::models::set::Set;

    #[test]
    fn test_set_add() {
        let mut set = Set::new();
        set.add(1);
        assert!(set.contains(&1));
    }
}
