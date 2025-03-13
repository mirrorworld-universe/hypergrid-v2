use crate::Storage;
use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MockStorage {
    db: HashMap<String, u64>,
    // db: HashMap<String, u64>,
}

impl MockStorage {
    pub fn new() -> Self {
        Self { db: HashMap::new() }
    }
}

#[async_trait::async_trait]
impl Storage for MockStorage {
    type Key = String;
    type Value = u64;

    async fn store_account(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value> {
        self.db.insert(key, value)
    }

    async fn get_account(&self, key: &Self::Key) -> Option<&Self::Value> {
        self.db.get(key)
    }
}
