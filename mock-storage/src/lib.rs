use grid_core::GridStorage;
use std::collections::HashMap;

pub struct MockStorage;

impl MockStorage {
    pub fn new() -> Self {
        Self {}
    }
}

impl GridStorage for MockStorage {}
