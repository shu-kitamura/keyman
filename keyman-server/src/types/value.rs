#[derive(Debug, PartialEq)]
struct Value {
    data: Vec<u8>,
}

impl Value {
    fn new(data: Vec<u8>) -> Self {
        Value { data }
    }
}

impl From<&[u8]> for Value {
    fn from(value: &[u8]) -> Self {
        Value::new(Vec::from(value))
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::from(value.as_bytes())
    }
}

#[cfg(test)]
mod tests {
    use crate::types::value::Value;

    #[test]
    fn test_new() {
        let val: Value = Value {
            data: vec![116, 101, 116, 116]
        };
        assert_eq!(val, Value::new(vec![116, 101, 116, 116]))
    }

    #[test]
    fn test_from() {
        let val: Value = Value {
            data: vec![116, 101, 115, 116]
        };
        assert_eq!(val, Value::from(&vec![116, 101, 115, 116][..]));
        assert_eq!(val, Value::from("test"));
    }
}