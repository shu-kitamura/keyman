#[derive(Debug, PartialEq)]
struct Key {
    row_key: String,
    column: String,
    timestamp: i64,
    is_deleted: bool,
}

impl Key {
    fn new(row_key: &str, column: &str, timestamp: i64, is_deleted: bool) -> Self {
        Key {
            row_key: String::from(row_key),
            column: String::from(column),
            timestamp,
            is_deleted,
        }
    } 
}

#[cfg(test)]
mod tests {
    use crate::types::key::Key;

    #[test]
    fn test_new() {
        let key: Key = Key {
            row_key: "test_key".to_string(),
            column: "test_col".to_string(),
            timestamp: 1743077075,
            is_deleted: false
        };
        assert_eq!(key, Key::new("test_key", "test_col", 1743077075, false));
    }
}