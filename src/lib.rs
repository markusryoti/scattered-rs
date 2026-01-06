use std::collections::HashMap;

pub struct Store {
    db: HashMap<String, String>,
}

impl Store {
    pub fn new() -> Self {
        Store { db: HashMap::new() }
    }

    pub fn put(&mut self, key: String, value: String) {
        self.db.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.db.get(key).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_and_retrieve() {
        let mut store = Store::new();

        assert_eq!(store.put("key".to_string(), "value".to_string()), ());
        assert_eq!(store.get("key"), Some("value".to_string()));
    }
}
